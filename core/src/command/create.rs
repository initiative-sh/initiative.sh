use crate::app::{AppMeta, AutocompleteSuggestion, CommandAlias};
use crate::command::prelude::*;
use crate::storage::{Change, Record, RepositoryError, StorageCommand};
use crate::world::npc::{Age, Ethnicity, Gender, NpcData, Species};
use crate::world::place::{PlaceData, PlaceType};
use crate::world::thing::ThingData;
use crate::world::{Field, WorldCommand};
use initiative_macros::TokenMarker;

#[derive(Clone, Debug)]
pub struct Create;

#[derive(TokenMarker)]
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
            optional(keyword_m(Marker::CreateKeyword, "create")),
            or([
                any_of([
                    keyword_list(["a", "an"]),
                    sequence([
                        keyword_list(["named", "called"]),
                        any_phrase_m(Marker::Name),
                    ]),
                    keyword_list_m(Marker::PlaceType, PlaceType::get_words()),
                ]),
                any_of([
                    keyword_list(["a", "an"]),
                    keyword_list_m(Marker::NpcNoun, ["character", "npc", "person"]),
                    sequence([
                        keyword_list(["named", "called"]),
                        any_phrase_m(Marker::Name),
                    ]),
                    keyword_list_m(Marker::Age, Age::get_words()),
                    keyword_list_m(Marker::Ethnicity, Ethnicity::get_words()),
                    keyword_list_m(Marker::Gender, Gender::get_words()),
                    keyword_list_m(Marker::Species, Species::get_words()),
                ]),
            ]),
        ])
    }

    fn autocomplete<'a>(
        &self,
        fuzzy_match: FuzzyMatch<'a>,
        _input: &str,
    ) -> Option<AutocompleteSuggestion> {
        let token_match = fuzzy_match.token_match();

        let record = {
            let token_sequence = token_match.match_meta.sequence()?;
            let record = token_sequence[1].match_meta.record()?;

            record
        };

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

    fn get_priority(&self, token_match: &TokenMatch) -> Option<CommandPriority> {
        if token_match.contains_marker(Marker::CreateKeyword) {
            Some(CommandPriority::Canonical)
        } else if token_match
            .find_markers(&[Marker::NpcNoun, Marker::Species, Marker::PlaceType])
            .next()
            .is_some()
        {
            Some(CommandPriority::Fuzzy)
        } else {
            None
        }
    }

    async fn run<'a>(
        &self,
        token_match: TokenMatch<'a>,
        app_meta: &mut AppMeta,
    ) -> Result<String, String> {
        let thing_token = token_match
            .find_markers(&[Marker::NpcNoun, Marker::Species, Marker::PlaceType])
            .next()
            .unwrap();

        let original_thing_data: ThingData = if thing_token.is_marked_with(&Marker::PlaceType) {
            let mut place_data = PlaceData::default();

            for marked_token in token_match.find_markers(&[Marker::Name, Marker::PlaceType]) {
                let marker = marked_token.token.marker.unwrap();
                let match_meta = &marked_token.match_meta;

                if marker == Marker::Name {
                    place_data.name = match_meta.phrase().into();
                } else if marker == Marker::PlaceType {
                    place_data.subtype = match_meta.phrase().and_then(|s| s.parse().ok()).into();
                }
            }

            place_data.into()
        } else {
            let mut npc_data = NpcData::default();

            for marked_token in token_match.find_markers(&[
                Marker::Age,
                Marker::Ethnicity,
                Marker::Gender,
                Marker::Name,
                Marker::Species,
            ]) {
                let marker = marked_token.token.marker.unwrap();
                let match_meta = &marked_token.match_meta;

                if marker == Marker::Age {
                    npc_data.age = match_meta.phrase().and_then(|s| s.parse().ok()).into();
                } else if marker == Marker::Ethnicity {
                    npc_data.ethnicity = match_meta.phrase().and_then(|s| s.parse().ok()).into();
                } else if marker == Marker::Gender {
                    npc_data.gender = match_meta.phrase().and_then(|s| s.parse().ok()).into();

                    if let Some(age) = match_meta.phrase().and_then(|s| s.parse().ok()) {
                        // If the gender also implies age *and* the age has not been otherwise
                        // specified, apply the implied age. "Boy" should be young, "old boy"
                        // should be old.
                        npc_data.age.replace(age);
                        npc_data.age.lock();
                    }
                } else if marker == Marker::Name {
                    npc_data.name = match_meta.phrase().into();
                } else if marker == Marker::Species {
                    npc_data.species = match_meta.phrase().and_then(|s| s.parse().ok()).into();
                }
            }

            npc_data.into()
        };

        let mut output = None;

        for _ in 0..10 {
            let mut thing_data = original_thing_data.clone();
            thing_data.regenerate(&mut app_meta.rng, &app_meta.demographics);
            let mut command_alias = None;

            let (message, change) = match thing_data.name() {
                Field::Locked(Some(name)) => {
                    (
                        Some(format!(
                            "\n\n_Because you specified a name, {name} has been automatically added to your `journal`. Use `undo` to remove {them}._",
                            name = name,
                            them = thing_data.gender().them(),
                        )),
                        Change::CreateAndSave { thing_data, uuid: None },
                    )
                }
                Field::Unlocked(Some(name)) => {
                    command_alias = Some(CommandAlias::literal(
                        "save",
                        format!("save {}", name),
                        StorageCommand::Save {
                            name: name.to_string(),
                        }
                        .into(),
                    ));

                    app_meta.command_aliases.insert(CommandAlias::literal(
                        "more",
                        format!("create {}", original_thing_data.display_description()),
                        WorldCommand::CreateMultiple {
                            thing_data: original_thing_data.clone(),
                        }
                        .into(),
                    ));

                    (
                        Some(format!(
                            "\n\n_{name} has not yet been saved. Use ~save~ to save {them} to your `journal`. For more suggestions, type ~more~._",
                            name = name,
                            them = thing_data.gender().them(),
                        )),
                        Change::Create { thing_data, uuid: None },
                    )
                }
                _ => (None, Change::Create { thing_data, uuid: None }),
            };

            match app_meta.repository.modify(change).await {
                Ok(Some(Record { thing, .. })) => {
                    output = Some(format!(
                        "{}{}",
                        thing.display_details(
                            app_meta
                                .repository
                                .load_relations(&thing)
                                .await
                                .unwrap_or_default(),
                        ),
                        message.as_ref().map_or("", String::as_str),
                    ));

                    if let Some(alias) = command_alias {
                        app_meta.command_aliases.insert(alias);
                    }

                    break;
                }

                Err((
                    Change::Create { thing_data, .. } | Change::CreateAndSave { thing_data, .. },
                    RepositoryError::NameAlreadyExists(other_thing),
                )) => if thing_data.name().is_locked() {
                    return Err(format!(
                        "That name is already in use by {}.",
                        other_thing.display_summary(),
                    ));
                },

                Err((Change::Create { thing_data, .. }, RepositoryError::MissingName)) => return Err(format!("There is no name generator implemented for that type. You must specify your own name using `{} named [name]`.", thing_data.display_description())),

                Ok(None) | Err(_) => return Err("An error occurred.".to_string()),
            }
        }

        output.ok_or_else(|| {
            format!(
                "Couldn't create a unique {} name.",
                original_thing_data.display_description(),
            )
        })
    }

    fn get_canonical_form_of(&self, token_match: &TokenMatch) -> Option<String> {
        let thing_token = token_match
            .find_markers(&[Marker::NpcNoun, Marker::Species, Marker::PlaceType])
            .next()
            .unwrap();

        let result = if thing_token.is_marked_with(&Marker::PlaceType) {
            let mut result = PlaceType::try_from(thing_token.match_meta.phrase()?)
                .ok()?
                .to_string();

            if let Some(name) = token_match
                .find_markers(&[Marker::Name])
                .next()
                .and_then(|token| token.match_meta.phrase())
            {
                result.push_str(&format!(" named \"{}\"", name));
            }

            result
        } else {
            let mut result = String::new();

            if let Some(age_str) = token_match
                .find_markers(&[Marker::Age])
                .next()
                .and_then(|token| token.match_meta.phrase())
                .and_then(|s| Age::try_from(s).ok())
                .map(|age| age.as_str())
            {
                result.push_str(age_str);
                result.push(' ');
            }

            if let Some(gender_str) = token_match
                .find_markers(&[Marker::Gender])
                .next()
                .and_then(|token| token.match_meta.phrase())
                .and_then(|s| Gender::try_from(s).ok())
                .map(|gender| gender.as_str())
            {
                result.push_str(gender_str);
                result.push(' ');
            }

            if let Some(ethnicity_str) = token_match
                .find_markers(&[Marker::Ethnicity])
                .next()
                .and_then(|token| token.match_meta.phrase())
                .and_then(|s| Ethnicity::try_from(s).ok())
                .map(|ethnicity| ethnicity.as_str())
            {
                result.push_str(ethnicity_str);
                result.push(' ');
            }

            if let Some(species_str) = token_match
                .find_markers(&[Marker::Species])
                .next()
                .and_then(|token| token.match_meta.phrase())
                .and_then(|s| Species::try_from(s).ok())
                .map(|species| species.as_str())
            {
                result.push_str(species_str);
            } else {
                result.push_str("character");
            }

            if let Some(name) = token_match
                .find_markers(&[Marker::Name])
                .next()
                .and_then(|token| token.match_meta.phrase())
            {
                result.push_str(&format!(" named \"{}\"", name));
            }

            result
        };

        if result.starts_with(&['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']) {
            Some(format!("create an {result}"))
        } else {
            Some(format!("create a {result}"))
        }
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
