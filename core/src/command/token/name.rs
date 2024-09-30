//! Matches the name of a Thing that exists in recent or journal. Where a real Thing was matched,
//! the meta field will include the matched Record for further processing (eg. customized
//! autocomplete comments). NOTE: Will also return *syntactically possible* but *incorrect*
//! matches, even when a correct match is present, to facilitate generating user-friendly error
//! messages like "there is no character named xyz" at higher levels.

use super::{Match, MatchType, Token, TokenType};

use crate::app::AppMeta;
use crate::utils::CaseInsensitiveStr;

use std::pin::Pin;

use async_stream::stream;
use futures::join;
use futures::prelude::*;

pub fn match_input<'a, M>(
    token: Token<'a, M>,
    input: &'a str,
    app_meta: &'a AppMeta,
) -> Pin<Box<dyn Stream<Item = MatchType<'a, M>> + 'a>>
where
    M: Clone,
{
    assert!(matches!(token.token_type, TokenType::Name));

    Box::pin(stream! {
        use crate::utils::quoted_phrases;

        let phrases: Vec<_> = quoted_phrases(input).collect();
        let mut unmatched_phrases = phrases.clone();

        if let Some(first_phrase) = phrases.first() {
            // unwrap: we've checked that there's a first(), so there must be a last()
            let last_phrase = phrases.last().unwrap();

            let records: Vec<_> = if first_phrase.is_quoted() {
                // Need to query both quoted and unquoted versions of the phrase
                let (get_by_name, get_by_name_start) = join!(
                    app_meta.repository.get_by_name(first_phrase.as_str()),
                    app_meta.repository.get_by_name_start(first_phrase.as_original_str()),
                );

                get_by_name.into_iter().chain(get_by_name_start.into_iter().flatten()).collect()
            } else {
                app_meta.repository
                    .get_by_name_start(first_phrase.as_str())
                    .await
                    .unwrap()
            };

            for record in records.into_iter() {
                // unwrap: result of get_by_name_start(), so it must have a name
                let thing_name = record.thing.name().value().unwrap();

                if thing_name.eq_ci(last_phrase) {
                    yield MatchType::Exact(Match::new(token.clone(), record));
                    if let Some(i) = unmatched_phrases.iter().position(|w| w == last_phrase) {
                        unmatched_phrases.swap_remove(i);
                    }
                    continue;
                } else if last_phrase.can_complete() {
                    if let Some(completion) = thing_name.strip_prefix_ci(last_phrase).map(str::to_string) {
                        yield MatchType::Partial(
                            Match::new(token.clone(), record),
                            Some(completion),
                        );

                        continue;
                    }
                }

                for phrase in phrases[0..phrases.len() - 1].iter() {
                    if let Some(i) = unmatched_phrases.iter().position(|w| w == phrase) {
                        unmatched_phrases.swap_remove(i);
                    }

                    if thing_name.eq_ci(phrase) {
                        yield MatchType::Overflow(
                            Match::new(token.clone(), record),
                            &input[phrase.range().end..],
                        );
                        break;
                    }
                }
            }

            for unmatched_phrase in unmatched_phrases {
                if unmatched_phrase.is_at_end() {
                    yield MatchType::Exact(token.clone().into());
                } else {
                    yield MatchType::Overflow(
                        token.clone().into(),
                        unmatched_phrase.after().as_own_str(&input),
                    );
                }
            }
        }
    })
}

#[cfg(test)]
mod test {
    use super::super::test::{assert_stream_empty, assert_stream_eq};
    use super::*;
    use crate::app::Event;
    use crate::storage::{MemoryDataStore, Record, RecordStatus};
    use crate::world::npc::NpcData;
    use uuid::Uuid;

    #[tokio::test]
    async fn match_input_test_overflow_partial() {
        let things = &[
            NpcData {
                name: "Medium".into(),
                ..Default::default()
            }
            .into_thing(Uuid::new_v4()),
            NpcData {
                name: "\"Medium\" Dave".into(),
                ..Default::default()
            }
            .into_thing(Uuid::new_v4()),
            NpcData {
                name: "\"Medium\" Dave Lilywhite".into(),
                ..Default::default()
            }
            .into_thing(Uuid::new_v4()),
        ][..];

        let data_store: MemoryDataStore = things.iter().cloned().collect();
        let app_meta = AppMeta::new(data_store, &event_dispatcher);

        let token = Token {
            token_type: TokenType::Name,
            marker: (),
        };

        assert_stream_eq(
            vec![
                MatchType::Overflow(
                    Match::new(
                        token.clone(),
                        Record {
                            status: RecordStatus::Saved,
                            thing: things[0].clone(),
                        },
                    ),
                    " Dave Lily",
                ),
                MatchType::Overflow(
                    Match::new(
                        token.clone(),
                        Record {
                            status: RecordStatus::Saved,
                            thing: things[1].clone(),
                        },
                    ),
                    " Lily",
                ),
                MatchType::Partial(
                    Match::new(
                        token.clone(),
                        Record {
                            status: RecordStatus::Saved,
                            thing: things[2].clone(),
                        },
                    ),
                    Some("white".to_string()),
                ),
                MatchType::Exact(token.clone().into()),
            ],
            match_input(token, "\"Medium\" Dave Lily", &app_meta),
        )
        .await;
    }

    #[tokio::test]
    async fn match_input_test_exact() {
        let things = &[NpcData {
            name: "Jonathan Teatime".into(),
            ..Default::default()
        }
        .into_thing(Uuid::new_v4())][..];

        let data_store: MemoryDataStore = things.iter().cloned().collect();
        let app_meta = AppMeta::new(data_store, &event_dispatcher);

        {
            let token = Token {
                token_type: TokenType::Name,
                marker: (),
            };

            assert_stream_eq(
                vec![
                    MatchType::Overflow(token.clone().into(), " teatime"),
                    MatchType::Exact(Match::new(
                        token.clone(),
                        Record {
                            status: RecordStatus::Saved,
                            thing: things[0].clone(),
                        },
                    )),
                ],
                match_input(token, "jonathan teatime", &app_meta),
            )
            .await;
        }

        {
            let token = Token {
                token_type: TokenType::Name,
                marker: (),
            };

            assert_stream_eq(
                vec![
                    MatchType::Partial(
                        Match::new(
                            token.clone(),
                            Record {
                                status: RecordStatus::Saved,
                                thing: things[0].clone(),
                            },
                        ),
                        Some(" Teatime".to_string()),
                    ),
                    MatchType::Exact(token.clone().into()),
                ],
                match_input(token, "Jonathan", &app_meta),
            )
            .await;
        }
    }

    fn event_dispatcher(_event: Event) {}
}
