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

#[derive(Clone, Copy, Debug, PartialEq, WordList, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BuildingType {
    #[term = "building"]
    Any,

    Business(business::BusinessType),
    Education(education::EducationType),
    Government(government::GovernmentType),
    Military(military::MilitaryType),
    Religious(religious::ReligiousType),
    #[alias = "house"]
    #[alias = "lodge"]
    #[alias = "manor"]
    #[alias = "mansion"]
    Residence,
    Travel(travel::TravelType),
}

pub fn generate(place: &mut Place, rng: &mut impl Rng, demographics: &Demographics) {
    #[allow(clippy::collapsible_match)]
    if let Some(PlaceType::Building(subtype)) = place.subtype.value() {
        #[allow(clippy::single_match)]
        match subtype {
            BuildingType::Business(_) => business::generate(place, rng, demographics),
            _ => {}
        }
    }
}