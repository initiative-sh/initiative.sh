use initiative_macros::WordList;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, WordList, Serialize, Deserialize)]
#[serde(into = "&'static str", try_from = "&str")]
pub enum GeographyType {
    Archipelago,
    Barrens,
    Coastline,
    Continent,
    Desert,
    Forest,
    Jungle,
    Lake,
    Marsh,
    Mesa,
    Moor,
    Mountain,
    Ocean,
    Plain,
    Plateau,
    Reef,
    Sea,
    Swamp,
    Tundra,
    Wasteland,
    World,
}

impl GeographyType {
    pub const fn get_emoji(&self) -> Option<&'static str> {
        match self {
            Self::Archipelago => Some("🏝"),
            Self::Barrens | Self::Desert | Self::Wasteland => Some("🏜"),
            Self::Coastline | Self::Lake | Self::Sea | Self::Ocean => Some("🌊"),
            Self::Forest | Self::Jungle => Some("🌳"),
            Self::Mountain => Some("⛰"),
            Self::Tundra => Some("❄"),
            Self::World => Some("🌐"),
            Self::Continent
            | Self::Marsh
            | Self::Mesa
            | Self::Moor
            | Self::Plain
            | Self::Plateau
            | Self::Reef
            | Self::Swamp => None,
        }
    }
}
