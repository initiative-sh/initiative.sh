use initiative_macros::WordList;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, WordList, Serialize, Deserialize)]
#[serde(into = "&'static str", try_from = "&str")]
pub enum EducationType {
    Academy,
    College,
    Library,
    School,
    University,
}

impl EducationType {
    pub const fn get_emoji(&self) -> Option<&'static str> {
        None
    }
}
