mod inn;

use super::BuildingType;
use crate::world::place::{Place, PlaceType};
use crate::world::Demographics;
use initiative_macros::WordList;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, WordList, Serialize, Deserialize)]
pub enum BusinessType {
    #[term = "business"]
    #[alias = "shop"]
    #[alias = "store"]
    Any,

    Arena,
    Armorer,
    Bakery,
    Bank,
    Bathhouse,
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
    Inn,
    Jeweller,
    Lumberyard,
    MagicShop,
    Mill,
    PetStore,
    Restaurant,
    Smithy,
    SpecialtyShop,
    SpiritsShop,
    Stable,
    #[alias = "bar"]
    #[alias = "nightclub"]
    Tavern,
    TextilesShop,
    Theater,
    TradingPost,
    Vault,
    Wainwright,
    Warehouse,
    Weaponsmith,
    Woodshop,
}

pub fn generate(place: &mut Place, rng: &mut impl Rng, demographics: &Demographics) {
    #[allow(clippy::collapsible_match)]
    if let Some(PlaceType::Building(BuildingType::Business(subtype))) = place.subtype.value() {
        #[allow(clippy::single_match)]
        match subtype {
            BusinessType::Inn => inn::generate(place, rng, demographics),
            _ => {}
        }
    }
}
