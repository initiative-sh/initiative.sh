use crate::command::prelude::*;
use crate::utils::quoted_phrases;

use std::pin::Pin;

use async_stream::stream;
use futures::prelude::*;

pub fn match_input<'a>(
    token: &'a Token,
    input: &'a str,
) -> Pin<Box<dyn Stream<Item = FuzzyMatch<'a>> + 'a>> {
    assert!(matches!(token.token_type, TokenType::AnyPhrase));

    Box::pin(stream! {
        let mut phrases = quoted_phrases(input).peekable();

        while let Some(phrase) = phrases.next() {
            let token_match = TokenMatch::new(token, phrase.as_str());

            if phrases.peek().is_none() {
                yield FuzzyMatch::Exact(token_match);
            } else {
                yield FuzzyMatch::Overflow(token_match, phrase.after());
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
    async fn match_input_test_empty() {
        test::assert_empty!(
            any_phrase()
                .match_input("  ", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_simple() {
        let token = any_phrase();

        test::assert_eq_unordered!(
            [
                FuzzyMatch::Overflow(TokenMatch::new(&token, "badger"), " badger badger".into()),
                FuzzyMatch::Overflow(TokenMatch::new(&token, "badger badger"), " badger".into()),
                FuzzyMatch::Exact(TokenMatch::new(&token, "badger badger badger")),
            ],
            token
                .match_input("badger badger badger", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_quoted() {
        let token = any_phrase_m(Marker::Token);

        test::assert_eq_unordered!(
            [
                FuzzyMatch::Overflow(TokenMatch::new(&token, "Nott"), " \"The Brave\" ".into()),
                FuzzyMatch::Exact(TokenMatch::new(&token, "Nott \"The Brave\"")),
            ],
            token
                .match_input(" Nott \"The Brave\" ", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }
}
