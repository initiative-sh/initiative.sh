use super::prelude::*;

pub fn match_input<'input>(
    token: &Token,
    input: Substr<'input>,
) -> Pin<Box<dyn Stream<Item = FuzzyMatchList<'input>> + 'input>> {
    assert!(matches!(token.kind, TokenKind::AnyPhrase));
    let marker_hash = token.marker_hash;

    Box::pin(stream! {
        let mut phrases = quoted_phrases(input).peekable();

        while let Some(phrase) = phrases.next() {
            let match_part = MatchPart::new(phrase.clone(), marker_hash);

            if phrases.peek().is_none() {
                yield FuzzyMatchList::new_exact(match_part);
            } else {
                yield FuzzyMatchList::new_overflow(match_part, phrase.after());
            }
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
    async fn match_input_test_empty() {
        test::assert_empty!(
            any_phrase()
                .match_input("  ", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_simple() {
        let token = any_phrase();

        test::assert_eq_unordered!(
            [
                FuzzyMatchList::new_overflow(
                    MatchPart::new_unmarked("badger".into()),
                    " badger badger".into(),
                ),
                FuzzyMatchList::new_overflow(
                    MatchPart::new_unmarked("badger badger".into()),
                    " badger".into(),
                ),
                FuzzyMatchList::new_exact(MatchPart::new_unmarked("badger badger badger".into())),
            ],
            token
                .match_input("badger badger badger", &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }

    #[tokio::test]
    async fn match_input_test_quoted() {
        let token = any_phrase().with_marker(Marker::Token);

        test::assert_eq_unordered!(
            [
                FuzzyMatchList::new_overflow(
                    MatchPart::new_unmarked("Nott".into()).with_marker(Marker::Token),
                    r#" "The Brave" "#.into(),
                ),
                FuzzyMatchList::new_exact(
                    MatchPart::new_unmarked(r#"Nott "The Brave""#.into())
                        .with_marker(Marker::Token),
                ),
            ],
            token
                .match_input(r#" Nott "The Brave" "#, &test::app_meta())
                .collect::<Vec<_>>()
                .await,
        );
    }
}
