use super::Substr;
use std::str::CharIndices;

/// Iterate through all words in the input, treating multiple words surrounded by quotation marks
/// as a single word. Returns [`Substr`] objects, preserving the word's context within the larger
/// string.
///
/// # Examples
///
/// ```
/// # use initiative_core::utils::quoted_words;
/// let mut iter = quoted_words(r#"   Ronny  "Two Spoons" Johnson  "#)
///     .map(|substr| substr.as_str());
///
/// assert_eq!(Some("Ronny"), iter.next());
/// assert_eq!(Some("Two Spoons"), iter.next());
/// assert_eq!(Some("Johnson"), iter.next());
/// assert_eq!(None, iter.next());
/// ```
///
/// ## Interacting with the [`Substr`] object
///
/// ```
/// # use initiative_core::utils::quoted_words;
/// let mut iter = quoted_words(r#"   Ronny  "Two Spoons" Johnson  "#);
/// let word = iter.nth(1).unwrap();
///
/// assert_eq!("Two Spoons", word.as_str());
/// assert_eq!(r#""Two Spoons""#, word.as_outer_str());
/// assert_eq!(" Johnson  ", word.after().as_str());
/// assert_eq!(r#"   Ronny  "Two Spoons" Johnson  "#, word.as_original_str());
/// ```
pub fn quoted_words<'a, W>(phrase: W) -> impl Iterator<Item = Substr<'a>>
where
    W: Into<Substr<'a>>,
{
    QuotedWordIter::new(phrase.into())
}

/// Iterate through the possible phrases in the input (always starting from the first word). In the
/// event that the first word is quoted per [`quoted_words`], the first result will be the contents
/// of the quotes, but subsequent results will include the quotes as part of a larger phrase.
///
/// Also like `quoted_words`, the returned values are [`Substr`] objects, which can be cast back to
/// `&str` using [`Substr::as_str`].
///
/// # Examples
///
/// ```
/// # use initiative_core::utils::quoted_phrases;
/// let mut iter = quoted_phrases(r#"  "Medium" Dave Lilywhite  "#)
///     .map(|substr| substr.as_str());
///
/// assert_eq!(Some("Medium"), iter.next());
/// assert_eq!(Some(r#""Medium" Dave"#), iter.next());
/// assert_eq!(Some(r#""Medium" Dave Lilywhite"#), iter.next());
/// assert_eq!(None, iter.next());
/// ```
pub fn quoted_phrases<'a, W>(phrase: W) -> impl Iterator<Item = Substr<'a>>
where
    W: Into<Substr<'a>>,
{
    let mut iter = quoted_words(phrase);
    let first = iter.next();
    let start = first.as_ref().map(|f| f.range().start).unwrap_or(0);

    first.into_iter().chain(iter.map(move |substr| {
        Substr::new(
            substr.as_original_str(),
            start..substr.range().end,
            start..substr.range().end,
        )
    }))
}

pub struct QuotedWordIter<'a> {
    phrase: Substr<'a>,
    char_iter: CharIndices<'a>,
    quote_len: Option<usize>,
}

impl<'a> QuotedWordIter<'a> {
    fn new(phrase: Substr<'a>) -> Self {
        Self {
            char_iter: phrase.as_original_str().char_indices(),
            phrase,
            quote_len: None,
        }
    }
}

impl<'a> Iterator for QuotedWordIter<'a> {
    type Item = Substr<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (quote_char, first_index) = if let Some(quote_len) = self.quote_len {
            self.quote_len = None;

            if let Some((i, c)) = self.char_iter.next() {
                if c == '"' {
                    return Some(Substr::new(
                        self.phrase.as_original_str(),
                        i..i,
                        i - quote_len..i + c.len_utf8(),
                    ));
                } else {
                    (
                        self.phrase.as_original_str()[i - quote_len..i]
                            .chars()
                            .next(),
                        i,
                    )
                }
            } else {
                let original_str = self.phrase.as_original_str();
                return Some(Substr::new(
                    original_str,
                    original_str.len()..original_str.len(),
                    original_str.len() - quote_len..original_str.len(),
                ));
            }
        } else {
            let (first_index, first_char) = loop {
                if let Some((i, c)) = self.char_iter.next() {
                    if !c.is_whitespace() {
                        break (i, c);
                    }
                } else {
                    return None;
                }
            };

            if first_char == '"' {
                if let Some((i, c)) = self.char_iter.next() {
                    if c == '"' {
                        // Empty quotes = yield empty string
                        return Some(Substr::new(
                            self.phrase.as_original_str(),
                            i..i,
                            i - first_char.len_utf8()..i + c.len_utf8(),
                        ));
                    } else {
                        (Some(first_char), i)
                    }
                } else {
                    let original_str = self.phrase.as_original_str();
                    return Some(Substr::new(
                        original_str,
                        original_str.len()..original_str.len(),
                        first_index..original_str.len(),
                    ));
                }
            } else {
                (None, first_index)
            }
        };

        let last_index = loop {
            if let Some((i, c)) = self.char_iter.next() {
                if let Some(quote_char) = quote_char {
                    if c == '"' {
                        return Some(Substr::new(
                            self.phrase.as_original_str(),
                            first_index..i,
                            first_index - quote_char.len_utf8()..i + c.len_utf8(),
                        ));
                    }
                } else if c == '"' {
                    self.quote_len = Some(c.len_utf8());
                    break i;
                } else if c.is_whitespace() {
                    break i;
                }
            } else if let Some(quote_char) = quote_char {
                let original_str = self.phrase.as_original_str();
                return Some(Substr::new(
                    original_str,
                    first_index..original_str.len(),
                    first_index - quote_char.len_utf8()..original_str.len(),
                ));
            } else {
                break self.phrase.as_original_str().len();
            }
        };

        Some(Substr::new(
            self.phrase.as_original_str(),
            first_index..last_index,
            first_index..last_index,
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn quoted_word_iter_test() {
        let input = "a boy \n named \"Johnny Cash\"";
        let mut input_iter = quoted_words(input);

        let word = input_iter.next().unwrap();
        assert_eq!("a", word.as_str());
        assert_eq!(0..1, word.range());

        let word = input_iter.next().unwrap();
        assert_eq!("boy", word.as_str());
        assert_eq!(2..5, word.range());

        let word = input_iter.next().unwrap();
        assert_eq!("named", word.as_str());
        assert_eq!(8..13, word.range());

        let word = input_iter.next().unwrap();
        assert_eq!("Johnny Cash", word.as_str());
        assert_eq!(14..27, word.range());

        assert!(input_iter.next().is_none());
    }

    #[test]
    fn quoted_phrases_test() {
        let input = "  \"Medium\" Dave  ";
        let mut input_iter = quoted_phrases(input);

        let substr = input_iter.next().unwrap();
        assert_eq!("Medium", substr.as_str());
        assert_eq!(2..10, substr.range());

        let substr = input_iter.next().unwrap();
        assert_eq!("\"Medium\" Dave", substr.as_str());
        assert_eq!(2..15, substr.range());

        assert!(input_iter.next().is_none());
    }

    #[test]
    fn quoted_phrases_test_repeated() {
        assert_eq!(
            vec!["badger", "badger badger", "badger badger badger"],
            quoted_phrases("badger badger badger")
                .map(|w| w.as_str())
                .collect::<Vec<_>>(),
        );
    }

    #[test]
    fn quoted_word_iter_test_trailing_comma() {
        let input = "\"Legolas\", an elf";
        let mut input_iter = quoted_words(input);

        let word = input_iter.next().unwrap();
        assert_eq!("Legolas", word.as_str());
        assert_eq!(0..9, word.range());

        let word = input_iter.next().unwrap();
        assert_eq!(",", word.as_str());
        assert_eq!(9..10, word.range());

        let word = input_iter.next().unwrap();
        assert_eq!("an", word.as_str());
        assert_eq!(11..13, word.range());

        let word = input_iter.next().unwrap();
        assert_eq!("elf", word.as_str());
        assert_eq!(14..17, word.range());

        assert!(input_iter.next().is_none());
    }

    #[test]
    fn quoted_word_iter_test_empty_quotes() {
        let input = "\"\"";
        let mut input_iter = quoted_words(input);

        let word = input_iter.next().unwrap();
        assert_eq!("", word.as_str());
        assert_eq!(0..2, word.range());

        assert!(input_iter.next().is_none());
    }

    #[test]
    fn quoted_word_iter_test_empty_quotes_mid_word() {
        let input = "  bl\"\"ah ";
        let mut input_iter = quoted_words(input);

        let word = input_iter.next().unwrap();
        assert_eq!("bl", word.as_str());
        assert_eq!(2..4, word.range());

        let word = input_iter.next().unwrap();
        assert_eq!("", word.as_str());
        assert_eq!(4..6, word.range());

        let word = input_iter.next().unwrap();
        assert_eq!("ah", word.as_str());
        assert_eq!(6..8, word.range());

        assert!(input_iter.next().is_none());
    }

    #[test]
    fn quoted_word_iter_test_unclosed_quote() {
        let input = "  bl\"ah ";
        let mut input_iter = quoted_words(input);

        let word = input_iter.next().unwrap();
        assert_eq!("bl", word.as_str());
        assert_eq!(2..4, word.range());

        let word = input_iter.next().unwrap();
        assert_eq!("ah ", word.as_str());
        assert_eq!(4..8, word.range());

        assert!(input_iter.next().is_none());
    }

    #[test]
    fn quoted_word_iter_test_unclosed_quote_at_end() {
        let input = " \"";
        let mut input_iter = quoted_words(input);

        let word = input_iter.next().unwrap();
        assert_eq!("", word.as_str());
        assert_eq!(1..2, word.range());

        assert!(input_iter.next().is_none());
    }

    #[test]
    fn quoted_word_iter_test_trailing_quote() {
        let input = "  bl\"";
        let mut input_iter = quoted_words(input);

        let word = input_iter.next().unwrap();
        assert_eq!("bl", word.as_str());
        assert_eq!(2..4, word.range());

        let word = input_iter.next().unwrap();
        assert_eq!("", word.as_str());
        assert_eq!(4..5, word.range());

        assert!(input_iter.next().is_none());
    }

    #[test]
    fn quoted_word_iter_test_single_letter() {
        let input = "🥔";
        let mut input_iter = quoted_words(input);

        let word = input_iter.next().unwrap();
        assert_eq!("🥔", word.as_str());
        assert_eq!(0..4, word.range());

        assert!(input_iter.next().is_none());
    }

    #[test]
    fn quoted_word_iter_test_empty() {
        assert!(quoted_words("").next().is_none());
        assert!(quoted_words(" ").next().is_none());
    }
}
