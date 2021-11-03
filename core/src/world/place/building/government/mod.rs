use initiative_macros::WordList;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, WordList, Serialize, Deserialize)]
pub enum GovernmentType {
    Court,
    // Dungeon,
    Embassy,
    #[alias = "watch-house"]
    Guardhouse,
    Palace,
    #[alias = "jail"]
    Prison,
}
