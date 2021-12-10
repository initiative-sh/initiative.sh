use super::{Field, Thing};
use crate::app::{AppMeta, CommandAlias, Runnable};
use crate::storage::{Change, RepositoryError, StorageCommand};
use crate::world::syntax::{
    ThingDescription
};
use async_trait::async_trait;
use initiative_macros::{Autocomplete, ContextAwareParse, Display};
use std::ops::Range;

#[derive(Autocomplete, Clone, ContextAwareParse, Debug, Display, PartialEq)]
pub enum WorldCommand {
    #[command(alias = "[description]")]
    Create { description: ThingDescription },
    #[command(ignore)]
    CreateMultiple { thing: Thing },
    /*
    #[command(syntax = "[name] is [description]")]
    EditNpc {
        name: ThingName<Npc, FromAny>,
        description: NpcDescription,
    },
    #[command(syntax = "[name] is [description]")]
    EditPlace {
        name: ThingName<Place, FromAny>,
        description: PlaceDescription,
    },
    */
}

#[async_trait(?Send)]
impl Runnable for WorldCommand {
    async fn run(self, _input: &str, app_meta: &mut AppMeta) -> Result<String, String> {
        match self {
            Self::Create { description } => {
                let (diff, _unknown_words) = description.into_thing_with_unknown_words();
                let mut output = None;

                if let Some(place) = diff.place() {
                    if place.subtype.value().map_or(true, |t| t.as_str() != "inn")
                        && place.name.is_unlocked()
                    {
                        return Err(format!("The only place name generator currently implemented is `inn`. For other types, you must specify a name using `{} named [name]`.", place.display_description()));
                    }
                }

                for _ in 0..10 {
                    let mut thing = diff.clone();
                    thing.regenerate(&mut app_meta.rng, &app_meta.demographics);
                    let mut temp_output = format!(
                        "{}",
                        thing.display_details(
                            app_meta
                                .repository
                                .load_relations(&thing)
                                .await
                                .unwrap_or_default()
                        )
                    );
                    let mut command_alias = None;

                    let change = match thing.name() {
                        Field::Locked(Some(name)) => {
                            temp_output.push_str(&format!(
                                    "\n\n_Because you specified a name, {name} has been automatically added to your `journal`. Use `undo` to remove {them}._",
                                    name = name,
                                    them = thing.gender().them(),
                                ));

                            Change::CreateAndSave { thing }
                        }
                        Field::Unlocked(Some(name)) => {
                            temp_output.push_str(&format!(
                                    "\n\n_{name} has not yet been saved. Use ~save~ to save {them} to your `journal`. For more suggestions, type ~more~._",
                                    name = name,
                                    them = thing.gender().them(),
                                ));

                            command_alias = Some(CommandAlias::literal(
                                "save".to_string(),
                                format!("save {}", name),
                                StorageCommand::Save { name: name.into() }.into(),
                            ));

                            app_meta.command_aliases.insert(CommandAlias::literal(
                                "more".to_string(),
                                format!("create {}", diff.display_description()),
                                WorldCommand::CreateMultiple {
                                    thing: diff.clone(),
                                }
                                .into(),
                            ));

                            Change::Create { thing }
                        }
                        _ => Change::Create { thing },
                    };

                    match app_meta.repository.modify(change).await {
                        Ok(_) => {
                            output = Some(temp_output);

                            if let Some(alias) = command_alias {
                                app_meta.command_aliases.insert(alias);
                            }

                            break;
                        }
                        Err((Change::Create { thing }, RepositoryError::NameAlreadyExists))
                        | Err((
                            Change::CreateAndSave { thing },
                            RepositoryError::NameAlreadyExists,
                        )) => {
                            if thing.name().is_locked() {
                                if let Ok(other_thing) = app_meta
                                    .repository
                                    .get_by_name(thing.name().value().unwrap())
                                    .await
                                {
                                    return Err(format!(
                                        "That name is already in use by {}.",
                                        other_thing.display_summary(),
                                    ));
                                } else {
                                    return Err("That name is already in use.".to_string());
                                }
                            }
                        }
                        Err(_) => return Err("An error occurred.".to_string()),
                    }
                }

                if let Some(output) = output {
                    Ok(output)
                    //Ok(append_unknown_words_notice(output, input, unknown_words))
                } else {
                    Err(format!(
                        "Couldn't create a unique {} name.",
                        diff.display_description(),
                    ))
                }
            }
            Self::CreateMultiple { thing } => {
                let mut output = format!(
                    "# Alternative suggestions for \"{}\"",
                    thing.display_description(),
                );

                for i in 1..=10 {
                    let mut thing_output = None;

                    for _ in 0..10 {
                        let mut thing = thing.clone();
                        thing.regenerate(&mut app_meta.rng, &app_meta.demographics);
                        let temp_thing_output = format!(
                            "{}~{}~ {}",
                            if i == 1 { "\n\n" } else { "\\\n" },
                            i % 10,
                            thing.display_summary(),
                        );
                        let command_alias = CommandAlias::literal(
                            (i % 10).to_string(),
                            format!("load {}", thing.name()),
                            StorageCommand::Load {
                                name: thing.name().value().and_then(|s| s.parse().ok()).unwrap(),
                            }
                            .into(),
                        );

                        match app_meta.repository.modify(Change::Create { thing }).await {
                            Ok(_) => {
                                app_meta.command_aliases.insert(command_alias);
                                thing_output = Some(temp_thing_output);
                                break;
                            }
                            Err((_, RepositoryError::NameAlreadyExists)) => {}
                            Err(_) => return Err("An error occurred.".to_string()),
                        }
                    }

                    if let Some(thing_output) = thing_output {
                        output.push_str(&thing_output);
                    } else {
                        output.push_str("\n\n! An error occurred generating additional results.");
                        break;
                    }
                }

                app_meta.command_aliases.insert(CommandAlias::literal(
                    "more".to_string(),
                    format!("create {}", thing.display_description()),
                    Self::CreateMultiple { thing }.into(),
                ));

                output.push_str("\n\n_For even more suggestions, type ~more~._");

                Ok(output)
            }
            /*
            Self::EditNpc { name, description } => {
                match app_meta.repository.modify(Change::Edit {
                          name: name.into(),
                          uuid: None,
                          diff: ThingDescription::from(description).into_thing(),
                      }).await {
                      Ok(Some(thing)) if matches!(app_meta.repository.undo_history().next(), Some(Change::EditAndUnsave { .. })) => Ok(format!(
                          "{}\n\n_{} was successfully edited and automatically saved to your `journal`. Use `undo` to reverse this._",
                          thing.display_details(app_meta.repository.load_relations(&thing).await.unwrap_or_default()),
                          name,
                      )),
                      Ok(Some(thing)) => Ok(format!(
                          "{}\n\n_{} was successfully edited. Use `undo` to reverse this._",
                          thing.display_details(app_meta.repository.load_relations(&thing).await.unwrap_or_default()),
                          name,
                      )),
                      Err((_, RepositoryError::NotFound)) => Err(format!(r#"There is no character named "{}"."#, name)),
                      _ => Err(format!("Couldn't edit `{}`.", name)),
                  }
            }
            */
        }
    }
}

fn append_unknown_words_notice(
    mut output: String,
    input: &str,
    mut unknown_words: Vec<Range<usize>>,
) -> String {
    if !unknown_words.is_empty() {
        output.push_str(
            "\n\n! initiative.sh doesn't know some of those words, but it did its best.\n\n\\> ",
        );

        {
            let mut pos = 0;
            for word_range in unknown_words.iter() {
                output.push_str(&input[pos..word_range.start]);
                pos = word_range.end;
                output.push_str("**");
                output.push_str(&input[word_range.clone()]);
                output.push_str("**");
            }
            output.push_str(&input[pos..]);
        }

        output.push_str("\\\n\u{a0}\u{a0}");

        {
            let mut words = unknown_words.drain(..);
            let mut unknown_word = words.next();
            for (i, _) in input.char_indices() {
                if unknown_word.as_ref().map_or(false, |word| i >= word.end) {
                    unknown_word = words.next();
                }

                if let Some(word) = &unknown_word {
                    output.push(if i >= word.start { '^' } else { '\u{a0}' });
                } else {
                    break;
                }
            }
        }

        output.push_str("\\\nWant to help improve its vocabulary? Join us [on Discord](https://discord.gg/ZrqJPpxXVZ) and suggest your new words!");
    }
    output
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::app::{assert_autocomplete, Autocomplete, ContextAwareParse};
    use crate::storage::NullDataStore;
    use crate::world::npc::{Npc, Age, Gender, Species};
    use crate::world::syntax::{NpcDescription, NpcTerm, PlaceDescription, PlaceTerm, ThingDescription};
    use crate::Event;
    use tokio_test::block_on;

    #[test]
    fn parse_input_test() {
        let mut app_meta = app_meta();

        assert_eq!(
            (None, vec![create(NpcDescription::default())]),
            block_on(WorldCommand::parse_input("npc", &app_meta)),
        );

        assert_eq!(
            (Some(create(NpcDescription::default())), Vec::new()),
            block_on(WorldCommand::parse_input("create npc", &app_meta)),
        );

        assert_eq!(
            (
                None,
                vec![create(NpcDescription::from_iter(
                    [NpcTerm::Species {
                        species: Species::Elf,
                    }]
                    .into_iter(),
                ))],
            ),
            block_on(WorldCommand::parse_input("elf", &app_meta)),
        );

        assert_eq!(
            (None, Vec::<WorldCommand>::new()),
            block_on(WorldCommand::parse_input("potato", &app_meta)),
        );

        /*
        {
            block_on(
                app_meta.repository.modify(Change::Create {
                    thing: Npc {
                        name: "Spot".into(),
                        ..Default::default()
                    }
                    .into(),
                }),
            )
            .unwrap();

            assert_eq!(
                (
                    None,
                    vec![WorldCommand::Edit {
                        name: "Spot".into(),
                        diff: ParsedThing {
                            thing: Npc {
                                age: Age::Child.into(),
                                gender: Gender::Masculine.into(),
                                ..Default::default()
                            }
                            .into(),
                            unknown_words: vec![10..14],
                            word_count: 2,
                        },
                    }],
                ),
                block_on(WorldCommand::parse_input("Spot is a good boy", &app_meta)),
            );
        }
        */
    }

    #[test]
    fn autocomplete_test() {
        let mut app_meta = app_meta();

        block_on(
            app_meta.repository.modify(Change::Create {
                thing: Npc {
                    name: "Potato Johnson".into(),
                    species: Species::Elf.into(),
                    gender: Gender::NonBinaryThey.into(),
                    age: Age::Adult.into(),
                    ..Default::default()
                }
                .into(),
            }),
        )
        .unwrap();

        vec![
            ("npc", "create person"),
            // Species
            ("dragonborn", "create dragonborn"),
            ("dwarf", "create dwarf"),
            ("elf", "create elf"),
            ("gnome", "create gnome"),
            ("half-elf", "create half-elf"),
            ("half-orc", "create half-orc"),
            ("halfling", "create halfling"),
            ("human", "create human"),
            ("tiefling", "create tiefling"),
            // PlaceType
            ("inn", "create inn"),
        ]
        .drain(..)
        .for_each(|(word, summary)| {
            assert_eq!(
                vec![(word.into(), summary.into())],
                block_on(WorldCommand::autocomplete(word, &app_meta, true)),
            );

            assert_eq!(
                vec![(word.into(), summary.into())],
                block_on(WorldCommand::autocomplete(
                    &word.to_uppercase(),
                    &app_meta,
                    true,
                )),
            );
        });

        assert_autocomplete(
            &[
                ("baby", "create infant"),
                ("bakery", "create bakery"),
                ("bank", "create bank"),
                ("bar", "create bar"),
                ("barony", "create barony"),
                ("barracks", "create barracks"),
                ("barrens", "create barrens"),
                ("base", "create base"),
                ("bathhouse", "create bathhouse"),
                ("beach", "create beach"),
                ("blacksmith", "create blacksmith"),
                ("boy", "create child, he/him"),
                ("brewery", "create brewery"),
                ("bridge", "create bridge"),
                ("building", "create building"),
                ("business", "create business"),
            ][..],
            block_on(WorldCommand::autocomplete("b", &app_meta, true)),
        );

        assert_autocomplete(
            &[(
                "Potato Johnson is [character description]",
                "edit character",
            )][..],
            block_on(WorldCommand::autocomplete(
                "Potato Johnson",
                &app_meta,
                true,
            )),
        );

        assert_autocomplete(
            &[(
                "Potato Johnson is a [character description]",
                "edit character",
            )][..],
            block_on(WorldCommand::autocomplete(
                "Potato Johnson is a ",
                &app_meta,
                true,
            )),
        );

        assert_autocomplete(
            &[
                ("Potato Johnson is an elderly", "edit character"),
                ("Potato Johnson is an elf", "edit character"),
                ("Potato Johnson is an elvish", "edit character"),
                ("Potato Johnson is an enby", "edit character"),
            ][..],
            block_on(WorldCommand::autocomplete(
                "Potato Johnson is an e",
                &app_meta,
                true,
            )),
        );
    }

    #[test]
    fn display_test() {
        let app_meta = app_meta();

        vec![
            create(PlaceDescription::from_iter(
                [PlaceTerm::Subtype {
                    subtype: "inn".parse().unwrap(),
                }]
                .into_iter(),
            )),
            create(NpcDescription::default()),
            create(NpcDescription::from_iter(
                [NpcTerm::Species {
                    species: Species::Elf,
                }]
                .into_iter(),
            )),
        ]
        .drain(..)
        .for_each(|command| {
            let command_string = command.to_string();
            assert_ne!("", command_string);

            assert_eq!(
                (Some(command.clone()), Vec::new()),
                block_on(WorldCommand::parse_input(&command_string, &app_meta)),
                "{}",
                command_string,
            );

            assert_eq!(
                (Some(command), Vec::new()),
                block_on(WorldCommand::parse_input(
                    &command_string.to_uppercase(),
                    &app_meta
                )),
                "{}",
                command_string.to_uppercase(),
            );
        });
    }

    fn create(description: impl Into<ThingDescription>) -> WorldCommand {
        WorldCommand::Create {
            description: description.into(),
        }
    }

    fn event_dispatcher(_event: Event) {}

    fn app_meta() -> AppMeta {
        AppMeta::new(NullDataStore::default(), &event_dispatcher)
    }
}
