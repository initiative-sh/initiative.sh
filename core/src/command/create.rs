use crate::app::{AppMeta, AutocompleteSuggestion};
use crate::command::prelude::*;
use crate::storage::{Change, Record, RepositoryError};
use crate::world::npc::{Age, Ethnicity, Gender, NpcData, Species};
use crate::world::place::{PlaceData, PlaceType};
use crate::world::thing::{Thing, ThingData, ThingRelations};
use crate::world::Field;

use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Create;

struct CreateSuccess {
    thing: Thing,
    relations: Option<ThingRelations>,
    message: Option<CreateSuccessMessage>,
}

struct CreateMultiSuccess {
    thing_description: String,
    things: [Thing; 10],
}

enum CreateSuccessMessage {
    AutomaticallySaved { name: String, gender: Gender },
    NotYetSaved { name: String, gender: Gender },
}

enum CreateError {
    Generic,
    CouldntCreateUnique { description: String },
    NameAlreadyInUse { by_thing: Thing },
    NameRequired { canonical: Option<String> },
}

impl std::fmt::Display for CreateSuccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.thing.display_details(self.relations.as_ref()))?;

        if let Some(message) = &self.message {
            write!(f, "\n\n{message}")?;
        }

        Ok(())
    }
}

impl std::fmt::Display for CreateSuccessMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AutomaticallySaved { name, gender } => write!(
                f,
                "_Because you specified a name, {name} has been automatically added to your `journal`. Use `undo` to remove {them}._",
                them = gender.them(),
            ),
            Self::NotYetSaved { name, gender } => write!(
                f,
                "_{name} has not yet been saved. Use ~save~ to save {them} to your `journal`. For more suggestions, type ~more~._",
                them = gender.them(),
            ),
        }
    }
}

impl std::fmt::Display for CreateMultiSuccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"# Alternative suggestions for "{}""#,
            self.thing_description
        )?;

        for (i, thing) in self.things.iter().enumerate() {
            write!(
                f,
                "{}~{}~ {}",
                if i == 0 { "\n\n" } else { "\\\n" },
                (i + 1) % 10,
                thing.display_summary(),
            )?;
        }

        Ok(())
    }
}

impl std::fmt::Display for CreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Generic => write!(f, "An error occurred."),
            Self::CouldntCreateUnique { description } => {
                write!(f, "Couldn't create a unique {description} name.")
            }
            Self::NameAlreadyInUse { by_thing } => write!(
                f,
                "That name is already in use by {}.",
                by_thing.display_summary(),
            ),
            Self::NameRequired { canonical } => write!(f, "There is no name generator implemented for that type. You must specify your own name using `{canonical} named [name]`.", canonical = canonical.as_ref().map(|s| s.as_str()).unwrap_or("")),
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
enum Marker {
    Age,
    CreateKeyword,
    Ethnicity,
    Gender,
    Name,
    NpcNoun,
    PlaceType,
    Species,
}

impl Command for Create {
    fn token(&self) -> Token {
        sequence([
            optional(keyword("create").with_marker(Marker::CreateKeyword)),
            or([
                any_of([
                    keyword_list(["a", "an"]),
                    sequence([
                        keyword_list(["named", "called"]),
                        any_phrase().with_marker(Marker::Name),
                    ]),
                    keyword_list(PlaceType::get_words()).with_marker(Marker::PlaceType),
                ]),
                any_of([
                    keyword_list(["a", "an"]),
                    keyword_list(["character", "npc", "person"]).with_marker(Marker::NpcNoun),
                    sequence([
                        keyword_list(["named", "called"]),
                        any_phrase().with_marker(Marker::Name),
                    ]),
                    keyword_list(Age::get_words()).with_marker(Marker::Age),
                    keyword_list(Ethnicity::get_words()).with_marker(Marker::Ethnicity),
                    keyword_list(Gender::get_words()).with_marker(Marker::Gender),
                    keyword_list(Species::get_words()).with_marker(Marker::Species),
                ]),
            ]),
        ])
    }

    fn autocomplete(
        &self,
        _fuzzy_match_list: FuzzyMatchList<'_>,
        _input: &str,
    ) -> Option<AutocompleteSuggestion> {
        todo!();
    }

    fn get_priority(&self, match_list: &MatchList) -> Option<CommandPriority> {
        if match_list.contains_marker(&Marker::CreateKeyword) {
            Some(CommandPriority::Canonical)
        } else if match_list
            .find_markers(&[Marker::NpcNoun, Marker::Species, Marker::PlaceType])
            .next()
            .is_some()
        {
            Some(CommandPriority::Fuzzy)
        } else {
            None
        }
    }

    async fn run(
        &self,
        match_list: MatchList<'_>,
        app_meta: &mut AppMeta,
    ) -> Result<impl std::fmt::Display, impl std::fmt::Display> {
        let thing_data = self.parse_thing_data(&match_list);

        for _ in 0..10 {
            if let Some(result) = self
                .try_generate_thing(&thing_data, &match_list, app_meta)
                .await
            {
                return result;
            }
        }

        Err(CreateError::CouldntCreateUnique {
            description: thing_data.display_description().to_string(),
        })
    }

    fn get_canonical_form_of(&self, match_list: &MatchList) -> Option<String> {
        let (thing_match, marker) = match_list
            .find_markers(&[Marker::NpcNoun, Marker::Species, Marker::PlaceType])
            .next()
            .unwrap();

        let result = if marker == &Marker::PlaceType {
            let mut result = PlaceType::try_from(thing_match.input()).ok()?.to_string();

            if let Some(name_match) = match_list.find_marker(&Marker::Name) {
                result.push_str(&format!(r#" named "{}""#, name_match.input()));
            }

            result
        } else {
            let mut result = String::new();

            if let Some(age_str) = match_list
                .find_marker(&Marker::Age)
                .map(MatchPart::input)
                .and_then(|s| Age::try_from(s).ok())
                .map(|age| age.as_str())
            {
                result.push_str(age_str);
                result.push(' ');
            }

            if let Some(gender_str) = match_list
                .find_marker(&Marker::Gender)
                .map(MatchPart::input)
                .and_then(|s| Gender::try_from(s).ok())
                .map(|gender| gender.as_str())
            {
                result.push_str(gender_str);
                result.push(' ');
            }

            if let Some(ethnicity_str) = match_list
                .find_marker(&Marker::Ethnicity)
                .map(MatchPart::input)
                .and_then(|s| Ethnicity::try_from(s).ok())
                .map(|ethnicity| ethnicity.as_str())
            {
                result.push_str(ethnicity_str);
                result.push(' ');
            }

            if let Some(species_str) = match_list
                .find_marker(&Marker::Species)
                .map(MatchPart::input)
                .and_then(|s| Species::try_from(s).ok())
                .map(|species| species.as_str())
            {
                result.push_str(species_str);
            } else {
                result.push_str("character");
            }

            if let Some(name) = match_list.find_marker(&Marker::Name).map(MatchPart::input) {
                result.push_str(&format!(r#" named "{}""#, name));
            }

            result
        };

        if result.starts_with(['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']) {
            Some(format!("create an {result}"))
        } else {
            Some(format!("create a {result}"))
        }
    }
}

impl Create {
    fn parse_thing_data(&self, match_list: &MatchList) -> ThingData {
        if match_list.contains_marker(&Marker::PlaceType) {
            self.parse_place_data(match_list).into()
        } else {
            self.parse_npc_data(match_list).into()
        }
    }

    fn parse_place_data(&self, match_list: &MatchList) -> PlaceData {
        let mut place_data = PlaceData::default();

        for (match_part, marker) in match_list.find_markers(&[Marker::Name, Marker::PlaceType]) {
            let phrase = match_part.input();

            match marker {
                Marker::Name => place_data.name = phrase.into(),
                Marker::PlaceType => place_data.subtype = phrase.parse().ok().into(),
                _ => unreachable!(),
            }
        }

        place_data
    }

    fn parse_npc_data(&self, match_list: &MatchList) -> NpcData {
        let mut npc_data = NpcData::default();

        for (match_part, marker) in match_list.find_markers(&[
            Marker::Age,
            Marker::Ethnicity,
            Marker::Gender,
            Marker::Name,
            Marker::Species,
        ]) {
            let phrase = match_part.input();

            match marker {
                Marker::Age => npc_data.age = phrase.parse().ok().into(),
                Marker::Ethnicity => npc_data.ethnicity = phrase.parse().ok().into(),
                Marker::Gender => {
                    npc_data.gender = phrase.parse().ok().into();

                    if let Ok(age) = phrase.parse() {
                        // If the gender also implies age *and* the age has not been otherwise
                        // specified, apply the implied age. "Boy" should be young, "old boy"
                        // should be old.
                        npc_data.age.replace(age);
                        npc_data.age.lock();
                    }
                }
                Marker::Name => npc_data.name = phrase.into(),
                Marker::Species => npc_data.species = phrase.parse().ok().into(),
                _ => unreachable!(),
            }
        }

        npc_data
    }

    async fn try_generate_thing(
        &self,
        original_thing_data: &ThingData,
        match_list: &MatchList<'_>,
        app_meta: &mut AppMeta,
    ) -> Option<Result<CreateSuccess, CreateError>> {
        let mut thing_data = original_thing_data.clone();
        thing_data.regenerate(&mut app_meta.rng, &app_meta.demographics);

        let (message, change) = match thing_data.name() {
            Field::Locked(Some(name)) => (
                Some(CreateSuccessMessage::AutomaticallySaved {
                    name: name.clone(),
                    gender: thing_data.gender(),
                }),
                Change::CreateAndSave {
                    thing_data,
                    uuid: None,
                },
            ),
            Field::Unlocked(Some(name)) => {
                app_meta.command_aliases_new.insert(Alias::new(
                    keyword("more"),
                    "",
                    AliasCommand::CreateMore {
                        thing_data: original_thing_data.clone(),
                    },
                ));

                (
                    Some(CreateSuccessMessage::NotYetSaved {
                        name: name.clone(),
                        gender: thing_data.gender(),
                    }),
                    Change::Create {
                        thing_data,
                        uuid: None,
                    },
                )
            }
            _ => (
                None,
                Change::Create {
                    thing_data,
                    uuid: None,
                },
            ),
        };

        match app_meta.repository.modify(change).await {
            Ok(Some(Record { thing, .. })) => {
                if original_thing_data.name().is_unlocked() {
                    app_meta.command_aliases_new.insert(Alias::new(
                        keyword("save"),
                        format!("save {}", thing.name()),
                        AliasCommand::Save { uuid: thing.uuid },
                    ));
                }

                Some(Ok(CreateSuccess {
                    relations: app_meta.repository.load_relations(&thing).await.ok(),
                    thing,
                    message,
                }))
            }

            Err((
                Change::Create { thing_data, .. } | Change::CreateAndSave { thing_data, .. },
                RepositoryError::NameAlreadyExists(other_thing),
            )) => {
                if thing_data.name().is_locked() {
                    Some(Err(CreateError::NameAlreadyInUse {
                        by_thing: other_thing,
                    }))
                } else {
                    None
                }
            }

            Err((Change::Create { .. }, RepositoryError::MissingName)) => {
                Some(Err(CreateError::NameRequired {
                    canonical: self.get_canonical_form_of(match_list),
                }))
            }

            Ok(None) | Err(_) => Some(Err(CreateError::Generic)),
        }
    }

    pub async fn more(
        &self,
        original_thing_data: &ThingData,
        app_meta: &mut AppMeta,
    ) -> Result<impl std::fmt::Display, impl std::fmt::Display> {
        let mut aliases = HashSet::with_capacity(10);
        let mut things = Vec::with_capacity(10);

        for i in ["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"] {
            for _ in 0..10 {
                let mut thing_data = original_thing_data.clone();
                thing_data.regenerate(&mut app_meta.rng, &app_meta.demographics);

                match app_meta
                    .repository
                    .modify(Change::Create {
                        thing_data,
                        uuid: None,
                    })
                    .await
                {
                    Ok(Some(Record { thing, .. })) => {
                        aliases.insert(Alias::new(
                            keyword(i),
                            format!("load {}", thing.name()),
                            AliasCommand::Load { uuid: thing.uuid },
                        ));

                        things.push(thing);
                    }
                    Ok(None) | Err((_, RepositoryError::NameAlreadyExists(_))) => {} // continue
                    Err(_) => return Err(CreateError::Generic),
                }
            }
        }

        app_meta.command_aliases_new.extend(aliases);

        Ok(CreateMultiSuccess {
            thing_description: original_thing_data.display_description().to_string(),
            things: things.try_into().unwrap(),
        })
    }
}

/*
#[cfg(test)]
mod test {
    use super::super::autocomplete;
    use super::*;

    use crate::app::{AppMeta, Event};
    use crate::storage::{Change, MemoryDataStore};
    use crate::world::npc::NpcData;
    use crate::world::place::PlaceData;

    use uuid::Uuid;

    #[tokio::test]
    async fn autocomplete_test() {
        let things = &[
            NpcData {
                name: "Cohen the Barbarian".into(),
                ..Default::default()
            }
            .into_thing(Uuid::new_v4()),
            PlaceData {
                name: "Copperhead".into(),
                ..Default::default()
            }
            .into_thing(Uuid::new_v4()),
        ];

        let mut app_meta = AppMeta::new(
            [NpcData {
                name: "Cut-Me-Own-Throat Dibbler".into(),
                ..Default::default()
            }
            .into_thing(Uuid::new_v4())]
            .into_iter()
            .collect::<MemoryDataStore>(),
            &event_dispatcher,
        );

        for thing in things {
            app_meta
                .repository
                .modify_without_undo(Change::Create {
                    thing_data: thing.data.clone(),
                    uuid: Some(thing.uuid),
                })
                .await
                .ok();
        }

        assert_eq!(
            vec![
                AutocompleteSuggestion::new(
                    "save Cohen the Barbarian",
                    "save character to journal"
                ),
                AutocompleteSuggestion::new("save Copperhead", "save place to journal"),
            ],
            autocomplete("save c", &app_meta).await,
        );

        assert_eq!(
            vec![AutocompleteSuggestion::new(
                "save Copperhead",
                "save place to journal"
            )],
            autocomplete("SAVE COPPERHEAD", &app_meta).await,
        );
    }

    fn event_dispatcher(_event: Event) {}
}
*/
