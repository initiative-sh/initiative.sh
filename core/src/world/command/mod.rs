use super::{Field, Npc, Place, Thing};
use crate::app::{AppMeta, Autocomplete, CommandAlias, ContextAwareParse, Runnable};
use crate::storage::{Change, RepositoryError, StorageCommand};
use crate::utils::{quoted_words, CaseInsensitiveStr};
use async_trait::async_trait;
use futures::join;
use std::borrow::Cow;
use std::fmt;
use std::ops::Range;

mod autocomplete;
mod parse;

#[derive(Clone, Debug, PartialEq)]
pub enum WorldCommand {
    Create {
        thing: ParsedThing<Thing>,
    },
    CreateMultiple {
        thing: Thing,
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
                let diff = parsed_thing.thing;
                let unknown_words = parsed_thing.unknown_words.to_owned();
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
                                StorageCommand::Change {
                                    change: Change::Save {
                                        name: name.to_string(),
                                    },
                                }
                                .into(),
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
                    Ok(append_unknown_words_notice(output, input, unknown_words))
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
            Self::Edit { name, diff } => {
                let ParsedThing {
                    thing: diff,
                    unknown_words,
                    word_count: _,
                } = diff;

                StorageCommand::Change {
                    change: Change::Edit {
                        id: name.as_str().into(),
                        name,
                        diff,
                    },
                }
                .run(input, app_meta)
                .await
                .map(|s| append_unknown_words_notice(s, input, unknown_words))
            }
        }
    }
}

#[async_trait(?Send)]
impl ContextAwareParse for WorldCommand {
    async fn parse_input(input: &str, app_meta: &AppMeta) -> (Option<Self>, Vec<Self>) {
        let mut exact_match = None;
        let mut fuzzy_matches = Vec::new();

        if let Some(Ok(thing)) = input
            .strip_prefix_ci("create ")
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
            .find(|word| word.as_str().eq_ci("is"))
        {
            let (name, description) = (
                input[..word.range().start].trim(),
                input[word.range().end..].trim(),
            );

            let (diff, thing) = if let Ok(thing) = app_meta.repository.get_by_name(name).await {
                (
                    match thing {
                        Thing::Npc(_) => description
                            .parse::<ParsedThing<Npc>>()
                            .map(|npc| npc.into_thing()),
                        Thing::Place(_) => description
                            .parse::<ParsedThing<Place>>()
                            .map(|npc| npc.into_thing()),
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

#[async_trait(?Send)]
impl Autocomplete for WorldCommand {
    async fn autocomplete(
        input: &str,
        app_meta: &AppMeta,
    ) -> Vec<(Cow<'static, str>, Cow<'static, str>)> {
        let mut suggestions = Vec::new();

        let (mut place_suggestions, mut npc_suggestions) = join!(
            Place::autocomplete(input, app_meta),
            Npc::autocomplete(input, app_meta),
        );

        suggestions.append(&mut place_suggestions);
        suggestions.append(&mut npc_suggestions);

        let mut input_words = quoted_words(input).skip(1);

        if let Some((is_word, next_word)) = input_words
            .find(|word| word.as_str().eq_ci("is"))
            .and_then(|word| input_words.next().map(|next_word| (word, next_word)))
        {
            if let Ok(thing) = app_meta
                .repository
                .get_by_name(input[..is_word.range().start].trim())
                .await
            {
                let split_pos = input.len() - input[is_word.range().end..].trim_start().len();

                let mut edit_suggestions = match thing {
                    Thing::Npc(_) => Npc::autocomplete(input[split_pos..].trim_start(), app_meta),
                    Thing::Place(_) => {
                        Place::autocomplete(input[split_pos..].trim_start(), app_meta)
                    }
                }
                .await;

                suggestions.reserve(edit_suggestions.len());

                edit_suggestions
                    .drain(..)
                    .map(|(a, _)| {
                        (
                            format!("{}{}", &input[..split_pos], a).into(),
                            format!("edit {}", thing.as_str()).into(),
                        )
                    })
                    .for_each(|suggestion| suggestions.push(suggestion));

                if next_word.as_str().in_ci(&["named", "called"]) && input_words.next().is_some() {
                    suggestions.push((
                        input.to_string().into(),
                        format!("rename {}", thing.as_str()).into(),
                    ));
                }
            }
        }

        if let Ok(thing) = app_meta.repository.get_by_name(input.trim_end()).await {
            suggestions.push((
                if input.ends_with(char::is_whitespace) {
                    format!("{}is [{} description]", input, thing.as_str())
                } else {
                    format!("{} is [{} description]", input, thing.as_str())
                }
                .into(),
                format!("edit {}", thing.as_str()).into(),
            ));
        } else if let Some((last_word_index, last_word)) =
            quoted_words(input).enumerate().skip(1).last()
        {
            if "is".starts_with_ci(last_word.as_str()) {
                if let Ok(thing) = app_meta
                    .repository
                    .get_by_name(input[..last_word.range().start].trim())
                    .await
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
                        }
                        .into(),
                        format!("edit {}", thing.as_str()).into(),
                    ))
                }
            } else if let Some(suggestion) = ["named", "called"]
                .iter()
                .find(|s| s.starts_with_ci(last_word.as_str()))
            {
                let second_last_word = quoted_words(input).nth(last_word_index - 1).unwrap();

                if second_last_word.as_str().eq_ci("is") {
                    if let Ok(thing) = app_meta
                        .repository
                        .get_by_name(input[..second_last_word.range().start].trim())
                        .await
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
                            }
                            .into(),
                            format!("rename {}", thing.as_str()).into(),
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
            Self::Create { thing } => write!(f, "create {}", thing.thing.display_description()),
            Self::CreateMultiple { thing } => {
                write!(f, "create  multiple {}", thing.display_description())
            }
            Self::Edit { name, diff } => {
                write!(f, "{} is {}", name, diff.thing.display_description())
            }
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
    use crate::app::assert_autocomplete;
    use crate::storage::NullDataStore;
    use crate::world::npc::{Age, Gender, Species};
    use crate::world::place::PlaceType;
    use crate::Event;
    use tokio_test::block_on;

    #[test]
    fn parse_input_test() {
        let mut app_meta = app_meta();

        assert_eq!(
            (None, vec![create(Npc::default())]),
            block_on(WorldCommand::parse_input("npc", &app_meta)),
        );

        assert_eq!(
            (Some(create(Npc::default())), Vec::new()),
            block_on(WorldCommand::parse_input("create npc", &app_meta)),
        );

        assert_eq!(
            (
                None,
                vec![create(Npc {
                    species: Species::Elf.into(),
                    ..Default::default()
                })],
            ),
            block_on(WorldCommand::parse_input("elf", &app_meta)),
        );

        assert_eq!(
            (None, Vec::<WorldCommand>::new()),
            block_on(WorldCommand::parse_input("potato", &app_meta)),
        );

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
                block_on(WorldCommand::autocomplete(word, &app_meta)),
            );

            assert_eq!(
                vec![(word.into(), summary.into())],
                block_on(WorldCommand::autocomplete(&word.to_uppercase(), &app_meta)),
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
            block_on(WorldCommand::autocomplete("b", &app_meta)),
        );

        assert_autocomplete(
            &[(
                "Potato Johnson is [character description]",
                "edit character",
            )][..],
            block_on(WorldCommand::autocomplete("Potato Johnson", &app_meta)),
        );

        assert_autocomplete(
            &[(
                "Potato Johnson is a [character description]",
                "edit character",
            )][..],
            block_on(WorldCommand::autocomplete(
                "Potato Johnson is a ",
                &app_meta,
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
            )),
        );
    }

    #[test]
    fn display_test() {
        let app_meta = app_meta();

        vec![
            create(Place {
                subtype: "inn".parse::<PlaceType>().ok().into(),
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

    fn create(thing: impl Into<Thing>) -> WorldCommand {
        WorldCommand::Create {
            thing: ParsedThing {
                thing: thing.into(),
                unknown_words: Vec::new(),
                word_count: 1,
            },
        }
    }

    fn event_dispatcher(_event: Event) {}

    fn app_meta() -> AppMeta {
        AppMeta::new(NullDataStore::default(), &event_dispatcher)
    }
}
