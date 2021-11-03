mod geography;
mod political;

use initiative_macros::WordList;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, WordList, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RegionType {
    #[term = "region"]
    Any,

    Geography(geography::GeographyType),
    Political(political::PoliticalType),
}
