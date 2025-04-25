//! Matches the name of a Thing that exists in recent or journal. Where a real Thing was matched,
//! the meta field will include the matched Record for further processing (eg. customized
//! autocomplete comments).

use crate::app::AppMeta;
use crate::command::prelude::*;
use crate::utils::{quoted_phrases, CaseInsensitiveStr, Substr};

use std::pin::Pin;

use async_stream::stream;
use futures::join;
use futures::prelude::*;

pub fn match_input<'token, 'app_meta>(
    token: &'token Token,
    input: Substr<'token>,
    app_meta: &'app_meta AppMeta,
) -> Pin<Box<dyn Stream<Item = FuzzyMatch<'token>> + 'app_meta>>
where
    'token: 'app_meta,
{
    assert!(matches!(token, Token::Name { .. }));

    let phrases: Vec<_> = quoted_phrases(input).collect();
    if phrases.is_empty() {
        return Box::pin(stream::empty());
    }

    Box::pin(stream! {
        let first_phrase = phrases.first().unwrap();

        // unwrap: we've checked that there's a first(), so there must be a last()
        let full_phrase = phrases.last().unwrap();

        let records: Vec<_> = if first_phrase.is_quoted() {
            // Need to query both quoted and unquoted versions of the phrase
            let (unquoted_name, quoted_name_matches) = join!(
                app_meta.repository.get_by_name(first_phrase.as_str()),
                app_meta.repository.get_by_name_start(first_phrase.as_outer_str()),
            );

            let mut quoted_name_matches = quoted_name_matches.unwrap_or_default();
            if let Ok(unquoted_name) = unquoted_name {
                quoted_name_matches.push(unquoted_name);
            }

            quoted_name_matches
        } else {
            app_meta.repository
                .get_by_name_start(first_phrase.as_str())
                .await
                .unwrap_or_default()
        };

        for record in records.into_iter() {
            // unwrap: result of get_by_name_start(), so it must have a name
            let thing_name = record.thing.name().value().unwrap();

            if thing_name.eq_ci(full_phrase) {
                yield FuzzyMatch::Exact(TokenMatch::new(token, record));
                continue;
            } else if full_phrase.can_complete() {
                if let Some(completion) = thing_name.strip_prefix_ci(full_phrase).map(str::to_string) {
                    yield FuzzyMatch::Partial(
                        TokenMatch::new(token, record),
                        Some(completion),
                    );

                    continue;
                }
            }

            for (i, phrase) in phrases[0..phrases.len() - 1].iter().enumerate() {
                if thing_name.eq_ci(phrase) || (i == 0 && thing_name.eq_ci(phrase.as_outer_str())) {
                    yield FuzzyMatch::Overflow(
                        TokenMatch::new(token, record),
                        phrase.after()
                    );
                    break;
                }
            }
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::storage::{Record, RecordStatus};
    use crate::test_utils as test;
    use uuid::Uuid;

    #[derive(Hash)]
    enum Marker {
        Token,
    }

    #[tokio::test]
    async fn match_input_test_simple() {
        let token = name_m(Marker::Token);

        test::assert_eq_unordered!(
            [FuzzyMatch::Exact(TokenMatch::new(
                &token,
                Record {
                    status: RecordStatus::Unsaved,
                    thing: test::thing::odysseus(),
                },
            ))],
            match_input(
                &token,
                "Odysseus".into(),
                &test::app_meta::with_test_data().await
            )
            .collect::<Vec<_>>()
            .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_empty() {
        test::assert_empty!(
            match_input(
                &name(),
                "    ".into(),
                &test::app_meta::with_test_data().await
            )
            .collect::<Vec<_>>()
            .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_quoted() {
        let token = name();

        let things = [
            test::npc().name("Medium").build_thing(Uuid::new_v4()),
            test::npc().name("\"Medium\"").build_thing(Uuid::new_v4()),
            test::npc()
                .name("\"Medium\" Dave Lilywhite")
                .build_thing(Uuid::new_v4()),
        ];

        let app_meta = test::app_meta::with_data_store::memory::with(things.clone(), []);

        test::assert_eq_unordered!(
            [
                FuzzyMatch::Overflow(
                    TokenMatch::new(
                        &token,
                        Record {
                            status: RecordStatus::Saved,
                            thing: things[0].clone(),
                        },
                    ),
                    " Dave Lily".into(),
                ),
                FuzzyMatch::Overflow(
                    TokenMatch::new(
                        &token,
                        Record {
                            status: RecordStatus::Saved,
                            thing: things[1].clone(),
                        },
                    ),
                    " Dave Lily".into(),
                ),
                FuzzyMatch::Partial(
                    TokenMatch::new(
                        &token,
                        Record {
                            status: RecordStatus::Saved,
                            thing: things[2].clone(),
                        },
                    ),
                    Some("white".to_string()),
                ),
            ],
            match_input(&token, "\"Medium\" Dave Lily".into(), &app_meta)
                .collect::<Vec<_>>()
                .await,
        );
    }
}
