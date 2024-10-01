use super::{MatchType, Token, TokenType};

use crate::utils::quoted_words;
use crate::utils::CaseInsensitiveStr;

use std::pin::Pin;

use async_stream::stream;
use futures::prelude::*;

pub fn match_input<'a>(
    token: Token<'a>,
    input: &'a str,
) -> Pin<Box<dyn Stream<Item = MatchType<'a>> + 'a>> {
    let TokenType::Keyword(keyword) = token.token_type else {
        unreachable!();
    };

    Box::pin(stream! {
        let mut iter = quoted_words(input);
        if let Some(first_word) = iter.next() {
            if keyword.eq_ci(first_word.as_str()) {
                if first_word.is_at_end() {
                    yield MatchType::Exact(token.into());
                } else {
                    yield MatchType::Overflow(token.into(), &input[first_word.range().end..]);
                }
            } else if first_word.can_complete() {
                if let Some(completion) = keyword.strip_prefix_ci(first_word) {
                    yield MatchType::Partial(token.into(), Some(completion.to_string()));
                }
            }
        }
    })
}

#[cfg(test)]
mod test {
    use super::super::Match;
    use super::*;

    #[tokio::test]
    async fn match_input_test_exact() {
        let token = Token {
            token_type: TokenType::Keyword("Nott"),
            marker: Some(20),
        };

        assert_eq!(
            &[MatchType::Exact(token.clone().into())][..],
            match_input(token.clone(), "nott").collect::<Vec<_>>().await,
        );
    }

    #[tokio::test]
    async fn match_input_test_overflow() {
        let token = Token {
            token_type: TokenType::Keyword("Nott"),
            marker: Some(20),
        };

        assert_eq!(
            &[MatchType::Overflow(token.clone().into(), " \"the brave\"",)][..],
            match_input(token.clone(), "nott \"the brave\"")
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_partial() {
        let token = Token {
            token_type: TokenType::Keyword("Nott"),
            marker: Some(20),
        };

        assert_eq!(
            &[MatchType::Partial(
                token.clone().into(),
                Some("tt".to_string())
            )][..],
            match_input(token.clone(), " no").collect::<Vec<_>>().await,
        );

        assert_eq!(
            Vec::<MatchType>::new(),
            match_input(token.clone(), " no ").collect::<Vec<_>>().await,
        );

        assert_eq!(
            Vec::<MatchType>::new(),
            match_input(token.clone(), "\"no\"")
                .collect::<Vec<_>>()
                .await,
        );
    }
}
