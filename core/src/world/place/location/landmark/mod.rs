use initiative_macros::WordList;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, WordList, Serialize, Deserialize)]
pub enum LandmarkType {
    Farm,
    Fountain,
    Garden,
    Harbor,
    Mine,
    #[alias = "statue"]
    Monument,
    Ruin,
    Street,
    Wall,
}