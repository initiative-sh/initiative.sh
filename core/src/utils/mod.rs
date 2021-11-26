pub use case_insensitive_str::CaseInsensitiveStr;
pub use quoted_word_chunk::QuotedWordChunk;
pub use quoted_word_iter::QuotedWords;
pub use split_multi::split_multi_ci;

mod case_insensitive_str;
mod quoted_word_chunk;
mod quoted_word_iter;
mod split_multi;

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

#[derive(Clone, Debug)]
pub struct Word<'a> {
    phrase: &'a str,
    pub inner_range: Range<usize>,
    pub outer_range: Range<usize>,
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
