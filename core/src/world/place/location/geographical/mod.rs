use initiative_macros::WordList;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, WordList)]
#[serde(into = "&'static str", try_from = "&str")]
pub enum GeographicalType {
    Beach,
    #[alias = "gorge"]
    Canyon,
    #[alias = "cavern"]
    Cave,
    Chasm,
    Glacier,
    Grove,
    Hill,
    Island,
    Monolith,
    Oasis,
    Pass,
    Peninsula,
    Ridge,
    Rift,
    River,
    Tree,
    #[alias = "vale"]
    Valley,
}

impl GeographicalType {
    pub const fn get_emoji(&self) -> Option<&'static str> {
        match self {
            Self::Beach => Some("🏖"),
            Self::Canyon | Self::Chasm | Self::River | Self::Valley => Some("🏞"),
            Self::Glacier => Some("🏔"),
            Self::Grove | Self::Tree => Some("🌳"),
            Self::Hill | Self::Pass | Self::Ridge => Some("⛰"),
            Self::Island | Self::Peninsula => Some("🏝"),
            Self::Monolith => Some("🗿"),
            Self::Oasis => Some("🌴"),
            Self::Cave | Self::Rift => None,
        }
    }
}
