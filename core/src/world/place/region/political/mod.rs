use initiative_macros::WordList;

#[derive(WordList)]
pub enum PoliticalType {
    Barony,
    CityState,
    Confederation,
    Country,
    County,
    Domain,
    Duchy,
    Empire,
    Kingdom,
    Nation,
    Principality,
    Province,
    Realm,
    Region,
    Territory,
}
