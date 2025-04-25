use crate::command::prelude::*;
use crate::storage::Record;
use crate::world::thing::ThingRelations;

use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Load;

#[derive(Debug, Eq, Hash, PartialEq)]
enum Marker {
    Load,
    Name,
    NotFound,
}

struct LoadSuccess {
    record: Record,
    relations: Option<ThingRelations>,
}

enum LoadError {
    Generic,
    NotFound { name: String },
}

impl std::fmt::Display for LoadSuccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.record.thing.display_details(self.relations.as_ref()),
        )?;

        if self.record.is_unsaved() {
            write!(
                f,
                "\n\n_{name} has not yet been saved. Use ~save~ to save {them} to your `journal`._",
                name = self.record.thing.name(),
                them = self.record.thing.gender().them(),
            )?;
        }

        Ok(())
    }
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::Generic => write!(f, "Couldn't load."),
            LoadError::NotFound { name } => write!(f, r#"No matches for "{name}"."#),
        }
    }
}

impl Command for Load {
    fn token<'a>(&self) -> Token {
        sequence([
            optional(keyword("load").with_marker(Marker::Load)),
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
        if let Some(record) = fuzzy_match_list
            .find_marker(&Marker::Name)
            .and_then(MatchPart::record)
        {
            Some(
                (
                    fuzzy_match_list.autocomplete()?,
                    if record.is_unsaved() {
                        format!("{} (unsaved)", record.thing.display_description())
                    } else {
                        record.thing.display_description().to_string()
                    },
                )
                    .into(),
            )
        } else if !fuzzy_match_list.contains_marker(&Marker::NotFound) {
            Some((fuzzy_match_list.autocomplete()?, "load an entry").into())
        } else {
            None
        }
    }

    fn get_priority(&self, match_list: &MatchList) -> Option<CommandPriority> {
        if match_list.contains_marker(&Marker::Load) {
            Some(CommandPriority::Canonical)
        } else if match_list.contains_marker(&Marker::Name) {
            Some(CommandPriority::Fuzzy)
        } else {
            None
        }
    }

    fn get_canonical_form_of(&self, match_list: &MatchList) -> Option<String> {
        Some(format!(
            "load {}",
            match_list
                .find_marker(&Marker::Name)?
                .record()?
                .thing
                .name()
        ))
    }

    async fn run(
        &self,
        match_list: MatchList<'_>,
        app_meta: &mut AppMeta,
    ) -> Result<impl std::fmt::Display, impl std::fmt::Display> {
        let (match_part, marker) = match_list
            .find_markers(&[Marker::Name, Marker::NotFound])
            .next()
            .unwrap();

        if marker == &Marker::Name {
            Ok(self
                .load_record(match_part.record().unwrap().clone(), app_meta)
                .await)
        } else {
            Err(LoadError::NotFound {
                name: match_part.term().unwrap().to_string(),
            })
        }
    }
}

impl Load {
    pub async fn run_with_uuid(
        &self,
        uuid: &Uuid,
        app_meta: &mut AppMeta,
    ) -> Result<impl std::fmt::Display, impl std::fmt::Display> {
        match app_meta.repository.get_by_uuid(uuid).await {
            Ok(record) => Ok(self.load_record(record, app_meta).await),
            Err(_) => Err(LoadError::Generic),
        }
    }

    async fn load_record(&self, record: Record, app_meta: &mut AppMeta) -> LoadSuccess {
        if record.is_unsaved() {
            app_meta.command_aliases_new.insert(Alias::new(
                keyword("save"),
                format!("save {}", record.thing.name()),
                AliasCommand::Save {
                    uuid: record.thing.uuid,
                },
            ));
        }
        LoadSuccess {
            relations: app_meta.repository.load_relations(&record.thing).await.ok(),
            record,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::command;
    use crate::test_utils as test;
    use futures::StreamExt as _;

    #[tokio::test]
    async fn run_test() {
        assert!(
            command::run("load odysseus", &mut test::app_meta::with_test_data().await)
                .await
                .unwrap()
                .contains("Load initiative.sh")
        );
    }

    #[tokio::test]
    async fn autocomplete_test() {
        test::assert_autocomplete_eq!(
            [("load", "load an entry")],
            Load.parse_autocomplete("l", &test::app_meta())
                .collect()
                .await,
        );

        test::assert_autocomplete_eq!(
            [("load [name]", "load an entry")],
            Load.parse_autocomplete("load", &test::app_meta())
                .collect()
                .await,
        );

        test::assert_autocomplete_eq!(
            [("load odysseus", "middle-aged human, he/him (unsaved)")],
            Load.parse_autocomplete("load o", &test::app_meta::with_test_data().await)
                .collect()
                .await,
        );
    }
}
