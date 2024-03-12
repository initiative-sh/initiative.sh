mod inn;
mod theater;

use super::BuildingType;
use crate::world::place::{Place, PlaceType};
use crate::world::Demographics;
use initiative_macros::WordList;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, WordList)]
#[serde(into = "&'static str", try_from = "&str")]
pub enum BusinessType {
    #[term = "business"]
    #[alias = "shop"]
    #[alias = "store"]
    Any,

    Arena,
    Armorer,
    Bakery,
    Bank,
    #[alias = "nightclub"]
    #[alias = "pub"]
    Bar,
    Bathhouse,
    #[alias = "smithy"]
    Blacksmith,
    Brewery,
    #[alias = "gambling-hall"]
    Casino,
    Club,
    Distillery,
    FightingPit,
    FoodCounter,
    Forge,
    FurnitureShop,
    Furrier,
    GeneralStore,
    GuildHall,
    ImportsShop,
    #[alias = "caravansary"]
    #[alias = "hotel"]
    #[alias = "lodge"]
    #[alias = "tavern"]
    Inn,
    Jeweller,
    Lumberyard,
    MagicShop,
    Mill,
    PetStore,
    Restaurant,
    SpecialtyShop,
    SpiritsShop,
    Stable,
    TextilesShop,
    Theater,
    TradingPost,
    Vault,
    Wainwright,
    Warehouse,
    Weaponsmith,
    Woodshop,
}

impl BusinessType {
    pub const fn get_emoji(&self) -> Option<&'static str> {
        match self {
            Self::Arena => Some("🏛"),
            Self::Armorer => Some("🛡"),
            Self::Bakery => Some("🍞"),
            Self::Bank | Self::Vault => Some("🏦"),
            Self::Bar => Some("🍻"),
            Self::Bathhouse => Some("🛁"),
            Self::Blacksmith | Self::Weaponsmith => Some("🗡"),
            Self::Brewery => Some("🍻"),
            Self::Casino => Some("🃏"),
            Self::Club => Some(""),
            Self::Distillery => Some("🥃"),
            Self::FightingPit => Some("⚔"),
            Self::FoodCounter => Some("🍲"),
            Self::Forge => Some("🔥"),
            Self::FurnitureShop => Some("🪑"),
            Self::Furrier => Some("🦊"),
            Self::Inn => Some("🏨"),
            Self::Jeweller => Some("💍"),
            Self::Lumberyard => Some("🪵"),
            Self::MagicShop => Some("🪄"),
            Self::Mill => Some("🌾"),
            Self::PetStore => Some("🐶"),
            Self::Restaurant => Some("🍽"),
            Self::SpiritsShop => Some("🥃"),
            Self::Stable => Some("🐎"),
            Self::Theater => Some("🎭"),
            Self::Warehouse => Some("📦"),
            Self::Woodshop => Some("🪚"),

            Self::Any
            | Self::GeneralStore
            | Self::GuildHall
            | Self::ImportsShop
            | Self::SpecialtyShop
            | Self::TextilesShop
            | Self::TradingPost
            | Self::Wainwright => Some("🪙"),
        }
    }
}

pub fn generate(place: &mut Place, rng: &mut impl Rng, demographics: &Demographics) {
    #[allow(clippy::collapsible_match)]
    if let Some(PlaceType::Building(BuildingType::Business(subtype))) = place.subtype.value() {
        #[allow(clippy::single_match)]
        match subtype {
            BusinessType::Inn => inn::generate(place, rng, demographics),
            BusinessType::Theater => theater::generate(place, rng, demographics),
            _ => {}
        }
    }
}
