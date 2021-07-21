use crate::app::{Context, Runnable};
use rand::Rng;

#[derive(Debug, PartialEq)]
pub enum StorageCommand {
    Load { query: String },
}

impl Runnable for StorageCommand {
    fn run(&self, context: &mut Context, _rng: &mut impl Rng) -> String {
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

    fn parse_input(input: &str, _context: &Context) -> Vec<Self> {
        if input.starts_with(char::is_uppercase) {
            vec![Self::Load {
                query: input.to_string(),
            }]
        } else {
            Vec::new()
        }
    }

    fn autocomplete(input: &str, context: &Context) -> Vec<(String, Self)> {
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
                .iter()
                .flat_map(|s| std::iter::repeat(s).zip(Self::parse_input(s.as_str(), context)))
                .map(|(s, c)| (s.clone(), c))
                .collect()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::world::{Location, Npc};

    #[test]
    fn parse_input_test() {
        let context = Context::default();

        assert_eq!(
            vec![StorageCommand::Load {
                query: "Gandalf the Grey".to_string()
            }],
            StorageCommand::parse_input("Gandalf the Grey", &context),
        );

        assert_eq!(
            Vec::<StorageCommand>::new(),
            StorageCommand::parse_input("potato", &context),
        );
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
                    StorageCommand::Load {
                        query: "Potato & Potato, Esq.".to_string(),
                    }
                ),
                (
                    "Potato Johnson".to_string(),
                    StorageCommand::Load {
                        query: "Potato Johnson".to_string(),
                    }
                ),
            ],
            StorageCommand::autocomplete("P", &context),
        );

        assert!(StorageCommand::autocomplete("p", &context).is_empty());
        assert!(StorageCommand::autocomplete("", &context).is_empty());
    }
}
