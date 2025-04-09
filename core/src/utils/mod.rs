#[cfg(any(test, feature = "integration-tests"))]
pub mod test_utils;

pub use case_insensitive_str::CaseInsensitiveStr;
pub use quoted_word_iter::quoted_words;
pub use substr::Substr;

mod case_insensitive_str;
mod quoted_word_iter;
mod substr;

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
