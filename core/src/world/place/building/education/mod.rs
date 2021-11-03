use initiative_macros::WordList;

#[derive(WordList)]
pub enum EducationType {
    Academy,
    College,
    Library,
    School,
    University,
}
