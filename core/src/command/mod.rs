//! Commands provide the primary interaction between the user and the backend: text in, text out.
//! A second dimension exists in that the commands are queried as the user types to provide
//! autocomplete suggestions.
//!
//! This simple concept belies a tremendous amount of complexity and redundancy in implementation,
//! especially in more complex syntaxes. A series of parser combinators is used to abstract away
//! most of the direct interaction with text, leaving each individual command to interact with the
//! user's input at a higher level.
//!
//! At the root of the process is the `Token`: a pattern returned by the `token()` method that
//! matches one or more input words. In the event that the user input matches the `Token` exactly
//! or partially (in the case of autocomplete), the parsed result (`TokenMatch`/`FuzzyMatch`) will
//! be dispatched to `autocomplete()`/`get_priority()` for further processing. This additional
//! processing step can be used to filter out false positives before displaying a match to the
//! user.
//!
//! The preferred way of parsing token trees is using markers: a `u8` value assigned to a Token
//! upon creation which persists through the TokenMatch data provided to
//! `autocomplete()`/`get_priority()`. The `initiative_macros::TokenMarker` derive macro is a
//! convenience tool that simplifies using enums for this purpose. It effectively abstracts away a
//! whole lot of `Marker::Foo as u8`. Rather than manually climbing through the token tree in
//! search of a particular marker, the `Token::find_markers()` method can be used to jump directly
//! (and recursively) to the token(s) of interest.
//!
//! If `get_priority()` returns `Some(x)`, the dispatcher (`run()`) will choose which command to
//! execute based on the returned priority: `CommandPriority::Canonical` matches will always be
//! executed, while `CommandPriority::Fuzzy` matches will run if no other matches are present. In
//! the event that additional fuzzy matches are present, all matched phrases will be displayed to
//! the user in their canonical form.
//!
//! # Creating a command
//!
//! Each command exists in its own module and implements the `Command` trait. Non-trivial command
//! mods usually also have a `Marker` enum that has the `TokenMarker` derive macro applied. New
//! commands should then be added to the `CommandList` enum in this module. That enum's derive
//! macro will handle the rest of the wiring.

/// All of the classes needed to implement a new command or token type.
#[expect(unused_imports)]
mod prelude {
    pub use super::token::{FuzzyMatch, MatchMeta, Token, TokenMatch, TokenType};
    pub use super::{Command, CommandPriority};
    pub use crate::app::{AppMeta, AutocompleteSuggestion};
}

mod about;
mod create;
mod save;

mod token;

use std::fmt::{self, Write};
use std::iter;

use crate::app::{
    AppMeta, Autocomplete, AutocompleteSuggestion, CommandMatches, ContextAwareParse, Runnable,
};
use initiative_macros::CommandList;

use token::{FuzzyMatch, Token, TokenMatch};

use async_stream::stream;
use async_trait::async_trait;
use futures::prelude::*;

pub trait Command {
    /// Return a single Token representing the command's syntax. If multiple commands are possible,
    /// Token::Or can be used as a wrapper to cover the options.
    fn token(&self) -> Token;

    /// Convert a matched token into a suggestion to be displayed to the user. Note that this
    /// method is not async; any metadata that may be needed for the autocomplete should be fetched
    /// during the match_input step of the token and embedded in the match_meta property of the
    /// TokenMatch object.
    fn autocomplete(&self, fuzzy_match: FuzzyMatch, input: &str) -> Option<AutocompleteSuggestion>;

    /// Get the priority of the command with a given input. See CommandPriority for details.
    /// `None` will be interpreted as a non-match.
    fn get_priority(&self, token_match: &TokenMatch) -> Option<CommandPriority>;

    /// Run the command represented by a matched token, returning the success or failure output to
    /// be displayed to the user.
    async fn run(&self, token_match: TokenMatch, app_meta: &mut AppMeta) -> Result<String, String>;

    /// Get the canonical form of the provided token match. Returns None if the match is invalid.
    fn get_canonical_form_of(&self, token_match: &TokenMatch) -> Option<String>;
}

#[derive(Clone, CommandList, Debug)]
enum CommandList {
    About(about::About),
    Create(create::Create),
    Save(save::Save),
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CommandPriority {
    /// There should be no more than one canonical command per input, distinguished by a unique
    /// prefix. The canonical command will always run if matched. If fuzzy matches also exist, they
    /// will be indicated after the output of the canonical command.
    Canonical,

    /// There may be multiple fuzzy matches for a given input. If no canonical command exists AND
    /// only one fuzzy match is found, that match will run. If multiple fuzzy matches are found,
    /// the user will be prompted which canonical form they wish to run.
    Fuzzy,
}

/// Interfaces with the legacy command traits.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransitionalCommand {
    canonical: String,
}

impl TransitionalCommand {
    pub fn new<S>(canonical: S) -> Self
    where
        S: AsRef<str>,
    {
        Self {
            canonical: canonical.as_ref().to_string(),
        }
    }
}

#[async_trait(?Send)]
impl Runnable for TransitionalCommand {
    async fn run(self, _input: &str, app_meta: &mut AppMeta) -> Result<String, String> {
        run(&self.canonical, app_meta).await
    }
}

#[async_trait(?Send)]
impl ContextAwareParse for TransitionalCommand {
    async fn parse_input(input: &str, app_meta: &AppMeta) -> CommandMatches<Self> {
        let mut command_matches = CommandMatches::default();

        let commands_tokens: Vec<(&CommandList, Token)> = CommandList::get_all()
            .iter()
            .map(|c| (c, c.token()))
            .collect();

        {
            let mut match_streams = stream::SelectAll::default();

            // Indexing the array avoids lifetime issues that would occur with an iterator
            #[expect(clippy::needless_range_loop)]
            for i in 0..commands_tokens.len() {
                match_streams.push(
                    stream::repeat(commands_tokens[i].0)
                        .zip(commands_tokens[i].1.match_input(input, app_meta)),
                );
            }

            while let Some((command, fuzzy_match)) = match_streams.next().await {
                if let FuzzyMatch::Exact(token_match) = fuzzy_match {
                    if let Some(priority) = command.get_priority(&token_match) {
                        if let Some(canonical) = command.get_canonical_form_of(&token_match) {
                            if priority == CommandPriority::Canonical {
                                command_matches.push_canonical(Self { canonical });
                            } else {
                                command_matches.push_fuzzy(Self { canonical });
                            }
                        }
                    }
                }
            }
        }

        command_matches
    }
}

#[async_trait(?Send)]
impl Autocomplete for TransitionalCommand {
    async fn autocomplete(input: &str, app_meta: &AppMeta) -> Vec<AutocompleteSuggestion> {
        autocomplete(input, app_meta).await
    }
}

impl fmt::Display for TransitionalCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.canonical)
    }
}

pub async fn autocomplete(input: &str, app_meta: &AppMeta) -> Vec<AutocompleteSuggestion> {
    let mut suggestions: Vec<_> =
        stream::select_all(CommandList::get_all().iter().map(|command| {
            Box::pin(stream! {
                let token = command.token();
                for await token_match in token.match_input(input, app_meta) {
                    if !matches!(token_match, FuzzyMatch::Overflow(..)) {
                        if let Some(suggestion) = command.autocomplete(token_match, input) {
                            yield suggestion;
                        }
                    }
                }
            })
        }))
        .collect()
        .await;

    suggestions.sort();
    suggestions.truncate(10);
    suggestions
}

pub async fn run(input: &str, app_meta: &mut AppMeta) -> Result<String, String> {
    // The only reason this vec exists is to ensure that the Tokens referenced by TokenMatch et al
    // outlive their references.
    let commands_tokens: Vec<(&CommandList, Token)> = CommandList::get_all()
        .iter()
        .map(|c| (c, c.token()))
        .collect();

    let mut token_matches: Vec<(&CommandList, CommandPriority, TokenMatch)> = Vec::new();

    {
        let mut match_streams = stream::SelectAll::default();

        // Indexing the array avoids lifetime issues that would occur with an iterator
        #[expect(clippy::needless_range_loop)]
        for i in 0..commands_tokens.len() {
            match_streams.push(
                stream::repeat(commands_tokens[i].0)
                    .zip(commands_tokens[i].1.match_input(input, app_meta)),
            );
        }

        while let Some((command, fuzzy_match)) = match_streams.next().await {
            if let Some(token_match) = fuzzy_match.into_exact() {
                if let Some(priority) = command.get_priority(&token_match) {
                    token_matches.push((command, priority, token_match));
                }
            }
        }
    }

    token_matches.sort_by_key(|&(_, command_priority, _)| command_priority);

    match token_matches.len() {
        0 => return Err(format!("Unknown command: \"{}\"", input)),
        1 => {
            let (command, _, token_match) = token_matches.pop().unwrap();
            return command.run(token_match, app_meta).await;
        }
        _ => {} // continue
    }

    if token_matches[0].1 == CommandPriority::Canonical {
        assert_ne!(token_matches[1].1, CommandPriority::Canonical);

        let (command, _, token_match) = token_matches.remove(0);
        let result = command.run(token_match, app_meta).await;

        let mut iter = token_matches
            .iter()
            .take_while(|(_, command_priority, _)| command_priority == &CommandPriority::Fuzzy)
            .peekable();

        if iter.peek().is_none() {
            result
        } else {
            let f = |s| {
                iter
                    .filter_map(|(command, _, token_match)| command.get_canonical_form_of(token_match))
                    .fold(
                        format!("{}\n\n! There are other possible interpretations of this command. Did you mean:\n", s),
                        |mut s, c| { write!(s, "\n* `{}`", c).unwrap(); s }
                    )
            };

            match result {
                Ok(s) => Ok(f(s)),
                Err(s) => Err(f(s)),
            }
        }
    } else {
        let first_token_match = token_matches.remove(0);

        let mut iter =
            iter::once(&first_token_match)
                .chain(token_matches.iter().take_while(|(_, command_priority, _)| {
                    command_priority == &first_token_match.1
                }))
                .filter_map(|(command, _, token_match)| command.get_canonical_form_of(token_match))
                .peekable();

        if iter.peek().is_none() {
            Err(format!("Unknown command: \"{}\"", input))
        } else {
            Err(iter.fold(
                "There are several possible interpretations of this command. Did you mean:\n"
                    .to_string(),
                |mut s, c| {
                    write!(s, "\n* `{}`", c).unwrap();
                    s
                },
            ))
        }
    }
}