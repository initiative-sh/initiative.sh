mod about;
mod save;

mod token;

use std::pin::Pin;

use crate::app::{AppMeta, AutocompleteSuggestion};

use token::{MatchType, Token};

use async_stream::stream;
use futures::prelude::*;

trait Command {
    /// Return a single Token representing the command's syntax. If multiple commands are possible,
    /// Token::Or can be used as a wrapper to cover the options.
    fn token(&self) -> Token;

    /// Convert a matched token into a suggestion to be displayed to the user.
    fn autocomplete(&self, input: &str, token_match: MatchType) -> Option<AutocompleteSuggestion>;

    /// Run the command represented by a matched token.
    async fn run(&self, token_match: MatchType, app_meta: &mut AppMeta) -> Result<String, String>;

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
                if !matches!(token_match, MatchType::Overflow(..)) {
                    if let Some(suggestion) = self.autocomplete(input, token_match) {
                        yield suggestion;
                    }
                }
            }
        })
    }
}

pub async fn autocomplete(input: &str, app_meta: &AppMeta) -> Vec<AutocompleteSuggestion> {
    let mut suggestions: Vec<_> = stream::select_all([
        about::About.parse_autocomplete(input, app_meta),
        save::Save.parse_autocomplete(input, app_meta),
    ])
    .collect()
    .await;

    suggestions.sort();
    suggestions.truncate(10);
    suggestions
}
