use crate::command::prelude::*;

const MAX_DEPTH: usize = 5;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenMatchIterator<'a, 'b> {
    root_token_match: &'b TokenMatch<'a>,
    cursor: [usize; MAX_DEPTH],
    cursor_depth: usize,
    has_returned_root: bool,
}

impl<'a, 'b> TokenMatchIterator<'a, 'b> {
    pub fn new(root_token_match: &'b TokenMatch<'a>) -> Self {
        TokenMatchIterator {
            root_token_match,
            cursor: [0; MAX_DEPTH],
            cursor_depth: 0,
            has_returned_root: false,
        }
    }

    fn cursor(&self) -> &[usize] {
        assert!(
            self.cursor_depth <= MAX_DEPTH,
            "Token match exceeds maximum recursion depth of {} for TokenMatchIterator\n\n{:?}",
            MAX_DEPTH,
            self.root_token_match
        );
        &self.cursor[..self.cursor_depth]
    }
}

impl<'a, 'b> Iterator for TokenMatchIterator<'a, 'b> {
    type Item = &'b TokenMatch<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.cursor_depth == 0 {
                if self.has_returned_root {
                    return None;
                } else {
                    self.has_returned_root = true;
                    if has_children(self.root_token_match) {
                        self.cursor_depth += 1;
                    }
                    return Some(self.root_token_match);
                }
            } else if let Some(token_match) = get_token_index(self.root_token_match, self.cursor())
            {
                if has_children(token_match) {
                    self.cursor_depth += 1;
                } else {
                    self.cursor[self.cursor_depth - 1] += 1;
                }

                return Some(token_match);
            } else {
                self.cursor_depth -= 1;
                self.cursor[self.cursor_depth] = 0;
                if let Some(i) = self.cursor_depth.checked_sub(1) {
                    self.cursor[i] += 1;
                }
            }
        }
    }
}

fn has_children(token_match: &TokenMatch<'_>) -> bool {
    match &token_match.match_meta {
        MatchMeta::Single(_) => true,
        MatchMeta::Sequence(v) => !v.is_empty(),
        MatchMeta::None | MatchMeta::Phrase(_) | MatchMeta::Record(_) => false,
    }
}

fn get_token_index<'a, 'b>(
    token_match: &'b TokenMatch<'a>,
    cursor: &[usize],
) -> Option<&'b TokenMatch<'a>> {
    if cursor.is_empty() {
        Some(token_match)
    } else {
        match &token_match.match_meta {
            MatchMeta::Sequence(v) => v
                .get(cursor[0])
                .and_then(|t| get_token_index(t, &cursor[1..])),
            MatchMeta::Single(t) if cursor[0] == 0 => get_token_index(t, &cursor[1..]),
            MatchMeta::None
            | MatchMeta::Phrase(_)
            | MatchMeta::Record(_)
            | MatchMeta::Single(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::storage::{Record, RecordStatus};
    use crate::test_utils as test;

    use futures::StreamExt;

    #[test]
    fn token_match_iterator_test_no_meta() {
        let token = keyword("badger");
        let token_match = TokenMatch::from(&token);

        assert_eq!(
            vec![&token_match],
            TokenMatchIterator::new(&token_match).collect::<Vec<_>>()
        );
    }

    #[test]
    fn token_match_iterator_test_phrase() {
        let token = any_word();
        let token_match = TokenMatch::new(&token, "abc");

        assert_eq!(
            vec![&token_match],
            TokenMatchIterator::new(&token_match).collect::<Vec<_>>()
        );
    }

    #[test]
    fn token_match_iterator_test_record() {
        let token = name();
        let token_match = TokenMatch::new(
            &token,
            Record {
                status: RecordStatus::Unsaved,
                thing: test::thing::odysseus(),
            },
        );

        let mut iter = TokenMatchIterator::new(&token_match);

        assert_eq!(Some(&token_match), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn token_match_iterator_test_single() {
        let child_token = keyword("badger");
        let root_token = optional(child_token.clone());
        let token_match = TokenMatch::new(&root_token, TokenMatch::from(&child_token));

        let mut iter = TokenMatchIterator::new(&token_match);

        assert_eq!(Some(&token_match), iter.next());
        assert_eq!(Some(&TokenMatch::from(&child_token)), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn token_match_iterator_test_sequence() {
        let child_tokens = [keyword("badger"), keyword("mushroom"), keyword("snake")];
        let root_token = sequence(child_tokens.clone());
        let token_match = TokenMatch::new(
            &root_token,
            child_tokens
                .iter()
                .map(TokenMatch::from)
                .collect::<Vec<_>>(),
        );

        let mut iter = TokenMatchIterator::new(&token_match);

        assert_eq!(Some(&token_match), iter.next());
        assert_eq!(Some(&TokenMatch::from(&child_tokens[0])), iter.next());
        assert_eq!(Some(&TokenMatch::from(&child_tokens[1])), iter.next());
        assert_eq!(Some(&TokenMatch::from(&child_tokens[2])), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn token_match_iterator_test_recursive() {
        let grandchild_token = any_word();
        let child_token = optional(grandchild_token.clone());
        let root_token = optional(child_token.clone());

        let token_match = TokenMatch::new(
            &root_token,
            TokenMatch::new(&child_token, TokenMatch::from(&grandchild_token)),
        );

        let mut iter = TokenMatchIterator::new(&token_match);

        assert_eq!(Some(&token_match), iter.next());
        assert_eq!(
            Some(&TokenMatch::new(
                &child_token,
                TokenMatch::from(&grandchild_token)
            )),
            iter.next()
        );
        assert_eq!(Some(&TokenMatch::from(&grandchild_token)), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }

    #[tokio::test]
    async fn token_match_iterator_test_nested() {
        let token = sequence([
            sequence([keyword_m(0, "badger"), keyword_m(1, "mushroom")]),
            optional(sequence([keyword_m(2, "snake")])),
        ]);

        let app_meta = test::app_meta();
        let mut stream = token.match_input("badger mushroom snake", &app_meta);
        let token_match = loop {
            match stream.next().await {
                Some(FuzzyMatch::Exact(token_match)) => break token_match,
                Some(_) => {}
                None => panic!(),
            }
        };

        let mut iter = TokenMatchIterator::new(&token_match)
            .filter(|token_match| token_match.token.marker_hash() != 0);

        for i in 0..=2 {
            assert!(iter
                .next()
                .is_some_and(|token_match| token_match.is_marked_with(i)));
        }

        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }
}
