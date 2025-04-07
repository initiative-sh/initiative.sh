use crate::app::{
    AppMeta, Autocomplete, AutocompleteSuggestion, CommandAlias, CommandMatches, ContextAwareParse,
    Runnable,
};
use crate::storage::{Change, Record, RepositoryError, StorageCommand};
use crate::utils::{quoted_words, CaseInsensitiveStr};
use crate::world::npc::NpcData;
use crate::world::place::PlaceData;
use crate::world::thing::{Thing, ThingData};
use crate::world::Field;
use async_trait::async_trait;
use futures::join;
use std::fmt;
use std::ops::Range;

mod autocomplete;
mod parse;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorldCommand {
    Create {
        parsed_thing_data: ParsedThing<ThingData>,
    },
    CreateMultiple {
        thing_data: ThingData,
    },
    Edit {
        name: String,
        parsed_diff: ParsedThing<ThingData>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedThing<T> {
    pub thing_data: T,
    pub unknown_words: Vec<Range<usize>>,
    pub word_count: usize,
}

#[async_trait(?Send)]
impl Runnable for WorldCommand {
    async fn run(self, input: &str, app_meta: &mut AppMeta) -> Result<String, String> {
        match self {
            Self::Create { parsed_thing_data } => {
                let original_thing_data = parsed_thing_data.thing_data;
                let unknown_words = parsed_thing_data.unknown_words.to_owned();
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

                if let Some(output) = output {
                    Ok(append_unknown_words_notice(output, input, unknown_words))
                } else {
                    Err(format!(
                        "Couldn't create a unique {} name.",
                        original_thing_data.display_description(),
                    ))
                }
            }
            Self::CreateMultiple { thing_data } => {
                let mut output = format!(
                    "# Alternative suggestions for \"{}\"",
                    thing_data.display_description(),
                );

                for i in 1..=10 {
                    let mut thing_output = None;

                    for _ in 0..10 {
                        let mut thing_data = thing_data.clone();
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
                                app_meta.command_aliases.insert(CommandAlias::literal(
                                    (i % 10).to_string(),
                                    format!("load {}", thing.name()),
                                    StorageCommand::Load {
                                        name: thing.name().to_string(),
                                    }
                                    .into(),
                                ));
                                thing_output = Some(format!(
                                    "{}~{}~ {}",
                                    if i == 1 { "\n\n" } else { "\\\n" },
                                    i % 10,
                                    thing.display_summary(),
                                ));
                                break;
                            }
                            Ok(None) | Err((_, RepositoryError::NameAlreadyExists(_))) => {} // silently retry
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
                    "more",
                    format!("create {}", thing_data.display_description()),
                    Self::CreateMultiple { thing_data }.into(),
                ));

                output.push_str("\n\n_For even more suggestions, type ~more~._");

                Ok(output)
            }
            Self::Edit { name, parsed_diff } => {
                let ParsedThing {
                    thing_data: thing_diff,
                    unknown_words,
                    word_count: _,
                } = parsed_diff;

                let thing_type = thing_diff.as_str();

                match app_meta.repository.modify(Change::Edit {
                        name: name.clone(),
                        uuid: None,
                        diff: thing_diff,
                    }).await {
                    Ok(Some(Record { thing, .. })) => Ok(
                        if matches!(app_meta.repository.undo_history().next(), Some(Change::EditAndUnsave { .. })) {
                            format!(
                                "{}\n\n_{} was successfully edited and automatically saved to your `journal`. Use `undo` to reverse this._",
                                thing.display_details(app_meta.repository.load_relations(&thing).await.unwrap_or_default()),
                                name,
                            )
                        } else {
                            format!(
                                "{}\n\n_{} was successfully edited. Use `undo` to reverse this._",
                                thing.display_details(app_meta.repository.load_relations(&thing).await.unwrap_or_default()),
                                name,
                            )
                        }
                    ),
                    Err((_, RepositoryError::NotFound)) => Err(format!(r#"There is no {} named "{}"."#, thing_type, name)),
                    _ => Err(format!("Couldn't edit `{}`.", name)),
                }
                .map(|s| append_unknown_words_notice(s, input, unknown_words))
            }
        }
    }
}

#[async_trait(?Send)]
impl ContextAwareParse for WorldCommand {
    async fn parse_input(input: &str, app_meta: &AppMeta) -> CommandMatches<Self> {
        let mut matches = CommandMatches::default();

        if let Some(Ok(parsed_thing_data)) = input
            .strip_prefix_ci("create ")
            .map(|s| s.parse::<ParsedThing<ThingData>>())
        {
            if parsed_thing_data.unknown_words.is_empty() {
                matches.push_canonical(Self::Create { parsed_thing_data });
            } else {
                matches.push_fuzzy(Self::Create { parsed_thing_data });
            }
        } else if let Ok(parsed_thing_data) = input.parse::<ParsedThing<ThingData>>() {
            matches.push_fuzzy(Self::Create { parsed_thing_data });
        }

        if let Some(word) = quoted_words(input)
            .skip(1)
            .find(|word| word.as_str().eq_ci("is"))
        {
            let (name, description) = (
                input[..word.range().start].trim(),
                input[word.range().end..].trim(),
            );

            let (diff, thing): (Result<ParsedThing<ThingData>, ()>, Option<Thing>) =
                if let Ok(Record { thing, .. }) = app_meta.repository.get_by_name(name).await {
                    (
                        match thing.data {
                            ThingData::Npc(_) => description
                                .parse::<ParsedThing<NpcData>>()
                                .map(|t| t.into_thing_data()),
                            ThingData::Place(_) => description
                                .parse::<ParsedThing<PlaceData>>()
                                .map(|t| t.into_thing_data()),
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

                matches.push_fuzzy(Self::Edit {
                    name,
                    parsed_diff: diff,
                });
            }
        }

        matches
    }
}

#[async_trait(?Send)]
impl Autocomplete for WorldCommand {
    async fn autocomplete(input: &str, app_meta: &AppMeta) -> Vec<AutocompleteSuggestion> {
        let mut suggestions = Vec::new();

        let (mut place_suggestions, mut npc_suggestions) = join!(
            PlaceData::autocomplete(input, app_meta),
            NpcData::autocomplete(input, app_meta),
        );

        suggestions.append(&mut place_suggestions);
        suggestions.append(&mut npc_suggestions);

        let mut input_words = quoted_words(input).skip(1);

        if let Some((is_word, next_word)) = input_words
            .find(|word| word.as_str().eq_ci("is"))
            .and_then(|word| input_words.next().map(|next_word| (word, next_word)))
        {
            if let Ok(Record { thing, .. }) = app_meta
                .repository
                .get_by_name(input[..is_word.range().start].trim())
                .await
            {
                let split_pos = input.len() - input[is_word.range().end..].trim_start().len();

                let edit_suggestions = match thing.data {
                    ThingData::Npc(_) => {
                        NpcData::autocomplete(input[split_pos..].trim_start(), app_meta)
                    }
                    ThingData::Place(_) => {
                        PlaceData::autocomplete(input[split_pos..].trim_start(), app_meta)
                    }
                }
                .await;

                suggestions.extend(edit_suggestions.into_iter().map(|suggestion| {
                    AutocompleteSuggestion::new(
                        format!("{}{}", &input[..split_pos], suggestion.term),
                        format!("edit {}", thing.as_str()),
                    )
                }));

                if next_word.as_str().in_ci(&["named", "called"]) && input_words.next().is_some() {
                    suggestions.push(AutocompleteSuggestion::new(
                        input.to_string(),
                        format!("rename {}", thing.as_str()),
                    ));
                }
            }
        }

        if let Ok(Record { thing, .. }) = app_meta.repository.get_by_name(input.trim_end()).await {
            suggestions.push(AutocompleteSuggestion::new(
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
            if "is".starts_with_ci(last_word.as_str()) {
                if let Ok(Record { thing, .. }) = app_meta
                    .repository
                    .get_by_name(input[..last_word.range().start].trim())
                    .await
                {
                    suggestions.push(AutocompleteSuggestion::new(
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
                .find(|s| s.starts_with_ci(last_word.as_str()))
            {
                let second_last_word = quoted_words(input).nth(last_word_index - 1).unwrap();

                if second_last_word.as_str().eq_ci("is") {
                    if let Ok(Record { thing, .. }) = app_meta
                        .repository
                        .get_by_name(input[..second_last_word.range().start].trim())
                        .await
                    {
                        suggestions.push(AutocompleteSuggestion::new(
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
            Self::Create { parsed_thing_data } => write!(
                f,
                "create {}",
                parsed_thing_data.thing_data.display_description()
            ),
            Self::CreateMultiple { thing_data } => {
                write!(f, "create  multiple {}", thing_data.display_description())
            }
            Self::Edit { name, parsed_diff } => {
                write!(
                    f,
                    "{} is {}",
                    name,
                    parsed_diff.thing_data.display_description()
                )
            }
        }
    }
}

impl<T: Into<ThingData>> ParsedThing<T> {
    pub fn into_thing_data(self) -> ParsedThing<ThingData> {
        ParsedThing {
            thing_data: self.thing_data.into(),
            unknown_words: self.unknown_words,
            word_count: self.word_count,
        }
    }
}

impl<T: Default> Default for ParsedThing<T> {
    fn default() -> Self {
        Self {
            thing_data: T::default(),
            unknown_words: Vec::default(),
            word_count: 0,
        }
    }
}

impl<T: Into<ThingData>> From<ParsedThing<T>> for ThingData {
    fn from(input: ParsedThing<T>) -> Self {
        input.thing_data.into()
    }
}

fn append_unknown_words_notice(
    mut output: String,
    input: &str,
    unknown_words: Vec<Range<usize>>,
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
            let mut words = unknown_words.into_iter();
            let mut unknown_word = words.next();
            for (i, _) in input.char_indices() {
                if unknown_word.as_ref().is_some_and(|word| i >= word.end) {
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
    use crate::test_utils as test;
    use crate::world::npc::{Age, Gender, NpcData, Species};
    use crate::world::place::{PlaceData, PlaceType};

    #[tokio::test]
    async fn parse_input_test() {
        let mut app_meta = test::app_meta();

        assert_eq!(
            CommandMatches::new_fuzzy(create(NpcData::default())),
            WorldCommand::parse_input("npc", &app_meta).await,
        );

        assert_eq!(
            CommandMatches::new_canonical(create(NpcData::default())),
            WorldCommand::parse_input("create npc", &app_meta).await,
        );

        assert_eq!(
            CommandMatches::new_fuzzy(create(NpcData {
                species: Species::Elf.into(),
                ..Default::default()
            })),
            WorldCommand::parse_input("elf", &app_meta).await,
        );

        assert_eq!(
            CommandMatches::default(),
            WorldCommand::parse_input("potato", &app_meta).await,
        );

        {
            app_meta
                .repository
                .modify(Change::Create {
                    thing_data: NpcData {
                        name: "Spot".into(),
                        ..Default::default()
                    }
                    .into(),
                    uuid: None,
                })
                .await
                .unwrap();

            assert_eq!(
                CommandMatches::new_fuzzy(WorldCommand::Edit {
                    name: "Spot".into(),
                    parsed_diff: ParsedThing {
                        thing_data: NpcData {
                            age: Age::Child.into(),
                            gender: Gender::Masculine.into(),
                            ..Default::default()
                        }
                        .into(),
                        #[expect(clippy::single_range_in_vec_init)]
                        unknown_words: vec![10..14],
                        word_count: 2,
                    },
                }),
                WorldCommand::parse_input("Spot is a good boy", &app_meta).await,
            );
        }
    }

    #[tokio::test]
    async fn autocomplete_test() {
        let app_meta = test::app_meta::with_test_data().await;

        for (word, summary) in [
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
        ] {
            test::assert_autocomplete_eq!(
                [(word, summary)],
                WorldCommand::autocomplete(word, &app_meta).await,
            );

            test::assert_autocomplete_eq!(
                [(word, summary)],
                WorldCommand::autocomplete(&word.to_uppercase(), &app_meta).await,
            );
        }

        test::assert_autocomplete_eq!(
            [
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
            ],
            WorldCommand::autocomplete("b", &app_meta).await,
        );

        test::assert_autocomplete_eq!(
            [("penelope is [character description]", "edit character")],
            WorldCommand::autocomplete("penelope", &app_meta).await,
        );

        test::assert_autocomplete_eq!(
            [("PENELOPE is a [character description]", "edit character")],
            WorldCommand::autocomplete("PENELOPE is a ", &app_meta).await,
        );

        test::assert_autocomplete_eq!(
            [
                ("penelope is an elderly", "edit character"),
                ("penelope is an elf", "edit character"),
                ("penelope is an elvish", "edit character"),
                ("penelope is an enby", "edit character"),
            ],
            WorldCommand::autocomplete("penelope is an e", &app_meta).await,
        );
    }

    #[tokio::test]
    async fn display_test() {
        let app_meta = test::app_meta();

        for command in [
            create(PlaceData {
                subtype: "inn".parse::<PlaceType>().ok().into(),
                ..Default::default()
            }),
            create(NpcData::default()),
            create(test::npc().species(Species::Elf).build()),
        ] {
            let command_string = command.to_string();
            assert_ne!("", command_string);

            assert_eq!(
                CommandMatches::new_canonical(command.clone()),
                WorldCommand::parse_input(&command_string, &app_meta).await,
                "{}",
                command_string,
            );

            assert_eq!(
                CommandMatches::new_canonical(command),
                WorldCommand::parse_input(&command_string.to_uppercase(), &app_meta).await,
                "{}",
                command_string.to_uppercase(),
            );
        }
    }

    fn parsed_thing(thing_data: impl Into<ThingData>) -> ParsedThing<ThingData> {
        ParsedThing {
            thing_data: thing_data.into(),
            unknown_words: Vec::new(),
            word_count: 1,
        }
    }

    fn create(thing_data: impl Into<ThingData>) -> WorldCommand {
        WorldCommand::Create {
            parsed_thing_data: parsed_thing(thing_data),
        }
    }
}
