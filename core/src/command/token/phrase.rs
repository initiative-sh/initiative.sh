use super::{Match, MatchType, Meta, Token, TokenType};

use crate::app::AppMeta;
use crate::utils::Word;

use std::iter;
use std::pin::Pin;

use futures::prelude::*;
use futures::task::{Context, Poll};
use pin_project::pin_project;

pub fn match_input<'a, M, W>(
    token: Token<'a, M>,
    input: &'a W,
    app_meta: &'a AppMeta,
) -> Pin<Box<dyn Stream<Item = MatchType<'a, M>> + 'a>>
where
    M: Clone,
    W: Into<Word<'a>>,
{
    Box::pin(PhraseMatchStream::new(token, input, app_meta))
}

#[pin_project]
struct PhraseMatchStream<'a, M>
where
    M: Clone,
{
    token: Token<'a, M>,
    input: &'a Word<'a>,
    app_meta: &'a AppMeta,
    first_token_stream: Pin<Box<dyn Stream<Item = MatchType<'a, M>> + 'a>>,
    next_match_remaining_stream: Option<(
        Match<'a, M>,
        Pin<Box<dyn Stream<Item = MatchType<'a, M>> + 'a>>,
    )>,
}

impl<'a, M> PhraseMatchStream<'a, M>
where
    M: Clone,
{
    pub fn new<W>(token: Token<'a, M>, input: &'a W, app_meta: &'a AppMeta) -> Self
    where
    W: Into<Word<'a>> {
        let TokenType::Phrase(tokens) = token.token_type else {
            unreachable!();
        };

        let first_token_stream = if tokens.is_empty() {
            Box::pin(stream::empty())
        } else {
            tokens[0].clone().match_input(input, app_meta)
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
        let this = self.project();

        let TokenType::Phrase(tokens) = this.token.token_type else {
            unreachable!();
        };

        loop {
            // An inner stream is currently present and running against a match. Process it (and
            // possibly return) before finding another match.
            if let Some((next_match, remaining_stream)) = this.next_match_remaining_stream.as_mut()
            {
                match Stream::poll_next(remaining_stream.as_mut(), cx) {
                    // Got a result: return the first match and leave the stream in place to poll
                    // again.
                    Poll::Ready(Some(match_type)) => {
                        return Poll::Ready(Some(match_type.map(|token_match| {
                            let Meta::Sequence(remaining_matches) = token_match.meta else {
                                unreachable!();
                            };

                            Match {
                                token: this.token.clone(),
                                phrase: token_match.phrase,
                                meta: Meta::Sequence(
                                    iter::once(next_match.clone())
                                        .chain(remaining_matches)
                                        .collect(),
                                ),
                            }
                        })));
                    }

                    // The stream is exhausted: drop it and return to processing the first token.
                    Poll::Ready(None) => *this.next_match_remaining_stream = None,

                    // The stream is pending and so are we.
                    Poll::Pending => return Poll::Pending,
                }
            }

            // If no inner stream is present, proceed to match the first token in the list against
            // the input.
            match Stream::poll_next(this.first_token_stream.as_mut(), cx) {
                // A partial match was found, so the phrase is incomplete.
                Poll::Ready(Some(MatchType::Partial(first_token_match))) => {
                    return Poll::Ready(Some(MatchType::Partial(Match {
                        token: this.token.clone(),
                        phrase: first_token_match.phrase.clone(),
                        meta: Meta::Sequence(vec![first_token_match]),
                    })));
                }

                // An exact match was found (no leftovers), so the phrase is complete if the token
                // list has been exhausted, or it's a partial match if there are tokens left to
                // match.
                Poll::Ready(Some(MatchType::Exact(first_token_match))) => {
                    let token_match = Match {
                        token: this.token.clone(),
                        phrase: first_token_match.phrase.clone(),
                        meta: Meta::Sequence(vec![first_token_match]),
                    };

                    return Poll::Ready(Some(if tokens.len() == 1 {
                        MatchType::Exact(token_match)
                    } else {
                        MatchType::Partial(token_match)
                    }));
                }

                // The token matched with leftovers, so we'll create a new sub-stream that takes
                // the remaining token list and match it against the remaining text. This will nest
                // repeatedly until either the token list or the text is exhausted.
                Poll::Ready(Some(MatchType::Overflow(first_token_match, remainder))) => {
                    if tokens.len() == 1 {
                        // This was the last token, so the entire phrase overflows.
                        return Poll::Ready(Some(MatchType::Overflow(
                            Match {
                                token: this.token.clone(),
                                phrase: first_token_match.phrase.clone(),
                                meta: Meta::Sequence(vec![first_token_match]),
                            },
                            remainder,
                        )));
                    } else {
                        // More tokens to parse, now the recursion starts.
                        let remaining_phrase_token = Token {
                            token_type: TokenType::Phrase(&tokens[1..]),
                            marker: this.token.marker.clone(),
                        };

                        *this.next_match_remaining_stream = Some((
                            first_token_match,
                            remaining_phrase_token.match_input(remainder, this.app_meta),
                        ));
                    }
                }

                // If the first token stream is empty or pending, then so are we.
                v @ Poll::Ready(None) | v @ Poll::Pending => return v,
            }
        }
    }
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
            AnyPhrase,
            AnyWord,
        };

        let phrase = [
            Token::new(TokenType::Keyword("Legolas"), Marker::Keyword),
            Token::new(TokenType::AnyPhrase, Marker::AnyPhrase),
            Token::new(TokenType::AnyWord, Marker::AnyWord),
        ];
        let [keyword_token, any_phrase_token, any_word_token] = &phrase;

        let phrase_token = Token::new(TokenType::Phrase(&phrase), Marker::Phrase);

        assert_stream_eq(
            vec![
                MatchType::Overflow(
                    Match {
                        token: phrase_token.clone(),
                        phrase: "Legolas is an".into(),
                        meta: Meta::Sequence(vec![
                            Match {
                                token: keyword_token.clone(),
                                phrase: "Legolas".into(),
                                meta: Meta::None,
                            },
                            Match {
                                token: any_phrase_token.clone(),
                                phrase: "is".into(),
                                meta: Meta::None,
                            },
                            Match {
                                token: any_word_token.clone(),
                                phrase: "an".into(),
                                meta: Meta::None,
                            },
                        ]),
                    },
                    " elf",
                ),
                MatchType::Exact(Match {
                    token: phrase_token.clone(),
                    phrase: "Legolas is an elf".into(),
                    meta: Meta::Sequence(vec![
                        Match {
                            token: keyword_token.clone(),
                            phrase: "Legolas".into(),
                            meta: Meta::None,
                        },
                        Match {
                            token: any_phrase_token.clone(),
                            phrase: "is an".into(),
                            meta: Meta::None,
                        },
                        Match {
                            token: any_word_token.clone(),
                            phrase: "elf".into(),
                            meta: Meta::None,
                        },
                    ]),
                }),
            ],
            phrase_token
                .clone()
                .match_input("Legolas is an elf", &app_meta),
        )
        .await;
    }

    fn event_dispatcher(_event: Event) {}
}
