use crate::app::AppMeta;
use crate::command::prelude::*;
use crate::utils::{quoted_words, Substr};

use std::pin::Pin;

use async_stream::stream;
use futures::prelude::*;

pub fn match_input<'input, 'stream>(
    token: &'stream Token,
    input: Substr<'input>,
    app_meta: &'stream AppMeta,
) -> Pin<Box<dyn Stream<Item = FuzzyMatchList<'input>> + 'stream>>
where
    'input: 'stream,
{
    let Token::Sequence { tokens } = token else {
        unreachable!();
    };

    match_input_with_tokens(tokens, input, app_meta)
}

pub fn match_input_with_tokens<'input, 'stream>(
    tokens: &'stream [Token],
    input: Substr<'input>,
    app_meta: &'stream AppMeta,
) -> Pin<Box<dyn Stream<Item = FuzzyMatchList<'input>> + 'stream>>
where
    'input: 'stream,
{
    if tokens.is_empty() {
        Box::pin(stream::once(future::ready(
            if quoted_words(input.clone()).next().is_some() {
                FuzzyMatchList::new_overflow(MatchList::default(), input)
            } else {
                FuzzyMatchList::new_exact(MatchList::default())
            },
        )))
    } else {
        Box::pin(stream! {
            // TokenMatch the first token in the sequence
            for await result in tokens[0].match_input(input, app_meta) {
                if tokens.len() > 1 {
                    let overflow_part = match &result.extra {
                        Some(FuzzyMatchPart::Overflow(overflow_part)) => {
                            overflow_part.clone()
                        }
                        Some(FuzzyMatchPart::Incomplete(_)) => {
                            yield result;
                            continue;
                        }
                        None => {
                            "".into()
                        }
                    };

                    for await next_result in
                        match_input_with_tokens(&tokens[1..], overflow_part, app_meta)
                    {
                        yield next_result.prepend(result.match_list.clone());
                    }
                } else {
                    yield result;
                }
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test_utils as test;

    #[derive(Hash)]
    enum Marker {
        Keyword,
        AnyPhrase,
        AnyWord,
    }

    #[tokio::test]
    async fn match_input_test_empty() {
        let sequence_token = sequence([]);
        test::assert_eq_unordered!(
            [FuzzyMatchList::new_overflow(
                MatchList::default(),
                "badger".into()
            )],
            sequence_token
                .match_input("badger", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_exact() {
        let tokens = [keyword("badger"), keyword("mushroom"), keyword("snake")];
        let sequence_token = sequence(tokens.clone());

        test::assert_eq_unordered!(
            [FuzzyMatchList::new_exact(vec![
                MatchPart::new_unmarked("BADGER".into()).with_term("badger"),
                MatchPart::new_unmarked("MUSHROOM".into()).with_term("mushroom"),
                MatchPart::new_unmarked("SNAKE".into()).with_term("snake"),
            ])],
            sequence_token
                .match_input("BADGER MUSHROOM SNAKE", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_incomplete() {
        let token = sequence([keyword("badger"), keyword("mushroom"), keyword("snake")]);

        test::assert_eq_unordered!(
            [FuzzyMatchList::new_incomplete_multi(
                vec![MatchPart::new_unmarked("BADGER".into()).with_term("badger")],
                MatchPart::new_unmarked("".into()).with_term("mushroom"),
            )],
            token
                .match_input("BADGER", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_partial() {
        let token = sequence([keyword("badger"), keyword("mushroom"), keyword("snake")]);

        test::assert_eq_unordered!(
            [FuzzyMatchList::new_incomplete_multi(
                vec![
                    MatchPart::new_unmarked("badger".into()).with_term("badger"),
                    MatchPart::new_unmarked("mushroom".into()).with_term("mushroom"),
                ],
                MatchPart::new_unmarked("sn".into()).with_term("snake"),
            )],
            token
                .match_input("badger mushroom sn", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_overflowing() {
        let token = sequence([keyword("badger"), keyword("mushroom"), keyword("snake")]);

        test::assert_eq_unordered!(
            [FuzzyMatchList::new_overflow(
                vec![
                    MatchPart::new_unmarked("badger".into()).with_term("badger"),
                    MatchPart::new_unmarked("mushroom".into()).with_term("mushroom"),
                    MatchPart::new_unmarked("snake".into()).with_term("snake"),
                ],
                " hippopotumus".into(),
            )],
            token
                .match_input("badger mushroom snake hippopotumus", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_with_any_phrase() {
        let token = sequence([
            keyword_m(Marker::Keyword, "badger"),
            any_phrase_m(Marker::AnyPhrase),
            any_word_m(Marker::AnyWord),
        ]);

        test::assert_eq_unordered!(
            [
                FuzzyMatchList::new_overflow(
                    vec![
                        MatchPart::new_unmarked("badger".into())
                            .with_marker(Marker::Keyword)
                            .with_term("badger"),
                        MatchPart::new_unmarked("badger".into()).with_marker(Marker::AnyPhrase),
                        MatchPart::new_unmarked("badger".into()).with_marker(Marker::AnyWord),
                    ],
                    " mushroom".into(),
                ),
                FuzzyMatchList::new_exact(vec![
                    MatchPart::new_unmarked("badger".into())
                        .with_marker(Marker::Keyword)
                        .with_term("badger"),
                    MatchPart::new_unmarked("badger badger".into()).with_marker(Marker::AnyPhrase),
                    MatchPart::new_unmarked("mushroom".into()).with_marker(Marker::AnyWord),
                ],),
                FuzzyMatchList::new_incomplete_multi(
                    vec![
                        MatchPart::new_unmarked("badger".into())
                            .with_marker(Marker::Keyword)
                            .with_term("badger"),
                        MatchPart::new_unmarked("badger badger mushroom".into())
                            .with_marker(Marker::AnyPhrase),
                    ],
                    MatchPart::new_unmarked("".into()).with_marker(Marker::AnyWord),
                ),
            ],
            token
                .match_input("badger badger badger mushroom", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }
}
