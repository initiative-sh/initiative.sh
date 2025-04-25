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

#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(test, derive(Clone))]
pub enum Token {
    /// See [`token_constructors::any_of`].
    AnyOf { tokens: Vec<Token> },

    /// See [`token_constructors::any_phrase`].
    AnyPhrase { marker_hash: u64 },

    /// See [`token_constructors::any_word`].
    AnyWord { marker_hash: u64 },

    /// See [`token_constructors::keyword`].
    Keyword {
        term: &'static str,
        marker_hash: u64,
    },

    /// See [`token_constructors::keyword_list`].
    KeywordList {
        terms: Vec<&'static str>,
        marker_hash: u64,
    },

    /// See [`token_constructors::name`].
    Name { marker_hash: u64 },

    /// See [`token_constructors::optional`].
    Optional { token: Box<Token> },

    /// See [`token_constructors::or`].
    Or { tokens: Vec<Token> },

    /// See [`token_constructors::sequence`].
    Sequence { tokens: Vec<Token> },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchPart<'input> {
    pub input: Substr<'input>,
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

        match &self {
            Token::AnyOf { .. } => any_of::match_input(self, input, app_meta),
            Token::AnyPhrase { .. } => any_phrase::match_input(self, input),
            Token::AnyWord { .. } => any_word::match_input(self, input),
            Token::Keyword { .. } => keyword::match_input(self, input),
            Token::KeywordList { .. } => keyword_list::match_input(self, input),
            Token::Name { .. } => name::match_input(self, input, app_meta),
            Token::Optional { .. } => optional::match_input(self, input, app_meta),
            Token::Or { .. } => or::match_input(self, input, app_meta),
            Token::Sequence { .. } => sequence::match_input(self, input, app_meta),
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

impl<'input> MatchPart<'input> {
    pub fn new(input: Substr<'input>, marker_hash: u64) -> Self {
        MatchPart {
            input,
            term: None,
            record: None,
            marker_hash,
        }
    }

    pub fn new_unmarked(input: Substr<'input>) -> Self {
        MatchPart {
            input,
            term: None,
            record: None,
            marker_hash: 0,
        }
    }

    pub fn with_marker<M>(mut self, marker: M) -> Self
    where
        M: Hash,
    {
        self.marker_hash = hash_marker(marker);
        self
    }

    pub fn with_term(mut self, term: &'static str) -> Self {
        self.term = Some(term);
        self
    }

    pub fn with_record(mut self, record: Record) -> Self {
        self.record = Some(record);
        self
    }

    pub fn has_marker<M>(&self, marker: M) -> bool
    where
        M: Hash,
    {
        self.has_marker_hash(hash_marker(marker))
    }

    pub fn has_marker_hash(&self, marker_hash: u64) -> bool {
        self.marker_hash == marker_hash
    }

    pub fn record(&self) -> Option<&Record> {
        self.record.as_ref()
    }

    pub fn term(&self) -> Option<&'static str> {
        self.term
    }
}

impl<'input> MatchList<'input> {
    pub fn iter(&self) -> impl std::iter::Iterator<Item = &MatchPart<'input>> {
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
        self.iter()
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

    pub fn into_match_list(self) -> Option<MatchList<'input>> {
        if self.extra.is_none() {
            Some(self.match_list)
        } else {
            None
        }
    }

    pub fn prepend(mut self, mut match_list: MatchList<'input>) -> Self {
        match_list.matches.append(&mut self.match_list.matches);
        self.match_list = match_list;
        self
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
    ///     "hail Odysseus",
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
    ///     "BADGER mushroom",
    ///     fuzzy_token_match.autocomplete_term().unwrap(),
    /// );
    /// # })
    /// ```
    pub fn autocomplete_term(&self) -> Option<String> {
        let mut suggestion = String::with_capacity(self.input()?.original_len());

        if let Some(FuzzyMatchPart::Incomplete(match_part)) = &self.extra {
            suggestion.push_str(match_part.input.before().as_str());

            if let Some(record) = &match_part.record {
                let name = record.thing.name().to_string();
                if let Some(suffix) = name.strip_prefix_ci(&match_part.input) {
                    suggestion.push_str(match_part.input.as_str());
                    suggestion.push_str(suffix);
                } else {
                    suggestion.push_str(&name);
                }
            } else if let Some(term) = &match_part.term {
                suggestion.push_str(term);
            } else {
                return None;
            }
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

/*
impl<'a> TokenMatch<'a> {
    /// Creates a new `TokenMatch` object containing a reference to the [`Token`] that was matched.
    ///
    /// `match_meta` is typically not passed directly, but rather via the `Into<T>` trait. In the
    /// case where `match_meta` is `MatchMeta::None`, `TokenMatch::from(&token)` is preferred.
    ///
    /// # Examples
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use initiative_core::command::prelude::*;
    /// # let app_meta = initiative_core::test_utils::app_meta::with_test_data().await;
    /// let token = any_word();
    ///
    /// // Use ::from to create a TokenMatch with no metadata.
    /// let token_match = TokenMatch::from(&token);
    /// assert_eq!(MatchMeta::None, token_match.match_meta);
    ///
    /// // Provide a &str for MatchMeta::Phrase.
    /// let token_match = TokenMatch::new(&token, "word");
    /// assert_eq!(MatchMeta::Phrase("word"), token_match.match_meta);
    ///
    /// // Provide a Record for MatchMeta::Record.
    /// let record = app_meta.repository.get_by_name("Odysseus").await.unwrap();
    /// let token_match = TokenMatch::new(&token, record);
    /// assert!(matches!(token_match.match_meta, MatchMeta::Record(_)));
    ///
    /// // Provide a Vec<TokenMatch> for MatchMeta::Sequence.
    /// let sequence_token = sequence([any_word()]);
    /// let token_match = TokenMatch::new(&sequence_token, vec![TokenMatch::from(&token)]);
    /// assert!(matches!(token_match.match_meta, MatchMeta::Sequence(_)));
    ///
    /// // Provide a TokenMatch for MatchMeta::Single.
    /// let optional_token = optional(any_word());
    /// let token_match = TokenMatch::new(&optional_token, TokenMatch::from(&token));
    /// assert!(matches!(token_match.match_meta, MatchMeta::Single(_)));
    /// # })
    /// ```
    pub fn new(token: &'a Token, match_meta: impl Into<MatchMeta<'a>>) -> Self {
        TokenMatch {
            token,
            match_meta: match_meta.into(),
        }
    }

    /// Returns `true` if the `TokenMatch` or any of its descendents contain the given marker.
    ///
    /// Returns `false` if the marker is not present.
    #[cfg_attr(not(any(test, feature = "integration-tests")), expect(dead_code))]
    pub fn contains_marker<M>(&'a self, marker: M) -> bool
    where
        M: Hash,
    {
        self.find_marker(marker).is_some()
    }

    /// Returns the first `TokenMatch` with a given marker in the token tree.
    ///
    /// Returns `None` if the patterns doesn't match.
    pub fn find_marker<M>(&'a self, marker: M) -> Option<&'a TokenMatch<'a>>
    where
        M: Hash,
    {
        TokenMatchIterator::new(self).find(move |token_match| token_match.is_marked_with(&marker))
    }

    /// Iterate through all TokenMatch objects in the tree with a given set of markers.
    #[cfg_attr(not(any(test, feature = "integration-tests")), expect(dead_code))]
    pub fn find_markers<'b, M>(
        &'a self,
        markers: &'b [M],
    ) -> impl Iterator<Item = &'a TokenMatch<'a>> + 'b
    where
        M: Hash,
        'a: 'b,
    {
        TokenMatchIterator::new(self)
            .filter(move |token_match| markers.iter().any(|m| token_match.is_marked_with(m)))
    }

    /// Returns `true` if the root-level token has the given `marker`.
    ///
    /// Returns `false` if it does not.
    pub fn is_marked_with<M>(&self, marker: M) -> bool
    where
        M: Hash,
    {
        let self_marker = self.token.marker_hash();
        self_marker != 0 && self_marker == hash_marker(marker)
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
*/

/*
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
*/

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

    /*
    #[test]
    fn token_match_find_marker_contains_marker_test() {
        let keyword_token = keyword_m(Marker::One, "badger");
        let sequence_token = sequence([keyword_token.clone()]);

        let token_match = TokenMatch::new(&sequence_token, vec![TokenMatch::from(&keyword_token)]);

        assert_eq!(
            Some(&TokenMatch::from(&keyword_token)),
            token_match.find_marker(Marker::One),
        );
        assert_eq!(None, token_match.find_marker(Marker::Two));

        assert!(token_match.contains_marker(Marker::One));
        assert!(!token_match.contains_marker(Marker::Two));
    }
    */

    /*
    #[test]
    fn token_match_find_markers_test() {
        let tokens = [
            keyword_m(Marker::One, "badger"),
            keyword_m(Marker::Two, "mushroom"),
            keyword_m(Marker::One, "snake"),
        ];
        let sequence_token = sequence(tokens.clone());
        let token_match = TokenMatch::new(
            &sequence_token,
            tokens.iter().map(TokenMatch::from).collect::<Vec<_>>(),
        );

        assert_eq!(
            vec![
                &TokenMatch::from(&tokens[0]),
                &TokenMatch::from(&tokens[1]),
                &TokenMatch::from(&tokens[2]),
            ],
            token_match
                .find_markers(&[Marker::One, Marker::Two])
                .collect::<Vec<_>>(),
        );

        assert_eq!(
            vec![&TokenMatch::from(&tokens[0]), &TokenMatch::from(&tokens[2])],
            token_match.find_markers(&[Marker::One]).collect::<Vec<_>>(),
        );
    }
    */

    /*
    #[test]
    fn token_match_is_marked_with_test() {
        let marked_token = keyword_m(Marker::One, "badger");
        let unmarked_token = keyword("mushroom");

        assert!(TokenMatch::from(&marked_token).is_marked_with(&Marker::One));
        assert!(!TokenMatch::from(&marked_token).is_marked_with(&Marker::Two));
        assert!(!TokenMatch::from(&unmarked_token).is_marked_with(&Marker::One));
    }
    */

    /*
    #[tokio::test]
    async fn token_match_new_test() {
        let token = keyword("I am a token");
        let record = test::app_meta::with_test_data()
            .await
            .repository
            .get_by_uuid(&test::npc::odysseus::UUID)
            .await
            .unwrap();

        let token_match = TokenMatch::from(&token);
        assert_eq!(MatchMeta::None, token_match.match_meta);

        let token_match = TokenMatch::new(&token, "word");
        assert_eq!(MatchMeta::Phrase("word"), token_match.match_meta);

        let token_match = TokenMatch::new(&token, record);
        assert!(matches!(token_match.match_meta, MatchMeta::Record(_)));

        let token_match = TokenMatch::new(&token, vec![TokenMatch::from(&token)]);
        assert!(matches!(token_match.match_meta, MatchMeta::Sequence(_)));

        let token_match = TokenMatch::new(&token, TokenMatch::from(&token));
        assert!(matches!(token_match.match_meta, MatchMeta::Single(_)));
    }
    */
}
