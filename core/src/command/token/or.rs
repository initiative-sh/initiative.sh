use super::{FuzzyMatch, Token, TokenMatch, TokenType};

use crate::app::AppMeta;

use std::pin::Pin;

use async_stream::stream;
use futures::prelude::*;

pub fn match_input<'a, 'b>(
    token: &'a Token<'a>,
    input: &'a str,
    app_meta: &'b AppMeta,
) -> Pin<Box<dyn Stream<Item = FuzzyMatch<'a>> + 'b>>
where
    'a: 'b,
{
    Box::pin(stream! {
        let TokenType::Or(tokens) = &token.token_type else {
            unreachable!();
        };

        let streams = tokens.into_iter().map(|token| token.match_input(input, app_meta));
        for await fuzzy_match in stream::select_all(streams) {
            yield fuzzy_match.map(|token_match| TokenMatch::new(token, token_match));
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::app::Event;
    use crate::storage::NullDataStore;
    use tokio_test::block_on;

    #[test]
    fn match_input_test() {
        let app_meta = AppMeta::new(NullDataStore, &event_dispatcher);
    }

    fn event_dispatcher(_event: Event) {}
}
