use initiative_macros::WordList;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, WordList, Serialize, Deserialize)]
#[serde(into = "&'static str", try_from = "&str")]
pub enum GovernmentType {
    Court,
    // Dungeon,
    Embassy,
    #[alias = "watch-house"]
    Guardhouse,
    Palace,
    #[alias = "jail"]
    Prison,
}

impl GovernmentType {
    pub const fn get_emoji(&self) -> Option<&'static str> {
        match self {
            Self::Embassy => Some("🚩"),
            Self::Guardhouse | Self::Prison => Some("🛡"),
            Self::Court | Self::Palace => Some("🏰"),
        }
    }
}
