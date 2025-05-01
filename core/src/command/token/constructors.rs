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
/// let mut stream = token.match_input("MUSHROOM SNAKE BADGER BADGER", &app_meta);
///
/// // Matches all three tokens. The final word was already consumed and so does not match.
/// let fuzzy_match_list = stream.next().await.unwrap();
/// let mut match_part_iter = fuzzy_match_list.complete_parts();
/// assert_eq!(Some("mushroom"), match_part_iter.next().unwrap().term());
/// assert_eq!(Some("snake"), match_part_iter.next().unwrap().term());
/// assert_eq!(Some("badger"), match_part_iter.next().unwrap().term());
/// assert_eq!(None, match_part_iter.next());
/// assert_eq!(" BADGER", fuzzy_match_list.overflow().unwrap());
///
/// // It will also return "ungreedy" overflowing results with two tokens
/// let fuzzy_match_list = stream.next().await.unwrap();
/// let mut match_part_iter = fuzzy_match_list.complete_parts();
/// assert_eq!(Some("mushroom"), match_part_iter.next().unwrap().term());
/// assert_eq!(Some("snake"), match_part_iter.next().unwrap().term());
/// assert_eq!(None, match_part_iter.next());
/// assert_eq!(" BADGER BADGER", fuzzy_match_list.overflow().unwrap());
///
/// // as well as only one token.
/// let fuzzy_match_list = stream.next().await.unwrap();
/// let mut match_part_iter = fuzzy_match_list.complete_parts();
/// assert_eq!(Some("mushroom"), match_part_iter.next().unwrap().term());
/// assert_eq!(None, match_part_iter.next());
/// assert_eq!(" SNAKE BADGER BADGER", fuzzy_match_list.overflow().unwrap());
///
/// // fin
/// assert_eq!(None, stream.next().await);
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
/// let mut stream = token.match_input(r#" "badger badger" mushroom snake "#, &app_meta);
///
/// // Ungreedily matches the quoted phrase as a single token,
/// let fuzzy_match_list = stream.next().await.unwrap();
/// assert_eq!(
///     "badger badger",
///     fuzzy_match_list.complete_parts().next().unwrap().input(),
/// );
/// assert_eq!(" mushroom snake ", fuzzy_match_list.overflow().unwrap());
///
/// // the first two "words",
/// let fuzzy_match_list = stream.next().await.unwrap();
/// assert_eq!(
///     r#""badger badger" mushroom"#,
///     fuzzy_match_list.complete_parts().next().unwrap().input(),
/// );
/// assert_eq!(" snake ", fuzzy_match_list.overflow().unwrap());
///
/// // and the whole phrase.
/// let fuzzy_match_list = stream.next().await.unwrap();
/// assert_eq!(
///     r#""badger badger" mushroom snake"#,
///     fuzzy_match_list.complete_parts().next().unwrap().input(),
/// );
/// assert_eq!(None, fuzzy_match_list.overflow());
///
/// assert_eq!(None, stream.next().await);
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
/// # use futures::StreamExt as _;
/// # tokio_test::block_on(async {
/// # let app_meta = initiative_core::test_utils::app_meta();
/// use initiative_core::command::prelude::*;
///
/// let token = any_word();
/// let mut stream = token.match_input("badger badger", &app_meta);
///
/// // Ungreedily matches the quoted phrase as a single token,
/// let fuzzy_match_list = stream.next().await.unwrap();
/// assert_eq!(
///     "badger",
///     fuzzy_match_list.complete_parts().next().unwrap().input(),
/// );
/// assert_eq!(" badger", fuzzy_match_list.overflow().unwrap());
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
/// let mut stream = token.match_input_exact("BADGER", &app_meta);
///
/// let match_list = stream.next().await.unwrap();
/// let match_part = match_list.parts().next().unwrap();
///
/// assert_eq!("BADGER", match_part.input());
/// assert_eq!(Some("badger"), match_part.term());
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
/// let mut stream = token.match_input("BADGER BADGER MUSHROOM", &app_meta);
///
/// // Only consumes one word, despite the repetition in the input.
/// let fuzzy_match_list = stream.next().await.unwrap();
/// let match_part = fuzzy_match_list.complete_parts().next().unwrap();
///
/// assert_eq!("BADGER", match_part.input());
/// assert_eq!(Some("badger"), match_part.term());
/// assert_eq!(" BADGER MUSHROOM", fuzzy_match_list.overflow().unwrap());
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
/// let odysseus = app_meta.repository.get_by_name("Odysseus").await.unwrap();
///
/// let token = name();
/// let match_list = token.match_input_exact("odysseus", &app_meta).next().await.unwrap();
///
/// // The matched Record can be accessed directly from the MatchList.
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
/// assert_eq!("ODYsseus", fuzzy_match_list.autocomplete_term().unwrap());
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
/// let mut stream = token.match_input("BADGER", &app_meta);
///
/// // Yields the full input sent to overflow,
/// let fuzzy_match_list = stream.next().await.unwrap();
/// assert_eq!(None, fuzzy_match_list.complete_parts().next());
/// assert_eq!("BADGER", fuzzy_match_list.overflow().unwrap());
///
/// // as well as the matched result if present.
/// let fuzzy_match_list = stream.next().await.unwrap();
/// let match_part = fuzzy_match_list.complete_parts().next().unwrap();
/// assert_eq!("badger", match_part.term().unwrap());
/// assert_eq!(None, fuzzy_match_list.overflow());
///
/// // fin
/// assert_eq!(None, stream.next().await);
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
/// let token = or([
///     keyword("badger").with_marker(Marker::Keyword),
///     any_word().with_marker(Marker::AnyWord),
/// ]);
/// let mut stream = token.match_input("badger badger", &app_meta);
///
/// // "badger" matches a provided keyword,
/// let fuzzy_match_list = stream.next().await.unwrap();
/// let match_part = fuzzy_match_list.complete_parts().next().unwrap();
/// assert!(match_part.has_marker(Marker::Keyword));
/// assert_eq!(" badger", fuzzy_match_list.overflow().unwrap());
///
/// // but it satisfies the wildcard any_word() case as well.
/// // It only ever matches a single token, so the second "badger" in the input is
/// // never consumed.
/// let fuzzy_match_list = stream.next().await.unwrap();
/// let match_part = fuzzy_match_list.complete_parts().next().unwrap();
/// assert!(match_part.has_marker(Marker::AnyWord));
/// assert_eq!(" badger", fuzzy_match_list.overflow().unwrap());
///
/// // fin
/// assert_eq!(None, stream.next().await);
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
/// let fuzzy_match_list = token
///     .match_input("BADGER MUSHROOM", &app_meta)
///     .next()
///     .await
///     .unwrap();
///
/// let mut match_part_iter = fuzzy_match_list.complete_parts();
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
