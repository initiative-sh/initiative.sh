mod any_phrase;
mod any_word;
mod keyword;
mod name;
mod or;
mod phrase;
mod token_marker_iterator;

use token_marker_iterator::TokenMarkerIterator;

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
pub struct TokenMatch<'a> {
    pub token: Token<'a>,
    pub meta: Meta<'a>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FuzzyMatch<'a> {
    Overflow(TokenMatch<'a>, &'a str),
    Exact(TokenMatch<'a>),
    Partial(TokenMatch<'a>, Option<String>),
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
    Sequence(Vec<TokenMatch<'a>>),
    Single(Box<TokenMatch<'a>>),
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

    pub fn match_input<'b>(
        self,
        input: &'a str,
        app_meta: &'b AppMeta,
    ) -> Pin<Box<dyn Stream<Item = FuzzyMatch<'a>> + 'b>>
    where 'a: 'b {
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

impl<'a> FuzzyMatch<'a> {
    pub fn map<F>(self, f: F) -> Self
    where
        F: FnOnce(TokenMatch<'a>) -> TokenMatch<'a>,
    {
        match self {
            FuzzyMatch::Overflow(token_match, overflow) => {
                FuzzyMatch::Overflow(f(token_match), overflow)
            }
            FuzzyMatch::Exact(token_match) => FuzzyMatch::Exact(f(token_match)),
            FuzzyMatch::Partial(token_match, completion) => {
                FuzzyMatch::Partial(f(token_match), completion)
            }
        }
    }

    pub fn token_match(&self) -> &TokenMatch<'a> {
        match self {
            FuzzyMatch::Overflow(token_match, _)
            | FuzzyMatch::Exact(token_match)
            | FuzzyMatch::Partial(token_match, _) => token_match,
        }
    }
}

impl<'a> TokenMatch<'a> {
    pub fn new(token: Token<'a>, meta: impl Into<Meta<'a>>) -> Self {
        TokenMatch {
            token,
            meta: meta.into(),
        }
    }

    pub fn find_marker(&'a self, marker: u8) -> impl Iterator<Item = &'a TokenMatch> {
        TokenMarkerIterator::new(self, marker)
    }
}

impl<'a> From<Token<'a>> for TokenMatch<'a> {
    fn from(input: Token<'a>) -> Self {
        TokenMatch {
            token: input,
            meta: Meta::None,
        }
    }
}

impl<'a> Meta<'a> {
    pub fn phrase(&self) -> Option<&str> {
        if let Meta::Phrase(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn record(&self) -> Option<&Record> {
        if let Meta::Record(r) = self {
            Some(r)
        } else {
            None
        }
    }

    pub fn sequence(&self) -> Option<&[TokenMatch]> {
        if let Meta::Sequence(v) = self {
            Some(v.as_slice())
        } else {
            None
        }
    }

    pub fn single(&self) -> Option<&TokenMatch> {
        if let Meta::Single(b) = self {
            Some(b.as_ref())
        } else {
            None
        }
    }
}

impl<'a> From<&'a str> for Meta<'a> {
    fn from(input: &'a str) -> Self {
        Meta::Phrase(input)
    }
}

impl<'a> From<Vec<TokenMatch<'a>>> for Meta<'a> {
    fn from(input: Vec<TokenMatch<'a>>) -> Self {
        Meta::Sequence(input)
    }
}

impl<'a> From<TokenMatch<'a>> for Meta<'a> {
    fn from(input: TokenMatch<'a>) -> Self {
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
        for result in stream.collect::<Vec<_>>().await {
            let Some(index) = expect_results
                .iter()
                .position(|expect_result| expect_result == &result)
            else {
                panic!(
                    "Not found in expected results: {:?}\n\nRemaining expected results: {:?}",
                    result, expect_results
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
