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
    let tokens = if let TokenType::Phrase(tokens) = &token.token_type {
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
        Box::pin(stream::empty())
    } else {
        Box::pin(stream! {
            // TokenMatch the first token in the phrase
            for await fuzzy_match in tokens[0].match_input(input, app_meta) {
                match fuzzy_match {

                    // First token is a partial match, so the phrase is incomplete
                    FuzzyMatch::Partial(token_match, completion) => yield FuzzyMatch::Partial(
                        TokenMatch::new(token, vec![token_match]),
                        completion,
                    ),

                    // First token is an exact match and is the only token in the phrase, so the
                    // phrase is also an exact match.
                    FuzzyMatch::Exact(token_match) if tokens.len() == 1 => yield FuzzyMatch::Exact(
                        TokenMatch::new(token, vec![token_match]),
                    ),

                    // First token is an exact match but there are more unmatched tokens, so the
                    // phrase is incomplete.
                    FuzzyMatch::Exact(token_match) => yield FuzzyMatch::Partial(
                        TokenMatch::new(token, vec![token_match]),
                        None,
                    ),

                    // First token overflows and is the only token in the phrase, so the phrase
                    // also overflows.
                    FuzzyMatch::Overflow(token_match, remainder) if tokens.len() == 1 =>
                            yield FuzzyMatch::Overflow(
                                TokenMatch::new(token, vec![token_match]),
                                remainder,
                            ),

                    // First token overflows but there are other tokens in the phrase, so we
                    // recurse with the remainder of the phrase.
                    FuzzyMatch::Overflow(token_match, remainder) => {
                        for await next_fuzzy_match in match_input_with_tokens(token, remainder.as_str(), app_meta, &tokens[1..]) {
                            yield next_fuzzy_match.map(|next_match| {
                                let MatchMeta::Sequence(next_meta_sequence) = next_match.match_meta else {
                                    unreachable!();
                                };

                                TokenMatch::new(
                                    token,
                                    iter::once(token_match.clone()).chain(next_meta_sequence.into_iter()).collect::<Vec<_>>(),
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
    use super::super::test::{assert_stream_empty, assert_stream_eq};
    use super::*;

    use crate::app::Event;
    use crate::storage::NullDataStore;
    use initiative_macros::TokenMarker;
    use tokio_test::block_on;

    #[tokio::test]
    async fn match_input_test() {
        let app_meta = AppMeta::new(NullDataStore, &event_dispatcher);

        #[derive(Clone, Copy, Debug, Eq, PartialEq, TokenMarker)]
        enum Marker {
            Phrase,
            Keyword,
            AnyPhrase,
            AnyWord,
        };

        let phrase = [
            Token::keyword_marked(Marker::Keyword, "Legolas"),
            Token::any_phrase_marked(Marker::AnyPhrase),
            Token::any_word_marked(Marker::AnyWord),
        ];
        let [keyword_token, any_phrase_token, any_word_token] = &phrase;

        let phrase_token = Token::phrase_marked(Marker::Phrase, phrase.to_vec());

        assert_stream_eq(
            vec![
                FuzzyMatch::Overflow(
                    TokenMatch {
                        token: &phrase_token,
                        meta: Meta::Sequence(vec![
                            keyword_token.into(),
                            TokenMatch::new(any_phrase_token, "is"),
                            TokenMatch::new(any_word_token, "an"),
                        ]),
                    },
                    " elf",
                ),
                FuzzyMatch::Exact(TokenMatch::new(
                    &phrase_token,
                    vec![
                        keyword_token.into(),
                        TokenMatch::new(any_phrase_token, "is an"),
                        TokenMatch::new(any_word_token, "elf"),
                    ],
                )),
                FuzzyMatch::Partial(
                    TokenMatch::new(
                        &phrase_token,
                        vec![
                            keyword_token.into(),
                            TokenMatch::new(any_phrase_token, "is an elf"),
                        ],
                    ),
                    None,
                ),
            ],
            phrase_token.match_input("Legolas is an elf", &app_meta),
        )
        .await;
    }

    fn event_dispatcher(_event: Event) {}
}
