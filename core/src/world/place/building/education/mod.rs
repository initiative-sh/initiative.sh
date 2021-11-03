use initiative_macros::WordList;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, WordList, Serialize, Deserialize)]
pub enum EducationType {
    Academy,
    College,
    Library,
    School,
    University,
}
