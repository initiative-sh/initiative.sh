use crate::app::{AppMeta, AutocompleteSuggestion};
use crate::command::prelude::*;
use crate::storage::{Change, Record};
use initiative_macros::TokenMarker;

#[derive(Clone, Debug)]
pub struct Save;

#[derive(TokenMarker)]
enum Marker {
    Name,
    NotFound,
}

impl Command for Save {
    fn token(&self) -> Token {
        sequence([
            keyword("save"),
            or([name_m(Marker::Name), any_phrase_m(Marker::NotFound)]),
        ])
    }

    fn autocomplete(
        &self,
        fuzzy_match: FuzzyMatch,
        _input: &str,
    ) -> Option<AutocompleteSuggestion> {
        let record = fuzzy_match
            .token_match()
            .find_marker(Marker::Name)?
            .match_meta
            .record()?;

        if record.is_saved() {
            None
        } else {
            Some(
                (
                    format!("save {}", record.thing.name()),
                    format!("save {} to journal", record.thing.as_str()),
                )
                    .into(),
            )
        }
    }

    fn get_priority(&self, _token_match: &TokenMatch) -> Option<CommandPriority> {
        Some(CommandPriority::Canonical)
    }

    fn get_canonical_form_of(&self, token_match: &TokenMatch) -> Option<String> {
        if let MatchMeta::Record(record) = &token_match.find_marker(Marker::Name)?.match_meta {
            Some(format!(r#"save "{}""#, record.thing.name()))
        } else {
            None
        }
    }

    async fn run(
        &self,
        token_match: TokenMatch<'_>,
        app_meta: &mut AppMeta,
    ) -> Result<String, String> {
        if let Some(record) = token_match
            .find_marker(Marker::Name)
            .and_then(|m| m.match_meta.record())
        {
            if record.is_saved() {
                Err(format!(
                    "{} has already been saved to your journal.",
                    record.thing.display_summary()
                ))
            } else {
                let Record { thing, .. } = record;

                app_meta
                    .repository
                    .modify(Change::Save {
                        name: thing.name().to_string(),
                        uuid: Some(thing.uuid),
                    })
                    .await
                    .map(|record| {
                        format!(
                            "{} was successfully saved. Use `undo` to reverse this.",
                            record.unwrap().thing.display_summary(),
                        )
                    })
                    .map_err(|_| format!("Couldn't save `{}`.", thing.display_summary()))
            }
        } else {
            let s = token_match
                .find_marker(Marker::NotFound)
                .and_then(|m| m.match_meta.phrase())
                .unwrap();

            Err(format!(r#"There is no entity named "{s}"."#))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::command::autocomplete;
    use crate::storage::Change;
    use crate::test_utils as test;

    #[tokio::test]
    async fn autocomplete_test() {
        let mut app_meta = test::app_meta::with_test_data().await;

        test::assert_autocomplete_eq!(
            [
                ("save Polyphemus", "save character to journal"),
                ("save Pylos", "save place to journal"),
            ],
            autocomplete("save p", &app_meta).await,
        );

        assert_eq!(
            vec![AutocompleteSuggestion::new(
                "save Odysseus",
                "save character to journal"
            )],
            autocomplete("SAVE ODYSSEUS", &app_meta).await,
        );
    }
}
