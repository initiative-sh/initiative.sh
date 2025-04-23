use crate::app::AppMeta;
use crate::command::prelude::*;

use std::pin::Pin;

use async_stream::stream;
use futures::prelude::*;

pub fn match_input<'a, 'b>(
    token: &'a Token,
    input: &'a str,
    app_meta: &'b AppMeta,
) -> Pin<Box<dyn Stream<Item = FuzzyMatch<'a>> + 'b>>
where
    'a: 'b,
{
    Box::pin(stream! {
        let TokenType::Or(tokens) = &token.token_type else {
            unreachable!();
        };

        let streams = tokens.iter().map(|token| token.match_input(input, app_meta));
        for await fuzzy_match in stream::select_all(streams) {
            yield fuzzy_match.map(|token_match| TokenMatch::new(token, token_match));
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test_utils as test;

    #[derive(Hash)]
    enum Marker {
        Or,
        AnyWord,
        Keyword,
    }

    #[tokio::test]
    async fn match_input_test_simple() {
        let tokens = [
            any_word_m(Marker::AnyWord),
            keyword_m(Marker::Keyword, "badger"),
        ];
        let [any_word_token, keyword_token] = tokens.clone();
        let or_token = or_m(Marker::Or, tokens);

        test::assert_eq_unordered!(
            [
                FuzzyMatch::Overflow(
                    TokenMatch::new(&or_token, TokenMatch::new(&any_word_token, "badger")),
                    " badger".into(),
                ),
                FuzzyMatch::Overflow(
                    TokenMatch::new(&or_token, TokenMatch::from(&keyword_token)),
                    " badger".into(),
                ),
            ],
            or_token
                .match_input("badger badger", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_partial() {
        let tokens = [keyword("badger"), keyword("badgering")];
        let [badger_token, badgering_token] = tokens.clone();
        let or_token = or(tokens);

        test::assert_eq_unordered!(
            [
                FuzzyMatch::Exact(TokenMatch::new(&or_token, TokenMatch::from(&badger_token))),
                FuzzyMatch::Partial(
                    TokenMatch::new(&or_token, TokenMatch::from(&badgering_token)),
                    Some("ing".to_string()),
                ),
            ],
            or_token
                .match_input("badger", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }
}
