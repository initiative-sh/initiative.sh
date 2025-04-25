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
            optional(keyword_m(Marker::Load, "load")),
            or([name_m(Marker::Name), any_phrase_m(Marker::NotFound)]),
        ])
    }

    fn autocomplete(
        &self,
        fuzzy_match: FuzzyMatch,
        _input: &str,
    ) -> Option<AutocompleteSuggestion> {
        let token_match = fuzzy_match.token_match();

        if let Some(record) = token_match
            .find_marker(Marker::Name)
            .and_then(TokenMatch::meta_record)
        {
            Some(
                (
                    if token_match.contains_marker(Marker::Load) {
                        format!("load {}", record.thing.name())
                    } else {
                        record.thing.name().to_string()
                    },
                    if record.is_saved() {
                        record.thing.display_description().to_string()
                    } else {
                        format!("{} (unsaved)", record.thing.display_description())
                    },
                )
                    .into(),
            )
        } else if token_match.contains_marker(Marker::Load)
            && !token_match.contains_marker(Marker::NotFound)
        {
            Some(("load [name]", "load an entry").into())
        } else {
            None
        }
    }

    fn get_priority(&self, token_match: &TokenMatch) -> Option<CommandPriority> {
        if token_match.contains_marker(Marker::Load) {
            Some(CommandPriority::Canonical)
        } else if token_match.contains_marker(Marker::Name) {
            Some(CommandPriority::Fuzzy)
        } else {
            None
        }
    }

    fn get_canonical_form_of(&self, token_match: &TokenMatch) -> Option<String> {
        token_match
            .find_marker(Marker::Name)
            .map(|name| format!("load {}", name.meta_record().unwrap().thing.name()))
    }

    async fn run(
        &self,
        token_match: TokenMatch<'_>,
        app_meta: &mut AppMeta,
    ) -> Result<impl std::fmt::Display, impl std::fmt::Display> {
        let (name_token_match, marker) = token_match
            .find_markers(&[Marker::Name, Marker::NotFound])
            .next()
            .unwrap();

        if marker == &Marker::Name {
            Ok(self
                .load_record(name_token_match.meta_record().unwrap().clone(), app_meta)
                .await)
        } else {
            Err(LoadError::NotFound {
                name: name_token_match.meta_phrase().unwrap().to_string(),
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
            [("load [name]", "load an entry")],
            Load.parse_autocomplete("l", &test::app_meta()).collect().await,
        );

        test::assert_autocomplete_eq!(
            [("load [name]", "load an entry")],
            Load.parse_autocomplete("load ", &test::app_meta()).collect().await,
        );

        test::assert_autocomplete_eq!(
            [("load odysseus", "load an entry")],
            Load.parse_autocomplete("load o", &test::app_meta()).collect().await,
        );
    }
}
