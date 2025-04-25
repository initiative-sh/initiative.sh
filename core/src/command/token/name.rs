use super::prelude::*;

pub fn match_input<'input, 'stream>(
    token: &'stream Token,
    input: Substr<'input>,
    app_meta: &'stream AppMeta,
) -> Pin<Box<dyn Stream<Item = FuzzyMatchList<'input>> + 'stream>>
where
    'input: 'stream,
{
    assert!(matches!(token.kind, TokenKind::Name));
    let marker_hash = token.marker_hash;

    let phrases: Vec<_> = quoted_phrases_all(input.clone()).collect();
    if phrases.is_empty() {
        return Box::pin(stream::once(future::ready(FuzzyMatchList::new_incomplete(
            MatchPart::new(input, marker_hash).with_term("[name]"),
        ))));
    }

    Box::pin(stream! {
        // unwrap: we've checked the None case above
        let first_phrase = phrases.first().unwrap();
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
                yield FuzzyMatchList::new_exact(MatchPart::new(full_phrase.clone(), marker_hash).with_record(record));
                continue;
            } else if full_phrase.can_complete() && thing_name.starts_with_ci(full_phrase) {
                yield FuzzyMatchList::new_incomplete(
                    MatchPart::new(full_phrase.clone(), marker_hash).with_record(record),
                );

                continue;
            }

            for phrase in &phrases[0..phrases.len() - 1] {
                if thing_name.eq_ci(phrase) {
                    yield FuzzyMatchList::new_overflow(
                        MatchPart::new(phrase.clone(), marker_hash).with_record(record),
                        phrase.after(),
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

    use crate::command::token::constructors::*;
    use crate::test_utils as test;
    use uuid::Uuid;

    #[derive(Hash)]
    enum Marker {
        Token,
    }

    #[tokio::test]
    async fn match_input_test_simple() {
        let token = name().with_marker(Marker::Token);
        let app_meta = test::app_meta::with_test_data().await;
        let record = app_meta
            .repository
            .get_by_uuid(&test::npc::odysseus::UUID)
            .await
            .unwrap();

        test::assert_eq_unordered!(
            [FuzzyMatchList::new_exact(
                MatchPart::new_unmarked("Odysseus".into())
                    .with_marker(Marker::Token)
                    .with_record(record)
            )],
            match_input(&token, "Odysseus".into(), &app_meta,)
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_quoted() {
        let token = name();

        let uuids = [Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()];

        let things = [
            test::npc().name("Medium").build_thing(uuids[0]),
            test::npc().name("\"Medium\"").build_thing(uuids[1]),
            test::npc()
                .name("\"Medium\" Dave Lilywhite")
                .build_thing(uuids[2]),
        ];

        let app_meta = test::app_meta::with_data_store::memory::with(things, []);

        let records = (
            app_meta.repository.get_by_uuid(&uuids[0]).await.unwrap(),
            app_meta.repository.get_by_uuid(&uuids[1]).await.unwrap(),
            app_meta.repository.get_by_uuid(&uuids[2]).await.unwrap(),
        );

        test::assert_eq_unordered!(
            [
                FuzzyMatchList::new_overflow(
                    MatchPart::new_unmarked("Medium".into()).with_record(records.0),
                    " Dave Lily".into()
                ),
                FuzzyMatchList::new_overflow(
                    MatchPart::new_unmarked(r#""Medium""#.into()).with_record(records.1),
                    " Dave Lily".into(),
                ),
                FuzzyMatchList::new_incomplete(
                    MatchPart::new_unmarked(r#""Medium" Dave Lily"#.into()).with_record(records.2),
                ),
            ],
            match_input(&token, r#"  "Medium" Dave Lily"#.into(), &app_meta)
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_empty() {
        let token = name().with_marker(Marker::Token);

        test::assert_eq_unordered!(
            [FuzzyMatchList::new_incomplete(
                MatchPart::new_unmarked("".into())
                    .with_marker(Marker::Token)
                    .with_term("[name]"),
            ),],
            match_input(&token, "   ".into(), &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }
}
