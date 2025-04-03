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

/// A variant of `any_of` with a marker assigned, making it easy to jump directly to the
/// matched result within the token tree.
///
/// # Examples
///
/// ```
/// # use futures::StreamExt as _;
/// # tokio_test::block_on(async {
/// # let app_meta = initiative_core::test_utils::app_meta();
/// use initiative_core::command::prelude::*;
///
/// #[derive(Hash)]
/// enum Marker {
///     AnyOf,
/// }
///
/// let query = "badger snake";
/// let token = sequence([
///     keyword("badger"),
///     any_of_m(Marker::AnyOf, [keyword("mushroom"), keyword("snake")]),
/// ]);
/// let token_match = token.match_input_exact(query, &app_meta).next().await.unwrap();
///
/// assert_eq!(
///     Some(&[TokenMatch::from(&keyword("snake"))][..]),
///     token_match.find_marker(Marker::AnyOf).unwrap().meta_sequence(),
/// );
/// # })
/// ```
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

/// A variant of `any_phrase` with a marker assigned, making it easy to jump directly to the
/// matched result within the token tree.
///
/// # Examples
///
/// ```
/// # use futures::StreamExt as _;
/// # tokio_test::block_on(async {
/// # let app_meta = initiative_core::test_utils::app_meta();
/// use initiative_core::command::prelude::*;
///
/// #[derive(Hash)]
/// enum Marker {
///     AnyPhrase,
/// }
///
/// let query = "badger mushroom snake";
/// let token = sequence([keyword("badger"), any_phrase_m(Marker::AnyPhrase)]);
/// let token_match = token.match_input_exact(query, &app_meta).next().await.unwrap();
///
/// assert_eq!(
///     Some("mushroom snake"),
///     token_match.find_marker(Marker::AnyPhrase).unwrap().meta_phrase(),
/// );
/// # })
/// ```
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

/// A variant of `any_word` with a marker assigned, making it easy to jump directly to the
/// matched result within the token tree.
///
/// # Examples
///
/// ```
/// # use futures::StreamExt as _;
/// # tokio_test::block_on(async {
/// # let app_meta = initiative_core::test_utils::app_meta();
/// use initiative_core::command::prelude::*;
///
/// #[derive(Hash)]
/// enum Marker {
///     AnyWord,
/// }
///
/// let query = "badger mushroom";
/// let token = sequence([keyword("badger"), any_phrase_m(Marker::AnyWord)]);
/// let token_match = token.match_input_exact(query, &app_meta).next().await.unwrap();
///
/// assert_eq!(
///     Some("mushroom"),
///     token_match.find_marker(Marker::AnyWord).unwrap().meta_phrase(),
/// );
/// # })
/// ```
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

/// A variant of `keyword` with a marker assigned, making it easy to jump directly to the
/// matched result within the token tree.
///
/// # Examples
///
/// ```
/// # use futures::StreamExt as _;
/// # tokio_test::block_on(async {
/// # let app_meta = initiative_core::test_utils::app_meta();
/// use initiative_core::command::prelude::*;
///
/// #[derive(Hash)]
/// enum Marker {
///     Keyword,
/// }
///
/// let token = sequence([
///     optional(keyword_m(Marker::Keyword, "badger")),
///     keyword("mushroom"),
/// ]);
///
/// // We can easily distinguish between the case when the keyword was matched
/// let query = "badger mushroom";
/// let token_match = token.match_input_exact(query, &app_meta).next().await.unwrap();
/// assert!(token_match.contains_marker(Marker::Keyword));
///
/// // and when it wasn't.
/// let query = "mushroom";
/// let token_match = token.match_input_exact(query, &app_meta).next().await.unwrap();
/// assert!(!token_match.contains_marker(Marker::Keyword));
/// # })
/// ```
pub fn keyword_m<M>(marker: M, keyword: &'static str) -> Token
where
    M: Hash,
{
    Token::new_m(marker, TokenType::Keyword(keyword))
}

/// Matches exactly one of a set of possible keywords, case-insensitively.
///
/// # Examples
///
/// ```
/// # use futures::StreamExt as _;
/// # tokio_test::block_on(async {
/// # let app_meta = initiative_core::test_utils::app_meta();
/// use initiative_core::command::prelude::*;
///
/// let token = keyword_list(["badger", "mushroom", "snake"]);
///
/// // Only consumes one word, despite the repetition in the input.
/// assert_eq!(
///     vec![FuzzyMatch::Overflow(
///         TokenMatch::new(&token, "badger"),
///         " badger mushroom".into(),
///     )],
///     token
///         .match_input("badger badger mushroom", &app_meta)
///         .collect::<Vec<_>>()
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
/// let token = keyword_list(["badge", "badger"]);
///
/// assert_eq!(
///     vec![
///         // The input appears in the keyword list,
///         FuzzyMatch::Exact(TokenMatch::new(&token, "badge")),
///
///         // but can also be completed to another word.
///         FuzzyMatch::Partial(
///             TokenMatch::new(&token, "badge"),
///             Some("r".to_string()),
///         ),
///     ],
///     token
///         .match_input("badge", &app_meta)
///         .collect::<Vec<_>>()
///         .await,
/// );
/// # })
/// ```
pub fn keyword_list<V>(keywords: V) -> Token
where
    V: IntoIterator<Item = &'static str>,
{
    Token::new(TokenType::KeywordList(keywords.into_iter().collect()))
}

/// A variant of `any_word` with a marker assigned, making it easy to jump directly to the
/// matched result within the token tree.
///
/// # Examples
///
/// ```
/// # use futures::StreamExt as _;
/// # tokio_test::block_on(async {
/// # let app_meta = initiative_core::test_utils::app_meta();
/// use initiative_core::command::prelude::*;
///
/// #[derive(Hash)]
/// enum Marker {
///     KeywordList,
/// }
///
/// let query = "badger mushroom";
/// let token = sequence([
///     keyword("badger"),
///     keyword_list_m(Marker::KeywordList, ["mushroom", "snake"]),
/// ]);
/// let token_match = token.match_input_exact(query, &app_meta).next().await.unwrap();
///
/// assert_eq!(
///     Some("mushroom"),
///     token_match.find_marker(Marker::KeywordList).unwrap().meta_phrase(),
/// );
/// # })
/// ```
pub fn keyword_list_m<M, V>(marker: M, keywords: V) -> Token
where
    M: Hash,
    V: IntoIterator<Item = &'static str>,
{
    Token::new_m(
        marker,
        TokenType::KeywordList(keywords.into_iter().collect()),
    )
}

/// Matches the name of a Thing found in the journal or recent entities.
///
/// # Examples
///
/// ```
/// # use futures::StreamExt as _;
/// # tokio_test::block_on(async {
/// # let app_meta = initiative_core::test_utils::app_meta::with_test_data().await;
/// use initiative_core::command::prelude::*;
///
/// let query = "odysseus";
/// let token = name();
/// let odysseus = app_meta.repository.get_by_name("Odysseus").await.unwrap();
/// let token_match = token.match_input_exact(query, &app_meta).next().await.unwrap();
///
/// assert_eq!(TokenMatch::new(&token, odysseus.clone()), token_match);
///
/// // The matched Record can be accessed directly from the TokenMatch tree.
/// assert_eq!(Some(&odysseus), token_match.meta_record());
/// # })
/// ```
///
/// ## Autocomplete
///
/// ```
/// # use futures::StreamExt as _;
/// # tokio_test::block_on(async {
/// # let app_meta = initiative_core::test_utils::app_meta::with_test_data().await;
/// use initiative_core::command::prelude::*;
///
/// let query = "ody";
/// let token = name();
/// let odysseus = app_meta.repository.get_by_name("Odysseus").await.unwrap();
///
/// assert_eq!(
///     Some(FuzzyMatch::Partial(
///         TokenMatch::new(&token, odysseus),
///         Some("sseus".to_string()),
///     )),
///     token.match_input(query, &app_meta).next().await,
/// );
/// # })
/// ```
pub fn name() -> Token {
    Token::new(TokenType::Name)
}

/// A variant of `name` with a marker assigned, making it easy to jump directly to the
/// matched result within the token tree.
///
/// # Examples
///
/// ```
/// # use futures::StreamExt as _;
/// # tokio_test::block_on(async {
/// # let app_meta = initiative_core::test_utils::app_meta::with_test_data().await;
/// use initiative_core::command::prelude::*;
///
/// #[derive(Hash)]
/// enum Marker {
///     Name,
/// }
///
/// let query = "hail odysseus";
/// let token = sequence([keyword("hail"), name_m(Marker::Name)]);
/// let odysseus = app_meta.repository.get_by_name("Odysseus").await.unwrap();
///
/// let token_match = token.match_input_exact(query, &app_meta).next().await.unwrap();
///
/// assert_eq!(
///     Some(&odysseus),
///     token_match.find_marker(Marker::Name).unwrap().meta_record(),
/// );
/// # })
/// ```
pub fn name_m<M>(marker: M) -> Token
where
    M: Hash,
{
    Token::new_m(marker, TokenType::Name)
}

/// Matches the input with and without the contained token.
///
/// # Examples
///
/// ```
/// # use futures::StreamExt as _;
/// # tokio_test::block_on(async {
/// # let app_meta = initiative_core::test_utils::app_meta();
/// use initiative_core::command::prelude::*;
///
/// let token = optional(keyword("badger"));
///
/// assert_eq!(
///     vec![
///         // Passes the input directly through to the overflow,
///         FuzzyMatch::Overflow(TokenMatch::from(&token), "badger".into()),
///
///         // as well as the matched result if present.
///         FuzzyMatch::Exact(TokenMatch::new(&token, TokenMatch::from(&keyword("badger")))),
///     ],
///     token
///         .match_input("badger", &app_meta)
///         .collect::<Vec<_>>()
///         .await,
/// );
/// # })
/// ```
pub fn optional(token: Token) -> Token {
    Token::new(TokenType::Optional(Box::new(token)))
}

/// A variant of `name` with a marker assigned, making it easy to jump directly to the
/// matched result within the token tree.
///
/// # Examples
///
/// We can inspect the metadata of the `optional` token to see if a match occurred or not. More
/// normally, the inner token would be marked and we would check to see if the marked token
/// exists in the tree (`TokenMatch::contains_token`).
///
/// ```
/// # use futures::StreamExt as _;
/// # tokio_test::block_on(async {
/// # let app_meta = initiative_core::test_utils::app_meta::with_test_data().await;
/// use initiative_core::command::prelude::*;
///
/// #[derive(Hash)]
/// enum Marker {
///     Optional,
/// }
///
/// let token = sequence([
///     optional_m(Marker::Optional, keyword("badger")),
///     keyword("mushroom"),
/// ]);
///
/// // We can inspect the metadata to see that this case contains a match
/// let query = "badger mushroom";
/// let token_match = token.match_input_exact(query, &app_meta).next().await.unwrap();
/// assert!(token_match.find_marker(Marker::Optional).unwrap().meta_single().is_some());
///
/// // and this case does not.
/// let query = "mushroom";
/// let token_match = token.match_input_exact(query, &app_meta).next().await.unwrap();
/// assert!(token_match.find_marker(Marker::Optional).unwrap().meta_single().is_none());
/// # })
/// ```
pub fn optional_m<M>(marker: M, token: Token) -> Token
where
    M: Hash,
{
    Token::new_m(marker, TokenType::Optional(Box::new(token)))
}

/// Matches exactly one of a set of possible tokens. The matched token will be included in the
/// result.
///
/// # Examples
///
/// ```
/// # use futures::StreamExt as _;
/// # tokio_test::block_on(async {
/// # let app_meta = initiative_core::test_utils::app_meta();
/// use initiative_core::command::prelude::*;
///
/// let token = or([keyword("badger"), any_word()]);
///
/// assert_eq!(
///     vec![
///         // "badger" matches a provided keyword,
///         FuzzyMatch::Overflow(
///             TokenMatch::new(&token, TokenMatch::from(&keyword("badger"))),
///             " badger".into(),
///         ),
///
///         // but it satisfies the wildcard any_word() case as well.
///         // It only ever matches a single token, so the second "badger" in the input is
///         // never consumed.
///         FuzzyMatch::Overflow(
///             TokenMatch::new(&token, TokenMatch::new(&any_word(), "badger")),
///             " badger".into(),
///         ),
///     ],
///     token
///         .match_input("badger badger", &app_meta)
///         .collect::<Vec<_>>()
///         .await,
/// );
/// # })
/// ```
pub fn or<V>(tokens: V) -> Token
where
    V: IntoIterator<Item = Token>,
{
    Token::new(TokenType::Or(tokens.into_iter().collect()))
}

/// A variant of `or` with a marker assigned.
pub fn or_m<M, V>(marker: M, tokens: V) -> Token
where
    M: Hash,
    V: IntoIterator<Item = Token>,
{
    Token::new_m(marker, TokenType::Or(tokens.into_iter().collect()))
}

/// Matches an exact sequence of tokens.
///
/// # Examples
///
/// ```
/// # use initiative_core::command::prelude::*;
/// # use futures::StreamExt as _;
/// # tokio_test::block_on(async {
/// # let app_meta = initiative_core::test_utils::app_meta();
/// let token = sequence([keyword("badger"), keyword("mushroom"), keyword("snake")]);
///
/// // The first two keywords are matched, but the third is not present.
/// assert_eq!(
///     vec![FuzzyMatch::Partial(
///         TokenMatch::new(&token, vec![
///             TokenMatch::from(&keyword("badger")),
///             TokenMatch::from(&keyword("mushroom")),
///         ]),
///         None,
///     )],
///     token
///         .match_input("badger mushroom", &app_meta)
///         .collect::<Vec<_>>()
///         .await,
/// );
/// # })
/// ```
pub fn sequence<V>(tokens: V) -> Token
where
    V: Into<Vec<Token>>,
{
    Token::new(TokenType::Sequence(tokens.into()))
}

/// A variant of `sequence` with a marker assigned.
pub fn sequence_m<M, V>(marker: M, tokens: V) -> Token
where
    M: Hash,
    V: Into<Vec<Token>>,
{
    Token::new_m(marker, TokenType::Sequence(tokens.into()))
}
