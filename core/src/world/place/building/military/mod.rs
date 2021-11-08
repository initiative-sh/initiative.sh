use initiative_macros::WordList;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, WordList, Serialize, Deserialize)]
#[serde(into = "&'static str", try_from = "&str")]
pub enum MilitaryType {
    Barracks,
    Base,
    Castle,
    Citadel,
    Fort,
    Fortress,
    Keep,
    Stronghold,
    Tower,
}

impl MilitaryType {
    pub const fn get_emoji(&self) -> Option<&'static str> {
        match self {
            Self::Castle
            | Self::Citadel
            | Self::Fort
            | Self::Fortress
            | Self::Keep
            | Self::Stronghold
            | Self::Tower => Some("🏰"),
            Self::Barracks | Self::Base => Some("⚔"),
        }
    }
}
