mod any_phrase;
mod any_word;
mod keyword;
mod name;
mod or;
mod phrase;

use crate::app::AppMeta;
use crate::storage::Record;

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
    pub token: Token<'a, M>,
    pub meta: Meta<'a, M>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MatchType<'a, M>
where
    M: Clone,
{
    Overflow(Match<'a, M>, &'a str),
    Exact(Match<'a, M>),
    Partial(Match<'a, M>, Option<String>),
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
    Name,

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
    Phrase(&'a str),
    Record(Record),
    Sequence(Vec<Match<'a, M>>),
    Single(Box<Match<'a, M>>),
}

impl<'a, M> Token<'a, M>
where
    M: Clone,
{
    pub fn new(token_type: TokenType<'a, M>, marker: M) -> Self {
        Token { token_type, marker }
    }

    pub fn match_input(
        self,
        input: &'a str,
        app_meta: &'a AppMeta,
    ) -> Pin<Box<dyn Stream<Item = MatchType<'a, M>> + 'a>> {
        match &self.token_type {
            TokenType::AnyOf(..) => todo!(),
            TokenType::AnyOfRepeated(..) => todo!(),
            TokenType::AnyPhrase => any_phrase::match_input(self, input),
            TokenType::AnyWord => any_word::match_input(self, input),
            TokenType::Name => name::match_input(self, input, app_meta),
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
            MatchType::Partial(token_match, completion) => {
                MatchType::Partial(f(token_match), completion)
            }
        }
    }

    pub fn token_match(&self) -> &Match<'a, M> {
        match self {
            MatchType::Overflow(token_match, _)
            | MatchType::Exact(token_match)
            | MatchType::Partial(token_match, _) => token_match,
        }
    }
}

impl<'a, M> Match<'a, M>
where
    M: Clone,
{
    pub fn new(token: Token<'a, M>, meta: impl Into<Meta<'a, M>>) -> Self {
        Match {
            token,
            meta: meta.into(),
        }
    }
}

impl<'a, M> From<Token<'a, M>> for Match<'a, M>
where
    M: Clone,
{
    fn from(input: Token<'a, M>) -> Self {
        Match {
            token: input,
            meta: Meta::None,
        }
    }
}

impl<'a, M> From<&'a str> for Meta<'a, M>
where
    M: Clone,
{
    fn from(input: &'a str) -> Self {
        Meta::Phrase(input)
    }
}

impl<'a, M> From<Vec<Match<'a, M>>> for Meta<'a, M>
where
    M: Clone,
{
    fn from(input: Vec<Match<'a, M>>) -> Self {
        Meta::Sequence(input)
    }
}

impl<'a, M> From<Match<'a, M>> for Meta<'a, M>
where
    M: Clone,
{
    fn from(input: Match<'a, M>) -> Self {
        Meta::Single(input.into())
    }
}

impl<'a, M> From<Record> for Meta<'a, M>
where
    M: Clone,
{
    fn from(input: Record) -> Self {
        Meta::Record(input)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    pub async fn assert_stream_eq<'a, T>(
        mut expect_results: Vec<T>,
        stream: Pin<Box<dyn Stream<Item = T> + 'a>>,
    ) where T: std::fmt::Debug + PartialEq {
        for match_type in stream.collect::<Vec<_>>().await {
            let Some(index) = expect_results
                .iter()
                .position(|expect_result| expect_result == &match_type)
            else {
                panic!(
                    "Not found in expected results: {:?}\n\nRemaining expected results: {:?}",
                    match_type, expect_results
                );
            };
            expect_results.swap_remove(index);
        }

        assert_eq!(
            Vec::<T>::new(),
            expect_results,
            "Expected all results to be exhausted",
        );
    }

    pub async fn assert_stream_empty<'a, M>(
        stream: Pin<Box<dyn Stream<Item = MatchType<'a, M>> + 'a>>,
    ) where
        M: std::fmt::Debug + std::cmp::PartialEq + Clone + Copy,
    {
        assert_stream_eq(Vec::<MatchType<'a, M>>::new(), stream).await;
    }
}
