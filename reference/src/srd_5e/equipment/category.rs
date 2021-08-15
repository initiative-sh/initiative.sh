use super::{Column, Item};
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

pub struct TableView<'a> {
    category: &'a ItemCategory,
    items: &'a [Item],
}

impl ItemCategory {
    pub fn name(&self) -> String {
        match self.name.as_str() {
            "Potion" | "Ring" | "Rod" | "Scroll" | "Wand" | "Weapon" => {
                let mut name = self.name.to_lowercase();
                name.push('s');
                name
            }
            "Staff" => "staves".to_string(),
            name => name.to_lowercase(),
        }
    }

    pub fn alt_names(&self) -> Vec<String> {
        match self.index.as_str() {
            "mounts-and-other-animals" => vec!["animals".to_string()],
            "waterborne-vehicles" => ["vehicles, waterborne", "ships", "boats"]
                .iter()
                .map(|&s| String::from(s))
                .collect(),
            _ if self.name.contains(' ') && !self.name.contains(" and ") => {
                let (start, end) = self.name.rsplit_once(' ').unwrap();
                vec![format!("{}, {}", end, start).to_lowercase()]
            }
            _ => Vec::new(),
        }
    }

    pub fn token(&self) -> String {
        crate::to_camel_case(self.index.as_str())
    }

    pub fn item_tokens(&self) -> Vec<String> {
        self.items.iter().map(|item| item.token()).collect()
    }

    pub fn display_table<'a>(&'a self, items: &'a [Item]) -> TableView {
        TableView {
            category: self,
            items,
        }
    }
}

impl<'a> fmt::Display for TableView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "# {}\n\n|",
            crate::capitalize(self.category.name().as_str())
        )?;

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
            .filter_map(|item| {
                if tokens.contains(&item.token()) {
                    Some((item, item.alt_name().unwrap_or_else(|| item.name())))
                } else {
                    None
                }
            })
            .collect();

        items.sort_by(|(_, a), (_, b)| a.cmp(b));

        items
            .iter()
            .try_for_each(|(item, _)| write!(f, "\n{}", item.display_table_row(columns)))?;

        Ok(())
    }
}
