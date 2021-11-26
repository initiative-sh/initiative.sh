use super::CaseInsensitiveStr;
use std::iter;

/// Split a string case-insensitively by multiple delimiters. The result is an iterator over
/// matching parts, exclusive of the matches themselves. The Vec yielded by the iterator will
/// always have words.len()+1 elements.
pub fn split_multi_ci<'a>(subject: &'a str, words: &'a [&'a str]) -> SplitMultiIter<'a> {
    SplitMultiIter::new(subject, words)
}

pub struct SplitMultiIter<'a> {
    subject: &'a str,
    word_offsets: Vec<(&'a str, usize)>,
    cursor: usize,
}

impl<'a> SplitMultiIter<'a> {
    pub fn new(subject: &'a str, words: &'a [&'a str]) -> Self {
        Self {
            subject,
            word_offsets: words.iter().map(|s| (*s, 0)).collect(),
            cursor: words.len() - 1,
        }
    }
}

impl<'a> iter::Iterator for SplitMultiIter<'a> {
    type Item = Vec<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.cursor >= self.word_offsets.len() {
                return None;
            }

            let (word, mut offset) = self.word_offsets[self.cursor];
            while !self.subject.is_char_boundary(offset) {
                offset += 1;
            }

            let range_end = self
                .word_offsets
                .get(self.cursor + 1)
                .map_or_else(|| self.subject.len(), |(_, i)| *i);

            if let Some(pos) = self.subject[offset..range_end].find_ci(word) {
                self.word_offsets[self.cursor].1 = offset + pos;

                if self.cursor == 0 {
                    let mut result = Vec::with_capacity(self.word_offsets.len() + 1);
                    let mut range_start = 0;
                    for (word, offset) in &self.word_offsets {
                        result.push(&self.subject[range_start..*offset]);
                        range_start = offset + word.len();
                    }
                    result.push(&self.subject[range_start..]);
                    self.word_offsets[0].1 += 1;
                    return Some(result);
                } else {
                    self.cursor -= 1;
                }
            } else {
                self.word_offsets[self.cursor].1 = 0;
                self.cursor += 1;
                if self.cursor < self.word_offsets.len() {
                    self.word_offsets[self.cursor].1 += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn split_multi_test() {
        let mut iter = split_multi_ci("foo bar baz", &["O", "BA"][..]);
        assert_eq!(Some(vec!["f", "o ", "r baz"]), iter.next());
        assert_eq!(Some(vec!["fo", " ", "r baz"]), iter.next());
        assert_eq!(Some(vec!["f", "o bar ", "z"]), iter.next());
        assert_eq!(Some(vec!["fo", " bar ", "z"]), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn split_multi_test_utf8() {
        let mut iter = split_multi_ci("p🥔tat🥔", &["p", "t"][..]);
        assert_eq!(Some(vec!["", "🥔", "at🥔"]), iter.next());
        assert_eq!(Some(vec!["", "🥔ta", "🥔"]), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());

        let mut iter = split_multi_ci("p🥔tat🥔", &["🥔", "t"][..]);
        assert_eq!(Some(vec!["p", "", "at🥔"]), iter.next());
        assert_eq!(Some(vec!["p", "ta", "🥔"]), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    /// It's a good example, but I can't write it as a doctest since it's only pub(crate).
    fn split_multi_test_raven() {
        let subject = "Take thy beak from out my heart, and take thy form from off my door!";
        let words = [" THY ", " MY "];
        let mut split_iter = split_multi_ci(subject, &words[..]);

        assert_eq!(
            Some(vec![
                "Take",
                "beak from out",
                "heart, and take thy form from off my door!",
            ]),
            split_iter.next(),
        );

        assert_eq!(
            Some(vec![
                "Take",
                "beak from out my heart, and take thy form from off",
                "door!",
            ]),
            split_iter.next(),
        );

        assert_eq!(
            Some(vec![
                "Take thy beak from out my heart, and take",
                "form from off",
                "door!",
            ]),
            split_iter.next(),
        );

        // We don't split by the second " thy " and the first " my " because they appear in the
        // wrong order.
        assert_eq!(None, split_iter.next());
    }
}
