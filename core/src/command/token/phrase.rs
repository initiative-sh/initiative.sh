use super::{Match, MatchType, Meta, Token, TokenType};

use crate::app::AppMeta;

use std::iter;
use std::pin::Pin;

use async_stream::stream;
use futures::prelude::*;
use futures::task::{Context, Poll};
use pin_project::pin_project;

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

#[pin_project]
struct PhraseMatchStream<'a, M>
where
    M: Clone,
{
    token: &'a Token<'a, M>,
    input: &'a str,
    app_meta: &'a AppMeta,
    first_token_stream: Pin<Box<dyn Stream<Item = MatchType<'a, M>> + 'a>>,
    next_match_remaining_stream: Option<(
        Match<'a, M>,
        Pin<Box<dyn Stream<Item = MatchType<'a, M>> + 'a>>,
        Token<'a, M>,
    )>,
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
            next_match_remaining_stream: None,
        }
    }
}

impl<'a, M> Stream for PhraseMatchStream<'a, M>
where
    M: Clone,
{
    type Item = MatchType<'a, M>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        let TokenType::Phrase(tokens) = this.token.token_type else {
            unreachable!();
        };

        loop {
            if let Some((next_match, remaining_stream, _)) =
                this.next_match_remaining_stream.as_mut()
            {
                match Stream::poll_next(remaining_stream.as_mut(), cx) {
                    Poll::Ready(Some(match_type)) => {
                        return Poll::Ready(Some(match_type.map(|token_match| {
                            let Meta::Sequence(remaining_matches) = token_match.meta else {
                                unreachable!();
                            };

                            Match {
                                token: this.token,
                                phrase: token_match.phrase,
                                meta: Meta::Sequence(
                                    iter::once(next_match.clone())
                                        .chain(remaining_matches)
                                        .collect(),
                                ),
                            }
                        })))
                    }
                    Poll::Ready(None) => {
                        this.next_match_remaining_stream.take();
                    }
                    Poll::Pending => return Poll::Pending,
                }
            }

            match Stream::poll_next(this.first_token_stream.as_mut(), cx) {
                Poll::Ready(Some(MatchType::Partial(first_token_match))) => {
                    return Poll::Ready(Some(MatchType::Partial(Match {
                        token: this.token,
                        phrase: first_token_match.phrase.clone(),
                        meta: Meta::Sequence(vec![first_token_match]),
                    })));
                }
                Poll::Ready(Some(MatchType::Exact(first_token_match))) => {
                    let token_match = Match {
                        token: this.token,
                        phrase: first_token_match.phrase.clone(),
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
                                token: this.token,
                                phrase: first_token_match.phrase.clone(),
                                meta: Meta::Sequence(vec![first_token_match]),
                            },
                            remainder,
                        )));
                    } else {
                        // More to parse, now the recursion starts.
                        let remaining_phrase_token = Token {
                            token_type: TokenType::Phrase(&tokens[1..]),
                            marker: this.token.marker.clone(),
                        };

                        this.next_match_remaining_stream.insert((
                            first_token_match,
                            remaining_phrase_token.match_input(remainder, this.app_meta),
                            remaining_phrase_token,
                        ));
                    }
                }
                v => return v,
            }
        }
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
