use initiative_macros::WordList;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, WordList, Serialize, Deserialize)]
#[serde(into = "&'static str", try_from = "&str")]
pub enum SettlementType {
    Camp,
    Campsite,
    Capital,
    #[alias = "metropolis"]
    City,
    #[alias = "ward"]
    #[alias = "quarter"]
    #[alias = "neighborhood"]
    District,
    Outpost,
    #[alias = "hamlet"]
    #[alias = "village"]
    #[alias = "parish"]
    Town,
}
