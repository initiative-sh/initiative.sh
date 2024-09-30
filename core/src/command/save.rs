use super::token::{MatchType, Meta, Token, TokenType};
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
                    token_type: TokenType::Name,
                    marker: (),
                },
            ]),
            marker: (),
        }
    }

    fn autocomplete(
        &self,
        input: &str,
        match_type: MatchType<Self::Marker>,
    ) -> Option<AutocompleteSuggestion> {
        let token_match = match &match_type {
            MatchType::Partial(token_match, _) => token_match,
            MatchType::Exact(token_match) => token_match,
            MatchType::Overflow(..) => return None,
        };

        let (name_match, matched_thing) = {
            let Meta::Sequence(token_sequence) = &token_match.meta else {
                unreachable!();
            };

            let Meta::Single(or_token_match) = &token_sequence[1].meta else {
                unreachable!();
            };

            (
                or_token_match,
                if let Meta::Thing(thing) = &or_token_match.meta {
                    Some(thing)
                } else {
                    None
                },
            )
        };

        // don't autocomplete invalid suggestions
        if let (Some(Marker::UnsavedThingName), Some(thing)) =
            (name_match.token.marker, matched_thing)
        {
            if let MatchType::Partial(_, Some(remainder)) = &match_type {
                Some(
                    (
                        format!("{}{}", input, remainder),
                        format!("save {} to journal", thing.as_str()),
                    )
                        .into(),
                )
            } else {
                Some(
                    (
                        input.to_string(),
                        format!("save {} to journal", thing.as_str()),
                    )
                        .into(),
                )
            }
        } else {
            None
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
                    "save cohen the Barbarian",
                    "save character to journal"
                ),
                AutocompleteSuggestion::new(
                    "save copperhead",
                    "save place to journal"
                ),
            ],
            autocomplete("save c", &app_meta).await,
        );

        assert_eq!(
            vec![AutocompleteSuggestion::new("about", "about initiative.sh")],
            autocomplete("SAVE CO", &app_meta).await,
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
