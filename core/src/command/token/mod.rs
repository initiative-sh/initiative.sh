pub mod constructors;

mod any_of;
mod any_phrase;
mod any_word;
mod keyword;
mod keyword_list;
mod name;
mod optional;
mod or;
mod sequence;

use std::hash::{DefaultHasher, Hash, Hasher};

use crate::app::AppMeta;
use crate::storage::Record;
use crate::utils::Substr;
use initiative_macros::From;

use futures::prelude::*;

#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(test, derive(Clone))]
pub struct Token {
    pub token_type: TokenType,
    pub marker: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenMatch<'a> {
    pub token: &'a Token,
    pub match_meta: MatchMeta<'a>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FuzzyMatch<'a> {
    Overflow(TokenMatch<'a>, Substr<'a>),
    Exact(TokenMatch<'a>),
    Partial(TokenMatch<'a>, Option<String>),
}

#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(test, derive(Clone))]
pub enum TokenType {
    /// See [`token_constructors::any_of`].
    AnyOf(Vec<Token>),

    /// See [`token_constructors::any_phrase`].
    AnyPhrase,

    /// See [`token_constructors::any_word`].
    AnyWord,

    /// See [`token_constructors::keyword`].
    Keyword(&'static str),

    /// See [`token_constructors::keyword_list`].
    KeywordList(Vec<&'static str>),

    /// See [`token_constructors::name`].
    Name,

    /// See [`token_constructors::optional`].
    Optional(Box<Token>),

    /// See [`token_constructors::or`].
    Or(Vec<Token>),

    /// See [`token_constructors::sequence`].
    Sequence(Vec<Token>),
}

#[derive(Clone, Debug, Eq, From, PartialEq)]
pub enum MatchMeta<'a> {
    None,
    Phrase(&'a str),
    Record(Record),
    Sequence(Vec<TokenMatch<'a>>),
    Single(Box<TokenMatch<'a>>),
}

impl Token {
    pub fn new(token_type: TokenType) -> Token {
        Token {
            token_type,
            marker: 0,
        }
    }

    pub fn new_m<M: Hash>(marker: M, token_type: TokenType) -> Token {
        Token {
            token_type,
            marker: hash_marker(marker),
        }
    }

    pub fn match_input<'a, 'b>(
        &'a self,
        input: &'a str,
        app_meta: &'b AppMeta,
    ) -> impl Stream<Item = FuzzyMatch<'a>> + 'b
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
            TokenType::Sequence(..) => sequence::match_input(self, input, app_meta),
        }
    }

    pub fn match_input_exact<'a, 'b>(
        &'a self,
        input: &'a str,
        app_meta: &'b AppMeta,
    ) -> impl Stream<Item = TokenMatch<'a>> + 'b
    where
        'a: 'b,
    {
        self.match_input(input, app_meta)
            .filter_map(|fuzzy_match| future::ready(fuzzy_match.into_exact()))
    }
}

impl<'a> TokenMatch<'a> {
    pub fn new(token: &'a Token, match_meta: impl Into<MatchMeta<'a>>) -> Self {
        TokenMatch {
            token,
            match_meta: match_meta.into(),
        }
    }

    #[cfg_attr(not(feature = "integration-tests"), expect(dead_code))]
    pub fn meta_phrase(&self) -> Option<&str> {
        self.match_meta.phrase()
    }

    #[cfg_attr(not(feature = "integration-tests"), expect(dead_code))]
    pub fn meta_record(&self) -> Option<&Record> {
        self.match_meta.record()
    }

    #[cfg_attr(not(feature = "integration-tests"), expect(dead_code))]
    pub fn meta_sequence(&self) -> Option<&[TokenMatch<'a>]> {
        self.match_meta.sequence()
    }

    #[cfg_attr(not(feature = "integration-tests"), expect(dead_code))]
    pub fn meta_single(&self) -> Option<&TokenMatch<'a>> {
        self.match_meta.single()
    }
}

impl<'a> From<&'a Token> for TokenMatch<'a> {
    fn from(input: &'a Token) -> Self {
        TokenMatch::new(input, MatchMeta::None)
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

    #[cfg_attr(not(feature = "integration-tests"), expect(dead_code))]
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

    pub fn into_sequence(self) -> Option<Vec<TokenMatch<'a>>> {
        if let MatchMeta::Sequence(v) = self {
            Some(v)
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

impl<'a> From<TokenMatch<'a>> for MatchMeta<'a> {
    fn from(input: TokenMatch<'a>) -> MatchMeta<'a> {
        Box::new(input).into()
    }
}

fn hash_marker<M: Hash>(marker: M) -> u64 {
    let mut hasher = DefaultHasher::default();
    marker.hash(&mut hasher);
    hasher.finish()
}
