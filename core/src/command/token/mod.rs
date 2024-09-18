use crate::app::AppMeta;
use crate::storage::{RecordSource, ThingType};
use crate::utils::CaseInsensitiveStr;
use crate::world::thing::Thing;

use futures::pin_mut;

//use futures::task::{Context, Poll};
use async_stream::stream;
use futures::stream::{self, Stream, StreamExt};

use std::marker::PhantomData;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Token<M> {
    token_type: TokenType<M>,
    marker: M,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Match<'a, M> {
    token: &'a Token<M>,
    phrase: &'a str,
    meta: Meta<'a, M>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MatchType<'a, M> {
    Full(Match<'a, M>),
    Partial(Match<'a, M>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TokenType<M> {
    /// One or more tokens, in any order but without repetition
    AnyOf(Vec<Token<M>>),

    /// One or more tokens, in any order with repetition
    AnyOfRepeated(Vec<Token<M>>),

    /// Any sequence of words
    AnyPhrase,

    /// Any single word
    AnyWord,

    /// The name of an existing thing
    Name(RecordSource, ThingType),

    /// Any one of the tokens
    Or(Vec<Token<M>>),

    /// The exact sequence of tokens
    Phrase(Vec<Token<M>>),

    /// A literal word
    Word(&'static str),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Meta<'a, M> {
    None,
    Thing(Thing),
    Sequence(Vec<Match<'a, M>>),
}

impl<M> Token<M> {
    pub fn match_partial<'a>(
        &'a self,
        input: &'a str,
        app_meta: &'a AppMeta,
    ) -> impl Stream<Item = MatchType<'a, M>> {
        todo!();
        /*
        stream! {
            match &self.token_type {
                TokenType::AnyOf(tokens) => {
                    let streams = tokens.into_iter().map(|token| token.match_partial(input, app_meta));
                    for await token_match in stream::select_all::select_all(streams) {
                        yield token_match;
                    }
                }
                TokenType::Name(record_source, thing_type) => {
                    for result in app_meta.repository.get_by_name_start((input, *record_source, *thing_type)).await.into_iter().flatten() {
                        let is_full_match = result.thing.name().value().map_or(false, |s| s.eq_ci(input));

                        let match_result = Match {
                            token: self,
                            phrase: input,
                            meta: Meta::Thing(result.thing),
                        };

                        yield if is_full_match {
                            MatchType::Full(match_result)
                        } else {
                            MatchType::Partial(match_result)
                        };
                    }
                }
                _ => todo!(),
            }
        }
        */
    }
}

struct Full;

struct Partial;

enum TokenMatchStream<'a, T, M> {
    Name(NameMatchStream<'a, T, M>),
}

struct NameMatchStream<'a, T, M> {
    stream: Box<dyn Stream<Item = MatchType<'a, M>>>,
    _phantom: PhantomData<T>,
}

impl<'a, T, M> NameMatchStream<'a, T, M> {
    pub fn new(
        input: &'a str,
        app_meta: &'a AppMeta,
        token: &'a Token<M>,
        record_source: RecordSource,
        thing_type: ThingType,
    ) -> Self {
        Self {
            stream: Box::new(stream! {
                for result in app_meta.repository.get_by_name_start((input, record_source, thing_type)).await.into_iter().flatten() {
                    let is_full_match = result.thing.name().value().map_or(false, |s| s.eq_ci(input));

                    let match_result = Match {
                        token,
                        phrase: input,
                        meta: Meta::Thing(result.thing),
                    };

                    yield if is_full_match {
                        MatchType::Full(match_result)
                    } else {
                        MatchType::Partial(match_result)
                    };
                }
            }),
            _phantom: PhantomData,
        }
    }
}
