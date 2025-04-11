#![cfg_attr(not(any(test, feature = "integration-tests")), expect(dead_code))]

use super::{Token, TokenType};
use std::hash::Hash;

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
