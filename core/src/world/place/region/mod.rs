mod geography;
mod political;

use initiative_macros::WordList;

#[derive(WordList)]
pub enum RegionType {
    #[term = "region"]
    Any,

    Geography(geography::GeographyType),
    Political(political::PoliticalType),
}
