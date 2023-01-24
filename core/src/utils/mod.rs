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

    pub fn as_own_str<'b>(&'a self, phrase: &'b str) -> &'b str {
        &phrase[self.inner_range.clone()]
    }

    pub fn range(&'a self) -> &'a Range<usize> {
        &self.outer_range
    }
}
