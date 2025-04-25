use super::TokenKind;
use crate::app::AppMeta;
use crate::command::prelude::*;
use crate::utils::Substr;

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
    let tokens = if let TokenKind::AnyOf { tokens } = &token.kind {
        tokens.iter().collect()
    } else {
        unreachable!();
    };

    match_input_with_tokens(tokens, input, app_meta)
}

pub fn match_input_with_tokens<'input, 'stream>(
    tokens: Vec<&'stream Token>,
    input: Substr<'input>,
    app_meta: &'stream AppMeta,
) -> Pin<Box<dyn Stream<Item = FuzzyMatchList<'input>> + 'stream>>
where
    'input: 'stream,
{
    Box::pin(stream! {
        // Attempt to match each token in turn
        for (test_token_index, test_token) in tokens.iter().enumerate() {
            for await result in test_token.match_input(input.clone(), app_meta) {
                if let Some(FuzzyMatchPart::Overflow(overflow_part)) = &result.extra {
                    let mut next_tokens = tokens.clone();
                    next_tokens.swap_remove(test_token_index);

                    for await next_result in
                        match_input_with_tokens(next_tokens, overflow_part.clone(), app_meta)
                    {
                        yield next_result.prepend(result.match_list.clone());
                    }
                }

                yield result;
            }
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils as test;

    #[derive(Hash)]
    enum Marker {
        AnyWord,
        Badger,
        Mushroom,
        Snake,
    }

    #[tokio::test]
    async fn match_input_test_partial() {
        let token = any_of([
            keyword("badger").with_marker(Marker::Badger),
            keyword("mushroom").with_marker(Marker::Mushroom),
            keyword("snake").with_marker(Marker::Snake),
        ]);

        test::assert_eq_unordered!(
            [FuzzyMatchList::new_incomplete(
                MatchPart::new_unmarked("mush".into())
                    .with_marker(Marker::Mushroom)
                    .with_term("mushroom")
            )],
            token
                .match_input("mush", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_exact() {
        let token = any_of([
            keyword("badger").with_marker(Marker::Badger),
            any_word().with_marker(Marker::AnyWord),
        ]);

        test::assert_eq_unordered!(
            [
                FuzzyMatchList::new_overflow(
                    MatchPart::new_unmarked("BADGER".into())
                        .with_marker(Marker::Badger)
                        .with_term("badger"),
                    " badger".into(),
                ),
                FuzzyMatchList::new_overflow(
                    MatchPart::new_unmarked("BADGER".into()).with_marker(Marker::AnyWord),
                    " badger".into(),
                ),
                FuzzyMatchList::new_exact(vec![
                    MatchPart::new_unmarked("BADGER".into()).with_marker(Marker::AnyWord),
                    MatchPart::new_unmarked("badger".into())
                        .with_marker(Marker::Badger)
                        .with_term("badger"),
                ]),
                FuzzyMatchList::new_exact(vec![
                    MatchPart::new_unmarked("BADGER".into())
                        .with_marker(Marker::Badger)
                        .with_term("badger"),
                    MatchPart::new_unmarked("badger".into()).with_marker(Marker::AnyWord),
                ]),
            ],
            token
                .match_input("BADGER badger", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_exact_overflow() {
        let token = any_of([
            keyword("badger").with_marker(Marker::Badger),
            keyword("mushroom").with_marker(Marker::Mushroom),
            keyword("snake").with_marker(Marker::Snake),
        ]);

        test::assert_eq_unordered!(
            [
                FuzzyMatchList::new_overflow(
                    MatchPart::new_unmarked("badger".into())
                        .with_marker(Marker::Badger)
                        .with_term("badger"),
                    " mushroom".into(),
                ),
                FuzzyMatchList::new_exact(vec![
                    MatchPart::new_unmarked("badger".into())
                        .with_marker(Marker::Badger)
                        .with_term("badger"),
                    MatchPart::new_unmarked("mushroom".into())
                        .with_marker(Marker::Mushroom)
                        .with_term("mushroom"),
                ])
            ],
            token
                .match_input("badger mushroom", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_overflow() {
        let token = any_of([keyword("badger")]);

        test::assert_eq_unordered!(
            [FuzzyMatchList::new_overflow(
                MatchPart::new_unmarked("badger".into()).with_term("badger"),
                " badger".into(),
            )],
            token
                .match_input("badger badger", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }
}
