use super::{Item, ItemCategory, Spell};
use crate::app::{autocomplete_phrase, AppMeta, Runnable};
use async_trait::async_trait;

#[derive(Clone, Debug, PartialEq)]
pub enum ReferenceCommand {
    Spell(Spell),
    Spells,
    Item(Item),
    ItemCategory(ItemCategory),
    OpenGameLicense,
}

#[async_trait(?Send)]
impl Runnable for ReferenceCommand {
    async fn run(&self, _app_meta: &mut AppMeta) -> String {
        let (output, name) = match self {
            Self::Spell(spell) => (format!("{}", spell), spell.get_name()),
            Self::Spells => (Spell::get_list().to_string(), "This listing"),
            Self::Item(item) => (format!("{}", item), item.get_name()),
            Self::ItemCategory(category) => (format!("{}", category), "This listing"),
            Self::OpenGameLicense => {
                return include_str!("../../../data/ogl-1.0a.md")
                    .trim_end()
                    .to_string()
            }
        };

        format!(
            "{}\n\n*{} is Open Game Content subject to the `Open Game License`.*",
            output, name
        )
    }

    fn summarize(&self) -> &str {
        match self {
            Self::Spell(_) => "SRD spell",
            Self::Spells => "SRD index",
            Self::Item(_) => "SRD item",
            Self::ItemCategory(_) => "SRD item category",
            Self::OpenGameLicense => "SRD license",
        }
    }

    fn parse_input(input: &str, _app_meta: &AppMeta) -> Vec<Self> {
        match input {
            "Open Game License" => return vec![Self::OpenGameLicense],
            "spells" => return vec![Self::Spells],
            _ => {}
        }

        if let Ok(spell) = input.parse() {
            vec![Self::Spell(spell)]
        } else if let Ok(item) = input.parse() {
            vec![Self::Item(item)]
        } else if let Ok(category) = input.parse() {
            vec![Self::ItemCategory(category)]
        } else {
            Vec::new()
        }
    }

    fn autocomplete(input: &str, app_meta: &AppMeta) -> Vec<(String, String)> {
        let mut suggestions = autocomplete_phrase(
            input,
            &mut ["Open Game License", "spells"]
                .iter()
                .chain(Spell::get_words().iter())
                .chain(Item::get_words().iter())
                .chain(ItemCategory::get_words().iter()),
        );

        suggestions.sort();
        suggestions.truncate(10);

        suggestions
            .iter()
            .flat_map(|s| std::iter::repeat(s).zip(Self::parse_input(s.as_str(), app_meta)))
            .map(|(s, c)| (s.clone(), c.summarize().to_string()))
            .collect()
    }
}
