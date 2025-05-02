use crate::app::{AppMeta, AutocompleteSuggestion};
use crate::command::prelude::*;
use crate::storage::{Change, Record};
use crate::world::thing::Thing;

use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Save;

#[derive(Debug, Eq, Hash, PartialEq)]
enum Marker {
    Name,
    NotFound,
}

struct SaveSuccess {
    thing: Thing,
}

enum SaveError {
    Generic { name: Option<String> },
    AlreadySaved { name: String },
    NotFound { name: String },
}

impl std::fmt::Display for SaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SaveError::Generic { name: None } => write!(f, "Couldn't save."),
            SaveError::Generic { name: Some(name) } => write!(f, "Couldn't save {name}."),
            SaveError::AlreadySaved { name } => write!(f, "{name} is already in the journal."),
            SaveError::NotFound { name } => write!(f, "There is no entity named {name}."),
        }
    }
}

impl From<Record> for SaveSuccess {
    fn from(value: Record) -> Self {
        Self { thing: value.thing }
    }
}

impl std::fmt::Display for SaveSuccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} was successfully saved. Use `undo` to reverse this.",
            self.thing.display_summary()
        )
    }
}

impl Command for Save {
    fn token(&self) -> Token {
        sequence([
            keyword("save"),
            or([
                name().with_marker(Marker::Name),
                any_phrase().with_marker(Marker::NotFound),
            ]),
        ])
    }

    fn autocomplete(
        &self,
        fuzzy_match_list: FuzzyMatchList,
        _input: &str,
    ) -> Option<AutocompleteSuggestion> {
        if let Some(name_token) = fuzzy_match_list.find_marker(&Marker::Name) {
            let record = name_token.record()?;

            if record.is_saved() {
                None
            } else {
                Some(
                    (
                        fuzzy_match_list.autocomplete()?,
                        format!("save {} to journal", record.thing.as_str()),
                    )
                        .into(),
                )
            }
        } else if !fuzzy_match_list.contains_marker(&Marker::NotFound) {
            Some((fuzzy_match_list.autocomplete()?, "save an entry to journal").into())
        } else {
            None
        }
    }

    fn get_priority(&self, _match_list: &MatchList) -> Option<CommandPriority> {
        Some(CommandPriority::Canonical)
    }

    fn get_canonical_form_of(&self, match_list: &MatchList) -> Option<String> {
        Some(format!(
            r#"save "{}""#,
            match_list
                .find_marker(&Marker::Name)?
                .record()?
                .thing
                .name(),
        ))
    }

    async fn run(
        &self,
        match_list: MatchList<'_>,
        app_meta: &mut AppMeta,
    ) -> Result<impl std::fmt::Display, impl std::fmt::Display> {
        if let Some(record) = match_list
            .find_marker(&Marker::Name)
            .and_then(|match_part| match_part.record())
        {
            self.run_with_record(record, app_meta).await
        } else {
            Err(SaveError::NotFound {
                name: match_list
                    .find_marker(&Marker::NotFound)
                    .unwrap()
                    .input()
                    .to_string(),
            })
        }
    }
}

impl Save {
    async fn run_with_record(
        &self,
        record: &Record,
        app_meta: &mut AppMeta,
    ) -> Result<SaveSuccess, SaveError> {
        if record.is_saved() {
            Err(SaveError::AlreadySaved {
                name: record.thing.name().to_string(),
            })
        } else {
            let Record { thing, .. } = record;

            app_meta
                .repository
                .modify(Change::Save {
                    name: thing.name().to_string(),
                    uuid: Some(thing.uuid),
                })
                .await
                .map(|record| SaveSuccess::from(record.unwrap()))
                .map_err(|_| SaveError::Generic {
                    name: Some(thing.name().to_string()),
                })
        }
    }

    pub async fn run_with_uuid(
        &self,
        uuid: &Uuid,
        app_meta: &mut AppMeta,
    ) -> Result<impl std::fmt::Display, impl std::fmt::Display> {
        match app_meta.repository.get_by_uuid(uuid).await {
            Ok(record) => self.run_with_record(&record, app_meta).await,
            Err(_) => Err(SaveError::Generic { name: None }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::command::autocomplete;
    use crate::test_utils as test;

    #[tokio::test]
    async fn autocomplete_test_partial() {
        let app_meta = test::app_meta::with_test_data().await;

        test::assert_autocomplete_eq!(
            [
                ("save polyphemus", "save character to journal"),
                ("save pylos", "save place to journal"),
            ],
            autocomplete("save p", &app_meta).await,
        );
    }

    #[tokio::test]
    async fn autocomplete_test_exact() {
        let app_meta = test::app_meta::with_test_data().await;

        assert_eq!(
            vec![AutocompleteSuggestion::new(
                "SAVE ODYSSEUS",
                "save character to journal"
            )],
            autocomplete("SAVE ODYSSEUS", &app_meta).await,
        );
    }
}
