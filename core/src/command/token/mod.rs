mod any_phrase;
mod any_word;
mod keyword;
mod name;
mod or;
mod phrase;

use crate::app::AppMeta;
use crate::storage::{RecordSource, ThingType};
use crate::utils::Word;
use crate::world::thing::Thing;

use futures::prelude::*;

use std::pin::Pin;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Token<'a, M>
where
    M: Clone,
{
    pub token_type: TokenType<'a, M>,
    pub marker: M,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Match<'a, M>
where
    M: Clone,
{
    token: &'a Token<'a, M>,
    phrase: Word<'a>,
    meta: Meta<'a, M>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MatchType<'a, M>
where
    M: Clone,
{
    Overflow(Match<'a, M>, &'a str),
    Exact(Match<'a, M>),
    Partial(Match<'a, M>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TokenType<'a, M>
where
    M: Clone,
{
    /// One or more tokens, in any order but without repetition
    AnyOf(&'a [Token<'a, M>]),

    /// One or more tokens, in any order with repetition
    AnyOfRepeated(&'a [Token<'a, M>]),

    /// Any sequence of words
    AnyPhrase,

    /// Any single word
    AnyWord,

    /// The name of an existing thing
    Name(RecordSource, ThingType),

    /// Any one of the tokens
    Or(&'a [Token<'a, M>]),

    /// The exact sequence of tokens
    Phrase(&'a [Token<'a, M>]),

    /// A literal word
    Keyword(&'a str),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Meta<'a, M>
where
    M: Clone,
{
    None,
    Thing(Thing),
    Sequence(Vec<Match<'a, M>>),
    Single(Box<Match<'a, M>>),
}

impl<'a, M> Token<'a, M>
where
    M: Clone,
{
    pub fn match_input(
        &'a self,
        input: &'a str,
        app_meta: &'a AppMeta,
    ) -> Pin<Box<dyn Stream<Item = MatchType<'a, M>> + 'a>> {
        match &self.token_type {
            TokenType::AnyOf(..) => todo!(),
            TokenType::AnyOfRepeated(..) => todo!(),
            TokenType::AnyPhrase => any_phrase::match_input(self, input),
            TokenType::AnyWord => any_word::match_input(self, input),
            TokenType::Name(..) => name::match_input(self, input, app_meta),
            TokenType::Or(..) => or::match_input(self, input, app_meta),
            TokenType::Phrase(..) => phrase::match_input(self, input, app_meta),
            TokenType::Keyword(..) => keyword::match_input(self, input),
        }
    }
}

impl<'a, M> MatchType<'a, M>
where
    M: Clone,
{
    pub fn map<F>(self, f: F) -> Self
    where
        F: FnOnce(Match<'a, M>) -> Match<'a, M>,
    {
        match self {
            MatchType::Overflow(token_match, overflow) => {
                MatchType::Overflow(f(token_match), overflow)
            }
            MatchType::Exact(token_match) => MatchType::Exact(f(token_match)),
            MatchType::Partial(token_match) => MatchType::Partial(f(token_match)),
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    pub async fn assert_stream_eq<'a, M>(
        mut expect_results: Vec<MatchType<'a, M>>,
        stream: Pin<Box<dyn Stream<Item = MatchType<'a, M>> + 'a>>,
    ) where
        M: std::fmt::Debug + std::cmp::PartialEq,
    {
        for match_type in stream.collect::<Vec<_>>().await {
            let Some(index) = expect_results
                .iter()
                .position(|expect_result| expect_result == &match_type)
            else {
                panic!("Not found in expected results: {:?}", match_type);
            };
            expect_results.swap_remove(index);
        }

        assert_eq!(
            Vec::<MatchType<M>>::new(),
            expect_results,
            "Expected all results to be exhausted",
        );
    }

    pub async fn assert_stream_empty<'a, M>(
        stream: Pin<Box<dyn Stream<Item = MatchType<'a, M>> + 'a>>,
    ) where
        M: std::fmt::Debug + std::cmp::PartialEq,
    {
        assert_stream_eq(Vec::<MatchType<'a, M>>::new(), stream).await;
    }
}
