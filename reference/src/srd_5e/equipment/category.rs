use super::{Column, Equipment};
use crate::srd_5e::Reference;
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct EquipmentCategory {
    index: String,
    name: String,
    equipment: Vec<Reference>,
}

pub struct TableView<'a> {
    category: &'a EquipmentCategory,
    equipment: &'a [Equipment],
}

impl EquipmentCategory {
    pub fn name(&self) -> String {
        if self.name == "Weapon" {
            "weapons".to_string()
        } else {
            self.name.to_lowercase()
        }
    }

    pub fn alt_name(&self) -> Option<String> {
        if self.name.contains(' ') && !self.name.contains(" and ") {
            let (start, end) = self.name.rsplit_once(' ').unwrap();
            Some(format!("{}, {}", end, start).to_lowercase())
        } else {
            None
        }
    }

    pub fn token(&self) -> String {
        crate::to_camel_case(self.index.as_str())
    }

    pub fn equipment_tokens(&self) -> Vec<String> {
        self.equipment.iter().map(|item| item.token()).collect()
    }

    pub fn display_table<'a>(&'a self, equipment: &'a [Equipment]) -> TableView {
        TableView {
            category: self,
            equipment,
        }
    }
}

impl<'a> fmt::Display for TableView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "# {}\n\n|",
            crate::capitalize(self.category.name.as_str())
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

        let tokens = self.category.equipment_tokens();

        let mut items: Vec<(&Equipment, String)> = self
            .equipment
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
