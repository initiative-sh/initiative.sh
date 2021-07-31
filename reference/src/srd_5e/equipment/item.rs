use super::Column;
use crate::srd_5e::{write_text_block, Reference};
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct Equipment {
    index: String,
    name: String,

    cost: ValueWithUnit,
    weight: Option<f32>,
    speed: Option<ValueWithUnit>,
    damage: Option<Damage>,
    two_handed_damage: Option<Damage>,
    armor_class: Option<ArmorClass>,
    str_minimum: Option<u8>,
    stealth_disadvantage: Option<bool>,
    capacity: Option<String>,
    range: Option<Range>,
    throw_range: Option<Range>,

    #[serde(default)]
    properties: Vec<Reference>,

    #[serde(default)]
    desc: Vec<String>,

    equipment_category: Reference,
    gear_category: Option<Reference>,
    armor_category: Option<String>,
    vehicle_category: Option<String>,
    tool_category: Option<String>,
    category_range: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ArmorClass {
    base: u8,
    dex_bonus: bool,
    max_bonus: Option<u8>,
}

#[derive(Debug, Deserialize)]
pub struct Damage {
    damage_dice: String,
    damage_type: Reference,
}

#[derive(Debug, Deserialize)]
pub struct ValueWithUnit {
    quantity: f32,
    unit: String,
}

#[derive(Debug, Deserialize)]
pub struct Range {
    normal: u16,
    long: Option<u16>,
}

impl Equipment {
    pub fn name(&self) -> String {
        let mut name = if self.name.contains(", ") {
            if let Some((start, end)) = self.name.split_once(", ") {
                if let Some((end, end_paren)) = end.split_once(" (") {
                    format!("{} {} ({}", end, start, end_paren)
                } else {
                    format!("{} {}", end, start)
                }
            } else {
                unreachable!();
            }
        } else {
            self.name.to_owned()
        };

        if self.equipment_category.index == "armor"
            && !name.contains(' ')
            && !["Breastplate", "Shield"].contains(&name.as_str())
        {
            name.push_str(" Armor");
        }

        crate::capitalize(name.as_str())
    }

    pub fn alt_name(&self) -> Option<String> {
        if self.name.contains(", ") {
            Some(crate::capitalize(self.name.as_str()))
        } else {
            None
        }
    }

    pub fn token(&self) -> String {
        crate::to_camel_case(self.index.as_str())
    }

    pub fn display_table_row<'a>(&'a self, columns: &'a [Column]) -> TableRowView {
        TableRowView {
            equipment: self,
            columns,
        }
    }

    pub fn display_details(&self) -> DetailsView {
        DetailsView(self)
    }

    pub fn get_category(&self) -> String {
        if self.name == "Weapon" {
            "Weapons".to_string()
        } else {
            self.name.clone()
        }
    }

    pub fn get_subcategory(&self) -> Option<String> {
        match self.equipment_category.index.as_str() {
            "adventuring-gear" => self
                .gear_category
                .as_ref()
                .map(|reference| reference.name.to_owned()),
            "armor" => self.armor_category.clone(),
            "mounts-and-vehicles" => self.vehicle_category.clone(),
            "tools" => self.tool_category.clone(),
            "weapon" => self.category_range.clone(),
            _ => None,
        }
    }
}

pub struct TableRowView<'a> {
    equipment: &'a Equipment,
    columns: &'a [Column],
}

pub struct DetailsView<'a>(&'a Equipment);

impl<'a> fmt::Display for TableRowView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let equipment = &self.equipment;

        if !self.columns.is_empty() {
            write!(f, "|")?;
        }

        for column in self.columns {
            match column {
                Column::ArmorClass => equipment.armor_class.as_ref().map(|ac| {
                    if equipment
                        .armor_category
                        .as_ref()
                        .map_or(false, |c| c == "Shield")
                    {
                        write!(f, " +{} |", ac)
                    } else {
                        write!(f, " {} |", ac)
                    }
                }),
                Column::CarryingCapacity => {
                    equipment.capacity.as_ref().map(|c| write!(f, " {} |", c))
                }
                Column::Cost => Some(write!(f, " {} |", equipment.cost)),
                Column::Damage => equipment.damage.as_ref().map(|d| write!(f, " {} |", d)),
                Column::Name => Some(write!(
                    f,
                    " `{}` |",
                    equipment.alt_name().unwrap_or_else(|| equipment.name()),
                )),
                Column::Properties => None,
                Column::Speed => equipment.speed.as_ref().map(|s| write!(f, " {} |", s)),
                Column::Stealth => equipment.stealth_disadvantage.map(|d| {
                    if d {
                        write!(f, " disadvantage |")
                    } else {
                        write!(f, " \u{2014} |")
                    }
                }),
                Column::Strength => equipment.str_minimum.map(|min| {
                    if min > 0 {
                        write!(f, " Str {} |", min)
                    } else {
                        write!(f, " \u{2014} |")
                    }
                }),
                Column::Weight => equipment.weight.map(|w| write!(f, " {} lb. |", w)),
            }
            .unwrap_or_else(|| write!(f, " |"))?;
        }

        Ok(())
    }
}

impl<'a> fmt::Display for DetailsView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let equipment = self.0;

        writeln!(f, "# {}", equipment.name())?;

        if let Some(subcategory) = equipment.get_subcategory() {
            writeln!(
                f,
                "*{} ({})*",
                equipment.equipment_category.name, subcategory
            )?;
        } else {
            writeln!(f, "*{}", equipment.equipment_category.name)?;
        }

        write!(f, "\n**Cost:** {}", equipment.cost)?;

        if let Some(damage) = &equipment.damage {
            write!(f, "\\\n**Damage:** {}", damage)?;
        }

        if let Some(ac) = &equipment.armor_class {
            if equipment
                .armor_category
                .as_ref()
                .map_or(false, |c| c == "Shield")
            {
                write!(f, "\\\n**Armor Class (AC):** +{}", ac)?;
            } else {
                write!(f, "\\\n**Armor Class (AC):** {}", ac)?;
            }
        }

        if let Some(min) = equipment.str_minimum {
            if min == 0 {
                write!(f, "\\\n**Strength:** any")?;
            } else {
                write!(f, "\\\n**Strength:** {}+", min)?;
            }
        }

        if let Some(throw_range) = &equipment.throw_range {
            write!(f, "\\\n**Range (thrown):** {}", throw_range)?;
        } else if let Some(range) = &equipment.range {
            if range.normal > 5 || range.long.is_some() {
                write!(f, "\\\n**Range:** {}", range)?;
            }
        }

        if let Some(disadvantage) = equipment.stealth_disadvantage {
            if disadvantage {
                write!(f, "\\\n**Stealth:** disadvantage")?;
            } else {
                write!(f, "\\\n**Stealth:** no impact")?;
            }
        }

        if let Some(weight) = &equipment.weight {
            write!(f, "\\\n**Weight:** {} lbs", weight)?;
        }

        // TODO: Properties

        if let Some(speed) = &equipment.speed {
            write!(f, "\\\n**Speed:** {}", speed)?;
        }

        if let Some(capacity) = &equipment.capacity {
            write!(f, "\\\n**Carrying Capacity:** {}", capacity)?;
        }

        if !equipment.desc.is_empty() {
            write!(f, "\n\n")?;
            write_text_block(f, &equipment.desc)?;
        }

        Ok(())
    }
}

impl fmt::Display for ArmorClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.base)?;
        if self.dex_bonus {
            write!(f, " + Dex modifier")?;
            if let Some(max_bonus) = self.max_bonus {
                write!(f, " (max {})", max_bonus)?;
            }
        }

        Ok(())
    }
}

impl fmt::Display for Damage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {}",
            self.damage_dice,
            self.damage_type.name.to_lowercase()
        )
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(long) = self.long {
            write!(f, "{}/{}", self.normal, long)
        } else {
            write!(f, "{}", self.normal)
        }
    }
}

impl fmt::Display for ValueWithUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.quantity, self.unit)
    }
}
