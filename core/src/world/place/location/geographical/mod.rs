use initiative_macros::WordList;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, WordList, Serialize, Deserialize)]
pub enum GeographicalType {
    Beach,
    #[alias = "gorge"]
    Canyon,
    #[alias = "cavern"]
    Cave,
    Chasm,
    Glacier,
    Grove,
    Hill,
    Island,
    Monolith,
    Oasis,
    Pass,
    Peninsula,
    Ridge,
    Rift,
    River,
    Tree,
    #[alias = "vale"]
    Valley,
}
