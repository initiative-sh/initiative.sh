mod geographical;
mod landmark;
mod settlement;

use initiative_macros::WordList;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, WordList)]
#[serde(into = "&'static str", try_from = "&str")]
pub enum LocationType {
    #[term = "location"]
    Any,

    Geographical(geographical::GeographicalType),
    Landmark(landmark::LandmarkType),
    Settlement(settlement::SettlementType),
}

impl LocationType {
    pub const fn get_emoji(&self) -> Option<&'static str> {
        match self {
            Self::Any => None,
            Self::Geographical(subtype) => subtype.get_emoji(),
            Self::Landmark(subtype) => subtype.get_emoji(),
            Self::Settlement(subtype) => subtype.get_emoji(),
        }
    }
}
