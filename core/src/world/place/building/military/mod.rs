use initiative_macros::WordList;

#[derive(WordList)]
pub enum MilitaryType {
    Barracks,
    Base,
    Castle,
    Citadel,
    Fort,
    Fortress,
    Keep,
    Stronghold,
    Tower,
}
