mod collection;
mod struct_cases;
mod tuple_cases;
mod unit_cases;

use initiative_macros::WordList;

#[derive(Debug, PartialEq, WordList)]
#[allow(dead_code)]
enum Colors {
    Black,
    Blue,
    Green,
    Orange,
    Purple,
    Red,
    White,
    Yellow,
}
