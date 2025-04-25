/// All of the classes needed to implement a new command or token type.
pub mod prelude;

mod about;
mod alias;
mod create;
mod load;
mod save;

mod token;

use std::fmt::{self, Write};
use std::iter;
use std::pin::Pin;

use crate::app::{
    AppMeta, Autocomplete, AutocompleteSuggestion, CommandMatches, ContextAwareParse, Runnable,
};
use crate::utils::quoted_words;
use initiative_macros::CommandList;

use token::{FuzzyMatchList, MatchList, Token};

use async_stream::stream;
use async_trait::async_trait;
use futures::prelude::*;

/// Commands provide the primary interaction between the user and the backend: text in, text out.
/// A second dimension exists in that the commands are queried as the user types to provide
/// autocomplete suggestions.
///
/// This simple concept belies a tremendous amount of complexity and redundancy in implementation,
/// especially in more complex syntaxes. A series of parser combinators is used to abstract away
/// most of the direct interaction with text, leaving each individual command to interact with the
/// user's input at a higher level.
///
/// At the root of the process is the [`Token`]: a pattern returned by the [`Command::token`] method
/// that matches one or more input words. In the event that the user input matches the `Token`
/// exactly or partially (in the case of autocomplete), the parsed result
/// ([`TokenMatch`]/[`FuzzyMatch`]) will be dispatched to [`Command::get_priority`] or
/// [`Command::autocomplete`] for further processing. These methods can return [`None`], indicating
/// that the input should be considered a non-match despite the provided `Token`.
///
/// The preferred way of parsing token trees is using markers: any value implementing [`Hash`]
/// (a custom enum by convention) assigned to a `Token` upon creation which persists through the
/// `TokenMatch` data provided to `autocomplete`/`get_priority`. Rather than manually climbing
/// through the token tree in search of a particular marker, the [`TokenMatch::find_markers`] method
/// and its kin can be used to jump directly (and recursively) to the token(s) of interest.
///
/// If `get_priority` returns `Some(x)`, the dispatcher ([`run`]) will choose which command to
/// execute based on the returned priority: [`CommandPriority::Canonical`] matches will always be
/// executed, while [`CommandPriority::Fuzzy`] matches will run if no other matches are present. In
/// the event that additional fuzzy matches are present, all matched phrases will be displayed to
/// the user in their canonical form. Once the parser has decided which command to run, its
/// [`Command::run`] method will be invoked and the result (success or failure) displayed to the
/// user.
///
/// # Creating a command
///
/// 1. Create a new module within [`crate::command`] with a unit struct for your command.
/// 2. Implement the [`Command`] trait for the struct.
/// 3. (Optional) Add a `Marker` enum with the [`Hash`] trait derived, to help with navigating the
///    match tree.
/// 4. Add your new command to the [`CommandList`] enum in this module. The derive macro on that
///    enum will handle the rest of the wiring.
///
/// For a simple example, see [`about::About`].
pub trait Command {
    /// Return a single Token representing the command's syntax. If multiple commands are possible,
    /// Token::Or can be used as a wrapper to cover the options.
    ///
    /// _See [`token::constructors`] for the available tokens and their constructors._
    fn token(&self) -> Token;

    /// Convert a matched token into a suggestion to be displayed to the user. Note that this
    /// method is not async; any metadata that may be needed for the autocomplete (eg. records
    /// matched by name) should be available through [`FuzzyMatch::token_match`].
    ///
    /// A possible match can be indicated by returning `Some(("command", "description").into())`.
    ///
    /// A non-match can be indicated by returning `None`.
    fn autocomplete(
        &self,
        fuzzy_match_list: FuzzyMatchList,
        input: &str,
    ) -> Option<AutocompleteSuggestion>;

    /// Get the priority of the command with a given input. See [`CommandPriority`] for details.
    ///
    /// `None` will be interpreted as a non-match.
    fn get_priority(&self, match_list: &MatchList) -> Option<CommandPriority>;

    /// Run the command represented by a matched token, returning the success or failure output to
    /// be displayed to the user.
    #[cfg_attr(feature = "integration-tests", expect(async_fn_in_trait))]
    async fn run(
        &self,
        match_list: MatchList,
        app_meta: &mut AppMeta,
    ) -> Result<impl std::fmt::Display, impl std::fmt::Display>;

    /// Get the canonical form of the provided [`MatchList`]. Returns `None` if the match is invalid.
    fn get_canonical_form_of(&self, match_list: &MatchList) -> Option<String>;

    /// A helper function to roughly provide Command::autocomplete(Command::token().match_input()),
    /// except that that wouldn't compile for all sorts of exciting reasons.
    fn parse_autocomplete<'a>(
        &'a self,
        input: &'a str,
        app_meta: &'a AppMeta,
    ) -> Pin<Box<dyn Stream<Item = AutocompleteSuggestion> + 'a>> {
        Box::pin(stream! {
            let token = self.token();
            for await fuzzy_match_list in token.match_input(input, app_meta) {
                if !fuzzy_match_list.is_overflow() {
                    if let Some(suggestion) = self.autocomplete(fuzzy_match_list, input) {
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
    #[command_list(ignore)]
    #[expect(dead_code)]
    Alias(alias::Alias),
    Create(create::Create),
    Load(load::Load),
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

            let mut match_streams = match_streams
                .filter_map(|(command, fuzzy_match_list)| {
                    future::ready(
                        fuzzy_match_list
                            .into_match_list()
                            .map(|match_list| (command, match_list)),
                    )
                })
                .filter_map(|(command, match_list)| {
                    future::ready(command.get_priority(&match_list).and_then(|priority| {
                        command
                            .get_canonical_form_of(&match_list)
                            .map(|canonical| (priority, canonical))
                    }))
                });

            while let Some((priority, canonical)) = match_streams.next().await {
                if priority == CommandPriority::Canonical {
                    command_matches.push_canonical(Self { canonical });
                } else {
                    command_matches.push_fuzzy(Self { canonical });
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
    if quoted_words(input).next().is_none() {
        return Vec::new();
    };

    let mut suggestions: Vec<_> = stream::iter(CommandList::get_all())
        .flat_map(|c| c.parse_autocomplete(input, app_meta))
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

    let mut match_lists: Vec<(&CommandList, CommandPriority, MatchList)> = Vec::new();

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

        while let Some((command, match_list)) = match_streams.next().await {
            if let Some(priority) = command.get_priority(&match_list) {
                match_lists.push((command, priority, match_list));
            }
        }
    }

    match_lists.sort_by_key(|&(_, command_priority, _)| command_priority);

    match match_lists.len() {
        0 => return Err(format!("Unknown command: \"{}\"", input)),
        1 => {
            let (command, _, match_list) = match_lists.pop().unwrap();
            return command
                .run(match_list, app_meta)
                .await
                .map(|s| s.to_string())
                .map_err(|e| e.to_string());
        }
        _ => {} // continue
    }

    if match_lists[0].1 == CommandPriority::Canonical {
        assert_ne!(match_lists[1].1, CommandPriority::Canonical);

        let (command, _, match_list) = match_lists.remove(0);
        let result = command
            .run(match_list, app_meta)
            .await
            .map(|s| s.to_string())
            .map_err(|e| e.to_string());

        let mut iter = match_lists
            .iter()
            .take_while(|(_, command_priority, _)| command_priority == &CommandPriority::Fuzzy)
            .peekable();

        if iter.peek().is_none() {
            result
        } else {
            let f = |s| {
                iter
                    .filter_map(|(command, _, match_list)| command.get_canonical_form_of(match_list))
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
        let first_match_list = match_lists.remove(0);

        let mut iter = iter::once(&first_match_list)
            .chain(
                match_lists
                    .iter()
                    .take_while(|(_, command_priority, _)| command_priority == &first_match_list.1),
            )
            .filter_map(|(command, _, match_list)| command.get_canonical_form_of(match_list))
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
