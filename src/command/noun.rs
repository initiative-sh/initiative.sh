use std::str::FromStr;

use super::ParseError;

#[derive(Clone, Copy, Debug)]
pub enum Noun {
    Inn,
    Residence,
    Shop,
    Temple,
    Warehouse,
}

impl FromStr for Noun {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        Ok(match raw {
            "inn" => Noun::Inn,
            "residence" | "home" => Noun::Residence,
            "temple" | "church" => Noun::Temple,
            "shop" => Noun::Shop,
            "warehouse" => Noun::Warehouse,
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
    fn from(noun: Noun) -> String {
        match noun {
            Noun::Inn => "inn",
            Noun::Residence => "residence",
            Noun::Shop => "shop",
            Noun::Temple => "temple",
            Noun::Warehouse => "warehouse",
        }
        .to_string()
    }
}
