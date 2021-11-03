use initiative_macros::WordList;

#[derive(WordList)]
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
