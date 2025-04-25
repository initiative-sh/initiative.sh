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
mod token_match_iterator;

use std::hash::{DefaultHasher, Hash, Hasher};

use token_match_iterator::TokenMatchIterator;

use crate::app::AppMeta;
use crate::storage::Record;
use crate::utils::Substr;
use initiative_macros::From;

use futures::prelude::*;

#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(test, derive(Clone))]
pub enum Token {
    /// See [`token_constructors::any_of`].
    AnyOf { tokens: Vec<Token> },

    /// See [`token_constructors::any_phrase`].
    AnyPhrase { marker: u64 },

    /// See [`token_constructors::any_word`].
    AnyWord { marker: u64 },

    /// See [`token_constructors::keyword`].
    Keyword { term: &'static str, marker: u64 },

    /// See [`token_constructors::keyword_list`].
    KeywordList {
        terms: Vec<&'static str>,
        marker: u64,
    },

    /// See [`token_constructors::name`].
    Name { marker: u64 },

    /// See [`token_constructors::optional`].
    Optional { token: Box<Token> },

    /// See [`token_constructors::or`].
    Or { tokens: Vec<Token> },

    /// See [`token_constructors::sequence`].
    Sequence { tokens: Vec<Token> },
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

#[derive(Clone, Debug, Eq, From, PartialEq)]
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
    ) -> impl Stream<Item = TokenMatch<'a>> + 'b
    where
        'a: 'b,
    {
        self.match_input(input, app_meta)
            .filter_map(|fuzzy_match| future::ready(fuzzy_match.into_exact()))
    }

    pub fn marker_hash(&self) -> u64 {
        match &self {
            Token::AnyPhrase { marker }
            | Token::AnyWord { marker }
            | Token::Keyword { marker, .. }
            | Token::KeywordList { marker, .. }
            | Token::Name { marker } => *marker,
            Token::AnyOf { .. }
            | Token::Optional { .. }
            | Token::Or { .. }
            | Token::Sequence { .. } => 0,
        }
    }
}

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

    #[cfg_attr(not(feature = "integration-tests"), expect(dead_code))]
    pub fn into_record(self) -> Option<Record> {
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

    #[test]
    fn token_match_is_marked_with_test() {
        let marked_token = keyword_m(Marker::One, "badger");
        let unmarked_token = keyword("mushroom");

        assert!(TokenMatch::from(&marked_token).is_marked_with(&Marker::One));
        assert!(!TokenMatch::from(&marked_token).is_marked_with(&Marker::Two));
        assert!(!TokenMatch::from(&unmarked_token).is_marked_with(&Marker::One));
    }

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
}
