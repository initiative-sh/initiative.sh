use std::str::FromStr;

use super::ParseError;

use initiative_macros::WordList;

#[derive(Debug, WordList)]
pub enum Verb {
    Tutorial,
    Help,
    Quit,
}
