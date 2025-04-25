use super::prelude::*;

pub fn match_input<'input, 'stream>(
    token: &'stream Token,
    input: Substr<'input>,
    app_meta: &'stream AppMeta,
) -> Pin<Box<dyn Stream<Item = FuzzyMatchList<'input>> + 'stream>>
where
    'input: 'stream,
{
    let TokenKind::Or { tokens } = &token.kind else {
        unreachable!();
    };

    Box::pin(stream::iter(tokens).flat_map(move |token| token.match_input(input.clone(), app_meta)))
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test_utils as test;

    #[derive(Hash)]
    enum Marker {
        AnyWord,
        Keyword,
    }

    #[tokio::test]
    async fn match_input_test_simple() {
        let token = or([
            any_word().with_marker(Marker::AnyWord),
            keyword("badger").with_marker(Marker::Keyword),
        ]);

        test::assert_eq_unordered!(
            [
                FuzzyMatchList::new_overflow(
                    MatchPart::new_unmarked("badger".into()).with_marker(Marker::AnyWord),
                    " badger".into(),
                ),
                FuzzyMatchList::new_overflow(
                    MatchPart::new_unmarked("badger".into())
                        .with_marker(Marker::Keyword)
                        .with_term("badger"),
                    " badger".into(),
                ),
            ],
            token
                .match_input("badger badger", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_partial() {
        let token = or([keyword("badger"), keyword("badgering")]);

        test::assert_eq_unordered!(
            [
                FuzzyMatchList::new_exact(
                    MatchPart::new_unmarked("badger".into()).with_term("badger"),
                ),
                FuzzyMatchList::new_incomplete(
                    MatchPart::new_unmarked("badger".into()).with_term("badgering"),
                ),
            ],
            token
                .match_input("badger", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }
}
