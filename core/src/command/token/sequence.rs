use crate::app::AppMeta;
use crate::command::prelude::*;
use crate::utils::Substr;

use std::iter;
use std::pin::Pin;

use async_stream::stream;
use futures::prelude::*;

pub fn match_input<'token, 'app_meta>(
    token: &'token Token,
    input: Substr<'token>,
    app_meta: &'app_meta AppMeta,
) -> Pin<Box<dyn Stream<Item = FuzzyMatchList<'token>> + 'app_meta>>
where
    'token: 'app_meta,
{
    let Token::Sequence { tokens } = token else {
        unreachable!();
    };

    match_input_with_tokens(token, input, app_meta, tokens)
}

pub fn match_input_with_tokens<'token, 'app_meta>(
    token: &'token Token,
    input: Substr<'token>,
    app_meta: &'app_meta AppMeta,
    tokens: &'token [Token],
) -> Pin<Box<dyn Stream<Item = FuzzyMatchList<'token>> + 'app_meta>>
where
    'token: 'app_meta,
{
    if tokens.is_empty() {
        // No tokens, no matches
        Box::pin(stream::iter([FuzzyMatchList::Overflow(
            token.into(),
            input,
        )]))
    } else {
        Box::pin(stream! {
            // TokenMatch the first token in the sequence
            for await fuzzy_match in tokens[0].match_input(input, app_meta) {
                match fuzzy_match {

                    // First token is a partial match, so the sequence is incomplete
                    FuzzyMatchList::Partial(token_match, completion) => {
                        yield FuzzyMatchList::Partial(
                            TokenMatch::new(token, vec![token_match]),
                            completion,
                        );
                    }

                    // First token is an exact match and is the only token in the sequence, so the
                    // sequence is also an exact match.
                    FuzzyMatchList::Exact(token_match) if tokens.len() == 1 => {
                        yield FuzzyMatchList::Exact(
                            TokenMatch::new(token, vec![token_match]),
                        );
                    }

                    // First token is an exact match but there are more unmatched tokens, so the
                    // sequence is incomplete.
                    FuzzyMatchList::Exact(token_match) => {
                        yield FuzzyMatchList::Partial(
                            TokenMatch::new(token, vec![token_match]),
                            None,
                        );
                    }

                    // First token overflows and is the only token in the sequence, so the sequence
                    // also overflows.
                    FuzzyMatchList::Overflow(token_match, remainder) if tokens.len() == 1 => {
                        yield FuzzyMatchList::Overflow(
                            TokenMatch::new(token, vec![token_match]),
                            remainder,
                        );
                    }

                    // First token overflows but there are other tokens in the sequence, so we
                    // recurse with the remainder of the sequence.
                    FuzzyMatchList::Overflow(token_match, remainder) => {
                        for await next_fuzzy_match in match_input_with_tokens(token, remainder, app_meta, &tokens[1..]) {
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

    #[tokio::test]
    async fn match_input_test_empty() {
        let sequence_token = sequence([]);
        test::assert_eq_unordered!(
            [FuzzyMatchList::Overflow(
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
            [FuzzyMatchList::Exact(TokenMatch::new(
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
            [FuzzyMatchList::Partial(
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
            [FuzzyMatchList::Partial(
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
        let sequence_token = sequence(tokens.clone());

        test::assert_eq_unordered!(
            [FuzzyMatchList::Overflow(
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
                FuzzyMatchList::Overflow(
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
                FuzzyMatchList::Exact(TokenMatch::new(
                    &sequence_token,
                    vec![
                        TokenMatch::from(&keyword_token),
                        TokenMatch::new(&any_phrase_token, "badger badger"),
                        TokenMatch::new(&any_word_token, "mushroom"),
                    ],
                )),
                FuzzyMatchList::Partial(
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
