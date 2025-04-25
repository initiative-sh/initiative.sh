use crate::command::prelude::*;
use crate::utils::{quoted_words, Substr};

use std::pin::Pin;

use async_stream::stream;
use futures::prelude::*;

pub fn match_input<'input, 'stream>(
    token: &'stream Token,
    input: Substr<'input>,
) -> Pin<Box<dyn Stream<Item = FuzzyMatchList<'input>> + 'stream>>
where
    'input: 'stream,
{
    let &Token::AnyWord { marker_hash } = token else {
        unreachable!();
    };

    Box::pin(stream! {
        let mut iter = quoted_words(input);
        if let Some(word) = iter.next() {
            let match_part = MatchPart::new(word.clone(), marker_hash);

            if word.is_at_end() {
                yield FuzzyMatchList::new_exact(match_part)
            } else {
                yield FuzzyMatchList::new_overflow(match_part, word.after());
            }
        } else {
            yield FuzzyMatchList::new_incomplete(MatchPart::new("".into(), marker_hash));
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils as test;

    #[derive(Hash)]
    enum Marker {
        Token,
    }

    #[tokio::test]
    async fn match_input_test_exact() {
        let token = any_word();

        test::assert_eq_unordered!(
            [FuzzyMatchList::new_exact(MatchPart::new_unmarked(
                "badger".into()
            ))],
            token
                .match_input("badger", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_overflow() {
        let token = any_word_m(Marker::Token);

        test::assert_eq_unordered!(
            [FuzzyMatchList::new_overflow(
                MatchPart::new_unmarked("badger".into()).with_marker(Marker::Token),
                "  mushroom  ".into(),
            )],
            token
                .match_input("  badger  mushroom  ", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_empty() {
        let token = any_word_m(Marker::Token);

        test::assert_eq_unordered!(
            [FuzzyMatchList::new_incomplete(
                MatchPart::new_unmarked("".into()).with_marker(Marker::Token),
            )],
            token
                .match_input("  ", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }
}
