use crate::app::AppMeta;
use crate::command::prelude::*;

use std::iter;
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
    let tokens = if let TokenType::Sequence(tokens) = &token.token_type {
        tokens
    } else {
        unreachable!();
    };

    match_input_with_tokens(token, input, app_meta, tokens)
}

pub fn match_input_with_tokens<'a, 'b>(
    token: &'a Token,
    input: &'a str,
    app_meta: &'b AppMeta,
    tokens: &'a [Token],
) -> Pin<Box<dyn Stream<Item = FuzzyMatch<'a>> + 'b>>
where
    'a: 'b,
{
    if tokens.is_empty() {
        // No tokens, no matches
        Box::pin(stream::iter([FuzzyMatch::Overflow(
            token.into(),
            input.into(),
        )]))
    } else {
        Box::pin(stream! {
            // TokenMatch the first token in the sequence
            for await fuzzy_match in tokens[0].match_input(input, app_meta) {
                match fuzzy_match {

                    // First token is a partial match, so the sequence is incomplete
                    FuzzyMatch::Partial(token_match, completion) => {
                        yield FuzzyMatch::Partial(
                            TokenMatch::new(token, vec![token_match]),
                            completion,
                        );
                    }

                    // First token is an exact match and is the only token in the sequence, so the
                    // sequence is also an exact match.
                    FuzzyMatch::Exact(token_match) if tokens.len() == 1 => {
                        yield FuzzyMatch::Exact(
                            TokenMatch::new(token, vec![token_match]),
                        );
                    }

                    // First token is an exact match but there are more unmatched tokens, so the
                    // sequence is incomplete.
                    FuzzyMatch::Exact(token_match) => {
                        yield FuzzyMatch::Partial(
                            TokenMatch::new(token, vec![token_match]),
                            None,
                        );
                    }

                    // First token overflows and is the only token in the sequence, so the sequence
                    // also overflows.
                    FuzzyMatch::Overflow(token_match, remainder) if tokens.len() == 1 => {
                        yield FuzzyMatch::Overflow(
                            TokenMatch::new(token, vec![token_match]),
                            remainder,
                        );
                    }

                    // First token overflows but there are other tokens in the sequence, so we
                    // recurse with the remainder of the sequence.
                    FuzzyMatch::Overflow(token_match, remainder) => {
                        for await next_fuzzy_match in match_input_with_tokens(token, remainder.as_str(), app_meta, &tokens[1..]) {
                            yield next_fuzzy_match.map(|next_match| {
                                TokenMatch::new(
                                    token,
                                    iter::once(token_match.clone())
                                        .chain(
                                            next_match
                                                .match_meta
                                                .into_sequence()
                                                .unwrap()
                                                .into_iter(),
                                        )
                                        .collect::<Vec<_>>(),
                                )
                            });
                        }
                    }
                }
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test_utils as test;

    #[derive(Hash)]
    enum Marker {
        Token,
    }

    #[tokio::test]
    async fn match_input_test_empty() {
        let sequence_token = sequence_m(Marker::Token, []);
        test::assert_eq_unordered!(
            [FuzzyMatch::Overflow(
                TokenMatch::from(&sequence_token),
                "badger".into()
            )],
            sequence_token
                .match_input("badger", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_exact() {
        let tokens = [keyword("badger"), keyword("mushroom"), keyword("snake")];
        let sequence_token = sequence(tokens.clone());

        test::assert_eq_unordered!(
            [FuzzyMatch::Exact(TokenMatch::new(
                &sequence_token,
                vec![
                    TokenMatch::from(&tokens[0]),
                    TokenMatch::from(&tokens[1]),
                    TokenMatch::from(&tokens[2]),
                ],
            ))],
            sequence_token
                .match_input("badger mushroom snake", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_incomplete() {
        let tokens = [keyword("badger"), keyword("mushroom"), keyword("snake")];
        let sequence_token = sequence(tokens.clone());

        test::assert_eq_unordered!(
            [FuzzyMatch::Partial(
                TokenMatch::new(
                    &sequence_token,
                    vec![TokenMatch::from(&tokens[0]), TokenMatch::from(&tokens[1])],
                ),
                None,
            )],
            sequence_token
                .match_input("badger mushroom", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_partial() {
        let tokens = [keyword("badger"), keyword("mushroom"), keyword("snake")];
        let sequence_token = sequence(tokens.clone());

        test::assert_eq_unordered!(
            [FuzzyMatch::Partial(
                TokenMatch::new(
                    &sequence_token,
                    vec![
                        TokenMatch::from(&tokens[0]),
                        TokenMatch::from(&tokens[1]),
                        TokenMatch::from(&tokens[2]),
                    ],
                ),
                Some("ake".to_string()),
            )],
            sequence_token
                .match_input("badger mushroom sn", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_overflowing() {
        let tokens = [keyword("badger"), keyword("mushroom"), keyword("snake")];
        let sequence_token = sequence_m(Marker::Token, tokens.clone());

        test::assert_eq_unordered!(
            [FuzzyMatch::Overflow(
                TokenMatch::new(
                    &sequence_token,
                    vec![
                        TokenMatch::from(&tokens[0]),
                        TokenMatch::from(&tokens[1]),
                        TokenMatch::from(&tokens[2]),
                    ],
                ),
                " hippopotumus".into(),
            )],
            sequence_token
                .match_input("badger mushroom snake hippopotumus", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_with_any_phrase() {
        let tokens = [keyword("badger"), any_phrase(), any_word()];
        let [keyword_token, any_phrase_token, any_word_token] = tokens.clone();
        let sequence_token = sequence(tokens);

        test::assert_eq_unordered!(
            [
                FuzzyMatch::Overflow(
                    TokenMatch::new(
                        &sequence_token,
                        vec![
                            TokenMatch::from(&keyword_token),
                            TokenMatch::new(&any_phrase_token, "badger"),
                            TokenMatch::new(&any_word_token, "badger"),
                        ],
                    ),
                    " mushroom".into(),
                ),
                FuzzyMatch::Exact(TokenMatch::new(
                    &sequence_token,
                    vec![
                        TokenMatch::from(&keyword_token),
                        TokenMatch::new(&any_phrase_token, "badger badger"),
                        TokenMatch::new(&any_word_token, "mushroom"),
                    ],
                )),
                FuzzyMatch::Partial(
                    TokenMatch::new(
                        &sequence_token,
                        vec![
                            TokenMatch::from(&keyword_token),
                            TokenMatch::new(&any_phrase_token, "badger badger mushroom"),
                        ],
                    ),
                    None,
                ),
            ],
            sequence_token
                .match_input("badger badger badger mushroom", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }
}
