use super::token::{FuzzyMatch, Meta, Token, TokenType, TokenMatch};
use super::Command;

use crate::app::{AppMeta, AutocompleteSuggestion};

pub struct Save;

impl Command for Save {
    fn token(&self) -> Token {
        Token {
            token_type: TokenType::Phrase(&[
                Token {
                    token_type: TokenType::Keyword("save"),
                    marker: None,
                },
                Token {
                    token_type: TokenType::Name,
                    marker: None,
                },
            ]),
            marker: None,
        }
    }

    fn autocomplete(&self, fuzzy_match: FuzzyMatch, _input: &str) -> Option<AutocompleteSuggestion> {
        let token_match = fuzzy_match.token_match();

        let record = {
            let Meta::Sequence(token_sequence) = &token_match.meta else {
                return None;
            };

            let Meta::Record(record) = &token_sequence[1].meta else {
                return None;
            };

            record
        };

        if record.is_saved() {
            None
        } else {
            Some(
                (
                    format!("save {}", record.thing.name()),
                    format!("save {} to journal", record.thing.as_str()),
                )
                    .into(),
            )
        }
    }

    async fn run(
        &self,
        _token_match: TokenMatch<'_>,
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
    use crate::world::place::PlaceData;

    use uuid::Uuid;

    #[tokio::test]
    async fn autocomplete_test() {
        let things = &[
            NpcData {
                name: "Cohen the Barbarian".into(),
                ..Default::default()
            }
            .into_thing(Uuid::new_v4()),
            PlaceData {
                name: "Copperhead".into(),
                ..Default::default()
            }
            .into_thing(Uuid::new_v4()),
        ];

        let mut app_meta = AppMeta::new(
            [NpcData {
                name: "Cut-Me-Own-Throat Dibbler".into(),
                ..Default::default()
            }
            .into_thing(Uuid::new_v4())]
            .into_iter()
            .collect::<MemoryDataStore>(),
            &event_dispatcher,
        );

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
            vec![
                AutocompleteSuggestion::new(
                    "save Cohen the Barbarian",
                    "save character to journal"
                ),
                AutocompleteSuggestion::new("save Copperhead", "save place to journal"),
            ],
            autocomplete("save c", &app_meta).await,
        );

        assert_eq!(
            vec![AutocompleteSuggestion::new(
                "save Copperhead",
                "save place to journal"
            )],
            autocomplete("SAVE COPPERHEAD", &app_meta).await,
        );
    }

    fn event_dispatcher(_event: Event) {}
}
