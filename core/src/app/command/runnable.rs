use crate::app::AppMeta;
use async_trait::async_trait;
use serde::Serialize;
use std::borrow::Cow;

#[async_trait(?Send)]
pub trait Runnable: Sized {
    async fn run(self, input: &str, app_meta: &mut AppMeta) -> Result<String, String>;
}

#[async_trait(?Send)]
pub trait ContextAwareParse: Sized {
    async fn parse_input(input: &str, app_meta: &AppMeta) -> CommandMatches<Self>;
}

#[async_trait(?Send)]
pub trait Autocomplete {
    async fn autocomplete(input: &str, app_meta: &AppMeta) -> Vec<AutocompleteSuggestion>;
}

#[cfg(test)]
pub fn assert_autocomplete(
    expected_suggestions: &[(&'static str, &'static str)],
    actual_suggestions: Vec<AutocompleteSuggestion>,
) {
    let mut expected: Vec<_> = expected_suggestions
        .into_iter()
        .map(|(a, b)| ((*a).into(), (*b).into()))
        .collect();
    expected.sort();

    let mut actual: Vec<(Cow<'static, str>, Cow<'static, str>)> = actual_suggestions
        .into_iter()
        .map(|suggestion| suggestion.into())
        .collect();
    actual.sort();

    assert_eq!(expected, actual);
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
#[serde(into = "(Cow<'static, str>, Cow<'static, str>)")]
pub struct AutocompleteSuggestion {
    pub term: Cow<'static, str>,
    pub summary: Cow<'static, str>,
}

impl AutocompleteSuggestion {
    pub fn new(term: impl Into<Cow<'static, str>>, summary: impl Into<Cow<'static, str>>) -> Self {
        Self {
            term: term.into(),
            summary: summary.into(),
        }
    }
}

impl From<AutocompleteSuggestion> for (Cow<'static, str>, Cow<'static, str>) {
    fn from(input: AutocompleteSuggestion) -> Self {
        (input.term, input.summary)
    }
}

impl From<(String, &'static str)> for AutocompleteSuggestion {
    fn from(input: (String, &'static str)) -> Self {
        AutocompleteSuggestion::new(input.0, input.1)
    }
}

impl From<(&'static str, String)> for AutocompleteSuggestion {
    fn from(input: (&'static str, String)) -> Self {
        AutocompleteSuggestion::new(input.0, input.1)
    }
}

impl From<(String, String)> for AutocompleteSuggestion {
    fn from(input: (String, String)) -> Self {
        AutocompleteSuggestion::new(input.0, input.1)
    }
}

impl From<(&'static str, &'static str)> for AutocompleteSuggestion {
    fn from(input: (&'static str, &'static str)) -> Self {
        AutocompleteSuggestion::new(input.0, input.1)
    }
}

/// Represents all possible parse results for a given input.
///
/// One of the key usability features (and major headaches) of initiative.sh is its use of fuzzy
/// command matching. Most parsers assume only one possible valid interpretation, but initiative.sh
/// has the concept of *canonical* and *fuzzy* matches.
///
/// **Canonical matches** are inputs that cannot possibly conflict with one another. This is
/// achieved using differing prefixes, eg. all SRD reference lookups are prefixed as "srd item
/// shield" (or "srd spell shield").
///
/// **Fuzzy matches** are commands that the user could possibly have meant with a given input. The
/// reason we need to be particularly careful with fuzzy matches is because the user can add
/// arbitrary names to their journal, and typing that arbitrary name is *usually* enough to pull it
/// up again. However, that arbitrary name could easily be another command, which the user may not
/// want to overwrite. (The notable built-in conflict is the example above, where "shield" is both
/// an item and a spell.)
///
/// || Canonical matches || Fuzzy matches || Result ||
/// | 0 | 0 | Error: "Unknown command: '...'" |
/// | 0 | 1 | The fuzzy match is run. |
/// | 0 | 2+ | Error: "There are several possible interpretations of this command. Did you mean:" |
/// | 1 | 0 | The canonical match is run. |
/// | 1 | 1+ | The canonical match is run, suffixed with the error: "There are other possible interpretations of this command. Did you mean:" |
///
/// Since the parsing logic in the code base runs several layers deep across a number of different
/// structs, this struct provides utilities for combining multiple CommandMatches instances of
/// differing types, and for transforming the inner type as needed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandMatches<T> {
    pub canonical_match: Option<T>,
    pub fuzzy_matches: Vec<T>,
}

impl<T: std::fmt::Debug> CommandMatches<T> {
    /// Create a new instance of CommandMatches with a canonical match.
    pub fn new_canonical(canonical_match: T) -> Self {
        CommandMatches {
            canonical_match: Some(canonical_match),
            fuzzy_matches: Vec::new(),
        }
    }

    /// Create a new instance of `CommandMatches` with a single fuzzy match.
    pub fn new_fuzzy(fuzzy_match: T) -> Self {
        CommandMatches {
            canonical_match: None,
            fuzzy_matches: vec![fuzzy_match],
        }
    }

    /// Push a new canonical match. Panics if a canonical match is already present.
    pub fn push_canonical(&mut self, canonical_match: T) {
        if let Some(old_canonical_match) = &self.canonical_match {
            panic!(
                "trying to overwrite existing canonical match {:?} with new match {:?}",
                old_canonical_match, canonical_match,
            );
        }

        self.canonical_match = Some(canonical_match);
    }

    /// Add a new fuzzy match to the list of possibilities.
    pub fn push_fuzzy(&mut self, fuzzy_match: T) {
        self.fuzzy_matches.push(fuzzy_match);
    }

    /// Combine the current CommandMatches with another object that can be massaged into the same
    /// type. The Vecs of fuzzy matches are combined. Panics if both objects lay claim to a
    /// canonical match.
    pub fn union<O>(mut self, other: CommandMatches<O>) -> Self
    where
        O: std::fmt::Debug,
        T: From<O>,
    {
        let CommandMatches {
            canonical_match,
            fuzzy_matches,
        } = other.into_subtype::<T>();

        if let Some(canonical_match) = canonical_match {
            self.push_canonical(canonical_match);
        }

        self.fuzzy_matches.reserve(fuzzy_matches.len());
        fuzzy_matches
            .into_iter()
            .for_each(|fuzzy_match| self.push_fuzzy(fuzzy_match));

        self
    }

    /// Variant of union() that resolves canonical conflicts by overwriting self.canonical_match
    /// with other.canonical_match instead of panicking.
    pub fn union_with_overwrite<O>(mut self, other: CommandMatches<O>) -> Self
    where
        O: std::fmt::Debug,
        T: From<O>,
    {
        let self_canonical_match = self.canonical_match.take();

        let mut result = self.union(other);

        if result.canonical_match.is_none() {
            result.canonical_match = self_canonical_match;
        }

        result
    }

    /// Convert a `CommandMatches<T>' into a `CommandMatches<O>`, massaging the inner type using
    /// its `From<T>` trait.
    ///
    /// This could be an impl of `From<CommandMatches<T>> for CommandMatches<O>`, but that produces
    /// a trait conflict due to limitations of Rust's type system.
    pub fn into_subtype<O>(self) -> CommandMatches<O>
    where
        O: From<T> + std::fmt::Debug,
    {
        CommandMatches {
            canonical_match: self
                .canonical_match
                .map(|canonical_match| canonical_match.into()),
            fuzzy_matches: self
                .fuzzy_matches
                .into_iter()
                .map(|fuzzy_match| fuzzy_match.into())
                .collect(),
        }
    }

    /// Consumes the struct, returning the following in priority order:
    ///
    /// 1. The canonical match, if present.
    /// 2. The first fuzzy match, if any are present.
    /// 3. `None`
    pub fn take_best_match(self) -> Option<T> {
        self.canonical_match
            .or_else(|| self.fuzzy_matches.into_iter().next())
    }
}

impl<T> From<T> for CommandMatches<T> {
    fn from(input: T) -> Self {
        CommandMatches {
            canonical_match: Some(input),
            fuzzy_matches: Vec::default(),
        }
    }
}

impl<T> Default for CommandMatches<T> {
    fn default() -> Self {
        CommandMatches {
            canonical_match: None,
            fuzzy_matches: Vec::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn command_matches_new_test() {
        {
            let command_matches = CommandMatches::new_canonical(true);
            assert_eq!(Some(true), command_matches.canonical_match);
            assert!(command_matches.fuzzy_matches.is_empty());
        }

        {
            let command_matches = CommandMatches::new_fuzzy(true);
            assert_eq!(Option::<bool>::None, command_matches.canonical_match);
            assert_eq!([true][..], command_matches.fuzzy_matches[..]);
        }

        {
            let command_matches = CommandMatches::default();
            assert_eq!(Option::<bool>::None, command_matches.canonical_match);
            assert!(command_matches.fuzzy_matches.is_empty());
        }
    }

    #[test]
    fn command_matches_push_test() {
        {
            let mut command_matches = CommandMatches::default();
            command_matches.push_canonical(true);
            assert_eq!(Some(true), command_matches.canonical_match);
            assert!(command_matches.fuzzy_matches.is_empty());
        }

        {
            let mut command_matches = CommandMatches::default();
            command_matches.push_fuzzy(1u8);
            command_matches.push_fuzzy(2);
            assert_eq!(Option::<u8>::None, command_matches.canonical_match);
            assert_eq!([1u8, 2][..], command_matches.fuzzy_matches[..]);
        }
    }

    #[test]
    #[should_panic(
        expected = "trying to overwrite existing canonical match true with new match false"
    )]
    fn command_matches_push_test_with_conflict() {
        let mut command_matches = CommandMatches::default();
        command_matches.push_canonical(true);
        command_matches.push_canonical(false);
    }

    #[test]
    fn command_matches_union_test() {
        let command_matches_1 = {
            let mut command_matches = CommandMatches::new_canonical(1u16);
            command_matches.push_fuzzy(2);
            command_matches.push_fuzzy(3);
            command_matches
        };
        let command_matches_2 = CommandMatches::new_fuzzy(4u8);

        let command_matches_result = command_matches_1.union(command_matches_2);

        assert_eq!(Some(1u16), command_matches_result.canonical_match);
        assert_eq!([2u16, 3, 4][..], command_matches_result.fuzzy_matches[..]);
    }

    #[test]
    #[should_panic(
        expected = "trying to overwrite existing canonical match true with new match false"
    )]
    fn command_matches_union_test_with_conflict() {
        CommandMatches::new_canonical(true).union(CommandMatches::new_canonical(false));
    }

    #[test]
    fn command_matches_union_with_overwrite_test() {
        assert_eq!(
            Some(2u16),
            CommandMatches::new_canonical(1u16)
                .union_with_overwrite(CommandMatches::new_canonical(2u8))
                .canonical_match,
        );
    }

    #[test]
    fn command_matches_take_best_match_test() {
        assert_eq!(
            Some(true),
            CommandMatches::new_canonical(true).take_best_match(),
        );
        assert_eq!(
            Some(true),
            CommandMatches::new_fuzzy(true).take_best_match(),
        );

        {
            let mut command_matches = CommandMatches::new_canonical(true);
            command_matches.push_fuzzy(false);
            assert_eq!(Some(true), command_matches.take_best_match());
        }

        assert_eq!(
            Option::<bool>::None,
            CommandMatches::<bool>::default().take_best_match(),
        );
    }
}
