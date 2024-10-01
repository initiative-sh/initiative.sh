use super::token::{FuzzyMatch, Token, TokenType, TokenMatch};
use super::Command;

use crate::app::{AppMeta, AutocompleteSuggestion};

pub struct About;

impl Command for About {
    fn token<'a>(&self) -> Token {
        Token {
            token_type: TokenType::Keyword("about"),
            marker: None,
        }
    }

    fn autocomplete(&self, _fuzzy_match: FuzzyMatch, _input: &str) -> Option<AutocompleteSuggestion> {
        Some(("about", "about initiative.sh").into())
    }

    async fn run(
        &self,
        _token_match: TokenMatch<'_>,
        _app_meta: &mut AppMeta,
    ) -> Result<String, String> {
        Ok(include_str!("../../../data/about.md")
            .trim_end()
            .to_string())
    }
}

#[cfg(test)]
mod test {
    use super::super::token::test::assert_stream_eq;
    use super::*;

    use crate::app::{AppMeta, Event};
    use crate::storage::NullDataStore;

    #[tokio::test]
    async fn autocomplete_test() {
        let app_meta = AppMeta::new(NullDataStore, &event_dispatcher);

        assert_stream_eq(
            vec![AutocompleteSuggestion::new("about", "about initiative.sh")],
            About.parse_autocomplete("a", &app_meta),
        )
        .await;
    }

    fn event_dispatcher(_event: Event) {}
}
