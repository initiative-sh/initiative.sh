use super::{Column, Item, MagicItem};
use crate::srd_5e::Reference;
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct ItemCategory {
    index: String,
    name: String,
    #[serde(rename = "equipment")]
    items: Vec<Reference>,
}

pub struct ItemTableView<'a> {
    category: &'a ItemCategory,
    items: &'a [Item],
}

pub struct MagicItemListView<'a> {
    category: &'a ItemCategory,
    magic_items: &'a [MagicItem],
    title: &'a str,
}

impl ItemCategory {
    pub fn name(&self) -> String {
        match self.name.as_str() {
            "Potion" | "Ring" | "Rod" | "Scroll" | "Wand" | "Weapon" => {
                format!("{}s", self.name)
            }
            "Staff" => "Staves".to_string(),
            name => name.to_string(),
        }
    }

    pub fn alt_names(&self) -> Vec<String> {
        match self.index.as_str() {
            "mounts-and-other-animals" => vec!["Animals".to_string()],
            "waterborne-vehicles" => ["Vehicles, Waterborne", "Ships", "Boats"]
                .iter()
                .map(|&s| String::from(s))
                .collect(),
            _ if self.name.contains(' ') && !self.name.contains(" and ") => {
                let (start, end) = self.name.rsplit_once(' ').unwrap();
                vec![format!("{}, {}", end, start)]
            }
            _ => Vec::new(),
        }
    }

    pub fn token(&self) -> String {
        crate::to_camel_case(&self.index)
    }

    pub fn item_tokens(&self) -> Vec<String> {
        self.items.iter().map(|item| item.token()).collect()
    }

    pub fn has_items(&self) -> bool {
        self.items
            .iter()
            .any(|item| item.url.contains("/equipment/"))
    }

    pub fn has_magic_items(&self) -> bool {
        self.items
            .iter()
            .any(|item| item.url.contains("/magic-items/"))
    }

    pub fn display_item_table<'a>(&'a self, items: &'a [Item]) -> ItemTableView<'a> {
        ItemTableView {
            category: self,
            items,
        }
    }

    pub fn display_magic_item_list<'a>(
        &'a self,
        magic_items: &'a [MagicItem],
        title: &'a str,
    ) -> MagicItemListView<'a> {
        MagicItemListView {
            category: self,
            magic_items,
            title,
        }
    }
}

impl fmt::Display for ItemTableView<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "# {}\n\n|", &self.category.name())?;

        let columns = if self.category.index.contains("armor") {
            &[
                Column::Name,
                Column::Cost,
                Column::ArmorClass,
                Column::Strength,
                Column::Stealth,
                Column::Weight,
            ][..]
        } else if self.category.index.contains("weapon") {
            &[
                Column::Name,
                Column::Cost,
                Column::Damage,
                Column::Weight,
                Column::Properties,
            ][..]
        } else if self.category.index == "mounts-and-other-animals" {
            &[
                Column::Name,
                Column::Cost,
                Column::Speed,
                Column::CarryingCapacity,
            ][..]
        } else if self.category.index != "tack-harness-and-drawn-vehicles"
            && self.category.index.contains("vehicle")
        {
            &[Column::Name, Column::Cost, Column::Speed][..]
        } else {
            &[Column::Name, Column::Cost, Column::Weight][..]
        };

        columns
            .iter()
            .try_for_each(|column| write!(f, " {} |", column))?;
        write!(f, "\n|")?;

        columns.iter().try_for_each(|column| match column {
            Column::CarryingCapacity | Column::Cost | Column::Speed | Column::Weight => {
                write!(f, "--:|")
            }
            _ => write!(f, "---|"),
        })?;

        let tokens = self.category.item_tokens();

        let mut items: Vec<(&Item, String)> = self
            .items
            .iter()
            .filter(|item| tokens.contains(&item.token()))
            .map(|item| (item, item.alt_name().unwrap_or_else(|| item.name())))
            .collect();

        items.sort_by(|(_, a), (_, b)| a.cmp(b));

        items
            .iter()
            .try_for_each(|(item, _)| write!(f, "\n{}", item.display_table_row(columns)))?;

        Ok(())
    }
}

impl fmt::Display for MagicItemListView<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "# {}", self.title)?;

        let tokens = self.category.item_tokens();

        let mut magic_items: Vec<&MagicItem> = self
            .magic_items
            .iter()
            .filter(|i| tokens.contains(&i.token()))
            .collect();

        magic_items.sort_by_key(|item| &item.name);

        magic_items
            .into_iter()
            .try_for_each(|item| write!(f, "\n* {}", item.display_summary()))?;

        Ok(())
    }
}
