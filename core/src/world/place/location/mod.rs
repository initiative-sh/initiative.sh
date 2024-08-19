mod geographical;
mod landmark;
mod settlement;

use initiative_macros::WordList;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::world::Demographics;

use super::{PlaceData, PlaceType};

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

pub fn generate(place: &mut PlaceData, rng: &mut impl Rng, demographics: &Demographics) {
    #[allow(clippy::collapsible_match)]
    if let Some(PlaceType::Location(subtype)) = place.subtype.value() {
        #[allow(clippy::single_match)]
        match subtype {
            LocationType::Geographical(_) => geographical::generate(place, rng, demographics),
            _ => {}
        }
    }
}
