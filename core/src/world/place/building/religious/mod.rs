use initiative_macros::WordList;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, WordList, Serialize, Deserialize)]
#[serde(into = "&'static str", try_from = "&str")]
pub enum ReligiousType {
    Abbey,
    #[alias = "necropolis"]
    #[alias = "graveyard"]
    Cemetery,
    Crypt,
    Mausoleum,
    #[alias = "hermitage"]
    #[alias = "nunnery"]
    Monastery,
    Shrine,
    #[alias = "church"]
    #[alias = "mosque"]
    #[alias = "synagogue"]
    Temple,
    Tomb,
}
