use crate::app::{Command, Context, Runnable};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum StorageCommand {
    Load { query: String },
}

impl StorageCommand {
    pub fn run(&self, context: &mut Context) -> String {
        match self {
            Self::Load { query } => {
                let lowercase_query = query.to_lowercase();
                if let Some(result) = context.recent().iter().find(|t| {
                    t.name()
                        .value()
                        .map_or(false, |s| s.to_lowercase() == lowercase_query)
                }) {
                    format!("{}", result.display_details())
                } else {
                    format!("No matches for \"{}\"", query)
                }
            }
        }
    }
}

impl FromStr for StorageCommand {
    type Err = ();

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw.starts_with(char::is_uppercase) {
            Ok(Self::Load {
                query: raw.to_string(),
            })
        } else {
            Err(())
        }
    }
}

impl Runnable for StorageCommand {
    fn autocomplete(input: &str, context: &Context) -> Vec<(String, Command)> {
        if !input
            .chars()
            .next()
            .map(char::is_uppercase)
            .unwrap_or_default()
        {
            Vec::new()
        } else {
            let mut suggestions: Vec<String> = context
                .recent()
                .iter()
                .filter_map(|thing| thing.name().value())
                .filter(|word| word.starts_with(input))
                .cloned()
                .collect();

            suggestions.sort();
            suggestions.truncate(10);

            suggestions
                .drain(..)
                .filter_map(|s| s.parse().ok().map(|c| (s, Command::Storage(c))))
                .collect()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::world::{Location, Npc};

    #[test]
    fn from_str_test() {
        let parsed_command = "Gandalf the Grey".parse();
        if let Ok(StorageCommand::Load { query }) = parsed_command {
            assert_eq!("Gandalf the Grey", query.as_str());
        } else {
            panic!("{:?}", parsed_command);
        }

        let parsed_command = "potato".parse::<StorageCommand>();
        assert!(matches!(parsed_command, Err(())), "{:?}", parsed_command);
    }

    #[test]
    fn autocomplete_test() {
        let mut context = Context::default();

        context.push_recent(
            Npc {
                name: "Potato Johnson".into(),
                ..Default::default()
            }
            .into(),
        );

        context.push_recent(
            Npc {
                name: "potato should be capitalized".into(),
                ..Default::default()
            }
            .into(),
        );

        context.push_recent(
            Location {
                name: "Potato & Potato, Esq.".into(),
                ..Default::default()
            }
            .into(),
        );

        context.push_recent(
            Location {
                name: "Spud Stop".into(),
                ..Default::default()
            }
            .into(),
        );

        assert_eq!(
            vec![
                (
                    "Potato & Potato, Esq.".to_string(),
                    Command::Storage(StorageCommand::Load {
                        query: "Potato & Potato, Esq.".to_string(),
                    })
                ),
                (
                    "Potato Johnson".to_string(),
                    Command::Storage(StorageCommand::Load {
                        query: "Potato Johnson".to_string(),
                    })
                ),
            ],
            StorageCommand::autocomplete("P", &context),
        );

        assert!(StorageCommand::autocomplete("p", &context).is_empty());
        assert!(StorageCommand::autocomplete("", &context).is_empty());
    }
}
