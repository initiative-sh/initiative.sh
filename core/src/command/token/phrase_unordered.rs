use super::{Match, MatchType, Meta, Token, TokenType};

use crate::app::AppMeta;

use std::iter;
use std::pin::Pin;

use async_stream::stream;
use futures::prelude::*;

pub fn match_input<'a>(
    token: Token<'a>,
    input: &'a str,
    app_meta: &'a AppMeta,
) -> Pin<Box<dyn Stream<Item = MatchType> + 'a>> {
    let tokens = if let TokenType::PhraseUnordered(tokens) = &token.token_type {
        tokens.to_vec()
    } else {
        unreachable!();
    };

    match_input_with_tokens(
        token,
        input,
        app_meta,
        tokens,
    )
}

pub fn match_input_with_tokens<'a, M>(
    token: Token<'a, M>,
    input: &'a str,
    app_meta: &'a AppMeta,
    tokens: Vec<Token<'a, M>>,
) -> Pin<Box<dyn Stream<Item = MatchType<'a, M>> + 'a>>
where
    M: Clone,
{
    Box::pin(stream! {
        // Attempt to match each token in turn
        for (test_token_index, test_token) in tokens.iter().enumerate() {
            for await match_type in test_token.clone().match_input(input, app_meta) {
                match match_type {

                    // The token is a partial match, so the phrase is incomplete
                    MatchType::Partial(token_match, completion) => yield MatchType::Partial(
                        Match::new(token.clone(), vec![token_match]),
                        completion,
                    ),

                    // First token is an exact match and is the only token in the phrase, so the
                    // phrase is also an exact match.
                    MatchType::Exact(token_match) if tokens.len() == 1 => yield MatchType::Exact(
                        Match::new(token.clone(), vec![token_match]),
                    ),

                    // First token is an exact match but there are more unmatched tokens, so the
                    // phrase is incomplete.
                    MatchType::Exact(token_match) => yield MatchType::Partial(
                        Match::new(token.clone(), vec![token_match]),
                        None,
                    ),

                    // First token overflows and is the only token in the phrase, so the phrase
                    // also overflows.
                    MatchType::Overflow(token_match, remainder) if tokens.len() == 1 =>
                            yield MatchType::Overflow(
                                Match::new(token.clone(), vec![token_match]),
                                remainder,
                            ),

                    // First token overflows but there are other tokens in the phrase, so we
                    // recurse with the remainder of the phrase.
                    MatchType::Overflow(token_match, remainder) => {
                        let next_tokens: Vec<_> =
                                tokens
                                    .iter()
                                    .take(test_token_index)
                                    .chain(tokens.iter().skip(test_token_index + 1))
                                    .cloned()
                                    .collect();

                        for await next_match_type in match_input_with_tokens(token.clone(), remainder, app_meta, next_tokens) {
                            yield next_match_type.map(|next_match| {
                                let Meta::Sequence(next_meta_sequence) = next_match.meta else {
                                    unreachable!();
                                };

                                Match::new(
                                    token.clone(),
                                    iter::once(token_match.clone()).chain(next_meta_sequence.into_iter()).collect::<Vec<_>>(),
                                )
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
    use super::super::test::{assert_stream_empty, assert_stream_eq};
    use super::*;

    use crate::app::Event;
    use crate::storage::NullDataStore;
    use tokio_test::block_on;

    #[tokio::test]
    async fn match_input_test() {
        let app_meta = AppMeta::new(NullDataStore, &event_dispatcher);

        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        enum Marker {
            Phrase,
            Keyword,
            AnyWord,
        };

        let phrase = [
            Token::new(TokenType::Keyword("badger"), Marker::Keyword as u8),
            Token::new(TokenType::AnyWord, Marker::AnyWord as u8),
        ];
        let [keyword_token, any_word_token] = &phrase;

        let phrase_token = Token::new(TokenType::PhraseUnordered(&phrase[..]), Marker::Phrase as u8);

        assert_stream_eq(
            vec![
                MatchType::Exact(Match::new(
                    phrase_token.clone(),
                    vec![
                        keyword_token.clone().into(),
                        Match::new(any_word_token.clone(), "badger"),
                    ],
                )),
                MatchType::Exact(Match::new(
                    phrase_token.clone(),
                    vec![
                        Match::new(any_word_token.clone(), "badger"),
                        keyword_token.clone().into(),
                    ],
                )),
            ],
            phrase_token.clone().match_input("badger badger", &app_meta),
        )
        .await;

        assert_stream_eq(
            vec![
                MatchType::Overflow(
                    Match::new(
                        phrase_token.clone(),
                        vec![
                            keyword_token.clone().into(),
                            Match::new(any_word_token.clone(), "badger"),
                        ],
                    ),
                    " badger",
                ),
                MatchType::Overflow(
                    Match::new(
                        phrase_token.clone(),
                        vec![
                            Match::new(any_word_token.clone(), "badger"),
                            keyword_token.clone().into(),
                        ],
                    ),
                    " badger",
                ),
            ],
            phrase_token
                .clone()
                .match_input("badger badger badger", &app_meta),
        )
        .await;
    }

    fn event_dispatcher(_event: Event) {}
}
