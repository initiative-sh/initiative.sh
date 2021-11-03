use initiative_macros::WordList;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, WordList, Serialize, Deserialize)]
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
