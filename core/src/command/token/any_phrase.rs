use super::{Match, MatchType, Meta, Token, TokenType};

use crate::utils::quoted_phrases;

use std::pin::Pin;

use async_stream::stream;
use futures::prelude::*;

pub fn match_input<'a, M>(
    token: Token<'a, M>,
    input: &'a str,
) -> Pin<Box<dyn Stream<Item = MatchType<'a, M>> + 'a>>
where
    M: Clone,
{
    assert!(matches!(token.token_type, TokenType::AnyPhrase));

    Box::pin(stream! {
        let mut phrases = quoted_phrases(input).peekable();

        while let Some(phrase) = phrases.next() {
            let token_match = Match {
                token: token.clone(),
                phrase: phrase.clone(),
                meta: Meta::None,
            };

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
            marker: (),
        };

        assert_eq!(
            &[
                MatchType::Overflow(
                    Match {
                        token: token.clone(),
                        phrase: "Nott".into(),
                        meta: Meta::None,
                    },
                    " \"The Brave\" ",
                ),
                MatchType::Exact(Match {
                    token: token.clone(),
                    phrase: "Nott \"The Brave\"".into(),
                    meta: Meta::None,
                }),
            ][..],
            match_input(token, " Nott \"The Brave\" ")
                .collect::<Vec<_>>()
                .await,
        );
    }
}
