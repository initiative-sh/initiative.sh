use super::{Match, MatchType, Meta, Token, TokenType};

use crate::app::AppMeta;

use std::iter;
use std::pin::Pin;

use async_stream::stream;
use futures::prelude::*;

pub fn match_input<'a, M>(
    token: &'a Token<M>,
    input: &'a str,
    app_meta: &'a AppMeta,
) -> Pin<Box<dyn Stream<Item = MatchType<'a, M>> + 'a>>
where
    M: Clone,
{
    let TokenType::Phrase(tokens) = token.token_type else {
        unreachable!();
    };

    Box::pin(stream! {
        for (i, token) in tokens.iter().enumerate() {
            for await match_type in token.match_input(input, app_meta) {
                match match_type {
                    MatchType::Partial(token_match) => todo!(),
                    MatchType::Exact(token_match) => todo!(),
                    MatchType::Overflow(token_match, remainder) => {
                        if i == tokens.len() {
                            yield MatchType::Overflow(
                                Match {
                                    token,
                                    phrase: token_match.phrase.clone(),
                                    meta: Meta::Sequence(vec![token_match]),
                                },
                                remainder,
                            );
                        } else {
                            let next_token = Token {
                                token_type: TokenType::Phrase(&tokens[1..]),
                                marker: token.marker.clone(),
                            };

                            for await next_match_type in next_token.match_input(remainder, app_meta) {
                                yield next_match_type.map(|next_match| {
                                    let Meta::Sequence(next_meta_sequence) = next_match.meta else {
                                        unreachable!();
                                    };

                                    Match {
                                        token,
                                        phrase: token_match.phrase.combine_with(next_match.phrase).unwrap(),
                                        meta: Meta::Sequence(iter::once(token_match.clone()).chain(next_meta_sequence.into_iter()).collect()),
                                    }
                                });
                            }
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

    use crate::app::Event;
    use crate::storage::NullDataStore;
    use tokio_test::block_on;

    #[test]
    fn match_input_test() {
        let app_meta = AppMeta::new(NullDataStore, &event_dispatcher);
    }

    fn event_dispatcher(_event: Event) {}
}
