use super::{Field, Npc, Place, Thing};
use crate::app::{AppMeta, Autocomplete, CommandAlias, ContextAwareParse, Runnable};
use crate::storage::{Change, RepositoryError, StorageCommand};
use crate::utils::quoted_words;
use async_trait::async_trait;
use std::fmt;
use std::ops::Range;

mod autocomplete;
mod parse;

#[derive(Clone, Debug, PartialEq)]
pub enum WorldCommand {
    Create {
        thing: ParsedThing<Thing>,
    },
    Edit {
        name: String,
        diff: ParsedThing<Thing>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParsedThing<T> {
    pub thing: T,
    pub unknown_words: Vec<Range<usize>>,
    pub word_count: usize,
}

#[async_trait(?Send)]
impl Runnable for WorldCommand {
    async fn run(self, input: &str, app_meta: &mut AppMeta) -> Result<String, String> {
        match self {
            Self::Create {
                thing: parsed_thing,
            } => {
                let thing = parsed_thing.thing;
                let mut thing_output = None;

                for _ in 0..10 {
                    let mut thing = thing.clone();
                    thing.regenerate(&mut app_meta.rng, &app_meta.demographics);
                    let mut temp_thing_output = format!("{}", thing.display_details());
                    let mut command_alias = None;

                    let change = if app_meta.repository.data_store_enabled() {
                        match thing.name() {
                            Field::Locked(Some(name)) => {
                                temp_thing_output.push_str(&format!(
                                    "\n\n_Because you specified a name, {name} has been automatically added to your `journal`. Use `undo` to remove {them}._",
                                    name = name,
                                    them = thing.gender().them(),
                                ));

                                Change::CreateAndSave { thing }
                            }
                            Field::Unlocked(Some(name)) => {
                                temp_thing_output.push_str(&format!(
                                    "\n\n_{name} has not yet been saved. Use ~save~ to save {them} to your `journal`._",
                                    name = name,
                                    them = thing.gender().them(),
                                ));

                                command_alias = Some(CommandAlias::literal(
                                    "save".to_string(),
                                    format!("save {}", name),
                                    StorageCommand::Change {
                                        change: Change::Save {
                                            name: name.to_string(),
                                        },
                                    }
                                    .into(),
                                ));

                                Change::Create { thing }
                            }
                            _ => Change::Create { thing },
                        }
                    } else {
                        Change::Create { thing }
                    };

                    match app_meta.repository.modify(change).await {
                        Ok(_) => {
                            thing_output = Some(temp_thing_output);

                            if let Some(command_alias) = command_alias {
                                app_meta.command_aliases.insert(command_alias);
                            }

                            break;
                        }
                        Err((Change::Create { thing }, RepositoryError::NameAlreadyExists))
                        | Err((
                            Change::CreateAndSave { thing },
                            RepositoryError::NameAlreadyExists,
                        )) => {
                            if thing.name().is_locked() {
                                if let Some(other_thing) = app_meta
                                    .repository
                                    .load(&thing.name().value().unwrap().into())
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

                let mut output = if let Some(thing_output) = thing_output {
                    thing_output
                } else {
                    return Err(format!(
                        "Couldn't create a unique {} name.",
                        thing.display_description(),
                    ));
                };

                if thing.name().is_none() {
                    for i in 1..=10 {
                        let mut thing_output = None;

                        for _ in 0..10 {
                            let mut thing = thing.clone();
                            thing.regenerate(&mut app_meta.rng, &app_meta.demographics);
                            let temp_thing_output =
                                format!("\\\n~{}~ {}", i % 10, thing.display_summary());
                            let command_alias = CommandAlias::literal(
                                (i % 10).to_string(),
                                format!("load {}", thing.name()),
                                StorageCommand::Load {
                                    name: thing.name().to_string(),
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
                            if i == 1 {
                                output.push_str("\n\n*Alternatives:* ");
                            }

                            output.push_str(&thing_output);
                        } else {
                            output
                                .push_str("\n\n! An error occurred generating additional results.");
                            break;
                        }
                    }
                }

                if !parsed_thing.unknown_words.is_empty() {
                    output.push_str("\n\n! initiative.sh doesn't know some of those words, but it did its best.\n\n\\> ");

                    {
                        let mut pos = 0;
                        for word_range in parsed_thing.unknown_words.iter() {
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
                        let mut pos = 0;
                        for word_range in parsed_thing.unknown_words {
                            (pos..word_range.start).for_each(|_| output.push('\u{a0}'));
                            pos = word_range.end;
                            word_range.for_each(|_| output.push('^'));
                        }
                    }

                    output.push_str("\\\nWant to help improve its vocabulary? Join us [on Discord](https://discord.gg/ZrqJPpxXVZ) and suggest your new words!");
                }

                Ok(output)
            }
            Self::Edit { name, diff } => {
                StorageCommand::Change {
                    change: Change::Edit {
                        id: name.as_str().into(),
                        name,
                        diff: diff.thing,
                    },
                }
                .run(input, app_meta)
                .await
            }
        }
    }
}

impl ContextAwareParse for WorldCommand {
    fn parse_input(input: &str, app_meta: &AppMeta) -> (Option<Self>, Vec<Self>) {
        let mut exact_match = None;
        let mut fuzzy_matches = Vec::new();

        if let Some(Ok(thing)) = input
            .strip_prefix("create ")
            .map(|s| s.parse::<ParsedThing<Thing>>())
        {
            if thing.unknown_words.is_empty() {
                exact_match = Some(Self::Create { thing });
            } else {
                fuzzy_matches.push(Self::Create { thing });
            }
        } else if let Ok(thing) = input.parse::<ParsedThing<Thing>>() {
            fuzzy_matches.push(Self::Create { thing });
        }

        if let Some(word) = quoted_words(input)
            .skip(1)
            .find(|word| word.as_str() == "is")
        {
            let (name, description) = (
                input[..word.range().start].trim(),
                input[word.range().end..].trim(),
            );

            let (diff, thing) = if let Some(thing) = app_meta.repository.load(&name.into()) {
                (
                    match thing {
                        Thing::Npc(_) => description
                            .parse::<ParsedThing<Npc>>()
                            .map(|npc| npc.into_thing()),
                        Thing::Place(_) => description
                            .parse::<ParsedThing<Place>>()
                            .map(|npc| npc.into_thing()),
                        Thing::Region(_) => unimplemented!(),
                    }
                    .or_else(|_| description.parse()),
                    Some(thing),
                )
            } else {
                // This will be an error when we try to run the command, but for now we'll pretend
                // it's valid so that we can provide a more coherent message.
                (description.parse(), None)
            };

            if let Ok(mut diff) = diff {
                let name = thing
                    .map(|t| t.name().to_string())
                    .unwrap_or_else(|| name.to_string());

                diff.unknown_words.iter_mut().for_each(|range| {
                    *range = range.start + word.range().end + 1..range.end + word.range().end + 1
                });

                fuzzy_matches.push(Self::Edit { name, diff });
            }
        }

        (exact_match, fuzzy_matches)
    }
}

impl Autocomplete for WorldCommand {
    fn autocomplete(input: &str, app_meta: &AppMeta) -> Vec<(String, String)> {
        let mut suggestions = Vec::new();

        suggestions.append(&mut Place::autocomplete(input, app_meta));
        suggestions.append(&mut Npc::autocomplete(input, app_meta));

        let mut input_words = quoted_words(input).skip(1);

        if let Some((is_word, next_word)) = input_words
            .find(|word| word.as_str() == "is")
            .and_then(|word| input_words.next().map(|next_word| (word, next_word)))
        {
            if let Some(thing) = app_meta
                .repository
                .load(&input[..is_word.range().start].trim().into())
            {
                let split_pos = input.len() - input[is_word.range().end..].trim_start().len();

                let mut edit_suggestions = match thing {
                    Thing::Npc(_) => Npc::autocomplete(input[split_pos..].trim_start(), app_meta),
                    Thing::Place(_) => {
                        Place::autocomplete(input[split_pos..].trim_start(), app_meta)
                    }
                    Thing::Region(_) => unimplemented!(),
                };

                suggestions.reserve(edit_suggestions.len());

                edit_suggestions
                    .drain(..)
                    .map(|(a, _)| {
                        (
                            format!("{}{}", &input[..split_pos], a),
                            format!("edit {}", thing.as_str()),
                        )
                    })
                    .for_each(|suggestion| suggestions.push(suggestion));

                if ["named", "called"].contains(&next_word.as_str()) && input_words.next().is_some()
                {
                    suggestions.push((input.to_string(), format!("rename {}", thing.as_str())));
                }
            }
        }

        if let Some(thing) = app_meta.repository.load(&input.trim_end().into()) {
            suggestions.push((
                if input.ends_with(char::is_whitespace) {
                    format!("{}is [{} description]", input, thing.as_str())
                } else {
                    format!("{} is [{} description]", input, thing.as_str())
                },
                format!("edit {}", thing.as_str()),
            ));
        } else if let Some((last_word_index, last_word)) =
            quoted_words(input).enumerate().skip(1).last()
        {
            if "is".starts_with(last_word.as_str()) {
                if let Some(thing) = app_meta
                    .repository
                    .load(&input[..last_word.range().start].trim().into())
                {
                    suggestions.push((
                        if last_word.range().end == input.len() {
                            format!(
                                "{}is [{} description]",
                                &input[..last_word.range().start],
                                thing.as_str(),
                            )
                        } else {
                            format!("{}[{} description]", &input, thing.as_str())
                        },
                        format!("edit {}", thing.as_str()),
                    ))
                }
            } else if let Some(suggestion) = ["named", "called"]
                .iter()
                .find(|s| s.starts_with(last_word.as_str()))
            {
                let second_last_word = quoted_words(input).nth(last_word_index - 1).unwrap();

                if second_last_word.as_str() == "is" {
                    if let Some(thing) = app_meta
                        .repository
                        .load(&input[..second_last_word.range().start].trim().into())
                    {
                        suggestions.push((
                            if last_word.range().end == input.len() {
                                format!(
                                    "{}{} [name]",
                                    &input[..last_word.range().start],
                                    suggestion,
                                )
                            } else {
                                format!("{}[name]", input)
                            },
                            format!("rename {}", thing.as_str()),
                        ));
                    }
                }
            }
        }

        suggestions
    }
}

impl fmt::Display for WorldCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Create { thing } => write!(f, "create {}", thing.thing.display_summary()),
            Self::Edit { name, diff } => write!(f, "{} is {}", name, diff.thing.display_summary()),
        }
    }
}

impl<T: Into<Thing>> ParsedThing<T> {
    pub fn into_thing(self) -> ParsedThing<Thing> {
        ParsedThing {
            thing: self.thing.into(),
            unknown_words: self.unknown_words,
            word_count: self.word_count,
        }
    }
}

impl<T: Default> Default for ParsedThing<T> {
    fn default() -> Self {
        Self {
            thing: T::default(),
            unknown_words: Vec::default(),
            word_count: 0,
        }
    }
}

impl<T: Into<Thing>> From<ParsedThing<T>> for Thing {
    fn from(input: ParsedThing<T>) -> Self {
        input.thing.into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::storage::NullDataStore;
    use crate::world::npc::{Age, Gender, Species};
    use crate::world::place::PlaceType;
    use tokio_test::block_on;

    #[test]
    fn parse_input_test() {
        let app_meta = AppMeta::new(NullDataStore::default());

        assert_eq!(
            (None, vec![create(Npc::default())],),
            WorldCommand::parse_input("npc", &app_meta),
        );

        assert_eq!(
            (Some(create(Npc::default())), Vec::new(),),
            WorldCommand::parse_input("create npc", &app_meta),
        );

        assert_eq!(
            (
                None,
                vec![create(Npc {
                    species: Species::Elf.into(),
                    ..Default::default()
                })],
            ),
            WorldCommand::parse_input("elf", &app_meta),
        );

        assert_eq!(
            (None, Vec::<WorldCommand>::new()),
            WorldCommand::parse_input("potato", &app_meta),
        );

        {
            let mut app_meta = AppMeta::new(NullDataStore::default());

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
                WorldCommand::parse_input("Spot is a good boy", &app_meta),
            );
        }
    }

    #[test]
    fn autocomplete_test() {
        let mut app_meta = AppMeta::new(NullDataStore::default());

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
                vec![(word.to_string(), summary.to_string())],
                WorldCommand::autocomplete(word, &app_meta),
            )
        });

        {
            let expected = vec![
                ("baby".to_string(), "create infant".to_string()),
                ("bar".to_string(), "create inn".to_string()),
                ("boy".to_string(), "create child, he/him".to_string()),
            ];

            let mut actual = WorldCommand::autocomplete("b", &app_meta);
            actual.sort();

            assert_eq!(expected, actual);
        }

        assert_autocomplete(
            &[(
                "Potato Johnson is [character description]",
                "edit character",
            )][..],
            WorldCommand::autocomplete("Potato Johnson", &app_meta),
        );

        assert_autocomplete(
            &[(
                "Potato Johnson is a [character description]",
                "edit character",
            )][..],
            WorldCommand::autocomplete("Potato Johnson is a ", &app_meta),
        );

        assert_autocomplete(
            &[
                ("Potato Johnson is an elderly", "edit character"),
                ("Potato Johnson is an elf", "edit character"),
                ("Potato Johnson is an elvish", "edit character"),
                ("Potato Johnson is an enby", "edit character"),
            ][..],
            WorldCommand::autocomplete("Potato Johnson is an e", &app_meta),
        );
    }

    #[test]
    fn display_test() {
        let app_meta = AppMeta::new(NullDataStore::default());

        vec![
            create(Place {
                subtype: PlaceType::Inn.into(),
                ..Default::default()
            }),
            create(Npc::default()),
            create(Npc {
                species: Some(Species::Elf).into(),
                ..Default::default()
            }),
        ]
        .drain(..)
        .for_each(|command| {
            let command_string = command.to_string();
            assert_ne!("", command_string);
            assert_eq!(
                (Some(command), Vec::new()),
                WorldCommand::parse_input(&command_string, &app_meta),
                "{}",
                command_string,
            );
        });
    }

    fn create(thing: impl Into<Thing>) -> WorldCommand {
        WorldCommand::Create {
            thing: ParsedThing {
                thing: thing.into(),
                unknown_words: Vec::new(),
                word_count: 1,
            },
        }
    }

    fn assert_autocomplete(
        expected_suggestions: &[(&str, &str)],
        actual_suggestions: Vec<(String, String)>,
    ) {
        let mut expected: Vec<_> = expected_suggestions
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect();
        expected.sort();

        let mut actual = actual_suggestions;
        actual.sort();

        assert_eq!(expected, actual);
    }
}
