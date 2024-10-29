mod any_of;
mod any_phrase;
mod any_word;
mod keyword;
mod keyword_list;
mod name;
mod optional;
mod or;
mod phrase;
mod token_marker_iterator;

use token_marker_iterator::TokenMarkerIterator;

use crate::app::AppMeta;
use crate::storage::Record;
use crate::utils::Word;

use std::pin::Pin;

use futures::prelude::*;

#[derive(Debug, Eq, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub marker: Option<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenMatch<'a> {
    pub token: &'a Token,
    pub match_meta: MatchMeta<'a>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FuzzyMatch<'a> {
    Overflow(TokenMatch<'a>, Word<'a>),
    Exact(TokenMatch<'a>),
    Partial(TokenMatch<'a>, Option<String>),
}

#[derive(Debug, Eq, PartialEq)]
pub enum TokenType {
    /// One or more tokens, in any order but without repetition
    AnyOf(Vec<Token>),

    /// Any sequence of words
    AnyPhrase,

    /// Any single word
    AnyWord,

    /// A literal word
    Keyword(&'static str),

    /// A list of literal words
    KeywordList(Vec<&'static str>),

    /// The name of an existing thing
    Name,

    Optional(Box<Token>),

    /// Any one of the tokens
    Or(Vec<Token>),

    /// The exact sequence of tokens
    Phrase(Vec<Token>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MatchMeta<'a> {
    None,
    Phrase(&'a str),
    Record(Record),
    Sequence(Vec<TokenMatch<'a>>),
    Single(Box<TokenMatch<'a>>),
}

impl Token {
    pub fn any_of<V>(tokens: V) -> Self
    where
        V: Into<Vec<Token>>,
    {
        Token {
            token_type: TokenType::AnyOf(tokens.into()),
            marker: None,
        }
    }

    pub fn any_of_marked<M, V>(marker: M, tokens: V) -> Self
    where
        M: Into<u8>,
        V: Into<Vec<Token>>,
    {
        Token {
            token_type: TokenType::AnyOf(tokens.into()),
            marker: Some(marker.into()),
        }
    }

    pub fn any_phrase() -> Self {
        Token {
            token_type: TokenType::AnyPhrase,
            marker: None,
        }
    }

    pub fn any_phrase_marked<M>(marker: M) -> Self
    where
        M: Into<u8>,
    {
        Token {
            token_type: TokenType::AnyPhrase,
            marker: Some(marker.into()),
        }
    }

    pub fn any_word() -> Self {
        Token {
            token_type: TokenType::AnyWord,
            marker: None,
        }
    }

    pub fn any_word_marked<M>(marker: M) -> Self
    where
        M: Into<u8>,
    {
        Token {
            token_type: TokenType::AnyWord,
            marker: Some(marker.into()),
        }
    }

    pub fn keyword(keyword: &'static str) -> Self {
        Token {
            token_type: TokenType::Keyword(keyword),
            marker: None,
        }
    }

    pub fn keyword_marked<M>(marker: M, keyword: &'static str) -> Self
    where
        M: Into<u8>,
    {
        Token {
            token_type: TokenType::Keyword(keyword),
            marker: Some(marker.into()),
        }
    }

    pub fn keyword_list<I>(keywords: I) -> Self
    where
        I: IntoIterator<Item = &'static str>,
    {
        Token {
            token_type: TokenType::KeywordList(keywords.into_iter().collect()),
            marker: None,
        }
    }

    pub fn keyword_list_marked<M, I>(marker: M, keywords: I) -> Self
    where
        M: Into<u8>,
        I: IntoIterator<Item = &'static str>,
    {
        Token {
            token_type: TokenType::KeywordList(keywords.into_iter().collect()),
            marker: Some(marker.into()),
        }
    }

    pub fn name() -> Self {
        Token {
            token_type: TokenType::Name,
            marker: None,
        }
    }

    pub fn name_marked<M>(marker: M) -> Self
    where
        M: Into<u8>,
    {
        Token {
            token_type: TokenType::Name,
            marker: Some(marker.into()),
        }
    }

    pub fn optional(token: Token) -> Self {
        Token {
            token_type: TokenType::Optional(Box::new(token)),
            marker: None,
        }
    }

    pub fn optional_marked<M>(marker: M, token: Token) -> Self
    where
        M: Into<u8>,
    {
        Token {
            token_type: TokenType::Optional(Box::new(token)),
            marker: Some(marker.into()),
        }
    }

    pub fn or<I>(tokens: I) -> Self
    where
        I: IntoIterator<Item = Token>,
    {
        Token {
            token_type: TokenType::Or(tokens.into_iter().collect()),
            marker: None,
        }
    }

    pub fn or_marked<M, I>(marker: M, tokens: I) -> Self
    where
        M: Into<u8>,
        I: IntoIterator<Item = Token>,
    {
        Token {
            token_type: TokenType::Or(tokens.into_iter().collect()),
            marker: Some(marker.into()),
        }
    }

    pub fn phrase<I>(tokens: I) -> Self
    where
        I: IntoIterator<Item = Token>,
    {
        Token {
            token_type: TokenType::Phrase(tokens.into_iter().collect()),
            marker: None,
        }
    }

    pub fn phrase_marked<M, I>(marker: M, tokens: I) -> Self
    where
        M: Into<u8>,
        I: IntoIterator<Item = Token>,
    {
        Token {
            token_type: TokenType::Phrase(tokens.into_iter().collect()),
            marker: Some(marker.into()),
        }
    }

    pub fn marker_is<M>(&self, marker: M) -> bool
    where
        M: Into<u8>,
    {
        self.marker.as_ref().map_or(false, |m| m == &marker.into())
    }

    pub fn match_input<'a, 'b>(
        &'a self,
        input: &'a str,
        app_meta: &'b AppMeta,
    ) -> Pin<Box<dyn Stream<Item = FuzzyMatch<'a>> + 'b>>
    where
        'a: 'b,
    {
        match &self.token_type {
            TokenType::AnyOf(..) => any_of::match_input(self, input, app_meta),
            TokenType::AnyPhrase => any_phrase::match_input(self, input),
            TokenType::AnyWord => any_word::match_input(self, input),
            TokenType::Keyword(..) => keyword::match_input(self, input),
            TokenType::KeywordList(..) => keyword_list::match_input(self, input),
            TokenType::Name => name::match_input(self, input, app_meta),
            TokenType::Optional(..) => optional::match_input(self, input, app_meta),
            TokenType::Or(..) => or::match_input(self, input, app_meta),
            TokenType::Phrase(..) => phrase::match_input(self, input, app_meta),
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

    pub fn into_exact(self) -> Option<TokenMatch<'a>> {
        if let FuzzyMatch::Exact(token_match) = self {
            Some(token_match)
        } else {
            None
        }
    }
}

impl<'a> TokenMatch<'a> {
    pub fn new(token: &'a Token, match_meta: impl Into<MatchMeta<'a>>) -> Self {
        TokenMatch {
            token,
            match_meta: match_meta.into(),
        }
    }

    pub fn find_markers<'b, T>(&'a self, markers: &'b [T]) -> TokenMarkerIterator<'a, 'b, T>
    where
        &'b T: PartialEq<u8>,
    {
        TokenMarkerIterator::new(self, markers)
    }
}

impl<'a> From<&'a Token> for TokenMatch<'a> {
    fn from(input: &'a Token) -> Self {
        TokenMatch::new(input, MatchMeta::None)
    }
}

impl<'a> MatchMeta<'a> {
    pub fn phrase(&self) -> Option<&str> {
        if let MatchMeta::Phrase(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn record(&self) -> Option<&Record> {
        if let MatchMeta::Record(r) = self {
            Some(r)
        } else {
            None
        }
    }

    pub fn sequence(&self) -> Option<&[TokenMatch<'a>]> {
        if let MatchMeta::Sequence(v) = self {
            Some(v.as_slice())
        } else {
            None
        }
    }

    pub fn single(&self) -> Option<&TokenMatch<'a>> {
        if let MatchMeta::Single(b) = self {
            Some(b.as_ref())
        } else {
            None
        }
    }
}

impl<'a> From<&'a str> for MatchMeta<'a> {
    fn from(input: &'a str) -> Self {
        MatchMeta::Phrase(input)
    }
}

impl<'a> From<Vec<TokenMatch<'a>>> for MatchMeta<'a> {
    fn from(input: Vec<TokenMatch<'a>>) -> Self {
        MatchMeta::Sequence(input)
    }
}

impl<'a> From<TokenMatch<'a>> for MatchMeta<'a> {
    fn from(input: TokenMatch<'a>) -> Self {
        MatchMeta::Single(input.into())
    }
}

impl<'a> From<Record> for MatchMeta<'a> {
    fn from(input: Record) -> Self {
        MatchMeta::Record(input)
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
