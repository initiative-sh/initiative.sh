mod keyword;

use crate::app::AppMeta;
use crate::storage::Record;
use crate::utils::Substr;

use futures::prelude::*;

#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(test, derive(Clone))]
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
    Overflow(TokenMatch<'a>, Substr<'a>),
    Exact(TokenMatch<'a>),
    Partial(TokenMatch<'a>, Option<String>),
}

#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(test, derive(Clone))]
pub enum TokenType {
    /// See [`token_constructors::keyword`].
    Keyword(&'static str),
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
    pub fn match_input<'a, 'b>(
        &'a self,
        input: &'a str,
        _app_meta: &'b AppMeta,
    ) -> impl Stream<Item = FuzzyMatch<'a>> + 'b
    where
        'a: 'b,
    {
        match &self.token_type {
            TokenType::Keyword(..) => keyword::match_input(self, input),
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
}

impl<'a> From<&'a Token> for TokenMatch<'a> {
    fn from(input: &'a Token) -> Self {
        TokenMatch::new(input, MatchMeta::None)
    }
}

impl<'a> FuzzyMatch<'a> {
    #[cfg_attr(not(feature = "integration-tests"), expect(dead_code))]
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
    #[cfg_attr(not(feature = "integration-tests"), expect(dead_code))]
    pub fn phrase(&self) -> Option<&str> {
        if let MatchMeta::Phrase(s) = self {
            Some(s)
        } else {
            None
        }
    }

    #[cfg_attr(not(feature = "integration-tests"), expect(dead_code))]
    pub fn record(&self) -> Option<&Record> {
        if let MatchMeta::Record(r) = self {
            Some(r)
        } else {
            None
        }
    }

    #[cfg_attr(not(feature = "integration-tests"), expect(dead_code))]
    pub fn sequence(&self) -> Option<&[TokenMatch<'a>]> {
        if let MatchMeta::Sequence(v) = self {
            Some(v.as_slice())
        } else {
            None
        }
    }

    #[cfg_attr(not(feature = "integration-tests"), expect(dead_code))]
    pub fn into_sequence(self) -> Option<Vec<TokenMatch<'a>>> {
        if let MatchMeta::Sequence(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[cfg_attr(not(feature = "integration-tests"), expect(dead_code))]
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

impl From<Record> for MatchMeta<'_> {
    fn from(input: Record) -> Self {
        MatchMeta::Record(input)
    }
}

pub mod token_constructors {
    use super::*;

    /// A single keyword, matched case-insensitively.
    ///
    /// # Examples
    ///
    /// ```
    /// # use futures::StreamExt as _;
    /// # tokio_test::block_on(async {
    /// # let app_meta = initiative_core::test_utils::app_meta();
    /// use initiative_core::command::prelude::*;
    ///
    /// let token = keyword("badger");
    ///
    /// assert_eq!(
    ///     Some(TokenMatch::from(&token)),
    ///     token
    ///         .match_input_exact("BADGER", &app_meta)
    ///         .next()
    ///         .await,
    /// );
    /// # })
    /// ```
    ///
    /// ## Autocomplete
    ///
    /// ```
    /// # use futures::StreamExt as _;
    /// # tokio_test::block_on(async {
    /// # let app_meta = initiative_core::test_utils::app_meta();
    /// use initiative_core::command::prelude::*;
    ///
    /// let token = keyword("badger");
    ///
    /// assert_eq!(
    ///     Some(FuzzyMatch::Partial(
    ///         TokenMatch::from(&token),
    ///         Some("er".to_string()),
    ///     )),
    ///     token
    ///         .match_input("badg", &app_meta)
    ///         .next()
    ///         .await,
    /// );
    /// # })
    /// ```
    pub fn keyword(keyword: &'static str) -> Token {
        Token {
            token_type: TokenType::Keyword(keyword),
            marker: None,
        }
    }

    /// A variant of `keyword` with a marker assigned.
    #[cfg_attr(not(any(test, feature = "integration-tests")), expect(dead_code))]
    pub fn keyword_m<M>(marker: M, keyword: &'static str) -> Token
    where
        M: Into<u8>,
    {
        Token {
            token_type: TokenType::Keyword(keyword),
            marker: Some(marker.into()),
        }
    }
}
