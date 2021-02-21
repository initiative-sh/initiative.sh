use initiative_macros::WordList;

#[derive(Clone, Copy, Debug, WordList)]
pub enum Noun {
    Inn,
    Residence,
    Shop,
    Temple,
    Warehouse,
}
