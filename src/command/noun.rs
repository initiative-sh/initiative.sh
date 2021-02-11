use super::ParseError;
use std::str::FromStr;

pub enum Noun {}

impl FromStr for Noun {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        Err(ParseError {
            message: "Something something".to_string(),
            input: raw.to_string(),
            highlight: None,
        })
    }
}
