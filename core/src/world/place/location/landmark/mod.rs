use initiative_macros::WordList;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, WordList)]
#[serde(into = "&'static str", try_from = "&str")]
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

impl LandmarkType {
    pub const fn get_emoji(&self) -> Option<&'static str> {
        match self {
            Self::Farm | Self::Garden => Some("🌱"),
            Self::Fountain => Some("⛲"),
            Self::Harbor => Some("⛵"),
            Self::Mine => Some("⚒"),
            Self::Ruin => Some("🏚"),
            Self::Street => Some("🏘"),
            Self::Wall => Some("🧱"),
            Self::Monument => Some("🗽"),
        }
    }
}
