use super::prelude::*;

pub fn match_input<'input, 'stream>(
    token: &'stream Token,
    input: Substr<'input>,
    app_meta: &'stream AppMeta,
) -> Pin<Box<dyn Stream<Item = FuzzyMatchList<'input>> + 'stream>>
where
    'input: 'stream,
{
    let TokenKind::Optional {
        token: optional_token,
    } = &token.kind
    else {
        unreachable!();
    };

    Box::pin(
        stream::once(future::ready(
            if quoted_words(input.clone()).next().is_some() {
                FuzzyMatchList::new_overflow(MatchList::default(), input.clone())
            } else {
                FuzzyMatchList::new_exact(MatchList::default())
            },
        ))
        .chain(optional_token.match_input(input, app_meta)),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test_utils as test;

    #[derive(Hash)]
    enum Marker {
        Keyword,
    }

    #[tokio::test]
    async fn match_input_test_simple() {
        let token = optional(keyword("badger").with_marker(Marker::Keyword));

        test::assert_eq_unordered!(
            [
                FuzzyMatchList::new_exact(
                    MatchPart::new_unmarked("badger".into())
                        .with_marker(Marker::Keyword)
                        .with_term("badger"),
                ),
                FuzzyMatchList::new_overflow(MatchList::default(), "badger".into()),
            ],
            token
                .match_input("badger", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_empty() {
        let token = optional(keyword("badger").with_marker(Marker::Keyword));

        test::assert_eq_unordered!(
            [
                FuzzyMatchList::new_exact(MatchList::default()),
                FuzzyMatchList::new_incomplete(
                    MatchPart::new_unmarked("".into())
                        .with_marker(Marker::Keyword)
                        .with_term("badger"),
                ),
            ],
            token
                .match_input("   ", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }
}
