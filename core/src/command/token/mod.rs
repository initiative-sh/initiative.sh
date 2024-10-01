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
pub struct Token<'a> {
    pub token_type: TokenType<'a>,
    pub marker: Option<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Match<'a> {
    pub token: Token<'a>,
    pub meta: Meta<'a>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MatchType<'a> {
    Overflow(Match<'a>, &'a str),
    Exact(Match<'a>),
    Partial(Match<'a>, Option<String>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TokenType<'a> {
    /// One or more tokens, in any order but without repetition
    AnyOf(&'a [Token<'a>]),

    /// One or more tokens, in any order with repetition
    AnyOfRepeated(&'a [Token<'a>]),

    /// Any sequence of words
    AnyPhrase,

    /// Any single word
    AnyWord,

    /// The name of an existing thing
    Name,

    /// Any one of the tokens
    Or(&'a [Token<'a>]),

    /// The exact sequence of tokens
    Phrase(&'a [Token<'a>]),

    /// A literal word
    Keyword(&'a str),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Meta<'a> {
    None,
    Phrase(&'a str),
    Record(Record),
    Sequence(Vec<Match<'a>>),
    Single(Box<Match<'a>>),
}

impl<'a> Token<'a> {
    pub fn new<T>(token_type: TokenType<'a>, marker: T) -> Self
    where
        T: Into<Option<u8>>,
    {
        Token {
            token_type,
            marker: marker.into(),
        }
    }

    pub fn match_input(
        self,
        input: &'a str,
        app_meta: &'a AppMeta,
    ) -> Pin<Box<dyn Stream<Item = MatchType<'a>> + 'a>> {
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

impl<'a> MatchType<'a> {
    pub fn map<F>(self, f: F) -> Self
    where
        F: FnOnce(Match<'a>) -> Match<'a>,
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

    pub fn token_match(&self) -> &Match<'a> {
        match self {
            MatchType::Overflow(token_match, _)
            | MatchType::Exact(token_match)
            | MatchType::Partial(token_match, _) => token_match,
        }
    }
}

impl<'a> Match<'a> {
    pub fn new(token: Token<'a>, meta: impl Into<Meta<'a>>) -> Self {
        Match {
            token,
            meta: meta.into(),
        }
    }
}

impl<'a> From<Token<'a>> for Match<'a> {
    fn from(input: Token<'a>) -> Self {
        Match {
            token: input,
            meta: Meta::None,
        }
    }
}

impl<'a> From<&'a str> for Meta<'a> {
    fn from(input: &'a str) -> Self {
        Meta::Phrase(input)
    }
}

impl<'a> From<Vec<Match<'a>>> for Meta<'a> {
    fn from(input: Vec<Match<'a>>) -> Self {
        Meta::Sequence(input)
    }
}

impl<'a> From<Match<'a>> for Meta<'a> {
    fn from(input: Match<'a>) -> Self {
        Meta::Single(input.into())
    }
}

impl<'a> From<Record> for Meta<'a> {
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
    ) where
        T: std::fmt::Debug + PartialEq,
    {
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

    pub async fn assert_stream_empty<'a, T>(stream: Pin<Box<dyn Stream<Item = T> + 'a>>)
    where
        T: std::fmt::Debug + PartialEq,
    {
        assert_stream_eq(Vec::<T>::new(), stream).await;
    }
}
