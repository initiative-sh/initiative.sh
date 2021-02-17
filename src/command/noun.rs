use std::str::FromStr;

use super::ParseError;

#[derive(Debug)]
pub enum Noun {
    Inn,
}

impl FromStr for Noun {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        Ok(match raw {
            "inn" => Self::Inn,
            _ => {
                return Err(ParseError {
                    message: "Unknown command.".to_string(),
                    input: raw.to_string(),
                    highlight: None,
                })
            }
        })
    }
}

impl From<Noun> for String {
    fn from(noun: Noun) -> Self {
        match noun {
            Noun::Inn => "inn",
        }
        .to_string()
    }
}
