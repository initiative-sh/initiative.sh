use crate::command::prelude::*;
use crate::utils::{quoted_words, CaseInsensitiveStr};

use std::pin::Pin;

use async_stream::stream;
use futures::prelude::*;

pub fn match_input<'a>(
    token: &'a Token,
    input: &'a str,
) -> Pin<Box<dyn Stream<Item = FuzzyMatch<'a>> + 'a>> {
    let TokenType::KeywordList(keywords) = &token.token_type else {
        unreachable!();
    };

    Box::pin(stream! {
        let mut iter = quoted_words(input).peekable();
        if let Some(first_word) = iter.next() {
            for &keyword in keywords.iter() {
                if keyword.eq_ci(first_word.as_str()) {
                    if iter.peek().is_none() {
                        yield FuzzyMatch::Exact(TokenMatch::new(token, keyword));
                    } else {
                        yield FuzzyMatch::Overflow(
                            TokenMatch::new(token, keyword),
                            first_word.after(),
                        );
                    }
                } else if first_word.can_complete() {
                    if let Some(completion) = keyword.strip_prefix_ci(&first_word) {
                        yield FuzzyMatch::Partial(
                            TokenMatch::new(token, keyword),
                            Some(completion.to_string()),
                        );
                    }
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_utils as test;

    #[derive(Hash)]
    enum Marker {
        Token,
    }

    #[tokio::test]
    async fn match_input_test_no_complete() {
        let token = keyword_list(["badger", "mushroom", "snake", "badgering"]);

        test::assert_eq_unordered!(
            [FuzzyMatch::Exact(TokenMatch::new(&token, "badger"))],
            token
                .match_input("badger ", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_partial() {
        let token = keyword_list_m(Marker::Token, ["polyp", "POLYPHEMUS"]);

        test::assert_eq_unordered!(
            [
                FuzzyMatch::Exact(TokenMatch::new(&token, "polyp")),
                FuzzyMatch::Partial(
                    TokenMatch::new(&token, "POLYPHEMUS"),
                    Some("HEMUS".to_string())
                ),
            ],
            token
                .match_input("polyp", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_overflow() {
        let token = keyword_list(["badger", "mushroom"]);

        test::assert_eq_unordered!(
            [FuzzyMatch::Overflow(
                TokenMatch::new(&token, "badger"),
                " mushroom".into(),
            )],
            token
                .match_input("badger mushroom", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }
}
