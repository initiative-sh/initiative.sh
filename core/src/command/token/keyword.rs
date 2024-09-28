use super::{Match, MatchType, Meta, Token, TokenType};

use crate::utils::quoted_words;
use crate::utils::CaseInsensitiveStr;

use std::pin::Pin;

use async_stream::stream;
use futures::prelude::*;

pub fn match_input<'a, M>(
    token: &'a Token<M>,
    input: &'a str,
) -> Pin<Box<dyn Stream<Item = MatchType<'a, M>> + 'a>>
where
    M: Clone,
{
    let TokenType::Keyword(keyword) = token.token_type else {
        unreachable!();
    };

    Box::pin(stream! {
        let mut iter = quoted_words(input);
        if let Some(first_word) = iter.next() {
            let token_match = Match {
                token,
                phrase: first_word.clone(),
                meta: Meta::None,
            };

            if keyword.eq_ci(first_word.as_str()) {
                if iter.next().is_some() {
                    yield MatchType::Overflow(token_match, &input[first_word.range().end..]);
                } else {
                    yield MatchType::Exact(token_match);
                }
            } else if first_word.completes_to_ci(keyword) {
                yield MatchType::Partial(token_match);
            }
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn match_input_test_exact() {
        let token = Token {
            token_type: TokenType::Word("Nott"),
            marker: (),
        };

        assert_eq!(
            &[MatchType::Exact(Match {
                token: &token,
                phrase: "nott",
                meta: Meta::None,
            })][..],
            match_input(&token, "nott").collect::<Vec<_>>().await,
        );
    }

    #[tokio::test]
    async fn match_input_test_overflow() {
        let token = Token {
            token_type: TokenType::Word("Nott"),
            marker: (),
        };

        assert_eq!(
            &[MatchType::Overflow(
                Match {
                    token: &token,
                    phrase: "nott",
                    meta: Meta::None,
                },
                " \"the brave\"",
            )][..],
            match_input(&token, "nott \"the brave\"")
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_partial() {
        let token = Token {
            token_type: TokenType::Word("Nott"),
            marker: (),
        };

        assert_eq!(
            &[MatchType::Partial(Match {
                token: &token,
                phrase: "no",
                meta: Meta::None,
            },)][..],
            match_input(&token, " no").collect::<Vec<_>>().await,
        );

        assert_eq!(
            Vec::<MatchType<()>>::new(),
            match_input(&token, " no ").collect::<Vec<_>>().await,
        );

        assert_eq!(
            Vec::<MatchType<()>>::new(),
            match_input(&token, "\"no\"").collect::<Vec<_>>().await,
        );
    }
}
