mod about;
mod save;

mod token;

use std::iter;
use std::pin::Pin;

use crate::app::{AppMeta, AutocompleteSuggestion};

use token::{FuzzyMatch, Token, TokenMatch};

use async_stream::stream;
use futures::prelude::*;

trait Command {
    /// Return a single Token representing the command's syntax. If multiple commands are possible,
    /// Token::Or can be used as a wrapper to cover the options.
    fn token(&self) -> Token;

    /// Convert a matched token into a suggestion to be displayed to the user.
    fn autocomplete(&self, fuzzy_match: FuzzyMatch, input: &str) -> Option<AutocompleteSuggestion>;

    /// Get the priority of the command with a given input. See CommandPriority for details.
    fn get_priority(&self, token_match: &TokenMatch) -> CommandPriority;

    /// Run the command represented by a matched token, returning the success or failure output to
    /// be displayed to the user.
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

enum CommandList {
    About(about::About),
    Save(save::Save),
}

impl CommandList {
    const fn get_all() -> &'static [CommandList] {
        &[
            CommandList::About(about::About),
            CommandList::Save(save::Save),
        ]
    }
}

impl Command for CommandList {
    fn token(&self) -> Token {
        match self {
            CommandList::About(c) => c.token(),
            CommandList::Save(c) => c.token(),
        }
    }

    fn autocomplete(&self, fuzzy_match: FuzzyMatch, input: &str) -> Option<AutocompleteSuggestion> {
        match self {
            CommandList::About(c) => c.autocomplete(fuzzy_match, input),
            CommandList::Save(c) => c.autocomplete(fuzzy_match, input),
        }
    }

    fn get_priority(&self, token_match: &TokenMatch) -> CommandPriority {
        match self {
            CommandList::About(c) => c.get_priority(token_match),
            CommandList::Save(c) => c.get_priority(token_match),
        }
    }

    fn get_canonical_form_of(&self, token_match: &TokenMatch) -> Option<String> {
        match self {
            CommandList::About(c) => c.get_canonical_form_of(token_match),
            CommandList::Save(c) => c.get_canonical_form_of(token_match),
        }
    }

    async fn run<'a>(&self, token_match: TokenMatch<'a>, app_meta: &mut AppMeta) -> Result<String, String> {
        match self {
            CommandList::About(c) => c.run(token_match, app_meta).await,
            CommandList::Save(c) => c.run(token_match, app_meta).await,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum CommandPriority {
    /// There should be no more than one canonical command per input, distinguished by a unique
    /// prefix. The canonical command will always run if matched. If fuzzy matches also exist, they
    /// will be indicated after the output of the canonical command.
    Canonical,

    /// There may be multiple fuzzy matches for a given input. If no canonical command exists AND
    /// only one fuzzy match is found, that match will run. If multiple fuzzy matches are found,
    /// the user will be prompted which canonical form they wish to run.
    FuzzyWillRun,

    /// Some fuzzy matches are known in advance to be invalid, eg. if they contain a name that does
    /// not exist. These will be deprioritized vs. potentially valid matches.
    FuzzyWillFail,
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
    let mut token_matches: Vec<(&CommandList, CommandPriority, TokenMatch)> =
        stream::iter(CommandList::get_all())
            .flat_map(|c| stream::repeat(c).zip(c.token().match_input(input, app_meta)))
            .filter_map(|(c, fuzzy_match)| {
                future::ready(if let FuzzyMatch::Exact(token_match) = fuzzy_match {
                    Some((c, c.get_priority(&token_match), token_match))
                } else {
                    None
                })
            })
            .collect()
            .await;

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
            .take_while(|(_, command_priority, _)| {
                command_priority == &CommandPriority::FuzzyWillRun
            })
            .peekable();

        if iter.peek().is_none() {
            result
        } else {
            let f = |s| {
                format!(
                    "{}\n\n! There are other possible interpretations of this command. Did you mean:\n{}",
                    s,
                    iter
                        .filter_map(|(command, _, token_match)| command.get_canonical_form_of(token_match))
                        .map(|s| format!("\n* `{}`", s)).collect::<String>(),
                )
            };

            match result {
                Ok(s) => Ok(f(s)),
                Err(s) => Err(f(s)),
            }
        }
    } else {
        let first_token_match = token_matches.remove(0);

        let mut iter = iter::once(&first_token_match).chain(
            token_matches
                .iter()
                .take_while(|(_, command_priority, _)| command_priority == &first_token_match.1),
        )
        .filter_map(|(command, _, token_match)| command.get_canonical_form_of(&token_match))
        .peekable();

        if iter.peek().is_none() {
            Err(format!("Unknown command: \"{}\"", input))
        } else {
            Err(format!(
                "There are several possible interpretations of this command. Did you mean:\n{}",
                iter.map(|s| format!("\n* `{}`", s)).collect::<String>(),
            ))
        }
    }
}
