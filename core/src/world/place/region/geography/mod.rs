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
        None
    }
}
