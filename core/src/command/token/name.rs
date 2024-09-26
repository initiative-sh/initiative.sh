use super::{Match, MatchType, Meta, Token, TokenType};

use crate::app::AppMeta;
use crate::utils::CaseInsensitiveStr;

use std::pin::Pin;

use async_stream::stream;
use futures::join;
use futures::prelude::*;

pub fn match_input<'a, M>(
    token: &'a Token<M>,
    input: &'a str,
    app_meta: &'a AppMeta,
) -> Pin<Box<dyn Stream<Item = MatchType<'a, M>> + 'a>> {
    let TokenType::Name(record_source, thing_type) = token.token_type else {
        unreachable!();
    };

    Box::pin(stream! {
        use crate::utils::quoted_phrases;

        let phrases: Vec<_> = quoted_phrases(input).collect();

        if let Some(first_phrase) = phrases.first() {
            // unwrap: we've checked that there's a first(), so there must be a last()
            let last_phrase = phrases.last().unwrap();

            let results: Vec<_> = if first_phrase.is_quoted() {
                // Need to query both quoted and unquoted versions of the phrase
                let (get_by_name, get_by_name_start) = join!(
                    app_meta.repository.get_by_name((
                        first_phrase.as_str(),
                        record_source,
                        thing_type,
                    )),
                    app_meta.repository.get_by_name_start((
                        first_phrase.as_original_str(),
                        record_source,
                        thing_type,
                    )),
                );

                get_by_name.into_iter().chain(get_by_name_start.into_iter().flatten()).collect()
            } else {
                app_meta.repository
                    .get_by_name_start((first_phrase.as_str(), record_source, thing_type))
                    .await
                    .unwrap()
            };

            for result in results {
                // unwrap: result of get_by_name_start(), so it must have a name
                let thing_name = result.thing.name().value().unwrap();

                if thing_name.eq_ci(last_phrase.as_str()) {
                    yield MatchType::Exact(
                        Match {
                            token,
                            phrase: input,
                            meta: Meta::Thing(result.thing),
                        }
                    );
                } else if last_phrase.completes_to_ci(thing_name) {
                    yield MatchType::Partial(
                        Match {
                            token,
                            phrase: input,
                            meta: Meta::Thing(result.thing),
                        }
                    );
                } else {
                    for phrase in phrases[0..phrases.len() - 1].iter() {
                        if thing_name.eq_ci(phrase.as_str()) {
                            yield MatchType::Overflow(
                                Match {
                                    token,
                                    phrase: phrase.as_original_own_str(input),
                                    meta: Meta::Thing(result.thing),
                                },
                                &input[phrase.range().end..],
                            );
                            break;
                        }
                    }
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
    use crate::storage::{MemoryDataStore, RecordSource, ThingType};
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
            token_type: TokenType::Name(RecordSource::Any, ThingType::Any),
            marker: (),
        };

        assert_stream_eq(
            vec![
                MatchType::Overflow(
                    Match {
                        token: &token,
                        phrase: "\"Medium\"",
                        meta: Meta::Thing(things[0].clone()),
                    },
                    " Dave Lily",
                ),
                MatchType::Overflow(
                    Match {
                        token: &token,
                        phrase: "\"Medium\" Dave",
                        meta: Meta::Thing(things[1].clone()),
                    },
                    " Lily",
                ),
                MatchType::Partial(Match {
                    token: &token,
                    phrase: "\"Medium\" Dave Lily",
                    meta: Meta::Thing(things[2].clone()),
                }),
            ],
            match_input(&token, "\"Medium\" Dave Lily", &app_meta),
        )
        .await;
    }

    #[tokio::test]
    async fn match_input_test_exact() {
        let things = &[NpcData {
            name: "Teatime".into(),
            ..Default::default()
        }
        .into_thing(Uuid::new_v4())][..];

        let data_store: MemoryDataStore = things.iter().cloned().collect();
        let app_meta = AppMeta::new(data_store, &event_dispatcher);

        {
            let token = Token {
                token_type: TokenType::Name(RecordSource::Any, ThingType::Any),
                marker: (),
            };

            assert_stream_eq(
                vec![MatchType::Exact(Match {
                    token: &token,
                    phrase: "teatime",
                    meta: Meta::Thing(things[0].clone()),
                })],
                match_input(&token, "teatime", &app_meta),
            )
            .await;
        }

        {
            let token = Token {
                token_type: TokenType::Name(RecordSource::Journal, ThingType::Npc),
                marker: (),
            };

            assert_stream_eq(
                vec![MatchType::Exact(Match {
                    token: &token,
                    phrase: "teatime",
                    meta: Meta::Thing(things[0].clone()),
                })],
                match_input(&token, "teatime", &app_meta),
            )
            .await;
        }

        assert_stream_empty(match_input(
            &Token {
                token_type: TokenType::Name(RecordSource::Any, ThingType::Place),
                marker: (),
            },
            "teatime",
            &app_meta,
        ))
        .await;

        assert_stream_empty(match_input(
            &Token {
                token_type: TokenType::Name(RecordSource::Recent, ThingType::Any),
                marker: (),
            },
            "teatime",
            &app_meta,
        ))
        .await;
    }

    fn event_dispatcher(_event: Event) {}
}
