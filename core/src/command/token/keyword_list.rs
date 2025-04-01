use crate::command::prelude::*;
use crate::utils::{quoted_phrases, CaseInsensitiveStr};

use std::pin::Pin;

use async_stream::stream;
use futures::prelude::*;

pub fn match_input<'a>(
    token: &'a Token,
    input: &'a str,
) -> Pin<Box<dyn Stream<Item = FuzzyMatch<'a>> + 'a>> {
    Box::pin(stream! {
        let TokenType::KeywordList(keywords) = &token.token_type else {
            unreachable!();
        };

        for phrase in quoted_phrases(input) {
            for keyword in keywords.iter() {
                if keyword.eq_ci(phrase.as_str()) {
                    if phrase.is_at_end() {
                        yield FuzzyMatch::Exact(token.into());
                    } else {
                        yield FuzzyMatch::Overflow(token.into(), phrase.after());
                    }
                } else if phrase.can_complete() {
                    if let Some(completion) = keyword.strip_prefix_ci(&phrase) {
                        yield FuzzyMatch::Partial(token.into(), Some(completion.to_string()));
                    }
                }
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
            token_type: TokenType::Keyword("Nott"),
            marker: Some(20),
        };

        assert_eq!(
            &[FuzzyMatch::Exact(token.into())][..],
            match_input(&token, "nott").collect::<Vec<_>>().await,
        );
    }

    #[tokio::test]
    async fn match_input_test_overflow() {
        let token = Token {
            token_type: TokenType::Keyword("Nott"),
            marker: Some(20),
        };

        assert_eq!(
            &[FuzzyMatch::Overflow(
                TokenMatch::from(&token),
                " \"the brave\""
            )][..],
            match_input(&token, "nott \"the brave\"")
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
            &[FuzzyMatch::Partial(
                TokenMatch::from(&token),
                Some("tt".to_string())
            )][..],
            match_input(&token, " no").collect::<Vec<_>>().await,
        );

        assert_eq!(
            Vec::<FuzzyMatch>::new(),
            match_input(&token, " no ").collect::<Vec<_>>().await,
        );

        assert_eq!(
            Vec::<FuzzyMatch>::new(),
            match_input(&token, "\"no\"").collect::<Vec<_>>().await,
        );
    }
}
