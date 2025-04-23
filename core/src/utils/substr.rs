use std::ops::{Deref, Range};

/// Represents a portion of a string slice. This behaviour mimics the core-level string slice
/// model, with two exceptions:
///
/// * A reference to the original slice is maintained, permitting logic like Substr::after() to
///   reference data outside of (after) the subslice.
/// * The concept of "inner" and "outer" slices exists, which is used to handle the contents of
///   quoted strings.
#[derive(Clone, Debug)]
pub struct Substr<'a> {
    phrase: &'a str,
    inner_range: Range<usize>,
    outer_range: Range<usize>,
}

impl<'a> Substr<'a> {
    pub fn new(phrase: &'a str, inner_range: Range<usize>, outer_range: Range<usize>) -> Self {
        assert!(inner_range.start >= outer_range.start);
        assert!(inner_range.end <= outer_range.end);
        assert!(outer_range.end <= phrase.len());

        Self {
            phrase,
            inner_range,
            outer_range,
        }
    }

    /// Returns a representation of the Substr as a normal string slice.
    pub fn as_str(&self) -> &'a str {
        &self.phrase[self.inner_range.clone()]
    }

    /// Returns the outer portion of the Substr, including quotes if present.
    pub fn as_outer_str(&self) -> &'a str {
        &self.phrase[self.outer_range.clone()]
    }

    /// Returns the entire input phrase.
    pub fn as_original_str(&self) -> &'a str {
        self.phrase
    }

    /// Returns the outer range of the Substr, ie. including quotes (if any).
    pub fn range(&self) -> Range<usize> {
        self.outer_range.clone()
    }

    /// Does this Substr appear at the end of the phrase (including any ignored characters)?
    pub fn is_at_end(&self) -> bool {
        self.outer_range.end == self.phrase.len()
    }

    /// Are there characters consumed but ignored by this Substr (ie. quotation marks)?
    pub fn is_quoted(&self) -> bool {
        self.inner_range != self.outer_range
    }

    /// Can the word be autocompleted? (Is it in such a position that adding characters to the end
    /// of the overall phrase will extend the current word?)
    pub fn can_complete(&self) -> bool {
        self.is_at_end() && !self.is_quoted()
    }

    /// Get the remainder of the phrase starting from the end of the Substr.
    pub fn after(&self) -> Substr<'a> {
        Substr {
            phrase: self.phrase,
            inner_range: self.outer_range.end..self.phrase.len(),
            outer_range: self.outer_range.end..self.phrase.len(),
        }
    }
}

impl Eq for Substr<'_> {}

impl PartialEq for Substr<'_> {
    fn eq(&self, other: &Self) -> bool {
        self == other.as_str()
    }
}

impl PartialEq<str> for Substr<'_> {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl AsRef<str> for Substr<'_> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Deref for Substr<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<'a> From<&'a str> for Substr<'a> {
    fn from(input: &'a str) -> Substr<'a> {
        Substr {
            phrase: input,
            inner_range: 0..input.len(),
            outer_range: 0..input.len(),
        }
    }
}

impl<'a> From<&'a String> for Substr<'a> {
    fn from(input: &'a String) -> Substr<'a> {
        Substr {
            phrase: input,
            inner_range: 0..input.len(),
            outer_range: 0..input.len(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_test() {
        assert_eq!(
            Substr {
                phrase: "abc",
                inner_range: 1..2,
                outer_range: 0..3,
            },
            Substr::new("abc", 1..2, 0..3),
        );

        Substr::new("", 0..0, 0..0);
        Substr::new("a", 0..1, 0..1);
    }

    #[test]
    #[should_panic]
    fn new_test_inner_starts_before_outer() {
        Substr::new("abc", 0..2, 1..3);
    }

    #[test]
    #[should_panic]
    fn new_test_inner_ends_after_outer() {
        Substr::new("abc", 1..3, 0..2);
    }

    #[test]
    #[should_panic]
    fn new_test_range_too_long() {
        Substr::new("abc", 0..2, 0..4);
    }

    #[test]
    fn as_str_test() {
        let (as_str, as_outer_str, as_original_str) = {
            let substr = Substr::new("abcde", 2..3, 1..4);
            (
                substr.as_str(),
                substr.as_outer_str(),
                substr.as_original_str(),
            )
        };

        assert_eq!("c", as_str);
        assert_eq!("bcd", as_outer_str);
        assert_eq!("abcde", as_original_str);
    }

    #[test]
    fn range_test() {
        assert_eq!(0..3, Substr::new("abc", 1..2, 0..3).range());
    }

    #[test]
    fn is_at_end_test() {
        assert!(Substr::new("abc", 1..2, 1..3).is_at_end());
        assert!(!Substr::new("abc", 1..2, 1..2).is_at_end());
    }

    #[test]
    fn is_quoted_test() {
        assert!(Substr::new("abc", 1..3, 0..3).is_quoted());
        assert!(!Substr::new("abc", 1..3, 1..3).is_quoted());
    }

    #[test]
    fn can_complete_test() {
        assert!(Substr::new("abc", 1..3, 1..3).can_complete());
        assert!(!Substr::new("abc", 1..2, 1..2).can_complete());
        assert!(!Substr::new("abc", 1..2, 1..3).can_complete());
    }

    #[test]
    fn after_test() {
        assert_eq!("c", Substr::new("abc", 0..1, 0..2).after().as_str());
    }
}
