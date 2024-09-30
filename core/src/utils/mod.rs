pub use case_insensitive_str::CaseInsensitiveStr;
pub use quoted_word_iter::{quoted_phrases, quoted_words};

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
        Self {
            phrase,
            inner_range,
            outer_range,
        }
    }

    pub fn as_str(&'a self) -> &'a str {
        &self.phrase[self.inner_range.clone()]
    }

    pub fn as_original_str(&'a self) -> &'a str {
        &self.phrase[self.outer_range.clone()]
    }

    pub fn as_own_str<'b>(&'a self, phrase: &'b str) -> &'b str {
        &phrase[self.inner_range.clone()]
    }

    pub fn as_original_own_str<'b>(&'a self, phrase: &'b str) -> &'b str {
        &phrase[self.outer_range.clone()]
    }

    pub fn range(&'a self) -> &'a Range<usize> {
        &self.outer_range
    }

    pub fn is_quoted(&self) -> bool {
        self.inner_range != self.outer_range
    }

    pub fn is_at_end(&self) -> bool {
        self.outer_range.end == self.phrase.len()
    }

    pub fn completes_to_ci<'b, S>(&self, other: S) -> bool
    where
        S: AsRef<str>,
    {
        let other: &str = other.as_ref();

        other.starts_with_ci(self.as_str()) && self.is_at_end() && !self.is_quoted()
    }

    pub fn combine_with(&self, other: Word<'a>) -> Option<Word<'a>> {
        if self.phrase == other.phrase {
            let range = (self.outer_range.start.min(other.outer_range.start))
                ..(self.outer_range.end.max(other.outer_range.end));

            Some(Word {
                phrase: self.phrase,
                inner_range: range.clone(),
                outer_range: range,
            })
        } else {
            None
        }
    }
}

impl<'a> PartialEq for Word<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<'a> AsRef<str> for Word<'a> {
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
