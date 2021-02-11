use super::ParseError;
use std::str::FromStr;

#[derive(Debug)]
pub enum Verb {
    Tutorial,
    Help,
    Quit,
}

impl FromStr for Verb {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        Ok(match raw {
            "tutorial" => Self::Tutorial,
            "help" => Self::Help,
            "quit" | "exit" | "q" => Self::Quit,
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

impl From<Verb> for String {
    fn from(verb: Verb) -> Self {
        match verb {
            Verb::Tutorial => "tutorial",
            Verb::Help => "help",
            Verb::Quit => "quit",
        }
        .to_string()
    }
}
