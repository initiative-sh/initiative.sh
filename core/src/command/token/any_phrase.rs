use super::{Match, MatchType, Token, TokenType};

use crate::utils::quoted_phrases;

use std::pin::Pin;

use async_stream::stream;
use futures::prelude::*;

pub fn match_input<'a>(
    token: Token<'a>,
    input: &'a str,
) -> Pin<Box<dyn Stream<Item = MatchType<'a>> + 'a>> {
    assert!(matches!(token.token_type, TokenType::AnyPhrase));

    Box::pin(stream! {
        let mut phrases = quoted_phrases(input).peekable();

        while let Some(phrase) = phrases.next() {
            let token_match = Match::new(token.clone(), phrase.as_own_str(input));

            if phrases.peek().is_none() {
                yield MatchType::Exact(token_match);
            } else {
                yield MatchType::Overflow(token_match, &input[phrase.range().end..]);
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
                MatchType::Overflow(Match::new(token.clone(), "Nott"), " \"The Brave\" "),
                MatchType::Exact(Match::new(token.clone(), "Nott \"The Brave\"")),
            ][..],
            match_input(token, " Nott \"The Brave\" ")
                .collect::<Vec<_>>()
                .await,
        );
    }
}
