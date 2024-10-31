use crate::command::prelude::*;
use crate::utils::{quoted_words, CaseInsensitiveStr};

use std::pin::Pin;

use async_stream::stream;
use futures::prelude::*;

pub fn match_input<'a>(
    token: &'a Token,
    input: &'a str,
) -> Pin<Box<dyn Stream<Item = FuzzyMatch<'a>> + 'a>> {
    #[expect(irrefutable_let_patterns)]
    let TokenType::Keyword(keyword) = token.token_type
    else {
        unreachable!();
    };

    Box::pin(stream! {
        let mut iter = quoted_words(input);
        if let Some(first_word) = iter.next() {
            if keyword.eq_ci(first_word.as_str()) {
                if first_word.is_at_end() {
                    yield FuzzyMatch::Exact(token.into());
                } else {
                    yield FuzzyMatch::Overflow(token.into(), first_word.after());
                }
            } else if first_word.can_complete() {
                if let Some(completion) = keyword.strip_prefix_ci(first_word) {
                    yield FuzzyMatch::Partial(token.into(), Some(completion.to_string()));
                }
            }
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::app::{AppMeta, Event};
    use crate::storage::NullDataStore;

    #[tokio::test]
    async fn match_input_test_exact() {
        let token = Token {
            token_type: TokenType::Keyword("Nott"),
            marker: Some(20),
        };

        assert_eq!(
            &[FuzzyMatch::Exact((&token).into())][..],
            token
                .match_input("nott", &app_meta())
                .collect::<Vec<_>>()
                .await,
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
                (&token).into(),
                " \"the brave\"".into(),
            )][..],
            token
                .match_input("nott \"the brave\"", &app_meta())
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
            &[FuzzyMatch::Partial((&token).into(), Some("tt".to_string()))][..],
            token
                .match_input(" no", &app_meta())
                .collect::<Vec<_>>()
                .await,
        );

        assert_eq!(
            Vec::<FuzzyMatch>::new(),
            token
                .match_input(" no ", &app_meta())
                .collect::<Vec<_>>()
                .await,
        );

        assert_eq!(
            Vec::<FuzzyMatch>::new(),
            token
                .match_input("\"no\"", &app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    fn app_meta() -> AppMeta {
        AppMeta::new(NullDataStore, &event_dispatcher)
    }

    fn event_dispatcher(_event: Event) {}
}
