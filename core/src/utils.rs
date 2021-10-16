use std::iter::Iterator;
use std::ops::Range;
use std::str::CharIndices;

pub fn quoted_words(phrase: &str) -> QuotedWordIterator<'_> {
    QuotedWordIterator::new(phrase)
}

pub fn capitalize(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut char_iter = input.chars();

    if let Some(c) = char_iter.next() {
        c.to_uppercase().for_each(|c| result.push(c));
    }
    char_iter.for_each(|c| result.push(c));

    result
}

pub struct Word<'a> {
    slice: &'a str,
    range: Range<usize>,
}

pub struct QuotedWordIterator<'a> {
    phrase: &'a str,
    char_iter: CharIndices<'a>,
    quote_len: Option<usize>,
}

impl<'a> Word<'a> {
    fn new(slice: &'a str, range: Range<usize>) -> Self {
        Self { slice, range }
    }

    pub fn as_str(&'a self) -> &'a str {
        self.slice
    }

    pub fn range(&'a self) -> &'a Range<usize> {
        &self.range
    }
}

impl<'a> QuotedWordIterator<'a> {
    fn new(phrase: &'a str) -> Self {
        Self {
            phrase,
            char_iter: phrase.char_indices(),
            quote_len: None,
        }
    }
}

impl<'a> Iterator for QuotedWordIterator<'a> {
    type Item = Word<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (quote_char, first_index) = if let Some(quote_len) = self.quote_len {
            self.quote_len = None;

            if let Some((i, c)) = self.char_iter.next() {
                if c == '"' {
                    return Some(Word::new(
                        &self.phrase[i..i],
                        i - quote_len..i + c.len_utf8(),
                    ));
                } else {
                    (self.phrase[i - quote_len..i].chars().next(), i)
                }
            } else {
                return Some(Word::new(
                    &self.phrase[self.phrase.len()..self.phrase.len()],
                    self.phrase.len() - quote_len..self.phrase.len(),
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
                        return Some(Word::new(
                            &self.phrase[i..i],
                            i - first_char.len_utf8()..i + c.len_utf8(),
                        ));
                    } else {
                        (Some(first_char), i)
                    }
                } else {
                    return Some(Word::new(
                        &self.phrase[self.phrase.len()..self.phrase.len()],
                        first_index..self.phrase.len(),
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
                        return Some(Word::new(
                            &self.phrase[first_index..i],
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
                return Some(Word::new(
                    &self.phrase[first_index..self.phrase.len()],
                    first_index - quote_char.len_utf8()..self.phrase.len(),
                ));
            } else {
                break self.phrase.len();
            }
        };

        Some(Word::new(
            &self.phrase[first_index..last_index],
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
        assert_eq!(0..1, *word.range());

        let word = input_iter.next().unwrap();
        assert_eq!("boy", word.as_str());
        assert_eq!(2..5, *word.range());

        let word = input_iter.next().unwrap();
        assert_eq!("named", word.as_str());
        assert_eq!(8..13, *word.range());

        let word = input_iter.next().unwrap();
        assert_eq!("Johnny Cash", word.as_str());
        assert_eq!(14..27, *word.range());

        assert!(input_iter.next().is_none());
    }

    #[test]
    fn quoted_word_iter_test_trailing_comma() {
        let input = "\"Legolas\", an elf";
        let mut input_iter = quoted_words(input);

        let word = input_iter.next().unwrap();
        assert_eq!("Legolas", word.as_str());
        assert_eq!(0..9, *word.range());

        let word = input_iter.next().unwrap();
        assert_eq!(",", word.as_str());
        assert_eq!(9..10, *word.range());

        let word = input_iter.next().unwrap();
        assert_eq!("an", word.as_str());
        assert_eq!(11..13, *word.range());

        let word = input_iter.next().unwrap();
        assert_eq!("elf", word.as_str());
        assert_eq!(14..17, *word.range());

        assert!(input_iter.next().is_none());
    }

    #[test]
    fn quoted_word_iter_test_empty_quotes() {
        let input = "\"\"";
        let mut input_iter = quoted_words(input);

        let word = input_iter.next().unwrap();
        assert_eq!("", word.as_str());
        assert_eq!(0..2, *word.range());

        assert!(input_iter.next().is_none());
    }

    #[test]
    fn quoted_word_iter_test_empty_quotes_mid_word() {
        let input = "  bl\"\"ah ";
        let mut input_iter = quoted_words(input);

        let word = input_iter.next().unwrap();
        assert_eq!("bl", word.as_str());
        assert_eq!(2..4, *word.range());

        let word = input_iter.next().unwrap();
        assert_eq!("", word.as_str());
        assert_eq!(4..6, *word.range());

        let word = input_iter.next().unwrap();
        assert_eq!("ah", word.as_str());
        assert_eq!(6..8, *word.range());

        assert!(input_iter.next().is_none());
    }

    #[test]
    fn quoted_word_iter_test_unclosed_quote() {
        let input = "  bl\"ah ";
        let mut input_iter = quoted_words(input);

        let word = input_iter.next().unwrap();
        assert_eq!("bl", word.as_str());
        assert_eq!(2..4, *word.range());

        let word = input_iter.next().unwrap();
        assert_eq!("ah ", word.as_str());
        assert_eq!(4..8, *word.range());

        assert!(input_iter.next().is_none());
    }

    #[test]
    fn quoted_word_iter_test_unclosed_quote_at_end() {
        let input = " \"";
        let mut input_iter = quoted_words(input);

        let word = input_iter.next().unwrap();
        assert_eq!("", word.as_str());
        assert_eq!(1..2, *word.range());

        assert!(input_iter.next().is_none());
    }

    #[test]
    fn quoted_word_iter_test_trailing_quote() {
        let input = "  bl\"";
        let mut input_iter = quoted_words(input);

        let word = input_iter.next().unwrap();
        assert_eq!("bl", word.as_str());
        assert_eq!(2..4, *word.range());

        let word = input_iter.next().unwrap();
        assert_eq!("", word.as_str());
        assert_eq!(4..5, *word.range());

        assert!(input_iter.next().is_none());
    }

    #[test]
    fn quoted_word_iter_test_single_letter() {
        let input = "🥔";
        let mut input_iter = quoted_words(input);

        let word = input_iter.next().unwrap();
        assert_eq!("🥔", word.as_str());
        assert_eq!(0..4, *word.range());

        assert!(input_iter.next().is_none());
    }
}
