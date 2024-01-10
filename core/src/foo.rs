use super::utils::Word;
use crate::app::AutocompleteSuggestion;
use std::iter;



#[derive(Clone, Debug, Eq, PartialEq)]
enum Token {
    Literal(&'static str),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ParsedToken<'a> {
    word: Word<'a>,
    token: Token,
}

async fn parse_tokens<'a, const N: usize>(tokens: [Token; N], input: &'a str) -> impl Iterator<Item = AutocompleteSuggestion> {
    None.into_iter()
}

fn main() -> impl Iterator<Item = AutocompleteSuggestion> {
    iter::empty()
        .chain(AutocompleteIterator::new(Token::Literal("blah")))
        .chain(AutocompleteIterator::new(Token::Literal("blah")))
        .take(10)
}

struct AutocompleteIterator<'a, const N: usize> {
    input: &'a str,
    tokens: [Token; N],
}

impl<'a, const N: usize> Iterator for AutocompleteIterator<'a, N> {
    type Item = AutocompleteSuggestion;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

