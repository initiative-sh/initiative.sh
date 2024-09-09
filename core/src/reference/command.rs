use super::{Condition, Item, ItemCategory, MagicItem, Spell, Trait};
use crate::app::{
    AppMeta, Autocomplete, AutocompleteSuggestion, CommandMatches, ContextAwareParse, Runnable,
};
use crate::utils::CaseInsensitiveStr;
use async_trait::async_trait;
use caith::Roller;
use std::fmt;
use std::iter::repeat;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReferenceCommand {
    Condition(Condition),
    Item(Item),
    ItemCategory(ItemCategory),
    MagicItem(MagicItem),
    OpenGameLicense,
    Spell(Spell),
    Spells,
    Trait(Trait),
}

#[async_trait(?Send)]
impl Runnable for ReferenceCommand {
    async fn run(self, _input: &str, _app_meta: &mut AppMeta) -> Result<String, String> {
        let (output, name) = match self {
            Self::Condition(condition) => (format!("{}", condition), condition.get_name()),
            Self::Item(item) => (format!("{}", item), item.get_name()),
            Self::ItemCategory(category) => (format!("{}", category), "This listing"),
            Self::MagicItem(magic_item) => (format!("{}", magic_item), magic_item.get_name()),
            Self::OpenGameLicense => {
                return Ok(include_str!("../../../data/ogl-1.0a.md")
                    .trim_end()
                    .to_string());
            }
            Self::Spell(spell) => (format!("{}", spell), spell.get_name()),
            Self::Spells => (Spell::get_list().to_string(), "This listing"),
            Self::Trait(t) => (t.to_string(), t.get_name()),
        };

        Ok(format!(
            "{}\n\n*{} is Open Game Content subject to the `Open Game License`.*",
            linkify_dice(&output),
            name,
        ))
    }
}

#[async_trait(?Send)]
impl ContextAwareParse for ReferenceCommand {
    async fn parse_input(input: &str, _app_meta: &AppMeta) -> CommandMatches<Self> {
        let mut matches = if input.eq_ci("Open Game License") {
            CommandMatches::new_canonical(Self::OpenGameLicense)
        } else if input.eq_ci("srd spells") {
            CommandMatches::new_canonical(Self::Spells)
        } else if let Some(condition) = input
            .strip_prefix_ci("srd condition ")
            .and_then(|s| s.parse().ok())
        {
            CommandMatches::new_canonical(Self::Condition(condition))
        } else if let Some(item_category) = input
            .strip_prefix_ci("srd item category ")
            .and_then(|s| s.parse().ok())
        {
            CommandMatches::new_canonical(Self::ItemCategory(item_category))
        } else if let Some(item) = input
            .strip_prefix_ci("srd item ")
            .and_then(|s| s.parse().ok())
        {
            CommandMatches::new_canonical(Self::Item(item))
        } else if let Some(magic_item) = input
            .strip_prefix_ci("srd magic item ")
            .and_then(|s| s.parse().ok())
        {
            CommandMatches::new_canonical(Self::MagicItem(magic_item))
        } else if let Some(spell) = input
            .strip_prefix_ci("srd spell ")
            .and_then(|s| s.parse().ok())
        {
            CommandMatches::new_canonical(Self::Spell(spell))
        } else if let Some(character_trait) = input
            .strip_prefix_ci("srd trait ")
            .and_then(|s| s.parse().ok())
        {
            CommandMatches::new_canonical(Self::Trait(character_trait))
        } else {
            CommandMatches::default()
        };

        if let Ok(condition) = input.parse() {
            matches.push_fuzzy(Self::Condition(condition));
        }
        if let Ok(item) = input.parse() {
            matches.push_fuzzy(Self::Item(item));
        }
        if let Ok(category) = input.parse() {
            matches.push_fuzzy(Self::ItemCategory(category));
        }
        if let Ok(magic_item) = input.parse() {
            matches.push_fuzzy(Self::MagicItem(magic_item));
        }
        if let Ok(spell) = input.parse() {
            matches.push_fuzzy(Self::Spell(spell));
        }
        if let Ok(character_trait) = input.parse() {
            matches.push_fuzzy(Self::Trait(character_trait));
        }
        if input.eq_ci("spells") {
            matches.push_fuzzy(Self::Spells);
        }

        matches
    }
}

#[async_trait(?Send)]
impl Autocomplete for ReferenceCommand {
    async fn autocomplete(input: &str, _app_meta: &AppMeta) -> Vec<AutocompleteSuggestion> {
        [
            ("Open Game License", "SRD license"),
            ("spells", "SRD index"),
        ]
        .into_iter()
        .chain(Spell::get_words().zip(repeat("SRD spell")))
        .chain(Condition::get_words().zip(repeat("SRD condition")))
        .chain(Item::get_words().zip(repeat("SRD item")))
        .chain(ItemCategory::get_words().zip(repeat("SRD item category")))
        .chain(MagicItem::get_words().zip(repeat("SRD magic item")))
        .chain(Trait::get_words().zip(repeat("SRD trait")))
        .filter(|(term, _)| term.starts_with_ci(input))
        .take(10)
        .map(|(term, summary)| AutocompleteSuggestion::new(term, summary))
        .collect()
    }
}

impl fmt::Display for ReferenceCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Condition(condition) => write!(f, "srd condition {}", condition.get_name()),
            Self::Item(item) => write!(f, "srd item {}", item.get_name()),
            Self::ItemCategory(category) => write!(f, "srd item category {}", category.get_name()),
            Self::MagicItem(item) => write!(f, "srd magic item {}", item.get_name()),
            Self::OpenGameLicense => write!(f, "Open Game License"),
            Self::Spell(spell) => write!(f, "srd spell {}", spell.get_name()),
            Self::Spells => write!(f, "srd spells"),
            Self::Trait(species_trait) => write!(f, "srd trait {}", species_trait.get_name()),
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
    use crate::storage::NullDataStore;
    use crate::Event;
    use tokio_test::block_on;

    #[test]
    fn display_test() {
        let app_meta = app_meta();

        [
            ReferenceCommand::Spell(Spell::Shield),
            ReferenceCommand::Spells,
            ReferenceCommand::Item(Item::Shield),
            ReferenceCommand::ItemCategory(ItemCategory::Shields),
            ReferenceCommand::MagicItem(MagicItem::DeckOfManyThings),
            ReferenceCommand::OpenGameLicense,
        ]
        .into_iter()
        .for_each(|command| {
            let command_string = command.to_string();
            assert_ne!("", command_string);

            assert_eq!(
                CommandMatches::new_canonical(command.clone()),
                block_on(ReferenceCommand::parse_input(&command_string, &app_meta)),
                "{}",
                command_string,
            );

            assert_eq!(
                CommandMatches::new_canonical(command),
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
        AppMeta::new(NullDataStore, &event_dispatcher)
    }
}
