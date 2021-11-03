use initiative_macros::WordList;

#[derive(WordList)]
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
