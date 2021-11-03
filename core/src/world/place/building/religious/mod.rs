use initiative_macros::WordList;

#[derive(WordList)]
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
