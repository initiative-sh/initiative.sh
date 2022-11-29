use initiative_macros::WordList;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, WordList)]
#[serde(into = "&'static str", try_from = "&str")]
pub enum TravelType {
    Bridge,
    DutyHouse,
    Ferry,
    Gate,
    Lighthouse,
    Market,
    Pier,
    Portal,
    Shipyard,
}

impl TravelType {
    pub const fn get_emoji(&self) -> Option<&'static str> {
        match self {
            Self::Bridge => Some("🌉"),
            Self::DutyHouse | Self::Market => Some("🪙"),
            Self::Ferry => Some("⛴"),
            Self::Gate => Some("🚪"),
            Self::Lighthouse | Self::Pier | Self::Shipyard => Some("⛵"),
            Self::Portal => None,
        }
    }
}
