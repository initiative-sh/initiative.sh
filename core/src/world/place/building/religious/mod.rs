mod shrine;
use initiative_macros::WordList;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::world::place::{PlaceData, PlaceType};
use crate::world::Demographics;

use super::BuildingType;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, WordList)]
#[serde(into = "&'static str", try_from = "&str")]
pub enum ReligiousType {
    Abbey,
    #[alias = "necropolis"]
    #[alias = "graveyard"]
    Cemetery,
    Crypt,
    Mausoleum,
    #[alias = "hermitage"]
    #[alias = "nunnery"]
    Monastery,
    Shrine,
    #[alias = "church"]
    #[alias = "mosque"]
    #[alias = "synagogue"]
    Temple,
    Tomb,
}

impl ReligiousType {
    pub const fn get_emoji(&self) -> Option<&'static str> {
        match self {
            Self::Abbey | Self::Monastery | Self::Shrine | Self::Temple => Some("🙏"),
            Self::Cemetery | Self::Crypt | Self::Mausoleum | Self::Tomb => Some("🪦"),
        }
    }
}

pub fn generate(place: &mut PlaceData, rng: &mut impl Rng, demographics: &Demographics) {
    #[expect(clippy::collapsible_match)]
    if let Some(PlaceType::Building(BuildingType::Religious(subtype))) = place.subtype.value() {
        #[expect(clippy::single_match)]
        match subtype {
            ReligiousType::Shrine => shrine::generate(place, rng, demographics),
            _ => {}
        }
    }
}
