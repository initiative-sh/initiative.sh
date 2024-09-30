use super::{Match, MatchType, Token, TokenType};

use crate::app::AppMeta;

use std::pin::Pin;

use async_stream::stream;
use futures::prelude::*;

pub fn match_input<'a, M>(
    token: Token<'a, M>,
    input: &'a str,
    app_meta: &'a AppMeta,
) -> Pin<Box<dyn Stream<Item = MatchType<'a, M>> + 'a>>
where
    M: Clone,
{
    let TokenType::Or(tokens) = token.token_type else {
        unreachable!();
    };

    Box::pin(stream! {
        let streams = tokens.into_iter().map(|token| token.clone().match_input(input, app_meta));
        for await match_type in stream::select_all(streams) {
            yield match_type.map(|token_match| Match::new(token.clone(), token_match));
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
