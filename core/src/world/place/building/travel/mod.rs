use initiative_macros::WordList;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, WordList, Serialize, Deserialize)]
pub enum TravelType {
    Bridge,
    DutyHouse,
    Ferry,
    Gate,
    Lighthouse,
    Market,
    Pier,
    Portal,
    Shipyard,
}
