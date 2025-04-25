#![cfg_attr(not(any(test, feature = "integration-tests")), expect(dead_code))]

use super::{Token, TokenKind};

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
///         // Matches all three tokens. The final word was already consumed and so does not match.
///         FuzzyMatchList::new_overflow(
///             vec![
///                 MatchPart::new_unmarked("MUSHROOM".into()).with_term("mushroom"),
///                 MatchPart::new_unmarked("SNAKE".into()).with_term("snake"),
///                 MatchPart::new_unmarked("BADGER".into()).with_term("badger"),
///             ],
///             " BADGER".into(),
///         ),
///
///         // It will also return "ungreedy" overflowing results with two tokens
///         FuzzyMatchList::new_overflow(
///             vec![
///                 MatchPart::new_unmarked("MUSHROOM".into()).with_term("mushroom"),
///                 MatchPart::new_unmarked("SNAKE".into()).with_term("snake"),
///             ],
///             " BADGER BADGER".into(),
///         ),
///
///         // as well as only one token.
///         FuzzyMatchList::new_overflow(
///             MatchPart::new_unmarked("MUSHROOM".into()).with_term("mushroom"),
///             " SNAKE BADGER BADGER".into(),
///         ),
///     ],
///     token
///         .match_input("MUSHROOM SNAKE BADGER BADGER", &app_meta)
///         .collect::<Vec<_>>()
///         .await,
/// );
/// # })
/// ```
pub fn any_of<V>(tokens: V) -> Token
where
    V: Into<Vec<Token>>,
{
    TokenKind::AnyOf {
        tokens: tokens.into(),
    }
    .into()
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
///         FuzzyMatchList::new_overflow(
///             MatchPart::new_unmarked("badger badger".into()),
///             " mushroom snake ".into(),
///         ),
///
///         // the first two "words",
///         FuzzyMatchList::new_overflow(
///             MatchPart::new_unmarked(r#""badger badger" mushroom"#.into()),
///             " snake ".into(),
///         ),
///
///         // and the whole phrase.
///         FuzzyMatchList::new_exact(
///             MatchPart::new_unmarked(r#""badger badger" mushroom snake"#.into()),
///         ),
///     ],
///     token
///         .match_input(r#" "badger badger" mushroom snake "#, &app_meta)
///         .collect::<Vec<_>>()
///         .await,
/// );
/// # })
/// ```
pub fn any_phrase() -> Token {
    TokenKind::AnyPhrase.into()
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
///     Some(MatchList::from(MatchPart::new_unmarked("BADGER".into()))),
///     token
///         .match_input_exact("BADGER", &app_meta)
///         .next()
///         .await,
/// );
/// # })
/// ```
pub fn any_word() -> Token {
    TokenKind::AnyWord.into()
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
///     Some(&MatchPart::new_unmarked("BADGER".into()).with_term("badger")),
///     token
///         .match_input_exact("BADGER", &app_meta)
///         .next()
///         .await
///         .unwrap()
///         .first(),
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
/// let fuzzy_match_list = token
///     .match_input("BADG", &app_meta)
///     .next()
///     .await
///     .unwrap();
///
/// assert_eq!(Some("BADGer".to_string()), fuzzy_match_list.autocomplete_term());
/// # })
/// ```
pub fn keyword(term: &'static str) -> Token {
    TokenKind::Keyword { term }.into()
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
///     vec![FuzzyMatchList::new_overflow(
///         MatchPart::new_unmarked("BADGER".into()).with_term("badger"),
///         " BADGER MUSHROOM".into(),
///     )],
///     token
///         .match_input("BADGER BADGER MUSHROOM", &app_meta)
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
/// let token = keyword_list(["badger", "mushroom"]);
///
/// let fuzzy_match_list = token
///     .match_input("MUSH", &app_meta)
///     .next()
///     .await
///     .unwrap();
///
/// assert_eq!(Some("MUSHroom".to_string()), fuzzy_match_list.autocomplete_term());
/// # })
/// ```
pub fn keyword_list<I>(terms: I) -> Token
where
    I: IntoIterator<Item = &'static str>,
{
    TokenKind::KeywordList {
        terms: terms.into_iter().collect(),
    }
    .into()
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
/// let match_list = token.match_input_exact(query, &app_meta).next().await.unwrap();
///
/// // The matched Record can be accessed directly from the TokenMatch tree.
/// assert_eq!(Some(&odysseus), match_list.first().unwrap().record());
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
/// let token = name();
///
/// let fuzzy_match_list = token
///     .match_input("ODY", &app_meta)
///     .next()
///     .await
///     .unwrap();
///
/// assert_eq!(
///     Some("ODYsseus".to_string()),
///     fuzzy_match_list.autocomplete_term(),
/// );
/// # })
/// ```
pub fn name() -> Token {
    TokenKind::Name.into()
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
///         FuzzyMatchList::new_overflow(vec![], "badger".into()),
///
///         // as well as the matched result if present.
///         FuzzyMatchList::new_exact(
///             MatchPart::new_unmarked("badger".into())
///                 .with_term("badger"),
///         ),
///     ],
///     token
///         .match_input("badger", &app_meta)
///         .collect::<Vec<_>>()
///         .await,
/// );
/// # })
/// ```
pub fn optional(token: Token) -> Token {
    TokenKind::Optional {
        token: token.into(),
    }
    .into()
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
/// #[derive(Hash)]
/// enum Marker {
///     Keyword,
///     AnyWord,
/// }
///
/// let token = or([keyword("badger").with_marker(Marker::Keyword),
/// any_word().with_marker(Marker::AnyWord)]);
///
/// assert_eq!(
///     vec![
///         // "badger" matches a provided keyword,
///         FuzzyMatchList::new_overflow(
///             MatchPart::new_unmarked("badger".into())
///                 .with_marker(Marker::Keyword)
///                 .with_term("badger"),
///             " badger".into(),
///         ),
///
///         // but it satisfies the wildcard any_word() case as well.
///         // It only ever matches a single token, so the second "badger" in the input is
///         // never consumed.
///         FuzzyMatchList::new_overflow(
///             MatchPart::new_unmarked("badger".into())
///                 .with_marker(Marker::AnyWord),
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
pub fn or<I>(tokens: I) -> Token
where
    I: IntoIterator<Item = Token>,
{
    TokenKind::Or {
        tokens: tokens.into_iter().collect(),
    }
    .into()
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
/// #[derive(Hash)]
/// enum Marker {
///     Badger,
///     Mushroom,
///     Snake
/// }
///
/// let token = sequence([
///     keyword("badger").with_marker(Marker::Badger),
///     keyword("mushroom").with_marker(Marker::Mushroom),
///     keyword("snake").with_marker(Marker::Snake),
/// ]);
///
/// let fuzzy_match_list = token
///     .match_input("BADGER MUSHROOM", &app_meta)
///     .next()
///     .await
///     .unwrap();
///
/// let mut match_part_iter = fuzzy_match_list.match_list.iter();
///
/// // The first two keywords are matched, but the third is not present.
/// assert!(match_part_iter.next().unwrap().has_marker(Marker::Badger));
/// assert!(match_part_iter.next().unwrap().has_marker(Marker::Mushroom));
/// assert_eq!(None, match_part_iter.next());
/// # })
/// ```
///
/// ## Autocomplete
///
/// ```
/// # use initiative_core::command::prelude::*;
/// # use futures::StreamExt as _;
/// # tokio_test::block_on(async {
/// # let app_meta = initiative_core::test_utils::app_meta();
///
/// let token = sequence([keyword("badger"), keyword("mushroom"), keyword("snake")]);
///
/// let fuzzy_match_list = token
///     .match_input("BADGER", &app_meta)
///     .next()
///     .await
///     .unwrap();
///
/// assert_eq!(
///     Some("BADGER mushroom".to_string()),
///     fuzzy_match_list.autocomplete_term(),
/// );
/// # })
/// ```
pub fn sequence<V>(tokens: V) -> Token
where
    V: Into<Vec<Token>>,
{
    TokenKind::Sequence {
        tokens: tokens.into(),
    }
    .into()
}
