use super::{Match, MatchType, Token, TokenType};

use crate::app::AppMeta;
use crate::storage::Record;
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
    let TokenType::Name(record_source, thing_type) = token.token_type else {
        unreachable!();
    };

    Box::pin(stream! {
        use crate::utils::quoted_phrases;

        let phrases: Vec<_> = quoted_phrases(input).collect();

        if let Some(first_phrase) = phrases.first() {
            // unwrap: we've checked that there's a first(), so there must be a last()
            let last_phrase = phrases.last().unwrap();

            let records: Vec<_> = if first_phrase.is_quoted() {
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

            for record in records.into_iter() {
                let Record { thing, .. } = record;

                // unwrap: result of get_by_name_start(), so it must have a name
                let thing_name = thing.name().value().unwrap();

                if thing_name.eq_ci(last_phrase) {
                    yield MatchType::Exact(Match::new(token.clone(), thing));
                    continue;
                } else if last_phrase.can_complete() {
                    if let Some(completion) = thing_name.strip_prefix_ci(last_phrase).map(str::to_string) {
                        yield MatchType::Partial(
                            Match::new(token.clone(), thing),
                            Some(completion),
                        );

                        continue;
                    }
                }

                for phrase in phrases[0..phrases.len() - 1].iter() {
                    if thing_name.eq_ci(phrase) {
                        yield MatchType::Overflow(
                            Match::new(token.clone(), thing),
                            &input[phrase.range().end..],
                        );
                        break;
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
                MatchType::Overflow(Match::new(token.clone(), things[0].clone()), " Dave Lily"),
                MatchType::Overflow(Match::new(token.clone(), things[1].clone()), " Lily"),
                MatchType::Partial(
                    Match::new(token.clone(), things[2].clone()),
                    Some("white".to_string()),
                ),
            ],
            match_input(token, "\"Medium\" Dave Lily", &app_meta),
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
                vec![MatchType::Exact(Match::new(
                    token.clone(),
                    things[0].clone(),
                ))],
                match_input(token, "teatime", &app_meta),
            )
            .await;
        }

        {
            let token = Token {
                token_type: TokenType::Name(RecordSource::Journal, ThingType::Npc),
                marker: (),
            };

            assert_stream_eq(
                vec![MatchType::Exact(Match::new(
                    token.clone(),
                    things[0].clone(),
                ))],
                match_input(token, "teatime", &app_meta),
            )
            .await;
        }

        assert_stream_empty(match_input(
            Token {
                token_type: TokenType::Name(RecordSource::Any, ThingType::Place),
                marker: (),
            },
            "teatime",
            &app_meta,
        ))
        .await;

        assert_stream_empty(match_input(
            Token {
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
