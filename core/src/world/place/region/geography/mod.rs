use initiative_macros::WordList;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, WordList, Serialize, Deserialize)]
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