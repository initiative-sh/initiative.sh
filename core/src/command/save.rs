use super::token::{MatchType, Token, TokenType};
use super::Command;

use crate::app::{AppMeta, AutocompleteSuggestion};
use crate::storage::{RecordSource, ThingType};

use std::iter;

pub struct Save;

impl Command for Save {
    type Marker = ();

    fn token<'a>(&self) -> Token<'a, Self::Marker> {
        Token {
            token_type: TokenType::Phrase(&[
                Token {
                    token_type: TokenType::Word("save"),
                    marker: (),
                },
                Token {
                    token_type: TokenType::Name(RecordSource::Recent, ThingType::Any),
                    marker: (),
                },
            ]),
            marker: (),
        }
    }

    fn autocomplete(&self, match_type: MatchType<Self::Marker>) -> Option<AutocompleteSuggestion> {
        match match_type {
            MatchType::Partial(..) => todo!(),
            MatchType::Exact(..) => todo!(),
            MatchType::Overflow(..) => todo!(),
        }
    }

    async fn run<'a>(
        &self,
        _token_match: MatchType<'a, Self::Marker>,
        _app_meta: &mut AppMeta,
    ) -> Result<String, String> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::super::autocomplete;
    use super::super::test::{assert_stream_empty, assert_stream_eq};
    use super::*;

    use crate::app::{AppMeta, Event};
    use crate::storage::MemoryDataStore;

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
