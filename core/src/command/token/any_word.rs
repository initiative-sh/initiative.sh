use super::{Match, MatchType, Meta, Token, TokenType};

use crate::utils::quoted_words;

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
    assert!(matches!(token.token_type, TokenType::AnyWord));

    Box::pin(stream! {
        let mut iter = quoted_words(input);
        if let Some(word) = iter.next() {
            let token_match = Match {
                token,
                phrase: word.clone(),
                meta: Meta::None,
            };

            if iter.next().is_some() {
                yield MatchType::Overflow(token_match, &input[word.range().end..]);
            } else {
                yield MatchType::Exact(token_match);
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
            token_type: TokenType::AnyWord,
            marker: (),
        };

        assert_eq!(
            &[MatchType::Exact(Match {
                token: &token,
                phrase: "Jesta",
                meta: Meta::None,
            })][..],
            match_input(token, "Jesta").collect::<Vec<_>>().await,
        );

        assert_eq!(
            &[MatchType::Overflow(
                Match {
                    token: &token,
                    phrase: "Nott",
                    meta: Meta::None,
                },
                " \"The Brave\" ",
            )][..],
            match_input(token, " Nott \"The Brave\" ")
                .collect::<Vec<_>>()
                .await,
        );
    }
}
