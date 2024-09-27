use super::token::{MatchType, Token, TokenType};
use super::Command;

use crate::app::{AppMeta, AutocompleteSuggestion};

pub struct About;

impl Command for About {
    type Marker = ();

    fn token<'a>(&self) -> Token<'a, Self::Marker> {
        Token {
            token_type: TokenType::Keyword("about"),
            marker: (),
        }
    }

    fn autocomplete(&self, _match_type: MatchType<Self::Marker>) -> Option<AutocompleteSuggestion> {
        Some(("about", "about initiative.sh").into())
    }

    async fn run<'a>(
        &self,
        _token_match: MatchType<'a, Self::Marker>,
        _app_meta: &mut AppMeta,
    ) -> Result<String, String> {
        Ok(include_str!("../../../data/about.md")
            .trim_end()
            .to_string())
    }
}

#[cfg(test)]
mod test {
    use super::super::autocomplete;
    use super::*;

    use crate::app::{AppMeta, Event};
    use crate::storage::NullDataStore;

    #[tokio::test]
    async fn autocomplete_test() {
        let app_meta = AppMeta::new(NullDataStore, &event_dispatcher);

        assert_eq!(
            vec![AutocompleteSuggestion::new("about", "about initiative.sh")],
            autocomplete("a", &app_meta).await,
        );

        assert_eq!(
            vec![AutocompleteSuggestion::new("about", "about initiative.sh")],
            autocomplete("ABOUT", &app_meta).await,
        );

        assert_eq!(
            Vec::<AutocompleteSuggestion>::new(),
            autocomplete("aboot", &app_meta).await,
        );

        assert_eq!(
            Vec::<AutocompleteSuggestion>::new(),
            autocomplete("about ", &app_meta).await,
        );
    }

    fn event_dispatcher(_event: Event) {}
}
