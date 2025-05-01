use crate::command::token::prelude::*;

pub fn match_input<'input>(
    token: &Token,
    input: Substr<'input>,
) -> Pin<Box<dyn Stream<Item = FuzzyMatchList<'input>> + 'input>> {
    let &Token {
        kind: TokenKind::Keyword { term },
        marker_hash,
        ..
    } = token
    else {
        unreachable!();
    };

    Box::pin(stream! {
        let mut iter = quoted_words(input.clone());
        if let Some(first_word) = iter.next() {
            if term.eq_ci(&first_word) {
                if first_word.is_at_end() {
                    yield FuzzyMatchList::new_exact(
                        MatchPart::new(first_word, marker_hash).with_term(term),
                    );
                } else {
                    yield FuzzyMatchList::new_overflow(
                        MatchPart::new(first_word.clone(), marker_hash).with_term(term),
                        first_word.after(),
                    );
                }
            } else if first_word.can_complete() && term.starts_with_ci(&first_word) {
                yield FuzzyMatchList::new_incomplete(
                    MatchPart::new(first_word, marker_hash).with_term(term),
                );
            }
        } else {
            yield FuzzyMatchList::new_incomplete(MatchPart::new(input.after(), marker_hash).with_term(term))
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::command::token::constructors::*;
    use crate::test_utils as test;

    #[derive(Hash)]
    enum Marker {
        Keyword,
    }

    #[tokio::test]
    async fn match_input_test_exact() {
        let token = keyword("badger").with_marker(Marker::Keyword);

        test::assert_eq_unordered!(
            [FuzzyMatchList::new_exact(
                MatchPart::new_unmarked("BADGER".into())
                    .with_marker(Marker::Keyword)
                    .with_term("badger"),
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
                MatchPart::new_unmarked("MUSHROOM".into()).with_term("mushroom"),
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
                MatchPart::new_unmarked("badger".into()).with_term("badger"),
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
                MatchPart::new_unmarked("badg".into()).with_term("badger"),
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

    #[tokio::test]
    async fn match_input_test_empty() {
        let token = keyword("badger").with_marker(Marker::Keyword);
        test::assert_eq_unordered!(
            [FuzzyMatchList::new_incomplete(
                MatchPart::new_unmarked("".into())
                    .with_marker(Marker::Keyword)
                    .with_term("badger"),
            )],
            token
                .match_input("  ", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }
}
