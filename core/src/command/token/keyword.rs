use crate::command::prelude::*;
use crate::utils::{quoted_words, CaseInsensitiveStr, Substr};

use std::pin::Pin;

use async_stream::stream;
use futures::prelude::*;

pub fn match_input<'input>(
    token: &Token,
    input: Substr<'input>,
) -> Pin<Box<dyn Stream<Item = FuzzyMatchList<'input>> + 'input>> {
    let &Token::Keyword { term, marker_hash } = token else {
        unreachable!();
    };

    Box::pin(stream! {
        let mut iter = quoted_words(input);
        if let Some(first_word) = iter.next() {
            if term.eq_ci(&first_word) {
                let match_part = MatchPart::new(first_word.clone(), marker_hash).matching(term);

                if first_word.is_at_end() {
                    yield FuzzyMatchList::new_exact(match_part);
                } else {
                    yield FuzzyMatchList::new_overflow(match_part, first_word.after());
                }
            } else if first_word.can_complete() && term.starts_with_ci(&first_word) {
                let match_part = MatchPart::new(first_word, marker_hash).matching(term);
                yield FuzzyMatchList::new_incomplete(match_part);
            }
        }
    })
}

#[cfg(test)]
mod test {
    use super::super::hash_marker;
    use super::*;

    use crate::test_utils as test;

    #[derive(Hash)]
    enum Marker {
        Keyword,
    }

    #[tokio::test]
    async fn match_input_test_exact() {
        let token = keyword_m(Marker::Keyword, "badger");

        test::assert_eq_unordered!(
            [FuzzyMatchList::new_exact(
                MatchPart::new("BADGER".into(), hash_marker(Marker::Keyword)).matching("badger")
            )],
            token
                .match_input("BADGER", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_sequential() {
        let token = keyword("mushroom");
        let input = quoted_words("BADGER MUSHROOM").nth(1).unwrap();

        test::assert_eq_unordered!(
            [FuzzyMatchList::new_exact(
                MatchPart::new("MUSHROOM".into(), 0).matching("mushroom")
            )],
            token
                .match_input(input.clone(), &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_overflow() {
        let token = keyword("badger");

        test::assert_eq_unordered!(
            [FuzzyMatchList::new_overflow(
                MatchPart::new("badger".into(), 0).matching("badger"),
                " mushroom snake".into(),
            )],
            token
                .match_input("badger mushroom snake", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_partial() {
        let token = keyword("badger");

        test::assert_eq_unordered!(
            [FuzzyMatchList::new_incomplete(
                MatchPart::new("badg".into(), 0).matching("badger")
            )],
            token
                .match_input(" badg", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );

        test::assert_empty!(
            token
                .match_input(" badg ", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );

        test::assert_empty!(
            token
                .match_input(r#""badg""#, &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }
}
