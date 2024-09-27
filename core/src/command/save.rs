use super::token::{MatchType, Token, TokenType};
use super::Command;

use crate::app::{AppMeta, AutocompleteSuggestion};
use crate::storage::{RecordSource, ThingType};

pub struct Save;

impl Command for Save {
    type Marker = ();

    fn token<'a>(&self) -> Token<'a, Self::Marker> {
        Token {
            token_type: TokenType::Phrase(&[
                Token {
                    token_type: TokenType::Keyword("save"),
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
    use super::*;

    use crate::app::{AppMeta, Event};
    use crate::storage::{Change, MemoryDataStore};
    use crate::world::npc::NpcData;

    use uuid::Uuid;

    #[tokio::test]
    async fn autocomplete_test() {
        let things = &[
            NpcData {
                name: "Cut-Me-Own-Throat Dibbler".into(),
                ..Default::default()
            }
            .into_thing(Uuid::new_v4()),
            NpcData {
                name: "Cohen the Barbarian".into(),
                ..Default::default()
            }
            .into_thing(Uuid::new_v4()),
        ];

        let mut app_meta = AppMeta::new(MemoryDataStore::default(), &event_dispatcher);
        for thing in things {
            app_meta
                .repository
                .modify_without_undo(Change::Create {
                    thing_data: thing.data.clone(),
                    uuid: Some(thing.uuid),
                })
                .await
                .ok();
        }

        assert_eq!(
            vec![AutocompleteSuggestion::new("about", "about initiative.sh")],
            autocomplete("save c", &app_meta).await,
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
