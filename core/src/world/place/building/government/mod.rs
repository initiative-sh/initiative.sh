use initiative_macros::WordList;

#[derive(WordList)]
pub enum GovernmentType {
    Court,
    Dungeon,
    Embassy,
    #[alias = "watch-house"]
    Guardhouse,
    Palace,
    #[alias = "jail"]
    Prison,
}
