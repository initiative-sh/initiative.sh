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
    let tokens = if let Token::AnyOf { tokens } = token {
        tokens.iter().collect()
    } else {
        unreachable!();
    };

    match_input_with_tokens(token, input, app_meta, tokens)
}

pub fn match_input_with_tokens<'input, 'stream>(
    token: &'stream Token,
    input: Substr<'input>,
    app_meta: &'stream AppMeta,
    tokens: Vec<&'stream Token>,
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
                        match_input_with_tokens(token, overflow_part.clone(), app_meta, next_tokens)
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

    use crate::command::token::hash_marker;
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
            keyword_m(Marker::Badger, "badger"),
            keyword_m(Marker::Mushroom, "mushroom"),
            keyword_m(Marker::Snake, "snake"),
        ]);

        test::assert_eq_unordered!(
            [FuzzyMatchList::new_incomplete(
                MatchPart::new("mush".into(), hash_marker(Marker::Mushroom)).matching("mushroom")
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
            keyword_m(Marker::Badger, "badger"),
            any_word_m(Marker::AnyWord),
        ]);

        test::assert_eq_unordered!(
            [
                FuzzyMatchList::new_overflow(
                    MatchPart::new("BADGER".into(), hash_marker(Marker::Badger)).matching("badger"),
                    " badger".into(),
                ),
                FuzzyMatchList::new_overflow(
                    MatchPart::new("BADGER".into(), hash_marker(Marker::AnyWord)),
                    " badger".into(),
                ),
                FuzzyMatchList::new_exact(vec![
                    MatchPart::new("BADGER".into(), hash_marker(Marker::AnyWord)),
                    MatchPart::new("badger".into(), hash_marker(Marker::Badger)).matching("badger"),
                ]),
                FuzzyMatchList::new_exact(vec![
                    MatchPart::new("BADGER".into(), hash_marker(Marker::Badger)).matching("badger"),
                    MatchPart::new("badger".into(), hash_marker(Marker::AnyWord)),
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
            keyword_m(Marker::Badger, "badger"),
            keyword_m(Marker::Mushroom, "mushroom"),
            keyword_m(Marker::Snake, "snake"),
        ]);

        test::assert_eq_unordered!(
            [
                FuzzyMatchList::new_overflow(
                    MatchPart::new("badger".into(), hash_marker(Marker::Badger)).matching("badger"),
                    " mushroom".into(),
                ),
                FuzzyMatchList::new_exact(vec![
                    MatchPart::new("badger".into(), hash_marker(Marker::Badger)).matching("badger"),
                    MatchPart::new("mushroom".into(), hash_marker(Marker::Mushroom))
                        .matching("mushroom"),
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
                MatchPart::new("badger".into(), 0).matching("badger"),
                " badger".into(),
            )],
            token
                .match_input("badger badger", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }
}
