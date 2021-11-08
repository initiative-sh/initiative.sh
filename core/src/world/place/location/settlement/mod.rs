use initiative_macros::WordList;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, WordList, Serialize, Deserialize)]
#[serde(into = "&'static str", try_from = "&str")]
pub enum SettlementType {
    #[alias = "campsite"]
    Camp,
    Capital,
    #[alias = "metropolis"]
    City,
    #[alias = "ward"]
    #[alias = "quarter"]
    #[alias = "neighborhood"]
    District,
    Outpost,
    #[alias = "hamlet"]
    #[alias = "village"]
    #[alias = "parish"]
    Town,
}

impl SettlementType {
    pub const fn get_emoji(&self) -> Option<&'static str> {
        match self {
            Self::Camp => Some("🏕"),
            Self::Capital | Self::City => Some("🏙"),
            Self::Outpost => Some("🚩"),
            Self::Town => Some("🏘"),
            Self::District => None,
        }
    }
}
