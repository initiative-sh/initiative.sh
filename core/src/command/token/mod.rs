mod any_of;
mod any_phrase;
mod any_word;
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
    /// See [`token_constructors::any_of`].
    AnyOf(Vec<Token>),

    /// See [`token_constructors::any_phrase`].
    AnyPhrase,

    /// See [`token_constructors::any_word`].
    AnyWord,

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

    /// Matches one or more of a set of tokens, in any order but without repetition.
    ///
    /// # Examples
    ///
    /// ```
    /// # use futures::StreamExt as _;
    /// # tokio_test::block_on(async {
    /// # let app_meta = initiative_core::test_utils::app_meta();
    /// use initiative_core::command::prelude::*;
    ///
    /// let token = any_of([keyword("badger"), keyword("mushroom"), keyword("snake")]);
    ///
    /// assert_eq!(
    ///     vec![
    ///         // "Ungreedy" version consuming only one token,
    ///         FuzzyMatch::Overflow(
    ///             TokenMatch::new(&token, vec![
    ///                 TokenMatch::from(&keyword("mushroom")),
    ///             ]),
    ///             " snake badger badger".into(),
    ///         ),
    ///
    ///         // two tokens,
    ///         FuzzyMatch::Overflow(
    ///             TokenMatch::new(&token, vec![
    ///                 TokenMatch::from(&keyword("mushroom")),
    ///                 TokenMatch::from(&keyword("snake")),
    ///             ]),
    ///             " badger badger".into(),
    ///         ),
    ///
    ///         // and all three tokens. The final word is repeated and so does not match.
    ///         FuzzyMatch::Overflow(
    ///             TokenMatch::new(&token, vec![
    ///                 TokenMatch::from(&keyword("mushroom")),
    ///                 TokenMatch::from(&keyword("snake")),
    ///                 TokenMatch::from(&keyword("badger")),
    ///             ]),
    ///             " badger".into(),
    ///         ),
    ///     ],
    ///     token
    ///         .match_input("mushroom snake badger badger", &app_meta)
    ///         .collect::<Vec<_>>()
    ///         .await,
    /// );
    /// # })
    /// ```
    #[cfg_attr(not(any(test, feature = "integration-tests")), expect(dead_code))]
    pub fn any_of<V>(tokens: V) -> Token
    where
        V: Into<Vec<Token>>,
    {
        Token {
            token_type: TokenType::AnyOf(tokens.into()),
            marker: None,
        }
    }

    /// A variant of `any_of` with a marker assigned.
    #[cfg_attr(not(any(test, feature = "integration-tests")), expect(dead_code))]
    pub fn any_of_m<M, V>(marker: M, tokens: V) -> Token
    where
        M: Into<u8>,
        V: Into<Vec<Token>>,
    {
        Token {
            token_type: TokenType::AnyOf(tokens.into()),
            marker: Some(marker.into()),
        }
    }

    /// Matches all sequences of one or more words. Quoted phrases are treated as single words.
    ///
    /// # Examples
    ///
    /// ```
    /// # use futures::StreamExt as _;
    /// # tokio_test::block_on(async {
    /// # let app_meta = initiative_core::test_utils::app_meta();
    /// use initiative_core::command::prelude::*;
    ///
    /// let token = any_phrase();
    ///
    /// assert_eq!(
    ///     vec![
    ///         // Ungreedily matches the quoted phrase as a single token,
    ///         FuzzyMatch::Overflow(
    ///             TokenMatch::new(&token, "badger badger"),
    ///             " mushroom snake ".into(),
    ///         ),
    ///
    ///         // the first two "words",
    ///         FuzzyMatch::Overflow(
    ///             TokenMatch::new(&token, r#""badger badger" mushroom"#),
    ///             " snake ".into(),
    ///         ),
    ///
    ///         // and the whole phrase.
    ///         FuzzyMatch::Exact(TokenMatch::new(&token, r#""badger badger" mushroom snake"#)),
    ///     ],
    ///     token
    ///         .match_input(r#" "badger badger" mushroom snake "#, &app_meta)
    ///         .collect::<Vec<_>>()
    ///         .await,
    /// );
    /// # })
    /// ```
    #[cfg_attr(not(any(test, feature = "integration-tests")), expect(dead_code))]
    pub fn any_phrase() -> Token {
        Token {
            token_type: TokenType::AnyPhrase,
            marker: None,
        }
    }

    /// A variant of `any_phrase` with a marker assigned.
    #[cfg_attr(not(any(test, feature = "integration-tests")), expect(dead_code))]
    pub fn any_phrase_m<M>(marker: M) -> Token
    where
        M: Into<u8>,
    {
        Token {
            token_type: TokenType::AnyPhrase,
            marker: Some(marker.into()),
        }
    }

    /// Matches any single word.
    ///
    /// # Examples
    ///
    /// ```
    /// # use initiative_core::command::prelude::*;
    /// # use futures::StreamExt as _;
    /// # tokio_test::block_on(async {
    /// # let app_meta = initiative_core::test_utils::app_meta();
    /// let token = any_word();
    ///
    /// assert_eq!(
    ///     Some(TokenMatch::new(&token, "BADGER")),
    ///     token
    ///         .match_input_exact("BADGER", &app_meta)
    ///         .next()
    ///         .await,
    /// );
    /// # })
    /// ```
    #[cfg_attr(not(any(test, feature = "integration-tests")), expect(dead_code))]
    pub fn any_word() -> Token {
        Token {
            token_type: TokenType::AnyWord,
            marker: None,
        }
    }

    /// A variant of `any_word` with a marker assigned.
    #[cfg_attr(not(any(test, feature = "integration-tests")), expect(dead_code))]
    pub fn any_word_m<M>(marker: M) -> Token
    where
        M: Into<u8>,
    {
        Token {
            token_type: TokenType::AnyWord,
            marker: Some(marker.into()),
        }
    }

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
