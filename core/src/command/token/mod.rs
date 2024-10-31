mod keyword;

use crate::app::AppMeta;
use crate::utils::Word;

use std::pin::Pin;

use futures::prelude::*;

#[derive(Debug, Eq, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub marker: Option<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenMatch<'a> {
    pub token: &'a Token,
    pub match_meta: MatchMeta,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FuzzyMatch<'a> {
    Overflow(TokenMatch<'a>, Word<'a>),
    Exact(TokenMatch<'a>),
    Partial(TokenMatch<'a>, Option<String>),
}

#[derive(Debug, Eq, PartialEq)]
pub enum TokenType {
    /// A literal word
    Keyword(&'static str),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MatchMeta {
    None,
}

impl Token {
    pub fn keyword(keyword: &'static str) -> Self {
        Token {
            token_type: TokenType::Keyword(keyword),
            marker: None,
        }
    }

    pub fn match_input<'a, 'b>(
        &'a self,
        input: &'a str,
        _app_meta: &'b AppMeta,
    ) -> Pin<Box<dyn Stream<Item = FuzzyMatch<'a>> + 'b>>
    where
        'a: 'b,
    {
        match &self.token_type {
            TokenType::Keyword(..) => keyword::match_input(self, input),
        }
    }
}

impl<'a> TokenMatch<'a> {
    pub fn new(token: &'a Token, match_meta: impl Into<MatchMeta>) -> Self {
        TokenMatch {
            token,
            match_meta: match_meta.into(),
        }
    }
}

impl<'a> From<&'a Token> for TokenMatch<'a> {
    fn from(input: &'a Token) -> Self {
        TokenMatch::new(input, MatchMeta::None)
    }
}

impl<'a> FuzzyMatch<'a> {
    pub fn into_exact(self) -> Option<TokenMatch<'a>> {
        if let FuzzyMatch::Exact(token_match) = self {
            Some(token_match)
        } else {
            None
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    pub async fn assert_stream_eq<'a, T>(
        mut expect_results: Vec<T>,
        stream: Pin<Box<dyn Stream<Item = T> + 'a>>,
    ) where
        T: std::fmt::Debug + PartialEq,
    {
        for result in stream.collect::<Vec<_>>().await {
            let Some(index) = expect_results
                .iter()
                .position(|expect_result| expect_result == &result)
            else {
                panic!(
                    "Not found in expected results: {:?}\n\nRemaining expected results: {:?}",
                    result, expect_results
                );
            };
            expect_results.swap_remove(index);
        }

        assert_eq!(
            Vec::<T>::new(),
            expect_results,
            "Expected all results to be exhausted",
        );
    }
}
