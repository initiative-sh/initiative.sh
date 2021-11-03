mod geography;
mod political;

use initiative_macros::WordList;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, WordList, Serialize, Deserialize)]
#[serde(into = "&'static str", try_from = "&str")]
pub enum RegionType {
    #[term = "region"]
    Any,

    Geography(geography::GeographyType),
    Political(political::PoliticalType),
}
