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
        None
    }
}
