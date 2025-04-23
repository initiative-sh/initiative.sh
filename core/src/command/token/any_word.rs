use crate::command::prelude::*;
use crate::utils::quoted_words;

use std::pin::Pin;

use async_stream::stream;
use futures::prelude::*;

pub fn match_input<'a>(
    token: &'a Token,
    input: &'a str,
) -> Pin<Box<dyn Stream<Item = FuzzyMatch<'a>> + 'a>> {
    assert!(matches!(token.token_type, TokenType::AnyWord));

    Box::pin(stream! {
        let mut iter = quoted_words(input);
        if let Some(word) = iter.next() {
            let token_match = TokenMatch::new(token, word.as_str());

            if word.is_at_end() {
                yield FuzzyMatch::Exact(token_match);
            } else {
                yield FuzzyMatch::Overflow(token_match, word.after());
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
        Token,
    }

    #[tokio::test]
    async fn match_input_test_exact() {
        let token = any_word();

        test::assert_eq_unordered!(
            [FuzzyMatch::Exact(TokenMatch::new(&token, "Jesta"))],
            token
                .match_input("Jesta", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_overflow() {
        let token = any_word_m(Marker::Token);

        test::assert_eq_unordered!(
            [FuzzyMatch::Overflow(
                TokenMatch::new(&token, "Nott"),
                " \"The Brave\" ".into()
            )],
            token
                .match_input(" Nott \"The Brave\" ", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }
}
