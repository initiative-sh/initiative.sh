mod geography;
mod political;

use initiative_macros::WordList;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, WordList)]
#[serde(into = "&'static str", try_from = "&str")]
pub enum RegionType {
    #[term = "region"]
    Any,

    Geography(geography::GeographyType),
    Political(political::PoliticalType),
}

impl RegionType {
    pub const fn get_emoji(&self) -> Option<&'static str> {
        match self {
            Self::Any => None,
            Self::Geography(subtype) => subtype.get_emoji(),
            Self::Political(subtype) => subtype.get_emoji(),
        }
    }
}
