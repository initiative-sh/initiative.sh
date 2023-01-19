mod business;
mod education;
mod government;
mod military;
mod religious;
mod travel;

use crate::world::place::{Place, PlaceType};
use crate::world::Demographics;
use initiative_macros::WordList;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, WordList)]
#[serde(into = "&'static str", try_from = "&str")]
pub enum BuildingType {
    #[term = "building"]
    Any,

    Business(business::BusinessType),
    Education(education::EducationType),
    Government(government::GovernmentType),
    Military(military::MilitaryType),
    Religious(religious::ReligiousType),
    #[alias = "house"]
    #[alias = "manor"]
    #[alias = "mansion"]
    Residence,
    Travel(travel::TravelType),
}

impl BuildingType {
    pub const fn get_emoji(&self) -> Option<&'static str> {
        match self {
            Self::Any => None,
            Self::Business(subtype) => subtype.get_emoji(),
            Self::Education(subtype) => subtype.get_emoji(),
            Self::Government(subtype) => subtype.get_emoji(),
            Self::Military(subtype) => subtype.get_emoji(),
            Self::Religious(subtype) => subtype.get_emoji(),
            Self::Residence => Some("🏠"),
            Self::Travel(subtype) => subtype.get_emoji(),
        }
    }
}

pub fn generate(place: &mut Place, rng: &mut impl Rng, demographics: &Demographics) {
    #[allow(clippy::collapsible_match)]
    if let Some(PlaceType::Building(subtype)) = place.subtype.value() {
        #[allow(clippy::single_match)]
        match subtype {
            BuildingType::Business(_) => business::generate(place, rng, demographics),
            BuildingType::Religious(_) => religious::generate(place, rng, demographics),
            _ => {}
        }
    }
}
