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

    #[tokio::test]
    async fn match_input_test() {
        let token = Token {
            token_type: TokenType::AnyPhrase,
            marker: Some(20),
        };

        assert_eq!(
            &[
                FuzzyMatch::Overflow(TokenMatch::new(token.clone(), "Nott"), " \"The Brave\" "),
                FuzzyMatch::Exact(TokenMatch::new(token.clone(), "Nott \"The Brave\"")),
            ][..],
            match_input(token, " Nott \"The Brave\" ")
                .collect::<Vec<_>>()
                .await,
        );
    }
}
