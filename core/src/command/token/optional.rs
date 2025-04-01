use crate::app::AppMeta;
use crate::command::prelude::*;

use std::pin::Pin;

use futures::prelude::*;

pub fn match_input<'a, 'b>(
    token: &'a Token,
    input: &'a str,
    app_meta: &'b AppMeta,
) -> Pin<Box<dyn Stream<Item = FuzzyMatch<'a>> + 'b>>
where
    'a: 'b,
{
    let TokenType::Optional(token) = &token.token_type else {
        unreachable!();
    };

    Box::pin(
        stream::iter([FuzzyMatch::Exact(token.as_ref().into())]).chain(
            token.match_input(input, app_meta).map(|fuzzy_match| {
                fuzzy_match.map(|token_match| TokenMatch::new(token, token_match))
            }),
        ),
    )
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
