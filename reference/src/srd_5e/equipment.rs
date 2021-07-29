use super::{write_text_block, Reference};
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
                format!("{} {}", end, start)
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

    pub fn display_table<'a>(&'a self, columns: &'a [Column]) -> TableView {
        TableView {
            equipment: self,
            columns,
        }
    }

    pub fn display_details(&self) -> DetailsView {
        DetailsView(self)
    }
}

pub struct TableView<'a> {
    equipment: &'a Equipment,
    columns: &'a [Column],
}

pub struct DetailsView<'a>(&'a Equipment);

pub enum Column {
    ArmorClass,
    CarryingCapacity,
    Cost,
    Damage,
    Name,
    Properties,
    Speed,
    Stealth,
    Strength,
    Weight,
}

impl<'a> fmt::Display for TableView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let equipment = &self.equipment;

        if !self.columns.is_empty() {
            write!(f, "|")?;
        }

        for column in self.columns {
            match column {
                Column::ArmorClass => equipment
                    .armor_class
                    .as_ref()
                    .map(|ac| write!(f, " {} |", ac)),
                Column::CarryingCapacity => {
                    equipment.capacity.as_ref().map(|c| write!(f, " {} |", c))
                }
                Column::Cost => Some(write!(f, " {} |", equipment.cost)),
                Column::Damage => equipment.damage.as_ref().map(|d| write!(f, " {} |", d)),
                Column::Name => Some(write!(f, " `{}` |", equipment.name())),
                Column::Properties => None,
                Column::Speed => equipment.speed.as_ref().map(|s| write!(f, " {} |", s)),
                Column::Stealth => {
                    if let Some(true) = equipment.stealth_disadvantage {
                        Some(write!(f, " disadvantage |"))
                    } else {
                        None
                    }
                }
                Column::Strength => equipment
                    .str_minimum
                    .map(|min| {
                        if min > 0 {
                            Some(write!(f, " Str {} |", min))
                        } else {
                            None
                        }
                    })
                    .flatten(),
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

        write!(
            f,
            "# {}\n*{}*\n",
            equipment.name(),
            equipment.equipment_category.name,
        )?;

        write!(f, "\n**Cost:** {}", equipment.cost)?;

        if let Some(damage) = &equipment.damage {
            write!(f, "\\\n**Damage:** {}", damage)?;
        }

        if let Some(ac) = &equipment.armor_class {
            write!(f, "\\\n**Armor Class (AC):** {}", ac)?;
        }

        if let Some(min) = equipment.str_minimum {
            if min == 0 {
                write!(f, "\\\n**Strength:** any")?;
            } else {
                write!(f, "\\\n**Strength:** {}+", min)?;
            }
        }

        if let Some(Range {
            normal,
            long: Some(long),
        }) = equipment.range
        {
            if equipment
                .throw_range
                .as_ref()
                .map_or(true, |throw| throw.long != Some(long))
            {
                write!(f, "\\\n**Range:** {}/{}", normal, long)?;
            }
        }

        if let Some(Range {
            normal,
            long: Some(long),
        }) = equipment.throw_range
        {
            write!(f, "\\\n**Range (thrown):** {}/{}", normal, long)?;
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

impl fmt::Display for ValueWithUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.quantity, self.unit)
    }
}
