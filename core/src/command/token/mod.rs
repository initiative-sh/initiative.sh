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
use crate::utils::{CaseInsensitiveStr as _, Substr};

use futures::prelude::*;

mod prelude {
    pub use super::MatchPartBuilder as _;
    pub use super::{FuzzyMatchList, FuzzyMatchPart, MatchList, MatchPart, Token, TokenKind};
    pub use crate::app::AppMeta;
    pub use crate::utils::{
        quoted_phrases, quoted_phrases_all, quoted_words, CaseInsensitiveStr, Substr,
    };

    pub use std::pin::Pin;

    pub use async_stream::stream;
    pub use futures::join;
    pub use futures::prelude::*;
}

#[derive(Debug, Eq, PartialEq)]
pub struct Token {
    kind: TokenKind,
    marker_hash: u64,
    placeholder: Option<&'static str>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum TokenKind {
    /// See [`token_constructors::any_of`].
    AnyOf { tokens: Vec<Token> },

    /// See [`token_constructors::any_phrase`].
    AnyPhrase,

    /// See [`token_constructors::any_word`].
    AnyWord,

    /// See [`token_constructors::keyword`].
    Keyword { term: &'static str },

    /// See [`token_constructors::keyword_list`].
    KeywordList { terms: Vec<&'static str> },

    /// See [`token_constructors::name`].
    Name,

    /// See [`token_constructors::optional`].
    Optional { token: Box<Token> },

    /// See [`token_constructors::or`].
    Or { tokens: Vec<Token> },

    /// See [`token_constructors::sequence`].
    Sequence { tokens: Vec<Token> },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchPart<'input> {
    input: Substr<'input>,
    term: Option<&'static str>,
    record: Option<Record>,
    marker_hash: u64,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MatchList<'input> {
    matches: Vec<MatchPart<'input>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FuzzyMatchList<'input> {
    pub match_list: MatchList<'input>,
    pub extra: Option<FuzzyMatchPart<'input>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FuzzyMatchPart<'input> {
    Overflow(Substr<'input>),
    Incomplete(MatchPart<'input>),
}

impl Token {
    /// Applies a marker to the Token. A marker can be anything that implements [`Hash`], but
    /// generally a dedicated enum is created for the purpose. The hash (but not the marker) is
    /// then made available through [`MatchPart`] and can be checked with [`MatchPart::has_marker`]
    /// and [`MatchPart::has_marker_hash`].
    pub fn with_marker<M>(mut self, marker: M) -> Self
    where
        M: Hash,
    {
        self.marker_hash = hash_marker(marker);
        self
    }

    /// Applies a placeholder to the token, shown when an autocomplete suggestion is possible but
    /// no text has yet been input.
    ///
    /// The placeholder must start with "[" and end with "]". Failing to satisfy this will cause a
    /// panic in dev builds.
    pub fn with_placeholder(mut self, placeholder: &'static str) -> Self {
        assert!(placeholder.starts_with('['));
        assert!(placeholder.ends_with(']'));

        self.placeholder = Some(placeholder);
        self
    }

    pub fn match_input<'input, 'token, S>(
        &'token self,
        input: S,
        app_meta: &'token AppMeta,
    ) -> impl Stream<Item = FuzzyMatchList<'input>> + 'token
    where
        'input: 'token,
        S: Into<Substr<'input>>,
    {
        let input = input.into();

        match &self.kind {
            TokenKind::AnyOf { .. } => any_of::match_input(self, input, app_meta),
            TokenKind::AnyPhrase => any_phrase::match_input(self, input),
            TokenKind::AnyWord => any_word::match_input(self, input),
            TokenKind::Keyword { .. } => keyword::match_input(self, input),
            TokenKind::KeywordList { .. } => keyword_list::match_input(self, input),
            TokenKind::Name => name::match_input(self, input, app_meta),
            TokenKind::Optional { .. } => optional::match_input(self, input, app_meta),
            TokenKind::Or { .. } => or::match_input(self, input, app_meta),
            TokenKind::Sequence { .. } => sequence::match_input(self, input, app_meta),
        }
    }

    pub fn match_input_exact<'a, 'b>(
        &'a self,
        input: &'a str,
        app_meta: &'b AppMeta,
    ) -> impl Stream<Item = MatchList<'a>> + 'b
    where
        'a: 'b,
    {
        self.match_input(input, app_meta)
            .filter_map(|fuzzy_match_list| future::ready(fuzzy_match_list.into_match_list()))
    }
}

pub trait MatchPartBuilder<'input> {
    fn new(input: Substr<'input>, marker_hash: u64) -> Self;

    fn new_unmarked(input: Substr<'input>) -> Self;

    fn with_marker<M>(self, marker: M) -> Self
    where
        M: Hash;

    fn with_term(self, term: &'static str) -> Self;

    fn with_record(self, record: Record) -> Self;
}

impl<'input> MatchPartBuilder<'input> for MatchPart<'input> {
    fn new(input: Substr<'input>, marker_hash: u64) -> Self {
        MatchPart {
            input,
            term: None,
            record: None,
            marker_hash,
        }
    }

    fn new_unmarked(input: Substr<'input>) -> Self {
        MatchPart {
            input,
            term: None,
            record: None,
            marker_hash: 0,
        }
    }

    fn with_marker<M>(mut self, marker: M) -> Self
    where
        M: Hash,
    {
        self.marker_hash = hash_marker(marker);
        self
    }

    fn with_term(mut self, term: &'static str) -> Self {
        self.term = Some(term);
        self
    }

    fn with_record(mut self, record: Record) -> Self {
        self.record = Some(record);
        self
    }
}

impl<'input> MatchPart<'input> {
    pub fn has_marker<M>(&self, marker: M) -> bool
    where
        M: Hash,
    {
        self.has_marker_hash(hash_marker(marker))
    }

    pub fn has_marker_hash(&self, marker_hash: u64) -> bool {
        self.marker_hash == marker_hash
    }

    pub fn input(&self) -> &Substr<'input> {
        &self.input
    }

    pub fn record(&self) -> Option<&Record> {
        self.record.as_ref()
    }

    pub fn term(&self) -> Option<&'static str> {
        self.term
    }
}

impl<'input> MatchList<'input> {
    pub fn parts(&self) -> impl std::iter::Iterator<Item = &MatchPart<'input>> {
        self.into_iter()
    }

    pub fn first(&self) -> Option<&MatchPart<'input>> {
        self.matches.first()
    }

    pub fn find_marker<M>(&self, marker: M) -> Option<&MatchPart<'input>>
    where
        M: Hash,
    {
        let marker_hash = hash_marker(marker);
        self.parts()
            .find(|match_part| match_part.has_marker_hash(marker_hash))
    }
}

impl<'input> FuzzyMatchList<'input> {
    pub fn new_exact<IntoMatchList>(match_list: IntoMatchList) -> Self
    where
        IntoMatchList: Into<MatchList<'input>>,
    {
        FuzzyMatchList {
            match_list: match_list.into(),
            extra: None,
        }
    }

    pub fn new_overflow<IntoMatchList>(match_list: IntoMatchList, overflow: Substr<'input>) -> Self
    where
        IntoMatchList: Into<MatchList<'input>>,
    {
        FuzzyMatchList {
            match_list: match_list.into(),
            extra: Some(FuzzyMatchPart::Overflow(overflow)),
        }
    }

    pub fn new_incomplete(match_part: MatchPart<'input>) -> Self {
        FuzzyMatchList {
            match_list: MatchList::default(),
            extra: Some(FuzzyMatchPart::Incomplete(match_part)),
        }
    }

    #[cfg_attr(not(any(test, feature = "integration-tests")), expect(dead_code))]
    pub fn new_incomplete_multi<IntoMatchList>(
        match_list: IntoMatchList,
        incomplete_part: MatchPart<'input>,
    ) -> Self
    where
        IntoMatchList: Into<MatchList<'input>>,
    {
        FuzzyMatchList {
            match_list: match_list.into(),
            extra: Some(FuzzyMatchPart::Incomplete(incomplete_part)),
        }
    }

    pub fn prepend(mut self, mut match_list: MatchList<'input>) -> Self {
        match_list.matches.append(&mut self.match_list.matches);
        self.match_list = match_list;
        self
    }

    pub fn into_match_list(self) -> Option<MatchList<'input>> {
        if self.extra.is_none() {
            Some(self.match_list)
        } else {
            None
        }
    }

    pub fn complete_parts(&self) -> impl Iterator<Item = &MatchPart<'input>> {
        self.match_list.parts()
    }

    /// Returns the appropriate autocomplete suggestion for the incomplete input, if possible.
    ///
    /// # Examples
    ///
    /// ```
    /// # use futures::StreamExt as _;
    /// # tokio_test::block_on(async {
    /// # let app_meta = initiative_core::test_utils::app_meta::with_test_data().await;
    /// use initiative_core::command::prelude::*;
    ///
    /// let query = "hail ody";
    /// let token = sequence([keyword("hail"), name()]);
    /// let fuzzy_token_match = token.match_input(query, &app_meta).next().await.unwrap();
    ///
    /// assert_eq!(
    ///     "hail odysseus",
    ///     fuzzy_token_match.autocomplete_term().unwrap(),
    /// );
    /// # })
    /// ```
    ///
    /// ```
    /// # use futures::StreamExt as _;
    /// # tokio_test::block_on(async {
    /// # let app_meta = initiative_core::test_utils::app_meta();
    /// use initiative_core::command::prelude::*;
    ///
    /// let query = "BADGER MUSH";
    /// let token = sequence([keyword("badger"), keyword("mushroom"), keyword("snake")]);
    /// let fuzzy_token_match = token.match_input(query, &app_meta).next().await.unwrap();
    ///
    /// assert_eq!(
    ///     "BADGER MUSHroom",
    ///     fuzzy_token_match.autocomplete_term().unwrap(),
    /// );
    /// # })
    /// ```
    pub fn autocomplete_term(&self) -> Option<String> {
        // We only need to autocomplete incomplete matches.
        let match_part = self.incomplete()?;

        let name = match_part
            .record
            .as_ref()
            .map(|record| record.thing.name().to_string());
        let term = name.as_deref().or(match_part.term)?;

        let mut suggestion = String::with_capacity(match_part.input.before().len() + term.len());
        suggestion.push_str(match_part.input.before().as_str());

        // can_complete() implies that the input doesn't end with whitespace, so we need to add
        // whitespace to our suggestion.
        if !suggestion.is_empty() && match_part.input.before().can_complete() {
            suggestion.push(' ');
        }

        if let Some(suffix) = term.strip_prefix_ci(&match_part.input) {
            suggestion.push_str(match_part.input.as_str());
            suggestion.push_str(suffix);
        } else {
            suggestion.push_str(term);
        }

        Some(suggestion)
    }

    /// Get the original input data.
    pub fn input(&self) -> Option<Substr<'input>> {
        Some(
            self.match_list
                .first()
                .or(
                    if let Some(FuzzyMatchPart::Incomplete(match_part)) = &self.extra {
                        Some(match_part)
                    } else {
                        None
                    },
                )?
                .input
                .as_original_substr(),
        )
    }

    pub fn is_overflow(&self) -> bool {
        matches!(self.extra, Some(FuzzyMatchPart::Overflow(_)))
    }

    pub fn is_exact(&self) -> bool {
        self.extra.is_none()
    }

    pub fn is_incomplete(&self) -> bool {
        matches!(self.extra, Some(FuzzyMatchPart::Incomplete(_)))
    }

    pub fn overflow(&self) -> Option<&Substr<'input>> {
        if let Some(FuzzyMatchPart::Overflow(remainder)) = &self.extra {
            Some(remainder)
        } else {
            None
        }
    }

    pub fn incomplete(&self) -> Option<&MatchPart<'input>> {
        if let Some(FuzzyMatchPart::Incomplete(match_part)) = &self.extra {
            Some(match_part)
        } else {
            None
        }
    }
}

impl From<TokenKind> for Token {
    fn from(kind: TokenKind) -> Self {
        Token {
            kind,
            marker_hash: 0,
            placeholder: None,
        }
    }
}

impl<'input> From<Vec<MatchPart<'input>>> for MatchList<'input> {
    fn from(matches: Vec<MatchPart<'input>>) -> Self {
        MatchList { matches }
    }
}

impl<'input> From<MatchPart<'input>> for MatchList<'input> {
    fn from(value: MatchPart<'input>) -> Self {
        MatchList {
            matches: vec![value],
        }
    }
}

impl<'parent, 'input> std::iter::IntoIterator for &'parent MatchList<'input> {
    type Item = &'parent MatchPart<'input>;
    type IntoIter = std::slice::Iter<'parent, MatchPart<'input>>;

    fn into_iter(self) -> Self::IntoIter {
        self.matches.iter()
    }
}

impl<'input> std::iter::FromIterator<MatchPart<'input>> for MatchList<'input> {
    fn from_iter<T: IntoIterator<Item = MatchPart<'input>>>(iter: T) -> Self {
        MatchList {
            matches: iter.into_iter().collect(),
        }
    }
}

fn hash_marker<M: Hash>(marker: M) -> u64 {
    let mut hasher = DefaultHasher::default();
    marker.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::prelude::*;
    use crate::test_utils as test;

    #[derive(Hash)]
    enum Marker {
        One,
        Two,
    }

    #[test]
    fn token_with_marker_test() {
        assert_ne!(
            any_word().with_marker(Marker::One),
            any_word().with_marker(Marker::Two),
        );

        assert_eq!(
            any_word().with_marker(Marker::One),
            any_word().with_marker(Marker::One),
        );
    }

    #[test]
    fn token_with_placeholder_test() {
        assert_ne!(
            any_word().with_placeholder("[one]"),
            any_word().with_placeholder("[two]"),
        );

        assert_eq!(
            any_word().with_placeholder("[one]"),
            any_word().with_placeholder("[one]"),
        );
    }

    #[test]
    #[should_panic]
    fn token_with_placeholder_test_panic() {
        any_word().with_placeholder("oops no brackets");
    }

    #[test]
    fn match_part_new_with_marker_test() {
        assert_eq!(
            MatchPart::new("badger".into(), hash_marker(Marker::One)),
            MatchPart::new_unmarked("badger".into()).with_marker(Marker::One),
        );

        assert_ne!(
            MatchPart::new_unmarked("badger".into()).with_marker(Marker::One),
            MatchPart::new_unmarked("badger".into()).with_marker(Marker::Two),
        );
    }

    #[test]
    fn match_part_with_term_test() {
        assert_eq!(
            Some("badger"),
            MatchPart::new_unmarked("BADGER".into())
                .with_term("badger")
                .term,
        );
    }

    #[test]
    fn match_part_has_marker_test() {
        let match_part = MatchPart::new("badger".into(), hash_marker(Marker::One));

        assert!(match_part.has_marker(Marker::One));
        assert!(!match_part.has_marker(Marker::Two));
        assert!(match_part.has_marker_hash(hash_marker(Marker::One)));
    }
}
