use crate::command::prelude::*;
use crate::utils::{quoted_words, CaseInsensitiveStr};

use std::pin::Pin;

use async_stream::stream;
use futures::prelude::*;

pub fn match_input<'a>(
    token: &'a Token,
    input: &'a str,
) -> Pin<Box<dyn Stream<Item = FuzzyMatch<'a>> + 'a>> {
    #[expect(irrefutable_let_patterns)]
    let TokenType::Keyword(keyword) = token.token_type
    else {
        unreachable!();
    };

    Box::pin(stream! {
        let mut iter = quoted_words(input);
        if let Some(first_word) = iter.next() {
            if keyword.eq_ci(first_word.as_str()) {
                if first_word.is_at_end() {
                    yield FuzzyMatch::Exact(token.into());
                } else {
                    yield FuzzyMatch::Overflow(token.into(), first_word.after());
                }
            } else if first_word.can_complete() {
                if let Some(completion) = keyword.strip_prefix_ci(first_word) {
                    yield FuzzyMatch::Partial(token.into(), Some(completion.to_string()));
                }
            }
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test_utils as test;

    #[derive(TokenMarker)]
    enum Marker {
        Keyword,
    }

    #[tokio::test]
    async fn match_input_test_exact() {
        let token = keyword_m(Marker::Keyword, "badger");

        test::assert_eq_unordered!(
            [FuzzyMatch::Exact(TokenMatch::from(&token))],
            token
                .match_input("BADGER", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_overflow() {
        let token = keyword("badger");

        test::assert_eq_unordered!(
            [FuzzyMatch::Overflow(
                TokenMatch::from(&token),
                " badger".into(),
            )],
            token
                .match_input("badger badger", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_partial() {
        let token = keyword("badger");

        test::assert_eq_unordered!(
            [FuzzyMatch::Partial(
                TokenMatch::from(&token),
                Some("er".to_string()),
            )],
            token
                .match_input(" badg", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );

        test::assert_empty!(
            token
                .match_input(" badg ", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );

        test::assert_empty!(
            token
                .match_input(r#""badg""#, &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }
}
