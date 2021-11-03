mod geographical;
mod landmark;
mod settlement;

use initiative_macros::WordList;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, WordList, Serialize, Deserialize)]
#[serde(into = "&'static str", try_from = "&str")]
pub enum LocationType {
    #[term = "location"]
    Any,

    Geographical(geographical::GeographicalType),
    Landmark(landmark::LandmarkType),
    Settlement(settlement::SettlementType),
}
