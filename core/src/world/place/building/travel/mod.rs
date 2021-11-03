use initiative_macros::WordList;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, WordList, Serialize, Deserialize)]
#[serde(into = "&'static str", try_from = "&str")]
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
