pub use category::ItemCategory;
pub use item::Item;
pub use magic_item::MagicItem;

mod category;
mod item;
mod magic_item;

use std::fmt;

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

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ArmorClass => write!(f, "Armor Class (AC)"),
            Self::CarryingCapacity => write!(f, "Carrying Capacity"),
            Self::Cost => write!(f, "Cost"),
            Self::Damage => write!(f, "Damage"),
            Self::Name => write!(f, "Name"),
            Self::Properties => write!(f, "Properties"),
            Self::Speed => write!(f, "Speed"),
            Self::Stealth => write!(f, "Stealth"),
            Self::Strength => write!(f, "Strength"),
            Self::Weight => write!(f, "Weight"),
        }
    }
}
