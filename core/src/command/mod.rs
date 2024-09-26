mod about;
mod save;

mod token;

use std::iter;
use std::pin::{pin, Pin};

use crate::app::{AppMeta, AutocompleteSuggestion};

use token::{Match, MatchType, Token};

use async_stream::stream;
use futures::pin_mut;
use futures::prelude::*;

trait Command {
    type Marker;

    fn token<'a>(&self) -> Token<'a, Self::Marker>;

    fn autocomplete<'a>(
        &self,
        token_match: MatchType<'a, Self::Marker>,
    ) -> Option<AutocompleteSuggestion>;

    async fn run<'a>(
        &self,
        token_match: MatchType<'a, Self::Marker>,
        app_meta: &mut AppMeta,
    ) -> Result<String, String>;
}

fn commands() -> impl Iterator<Item = impl Command> {
    [about::About].into_iter()
}

pub async fn autocomplete(input: &str, app_meta: &AppMeta) -> Vec<AutocompleteSuggestion> {
    let stream = stream::iter(commands()).flat_map(|command| {
        stream! {
            let token = command.token();
            for await token_match in token.match_input(input, app_meta) {
                if !matches!(token_match, MatchType::Overflow(..)) {
                    if let Some(suggestion) = command.autocomplete(token_match) {
                        yield suggestion;
                    }
                }
            }
        }
    });

    let mut suggestions: Vec<_> = stream.collect().await;
    suggestions.sort();
    suggestions.truncate(10);
    suggestions
}
