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
    let tokens = if let Token::AnyOf { tokens } = token {
        tokens.iter().collect()
    } else {
        unreachable!();
    };

    match_input_with_tokens(token, input, app_meta, tokens)
}

pub fn match_input_with_tokens<'a, 'b>(
    token: &'a Token,
    input: &'a str,
    app_meta: &'b AppMeta,
    tokens: Vec<&'a Token>,
) -> Pin<Box<dyn Stream<Item = FuzzyMatch<'a>> + 'b>>
where
    'a: 'b,
{
    Box::pin(stream! {
        // Attempt to match each token in turn
        for (test_token_index, test_token) in tokens.iter().enumerate() {
            for await fuzzy_match in test_token.match_input(input, app_meta) {
                match fuzzy_match {
                    // The token is a partial match, so the phrase is incomplete.
                    FuzzyMatch::Partial(token_match, completion) => {
                        yield FuzzyMatch::Partial(
                            TokenMatch::new(token, vec![token_match]),
                            completion,
                        );
                    }

                    // First token is an exact match, so the phrase is also an exact match.
                    FuzzyMatch::Exact(token_match) => {
                        yield FuzzyMatch::Exact(TokenMatch::new(token, vec![token_match]));
                    }

                    // First token overflows and is the only token in the phrase, so the phrase
                    // also overflows.
                    FuzzyMatch::Overflow(token_match, remainder) if tokens.len() == 1 => {
                        yield FuzzyMatch::Overflow(
                            TokenMatch::new(token, vec![token_match]),
                            remainder,
                        );
                    }

                    // First token overflows but there are other tokens in the phrase, so we
                    // recurse with the remainder of the phrase.
                    FuzzyMatch::Overflow(token_match, remainder) => {
                        let remainder_str = remainder.as_str();

                        yield FuzzyMatch::Overflow(
                            TokenMatch::new(token, vec![token_match.clone()]),
                            remainder,
                        );

                        let next_tokens: Vec<_> = tokens
                            .iter()
                            .take(test_token_index)
                            .chain(tokens.iter().skip(test_token_index + 1))
                            .copied()
                            .collect();

                        for await next_fuzzy_match in
                            match_input_with_tokens(token, remainder_str, app_meta, next_tokens)
                        {
                            yield next_fuzzy_match.map(|next_match| {
                                let mut next_meta_sequence = next_match.match_meta.into_sequence().unwrap();
                                next_meta_sequence.insert(0, token_match.clone());

                                TokenMatch::new(token, next_meta_sequence)
                            });
                        }
                    }
                }
            }
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test_utils as test;

    #[derive(Hash)]
    enum Marker {
        Keyword,
        AnyWord,
    }

    #[tokio::test]
    async fn match_input_test_partial() {
        let tokens = [keyword("badger"), keyword("mushroom"), keyword("snake")];
        let keyword_token = tokens[1].clone();
        let any_of_token = any_of(tokens);

        test::assert_eq_unordered!(
            [FuzzyMatch::Partial(
                TokenMatch::new(&any_of_token, vec![TokenMatch::from(&keyword_token)]),
                Some("room".to_string())
            )],
            any_of_token
                .match_input("mush", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_exact() {
        let tokens = [
            keyword_m(Marker::Keyword, "badger"),
            any_word_m(Marker::AnyWord),
        ];
        let [keyword_token, any_word_token] = tokens.clone();

        let any_of_token = any_of(tokens);

        test::assert_eq_unordered!(
            [
                FuzzyMatch::Overflow(
                    TokenMatch::new(&any_of_token, vec![TokenMatch::from(&keyword_token)]),
                    " badger".into(),
                ),
                FuzzyMatch::Overflow(
                    TokenMatch::new(
                        &any_of_token,
                        vec![TokenMatch::new(&any_word_token, "badger")],
                    ),
                    " badger".into(),
                ),
                FuzzyMatch::Exact(TokenMatch::new(
                    &any_of_token,
                    vec![
                        TokenMatch::from(&keyword_token),
                        TokenMatch::new(&any_word_token, "badger"),
                    ],
                )),
                FuzzyMatch::Exact(TokenMatch::new(
                    &any_of_token,
                    vec![
                        TokenMatch::new(&any_word_token, "badger"),
                        TokenMatch::from(&keyword_token),
                    ],
                )),
            ],
            any_of_token
                .match_input("badger badger", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_exact_overflow() {
        let tokens = [keyword("badger"), keyword("mushroom"), keyword("snake")];
        let [badger_token, mushroom_token, _] = tokens.clone();
        let any_of_token = any_of(tokens);

        test::assert_eq_unordered!(
            [
                FuzzyMatch::Overflow(
                    TokenMatch::new(&any_of_token, vec![TokenMatch::from(&badger_token)]),
                    " mushroom".into(),
                ),
                FuzzyMatch::Exact(TokenMatch::new(
                    &any_of_token,
                    vec![
                        TokenMatch::from(&badger_token),
                        TokenMatch::from(&mushroom_token)
                    ]
                )),
            ],
            any_of_token
                .match_input("badger mushroom", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_overflow() {
        let badger_token = keyword("badger");
        let any_of_token = any_of([badger_token.clone()]);

        test::assert_eq_unordered!(
            [FuzzyMatch::Overflow(
                TokenMatch::new(&any_of_token, vec![TokenMatch::from(&badger_token)]),
                " badger".into(),
            )],
            any_of_token
                .match_input("badger badger", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }
}
