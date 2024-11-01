use crate::command::prelude::*;
use crate::utils::quoted_words;

use std::pin::Pin;

use async_stream::stream;
use futures::prelude::*;

pub fn match_input<'a>(
    token: &'a Token,
    input: &'a str,
) -> Pin<Box<dyn Stream<Item = FuzzyMatch<'a>> + 'a>> {
    assert!(matches!(token.token_type, TokenType::AnyWord));

    Box::pin(stream! {
        let mut iter = quoted_words(input);
        if let Some(word) = iter.next() {
            let token_match = TokenMatch::new(token, word.as_str());

            if word.is_at_end() {
                yield FuzzyMatch::Exact(token_match);
            } else {
                yield FuzzyMatch::Overflow(token_match, word.after());
            }
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn match_input_test() {
        let token = Token {
            token_type: TokenType::AnyWord,
            marker: Some(20),
        };

        assert_eq!(
            &[FuzzyMatch::Exact(TokenMatch::new(token.clone(), "Jesta"))][..],
            match_input(token.clone(), "Jesta")
                .collect::<Vec<_>>()
                .await,
        );

        assert_eq!(
            &[FuzzyMatch::Overflow(
                TokenMatch::new(token.clone(), "Nott"),
                " \"The Brave\" "
            )][..],
            match_input(token.clone(), " Nott \"The Brave\" ")
                .collect::<Vec<_>>()
                .await,
        );
    }
}
