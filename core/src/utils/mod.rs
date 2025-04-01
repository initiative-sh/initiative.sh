pub use case_insensitive_str::CaseInsensitiveStr;
pub use quoted_word_iter::quoted_words;

mod case_insensitive_str;
mod quoted_word_iter;

use std::iter::Iterator;
use std::ops::Range;

pub fn capitalize(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut char_iter = input.chars();

    if let Some(c) = char_iter.next() {
        c.to_uppercase().for_each(|c| result.push(c));
    }
    char_iter.for_each(|c| result.push(c));

    result
}

pub fn pluralize(word: &str) -> (&str, &str) {
    match word {
        "Goose" => ("Geese", ""),
        "Beef" | "Carp" | "Cod" | "Deer" | "Perch" | "Potatoes" | "Sheep" | "Squid" => (word, ""),
        s if s.ends_with('f') => (&word[..(word.len() - 1)], "ves"),
        s if s.ends_with("ey") => (&word[..(word.len() - 2)], "ies"),
        s if s.ends_with('y') => (&word[..(word.len() - 1)], "ies"),
        s if s.ends_with(&['s', 'x', 'z'][..]) => (word, "es"),
        s if s.ends_with("ch") => (word, "es"),
        s if s.ends_with("sh") => (word, "es"),
        s if s.ends_with(&['s', 'x'][..]) => (word, "es"),
        _ => (word, "s"),
    }
}

#[derive(Clone, Debug, Eq)]
pub struct Word<'a> {
    phrase: &'a str,
    inner_range: Range<usize>,
    outer_range: Range<usize>,
}

impl<'a> Word<'a> {
    fn new(phrase: &'a str, inner_range: Range<usize>, outer_range: Range<usize>) -> Self {
        assert!(inner_range.start >= outer_range.start);
        assert!(inner_range.end <= outer_range.end);
        assert!(outer_range.end <= phrase.len());

        Self {
            phrase,
            inner_range,
            outer_range,
        }
    }

    pub fn as_str(&self) -> &'a str {
        &self.phrase[self.inner_range.clone()]
    }

    pub fn range(&self) -> Range<usize> {
        self.outer_range.clone()
    }

    /// Does this word appear at the end of the phrase (including any ignored characters)?
    pub fn is_at_end(&self) -> bool {
        self.outer_range.end == self.phrase.len()
    }

    /// Is the word quoted? (Are there characters consumed but ignored by this word?)
    pub fn is_quoted(&self) -> bool {
        self.inner_range != self.outer_range
    }

    /// Can the word be autocompleted? (Is it in such a position that adding characters to the end
    /// of the overall phrase will extend the current word?)
    pub fn can_complete(&self) -> bool {
        self.is_at_end() && !self.is_quoted()
    }

    pub fn after(&self) -> Word<'a> {
        Word {
            phrase: self.phrase,
            inner_range: self.outer_range.end..self.phrase.len(),
            outer_range: self.outer_range.end..self.phrase.len(),
        }
    }
}

impl PartialEq for Word<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl AsRef<str> for Word<'_> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> From<&'a str> for Word<'a> {
    fn from(input: &'a str) -> Word<'a> {
        Word {
            phrase: input,
            inner_range: 0..input.len(),
            outer_range: 0..input.len(),
        }
    }
}

impl<'a> From<&'a String> for Word<'a> {
    fn from(input: &'a String) -> Word<'a> {
        Word {
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
            Word {
                phrase: "abc",
                inner_range: 1..2,
                outer_range: 0..3,
            },
            Word::new("abc", 1..2, 0..3),
        );

        Word::new("", 0..0, 0..0);
        Word::new("a", 0..1, 0..1);
    }

    #[test]
    #[should_panic]
    fn new_test_inner_starts_before_outer() {
        Word::new("abc", 0..2, 1..3);
    }

    #[test]
    #[should_panic]
    fn new_test_inner_ends_after_outer() {
        Word::new("abc", 1..3, 0..2);
    }

    #[test]
    #[should_panic]
    fn new_test_range_too_long() {
        Word::new("abc", 0..2, 0..4);
    }

    #[test]
    fn as_str_test() {
        let substr = {
            let word = Word::new("abc", 1..2, 0..3);
            word.as_str()
        };

        assert_eq!("b", substr);
    }

    #[test]
    fn range_test() {
        assert_eq!(0..3, Word::new("abc", 1..2, 0..3).range());
    }

    #[test]
    fn is_at_end_test() {
        assert!(Word::new("abc", 1..2, 1..3).is_at_end());
        assert!(!Word::new("abc", 1..2, 1..2).is_at_end());
    }

    #[test]
    fn is_quoted_test() {
        assert!(Word::new("abc", 1..3, 0..3).is_quoted());
        assert!(!Word::new("abc", 1..3, 1..3).is_quoted());
    }

    #[test]
    fn can_complete_test() {
        assert!(Word::new("abc", 1..3, 1..3).can_complete());
        assert!(!Word::new("abc", 1..2, 1..2).can_complete());
        assert!(!Word::new("abc", 1..2, 1..3).can_complete());
    }

    #[test]
    fn after_test() {
        assert_eq!("c", Word::new("abc", 0..1, 0..2).after().as_str());
    }
}
