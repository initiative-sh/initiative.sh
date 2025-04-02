#![cfg_attr(not(any(test, feature = "integration-tests")), expect(dead_code))]

use super::{Token, TokenType};
use std::hash::Hash;

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
pub fn any_of<V>(tokens: V) -> Token
where
    V: Into<Vec<Token>>,
{
    Token::new(TokenType::AnyOf(tokens.into()))
}

/// A variant of `any_of` with a marker assigned.
pub fn any_of_m<M, V>(marker: M, tokens: V) -> Token
where
    M: Hash,
    V: Into<Vec<Token>>,
{
    Token::new_m(marker, TokenType::AnyOf(tokens.into()))
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
pub fn any_phrase() -> Token {
    Token::new(TokenType::AnyPhrase)
}

/// A variant of `any_phrase` with a marker assigned.
pub fn any_phrase_m<M>(marker: M) -> Token
where
    M: Hash,
{
    Token::new_m(marker, TokenType::AnyPhrase)
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
pub fn any_word() -> Token {
    Token::new(TokenType::AnyWord)
}

/// A variant of `any_word` with a marker assigned.
pub fn any_word_m<M>(marker: M) -> Token
where
    M: Hash,
{
    Token::new_m(marker, TokenType::AnyWord)
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
    Token::new(TokenType::Keyword(keyword))
}

/// A variant of `keyword` with a marker assigned.
pub fn keyword_m<M>(marker: M, keyword: &'static str) -> Token
where
    M: Hash,
{
    Token::new_m(marker, TokenType::Keyword(keyword))
}
