mod geographical;
mod landmark;
mod settlement;

use initiative_macros::WordList;

#[derive(WordList)]
pub enum LocationType {
    #[term = "location"]
    Any,

    Geographical(geographical::GeographicalType),
    Landmark(landmark::LandmarkType),
    Settlement(settlement::SettlementType),
}
