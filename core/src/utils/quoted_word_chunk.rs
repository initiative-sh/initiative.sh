use super::{QuotedWords, Word};
use std::ops::Range;

pub trait QuotedWordChunk<'a> {
    /// Break the input into chunks by quoted words, attempting to parse each chunk in turn.
    ///
    /// Unlike an iterator, the results must be advanced manually. The current value can be fetched
    /// with current(), and the cursor is advanced with the matches(), partially_matches(), and
    /// reject() methods.
    ///
    /// If a word is rejected, `reject_filter` will be called with the input word, allowing it to
    /// optionally specify a failure case ("word not found"). If it returns `None`, the word will
    /// be silently ignored.
    fn quoted_word_chunks<I, R>(&'a self, reject_filter: R) -> QuotedWordChunks<'a, I, R>
    where
        R: Fn(&'a str) -> Option<I>;
}

pub struct QuotedWordChunks<'a, I, R>
where
    R: Fn(&'a str) -> Option<I>,
{
    cursor: Range<usize>,
    cursor_last_match: Option<Range<usize>>,
    cursor_retry_match: Option<Range<usize>>,
    output: Vec<I>,
    phrase: &'a str,
    reject_count: usize,
    reject_filter: R,
    words: Vec<Word<'a>>,
}

impl<'a, I, R> QuotedWordChunks<'a, I, R>
where
    R: Fn(&'a str) -> Option<I>,
{
    pub fn current(&self) -> Option<&'a str> {
        let cursor = self.get_cursor();

        if cursor.end == cursor.start + 1 {
            self.words
                .get(cursor.start)
                .map(|w| w.as_own_str(self.phrase))
        } else if let (Some(start_word), Some(end_word)) =
            (self.words.get(cursor.start), self.words.get(cursor.end - 1))
        {
            Some(&self.phrase[start_word.outer_range.start..end_word.outer_range.end])
        } else {
            None
        }
    }

    pub fn matches(&mut self, item: I) {
        self.fill_rejected_words();

        let cursor = self.get_cursor();

        if self.cursor_retry_match.take().is_some() {
            self.output.pop();
        }

        self.output.push(item);
        self.set_start(cursor.end);
        self.cursor_last_match = Some(cursor);
    }

    pub fn partially_matches(&mut self) {
        if self.cursor.end >= self.words.len() {
            self.reject();
        } else {
            self.advance_end();
        }
    }

    pub fn reject(&mut self) {
        if let (None, Some(cursor_last_match)) = (&self.cursor_retry_match, &self.cursor_last_match)
        {
            self.cursor_retry_match = Some(cursor_last_match.start..self.cursor.start + 1);
        } else {
            self.advance_start();
        }
    }

    pub fn into_output(self) -> Vec<I> {
        self.into_output_with_reject_count().0
    }

    pub fn into_output_with_reject_count(mut self) -> (Vec<I>, usize) {
        self.fill_rejected_words();
        (self.output, self.reject_count)
    }

    fn get_cursor(&self) -> Range<usize> {
        self.cursor_retry_match
            .as_ref()
            .unwrap_or(&self.cursor)
            .to_owned()
    }

    fn advance_start(&mut self) {
        self.set_start(self.cursor.start + 1);
    }

    fn advance_end(&mut self) {
        self.cursor_retry_match.take();
        if self.cursor.end < self.words.len() {
            self.cursor.end += 1;
        } else {
            self.advance_start();
        }
    }

    fn set_start(&mut self, offset: usize) {
        self.cursor_retry_match = None;
        self.cursor = offset..offset + 1;
    }

    fn fill_rejected_words(&mut self) {
        for index in self.cursor_last_match.as_ref().map_or(0, |r| r.end)..self.get_cursor().start {
            if let Some(word) = self.words.get(index) {
                self.reject_count += 1;
                if let Some(value) = (self.reject_filter)(word.as_own_str(self.phrase)) {
                    self.output.push(value);
                }
            }
        }
    }
}

impl<'a, T: AsRef<str>> QuotedWordChunk<'a> for T {
    fn quoted_word_chunks<I, R>(&'a self, reject_filter: R) -> QuotedWordChunks<'a, I, R>
    where
        R: Fn(&'a str) -> Option<I>,
    {
        QuotedWordChunks {
            cursor: 0..1,
            cursor_last_match: None,
            cursor_retry_match: None,
            output: Vec::new(),
            phrase: self.as_ref(),
            reject_count: 0,
            reject_filter,
            words: self.quoted_words().collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn quoted_word_chunk_filter_test_strict_include_rejects() {
        let mut chunks = "My Word Count Isn't Evenly Divisible By Three."
            .quoted_word_chunks(|s| Some(s.to_uppercase()));
        let mut words_visited = Vec::new();

        while let Some(word) = chunks.current() {
            words_visited.push(word);

            match word.chars().filter(|c| c.is_whitespace()).count() {
                0..=1 => chunks.partially_matches(),
                2 => chunks.matches(word.to_lowercase()),
                _ => chunks.reject(),
            };
        }

        let (output, reject_count) = chunks.into_output_with_reject_count();

        assert_eq!(
            vec!["my word count", "isn't evenly divisible", "BY", "THREE."],
            output,
        );
        assert_eq!(2, reject_count);
        assert_eq!(
            vec![
                "My",
                "My Word",
                "My Word Count",
                "Isn't",
                "Isn't Evenly",
                "Isn't Evenly Divisible",
                "By",
                "By Three.",
                "Isn't Evenly Divisible By",
                "Three.",
                "Isn't Evenly Divisible By Three."
            ],
            words_visited,
        );
    }

    #[test]
    fn quoted_word_chunk_filter_test_strict_ignore_rejects() {
        let mut chunks = "This is not divisible by two either.".quoted_word_chunks(|_| None);
        let mut words_visited = Vec::new();

        while let Some(word) = chunks.current() {
            words_visited.push(word);

            match word.chars().filter(|c| c.is_whitespace()).count() {
                0 => chunks.partially_matches(),
                1 => chunks.matches(word.to_string()),
                _ => chunks.reject(),
            }
        }

        let (output, reject_count) = chunks.into_output_with_reject_count();

        assert_eq!(vec!["This is", "not divisible", "by two"], output);
        assert_eq!(1, reject_count);
        assert_eq!(
            vec![
                "This",
                "This is",
                "not",
                "not divisible",
                "by",
                "by two",
                "either.",
                "by two either."
            ],
            words_visited
        );
    }

    #[test]
    fn quoted_word_chunk_filter_test_permissive() {
        let mut chunks = "This is not divisible by two either.".quoted_word_chunks(|_| None);
        let mut words_visited = Vec::new();

        while let Some(word) = chunks.current() {
            words_visited.push(word);

            match word.chars().filter(|c| c.is_whitespace()).count() {
                0 => chunks.partially_matches(),
                _ => chunks.matches(word.to_string()),
            }
        }

        let (output, reject_count) = chunks.into_output_with_reject_count();

        assert_eq!(vec!["This is", "not divisible", "by two either."], output);
        assert_eq!(0, reject_count);
        assert_eq!(
            vec![
                "This",
                "This is",
                "not",
                "not divisible",
                "by",
                "by two",
                "either.",
                "by two either."
            ],
            words_visited
        );
    }

    #[test]
    fn quoted_word_chunk_filter_test_complex() {
        let mut chunks = "some_1 partial_2 no_3 partial_4 some_5 force_partial_6 no_7"
            .quoted_word_chunks(|s| Some(s.to_uppercase()));
        let mut words_visited = Vec::new();

        while let Some(word) = chunks.current() {
            words_visited.push(word);

            if word.contains("no_") {
                chunks.reject();
            } else if word.contains("force_partial_") {
                chunks.partially_matches();
            } else if word.starts_with("some_") {
                chunks.matches(word.to_string())
            } else {
                chunks.partially_matches();
            };
        }

        let (output, reject_count) = chunks.into_output_with_reject_count();

        assert_eq!(
            vec![
                "some_1 partial_2",
                "NO_3",
                "PARTIAL_4",
                "some_5",
                "FORCE_PARTIAL_6",
                "NO_7",
            ],
            output,
        );
        assert_eq!(4, reject_count);
        assert_eq!(
            vec![
                "some_1",
                "partial_2",
                "partial_2 no_3",
                "some_1 partial_2",
                "no_3",
                "some_1 partial_2 no_3",
                "partial_4",
                "partial_4 some_5",
                "partial_4 some_5 force_partial_6",
                "partial_4 some_5 force_partial_6 no_7",
                "some_1 partial_2 no_3 partial_4",
                "some_5",
                "force_partial_6",
                "force_partial_6 no_7",
                "some_5 force_partial_6",
                "no_7",
                "some_5 force_partial_6 no_7",
            ],
            words_visited
        );
    }

    #[test]
    fn quoted_word_chunk_test() {
        let mut chunks: QuotedWordChunks<(), _> = r#"aa "bb cc" dd"#.quoted_word_chunks(|_| None);
        let mut words_visited = Vec::new();

        while let Some(word) = chunks.current() {
            words_visited.push(word);
            chunks.partially_matches();
        }

        assert!(chunks.into_output().is_empty());

        assert_eq!(
            vec![
                "aa",
                r#"aa "bb cc""#,
                r#"aa "bb cc" dd"#,
                "bb cc",
                r#""bb cc" dd"#,
                "dd",
            ],
            words_visited,
        );
    }
}
