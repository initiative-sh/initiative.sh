use crate::app::AppMeta;
use crate::command::prelude::*;
use crate::utils::quoted_words;

use std::pin::Pin;

use futures::prelude::*;

pub fn match_input<'a, 'b>(
    token: &'a Token,
    input: &'a str,
    app_meta: &'b AppMeta,
) -> Pin<Box<dyn Stream<Item = FuzzyMatch<'a>> + 'b>>
where
    'a: 'b,
{
    let TokenType::Optional(optional_token) = &token.token_type else {
        unreachable!();
    };

    Box::pin(
        stream::iter([if quoted_words(input).next().is_none() {
            FuzzyMatch::Exact(token.into())
        } else {
            FuzzyMatch::Overflow(token.into(), input.into())
        }])
        .chain(
            optional_token
                .match_input(input, app_meta)
                .map(|fuzzy_match| {
                    fuzzy_match.map(|token_match| TokenMatch::new(token, token_match))
                }),
        ),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test_utils as test;

    #[derive(Hash)]
    enum Marker {
        Optional,
        Keyword,
    }

    #[tokio::test]
    async fn match_input_test_simple() {
        let keyword_token = keyword_m(Marker::Keyword, "badger");
        let optional_token = optional_m(Marker::Optional, keyword_token.clone());

        test::assert_eq_unordered!(
            [
                FuzzyMatch::Exact(TokenMatch::new(
                    &optional_token,
                    TokenMatch::from(&keyword_token)
                )),
                FuzzyMatch::Overflow(TokenMatch::from(&optional_token), "badger".into()),
            ],
            optional_token
                .match_input("badger", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_empty() {
        let token = optional(keyword("badger"));

        test::assert_eq_unordered!(
            [FuzzyMatch::Exact(TokenMatch::from(&token))],
            token
                .match_input("   ", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }
}
