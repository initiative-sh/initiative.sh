use initiative_macros::WordList;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, WordList)]
#[serde(into = "&'static str", try_from = "&str")]
pub enum PoliticalType {
    Barony,
    CityState,
    Confederation,
    Country,
    County,
    Domain,
    Duchy,
    Empire,
    Kingdom,
    Nation,
    Principality,
    Province,
    Realm,
    Region,
    Territory,
}

impl PoliticalType {
    pub const fn get_emoji(&self) -> Option<&'static str> {
        Some("👑")
    }
}
