/*
use super::token::{Match, Token};
use super::Command;

use crate::app::{AppMeta, AutocompleteSuggestion};

use futures::stream::{Stream, StreamExt};

pub struct About;

impl About {
    const fn token(&self) -> Token {
        Token::Word("about")
    }
}

impl Command for About {
    async fn autocomplete(
        self,
        input: &str,
        app_meta: &AppMeta,
    ) -> impl Stream<Item = AutocompleteSuggestion> {
        self.token().autocomplete(input, app_meta)
    }
}
*/
