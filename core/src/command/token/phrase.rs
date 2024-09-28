use super::{Match, MatchType, Meta, Token, TokenType};

use crate::app::AppMeta;

use std::iter;
use std::pin::Pin;

use async_stream::stream;
use futures::prelude::*;
use futures::task::{Context, Poll};

pub fn match_input<'a, M>(
    token: &'a Token<M>,
    input: &'a str,
    app_meta: &'a AppMeta,
) -> Pin<Box<dyn Stream<Item = MatchType<'a, M>> + 'a>>
where
    M: Clone,
{
    /*
    if tokens.is_empty() {
        return Box::pin(stream::empty());
    }
    */

    /*
    let first_token = &tokens[0];
    let remaining_tokens = &tokens[1..];
    let mut first_token_stream = first_token.match_input(input, app_meta).as_mut();
    let mut remaining_token_stream = None;

    Box::pin(stream::poll_fn(move |cx| {
        if remaining_token_stream.is_none() {
            match Stream::poll_next(first_token_stream, cx) {
                Poll::Ready(Some(MatchType::Partial(_token_match))) => todo!(),
                Poll::Ready(Some(MatchType::Exact(_token_match))) => todo!(),
                Poll::Ready(Some(MatchType::Overflow(token_match, remainder))) => {
                    remaining_token_stream = Some(Box::pin(stream! {
                        if remaining_tokens.is_empty() {
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
                                token_type: TokenType::Phrase(&remaining_tokens),
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
                    }));
                }
                v => return v,
            }
        }

        if let Some(stream) = remaining_token_stream.as_mut() {
            Stream::poll_next(stream.as_mut(), cx)
        } else {
            Poll::Ready(None)
        }
    }))
    */

    Box::pin(PhraseMatchStream::new(token, input, app_meta))
}

struct PhraseMatchStream<'a, M>
where
    M: Clone,
{
    token: &'a Token<'a, M>,
    input: &'a str,
    app_meta: &'a AppMeta,
    first_token_stream: Pin<Box<dyn Stream<Item = MatchType<'a, M>> + 'a>>,
    remaining_token_stream: Option<Pin<Box<dyn Stream<Item = MatchType<'a, M>> + 'a>>>,
}

impl<'a, M> PhraseMatchStream<'a, M>
where
    M: Clone,
{
    pub fn new(token: &'a Token<'a, M>, input: &'a str, app_meta: &'a AppMeta) -> Self {
        let TokenType::Phrase(tokens) = token.token_type else {
            unreachable!();
        };

        let first_token_stream = if tokens.is_empty() {
            Box::pin(stream::empty())
        } else {
            tokens[0].match_input(input, app_meta)
        };

        Self {
            token,
            input,
            app_meta,
            first_token_stream,
            remaining_token_stream: None,
        }
    }
}

impl<'a, M> Stream for PhraseMatchStream<'a, M>
where
    M: Clone,
{
    type Item = MatchType<'a, M>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let self_ref = Pin::get_mut(self);
        let TokenType::Phrase(tokens) = self.token.token_type else {
            unreachable!();
        };

        let inner_match_type = loop {
            match Stream::poll_next(self_ref.first_token_stream.as_mut(), cx) {
                Poll::Ready(Some(MatchType::Partial(first_token_match))) => {
                    return Poll::Ready(Some(MatchType::Partial(Match {
                        token: self.token,
                        phrase: first_token_match.phrase,
                        meta: Meta::Sequence(vec![first_token_match]),
                    })));
                }
                Poll::Ready(Some(MatchType::Exact(first_token_match))) => {
                    let token_match = Match {
                        token: self.token,
                        phrase: first_token_match.phrase,
                        meta: Meta::Sequence(vec![first_token_match]),
                    };

                    return Poll::Ready(Some(if tokens.len() == 1 {
                        MatchType::Exact(token_match)
                    } else {
                        MatchType::Partial(token_match)
                    }));
                }
                Poll::Ready(Some(MatchType::Overflow(first_token_match, remainder))) => {
                    if tokens.len() == 1 {
                        return Poll::Ready(Some(MatchType::Overflow(
                            Match {
                                token: self.token,
                                phrase: first_token_match.phrase,
                                meta: Meta::Sequence(vec![first_token_match]),
                            },
                            remainder,
                        )));
                    } else {
                        // More to parse, now the recursion starts.
                        let remaining_phrase_token = Token {
                            token_type: TokenType::Phrase(&tokens[1..]),
                            marker: &self_ref.token.marker,
                        };

                        let mut remaining_token_stream =
                            remaining_phrase_token.match_input(remainder, self.app_meta);

                        match Stream::poll_next(remaining_token_stream.as_mut(), cx) {
                            Poll::Ready(Some(match_type)) => {
                                // got a token right back, break out of the loop to handle it
                                break match_type;
                            }
                            Poll::Ready(None) => {
                                // nope, keep looping to the next token
                            }
                            Poll::Pending => {
                                // We'll revisit this on a future poll.
                                self.remaining_token_stream = Some(remaining_token_stream);
                                return Poll::Pending;
                            }
                        }
                    }
                }
                v => return v,
            }
        };

        Poll::Pending
    }
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
