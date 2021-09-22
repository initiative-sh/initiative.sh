use super::{Item, ItemCategory, MagicItem, Spell};
use crate::app::{autocomplete_phrase, AppMeta, Runnable};
use async_trait::async_trait;
use caith::Roller;

#[derive(Clone, Debug, PartialEq)]
pub enum ReferenceCommand {
    Spell(Spell),
    Spells,
    Item(Item),
    ItemCategory(ItemCategory),
    MagicItem(MagicItem),
    OpenGameLicense,
}

impl ReferenceCommand {
    fn summarize(&self) -> &str {
        match self {
            Self::Spell(_) => "SRD spell",
            Self::Spells => "SRD index",
            Self::Item(_) => "SRD item",
            Self::ItemCategory(_) => "SRD item category",
            Self::MagicItem(_) => "SRD magic item",
            Self::OpenGameLicense => "SRD license",
        }
    }
}

#[async_trait(?Send)]
impl Runnable for ReferenceCommand {
    async fn run(&self, _app_meta: &mut AppMeta) -> Result<String, String> {
        let (output, name) = match self {
            Self::Spell(spell) => (format!("{}", spell), spell.get_name()),
            Self::Spells => (Spell::get_list().to_string(), "This listing"),
            Self::Item(item) => (format!("{}", item), item.get_name()),
            Self::ItemCategory(category) => (format!("{}", category), "This listing"),
            Self::MagicItem(magic_item) => (format!("{}", magic_item), magic_item.get_name()),
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

    fn parse_input(input: &str, _app_meta: &AppMeta) -> (Option<Self>, Vec<Self>) {
        let mut exact_match = None;
        let mut fuzzy_matches = Vec::new();

        match input {
            "Open Game License" => exact_match = Some(Self::OpenGameLicense),
            "srd spells" => exact_match = Some(Self::Spells),
            _ => {
                if let Some(spell) = input.strip_prefix("srd spell ") {
                    if let Ok(spell) = spell.parse() {
                        exact_match = Some(Self::Spell(spell));
                    }
                } else if let Some(item) = input.strip_prefix("srd item ") {
                    if let Ok(item) = item.parse() {
                        exact_match = Some(Self::Item(item));
                    }
                } else if let Some(category) = input.strip_prefix("srd category ") {
                    if let Ok(category) = category.parse() {
                        exact_match = Some(Self::ItemCategory(category));
                    }
                } else if let Some(item) = input.strip_prefix("srd magic item ") {
                    if let Ok(item) = item.parse() {
                        exact_match = Some(Self::MagicItem(item));
                    }
                }
            }
        }

        if let Ok(spell) = input.parse() {
            fuzzy_matches.push(Self::Spell(spell));
        }
        if let Ok(item) = input.parse() {
            fuzzy_matches.push(Self::Item(item));
        }
        if let Ok(category) = input.parse() {
            fuzzy_matches.push(Self::ItemCategory(category));
        }
        if let Ok(magic_item) = input.parse() {
            fuzzy_matches.push(Self::MagicItem(magic_item));
        }
        if input == "spells" {
            fuzzy_matches.push(Self::Spells);
        }

        (exact_match, fuzzy_matches)
    }

    fn autocomplete(input: &str, app_meta: &AppMeta) -> Vec<(String, String)> {
        autocomplete_phrase(
            input,
            &mut ["Open Game License", "spells"]
                .iter()
                .chain(Spell::get_words().iter())
                .chain(Item::get_words().iter())
                .chain(ItemCategory::get_words().iter())
                .chain(MagicItem::get_words().iter()),
        )
        .drain(..)
        .filter_map(|s| {
            let (exact_match, mut fuzzy_matches) = Self::parse_input(&s, app_meta);

            exact_match
                .or_else(|| fuzzy_matches.drain(..).next())
                .map(|c| (s.clone(), c.summarize().to_string()))
        })
        .collect()
    }
}

fn linkify_dice(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut input_offset = 0;

    let mut hold = String::new();
    let mut hold_offset = 0;
    let mut hold_active = false;

    for part in input.split_inclusive(|c: char| c.is_whitespace() || c.is_ascii_punctuation()) {
        if !hold_active && part.contains(|c: char| c.is_ascii_digit()) && part.contains('d') {
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
                if hold_trimmed.contains('d')
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
