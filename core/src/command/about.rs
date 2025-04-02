use crate::command::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct About;

impl Command for About {
    fn token<'a>(&self) -> Token {
        keyword("about")
    }

    fn autocomplete(
        &self,
        _fuzzy_match: FuzzyMatch,
        _input: &str,
    ) -> Option<AutocompleteSuggestion> {
        Some(("about", "about initiative.sh").into())
    }

    fn get_priority(&self, _token_match: &TokenMatch) -> Option<CommandPriority> {
        Some(CommandPriority::Canonical)
    }

    fn get_canonical_form_of(&self, _token_match: &TokenMatch) -> Option<String> {
        Some("about".to_string())
    }

    async fn run(
        &self,
        _token_match: TokenMatch<'_>,
        _app_meta: &mut AppMeta,
    ) -> Result<String, String> {
        Ok(include_str!("../../../data/about.md")
            .trim_end()
            .to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test_utils as test;
    use futures::StreamExt as _;

    #[tokio::test]
    async fn run_test() {
        assert!(crate::command::run("about", &mut test::app_meta())
            .await
            .unwrap()
            .contains("About initiative.sh"));
    }

    #[tokio::test]
    async fn autocomplete_test() {
        test::assert_autocomplete_eq!(
            [("about", "about initiative.sh")],
            About
                .parse_autocomplete("a", &test::app_meta())
                .collect()
                .await,
        );
    }
}
