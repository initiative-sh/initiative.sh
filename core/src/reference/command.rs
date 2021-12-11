use super::{Item, ItemCategory, MagicItem, Spell};
use crate::app::{AppMeta, Runnable};
use async_trait::async_trait;
use caith::Roller;
use initiative_macros::{Autocomplete, ContextAwareParse};
use std::fmt;

#[derive(Autocomplete, Clone, ContextAwareParse, Debug, PartialEq)]
pub enum ReferenceCommand {
    #[command(syntax = "srd spell [spell]")]
    #[command(no_default_autocomplete)]
    #[command(alias = "[spell]")]
    /// SRD spell
    Spell {
        #[command(implements(WordList))]
        spell: Spell,
    },

    #[command(syntax = "srd spells")]
    #[command(no_default_autocomplete)]
    #[command(alias = "spells")]
    /// SRD spell list
    Spells,

    #[command(syntax = "srd item [item]")]
    #[command(no_default_autocomplete)]
    #[command(alias = "[item]")]
    /// SRD item
    Item {
        #[command(implements(WordList))]
        item: Item,
    },

    #[command(syntax = "srd item category [category]")]
    #[command(no_default_autocomplete)]
    #[command(alias = "[category]")]
    /// SRD item category
    ItemCategory {
        #[command(implements(WordList))]
        category: ItemCategory,
    },

    #[command(syntax = "srd magic item [item]")]
    #[command(no_default_autocomplete)]
    #[command(alias = "[item]")]
    /// SRD magic item
    MagicItem {
        #[command(implements(WordList))]
        item: MagicItem,
    },

    #[command(syntax = "Open Game License")]
    OpenGameLicense,
}

#[async_trait(?Send)]
impl Runnable for ReferenceCommand {
    async fn run(self, _input: &str, _app_meta: &mut AppMeta) -> Result<String, String> {
        let (output, name) = match self {
            Self::Spell { spell } => (format!("{}", spell), spell.get_name()),
            Self::Spells => (Spell::get_list().to_string(), "This listing"),
            Self::Item { item } => (format!("{}", item), item.get_name()),
            Self::ItemCategory { category } => (format!("{}", category), "This listing"),
            Self::MagicItem { item } => (format!("{}", item), item.get_name()),
            Self::OpenGameLicense => {
                return Ok(include_str!("../../../data/ogl-1.0a.md")
                    .trim_end()
                    .to_string());
            }
        };

        Ok(format!(
            "{}\n\n*{} is Open Game Content subject to the `Open Game License`.*",
            linkify_dice(&output),
            name,
        ))
    }
}

impl fmt::Display for ReferenceCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Spell { spell } => write!(f, "srd spell {}", spell.get_name()),
            Self::Spells => write!(f, "srd spells"),
            Self::Item { item } => write!(f, "srd item {}", item.get_name()),
            Self::ItemCategory { category } => {
                write!(f, "srd item category {}", category.get_name())
            }
            Self::MagicItem { item } => write!(f, "srd magic item {}", item.get_name()),
            Self::OpenGameLicense => write!(f, "Open Game License"),
        }
    }
}

fn linkify_dice(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut input_offset = 0;

    let mut hold = String::new();
    let mut hold_offset = 0;
    let mut hold_active = false;

    for part in input.split_inclusive(|c: char| c.is_whitespace() || c.is_ascii_punctuation()) {
        if !hold_active
            && part.contains(|c: char| c.is_ascii_digit())
            && part.contains(&['d', 'D'][..])
        {
            hold_active = true;
            hold_offset = input_offset;
        } else if hold_active && part.contains(char::is_alphabetic) {
            hold_active = false;
        }

        if hold_active {
            hold.push_str(part);
        } else {
            while !hold.is_empty() {
                let hold_trimmed = hold.trim();
                if hold_trimmed.contains(&['d', 'D'][..])
                    && Roller::new(hold_trimmed).map_or(false, |r| r.roll().is_ok())
                {
                    result.push('`');
                    result.push_str(hold_trimmed);
                    result.push('`');
                    result.push_str(&input[hold_offset + hold_trimmed.len()..input_offset]);
                    hold.clear();
                    break;
                }

                if let Some(pos) =
                    hold.rfind(|c: char| c.is_whitespace() || c.is_ascii_punctuation())
                {
                    hold.truncate(pos);

                    if hold.is_empty() {
                        result.push_str(&input[hold_offset..input_offset]);
                    }
                } else {
                    result.push_str(&input[hold_offset..input_offset]);
                    hold.clear();
                }
            }

            result.push_str(part);
        }

        input_offset += part.len();
    }

    result.push_str(&hold);
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::app::ContextAwareParse;
    use crate::storage::NullDataStore;
    use crate::Event;
    use tokio_test::block_on;

    #[test]
    fn display_test() {
        let app_meta = app_meta();

        vec![
            ReferenceCommand::Spell {
                spell: Spell::Shield,
            },
            ReferenceCommand::Spells,
            ReferenceCommand::Item { item: Item::Shield },
            ReferenceCommand::ItemCategory {
                category: ItemCategory::Shields,
            },
            ReferenceCommand::MagicItem {
                item: MagicItem::DeckOfManyThings,
            },
            ReferenceCommand::OpenGameLicense,
        ]
        .drain(..)
        .for_each(|command| {
            let command_string = command.to_string();
            assert_ne!("", command_string);

            assert_eq!(
                (Some(command.clone()), Vec::new()),
                block_on(ReferenceCommand::parse_input(&command_string, &app_meta)),
                "{}",
                command_string,
            );

            assert_eq!(
                (Some(command), Vec::new()),
                block_on(ReferenceCommand::parse_input(
                    &command_string.to_uppercase(),
                    &app_meta,
                )),
                "{}",
                command_string.to_uppercase(),
            );
        });
    }

    fn event_dispatcher(_event: Event) {}

    fn app_meta() -> AppMeta {
        AppMeta::new(NullDataStore::default(), &event_dispatcher)
    }
}
