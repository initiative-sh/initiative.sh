/// All of the classes needed to implement a new command or token type.
#[cfg(not(feature = "integration-tests"))]
mod prelude;
#[cfg(feature = "integration-tests")]
pub mod prelude;

mod about;

mod token;

use std::fmt::{self, Write};
use std::iter;
use std::pin::Pin;

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
    fn get_priority(&self, token_match: &TokenMatch) -> Option<CommandPriority>;

    /// Run the command represented by a matched token, returning the success or failure output to
    /// be displayed to the user.
    #[cfg_attr(feature = "integration-tests", expect(async_fn_in_trait))]
    async fn run(&self, token_match: TokenMatch, app_meta: &mut AppMeta) -> Result<String, String>;

    /// Get the canonical form of the provided token match. Return None if the match is invalid.
    fn get_canonical_form_of(&self, token_match: &TokenMatch) -> Option<String>;

    /// A helper function to roughly provide Command::autocomplete(Command::token().match_input()),
    /// except that that wouldn't compile for all sorts of exciting reasons.
    fn parse_autocomplete<'a>(
        &'a self,
        input: &'a str,
        app_meta: &'a AppMeta,
    ) -> Pin<Box<dyn Stream<Item = AutocompleteSuggestion> + 'a>> {
        Box::pin(stream! {
            let token = self.token();
            for await token_match in token.match_input(input, app_meta) {
                if !matches!(token_match, FuzzyMatch::Overflow(..)) {
                    if let Some(suggestion) = self.autocomplete(token_match, input) {
                        yield suggestion;
                    }
                }
            }
        })
    }
}

#[derive(Clone, CommandList, Debug)]
enum CommandList {
    About(about::About),
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
    let mut suggestions: Vec<_> = stream::select_all(
        CommandList::get_all()
            .iter()
            .map(|c| c.parse_autocomplete(input, app_meta)),
    )
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
                    .zip(commands_tokens[i].1.match_input_exact(input, app_meta)),
            );
        }

        while let Some((command, token_match)) = match_streams.next().await {
            if let Some(priority) = command.get_priority(&token_match) {
                token_matches.push((command, priority, token_match));
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
