use super::TokenKind;
use crate::command::prelude::*;
use crate::utils::{quoted_words, CaseInsensitiveStr, Substr};

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
    let TokenKind::KeywordList { terms } = &token.kind else {
        unreachable!();
    };
    let marker_hash = token.marker_hash;

    Box::pin(stream! {
        let mut iter = quoted_words(input).peekable();
        if let Some(first_word) = iter.next() {
            for term in terms {
                if term.eq_ci(&first_word) {
                    let match_part = MatchPart::new(first_word.clone(), marker_hash)
                        .with_term(term);

                    if iter.peek().is_none() {
                        yield FuzzyMatchList::new_exact(match_part);
                    } else {
                        yield FuzzyMatchList::new_overflow(match_part, first_word.after());
                    }
                } else if first_word.can_complete() && term.starts_with_ci(&first_word) {
                    yield FuzzyMatchList::new_incomplete(
                        MatchPart::new(first_word.clone(), marker_hash).with_term(term),
                    );
                }
            }
        } else {
            for term in terms {
                yield FuzzyMatchList::new_incomplete(MatchPart::new("".into(), marker_hash).with_term(term));
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils as test;

    #[derive(Hash)]
    enum Marker {
        Token,
    }

    #[tokio::test]
    async fn match_input_test_no_complete() {
        let token = keyword_list(["badger", "mushroom", "snake", "badgering"]);

        test::assert_eq_unordered!(
            [FuzzyMatchList::new_exact(
                MatchPart::new_unmarked("BADGER".into()).with_term("badger")
            )],
            token
                .match_input("BADGER ", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_partial() {
        let token = keyword_list(["badge", "BADGER"]).with_marker(Marker::Token);

        test::assert_eq_unordered!(
            [
                FuzzyMatchList::new_exact(
                    MatchPart::new_unmarked("BADGE".into())
                        .with_marker(Marker::Token)
                        .with_term("badge")
                ),
                FuzzyMatchList::new_incomplete(
                    MatchPart::new_unmarked("BADGE".into())
                        .with_marker(Marker::Token)
                        .with_term("BADGER")
                ),
            ],
            token
                .match_input("BADGE", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_overflow() {
        let token = keyword_list(["badger", "mushroom"]);

        test::assert_eq_unordered!(
            [FuzzyMatchList::new_overflow(
                MatchPart::new_unmarked("badger".into()).with_term("badger"),
                " mushroom".into(),
            )],
            token
                .match_input("badger mushroom", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_empty() {
        let token = keyword_list(["badger", "mushroom"]).with_marker(Marker::Token);

        test::assert_eq_unordered!(
            [
                FuzzyMatchList::new_incomplete(
                    MatchPart::new_unmarked("".into())
                        .with_marker(Marker::Token)
                        .with_term("badger"),
                ),
                FuzzyMatchList::new_incomplete(
                    MatchPart::new_unmarked("".into())
                        .with_marker(Marker::Token)
                        .with_term("mushroom"),
                ),
            ],
            token
                .match_input("   ", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }
}
