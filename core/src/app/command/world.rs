use super::Autocomplete;
use crate::world::location::{BuildingType, LocationType};
use crate::world::npc::Species;
use std::str::FromStr;

#[derive(Debug)]
pub enum WorldCommand {
    Location { location_type: LocationType },
    Npc { species: Option<Species> },
    //Region(RawCommand),
}

impl FromStr for WorldCommand {
    type Err = ();

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if let Ok(species) = raw.parse() {
            Ok(WorldCommand::Npc {
                species: Some(species),
            })
        } else if let Ok(location_type) = raw.parse() {
            Ok(WorldCommand::Location { location_type })
        } else if "npc" == raw {
            Ok(WorldCommand::Npc { species: None })
        } else {
            Err(())
        }
    }
}

impl Autocomplete for WorldCommand {
    fn autocomplete(input: &str) -> Vec<String> {
        let (start, partial) = input.split_at(
            input
                .rfind(char::is_whitespace)
                .map(|i| {
                    ((i + 1)..input.len())
                        .find(|&i| input.is_char_boundary(i))
                        .unwrap_or(input.len())
                })
                .unwrap_or(0),
        );

        if partial.is_empty() {
            return Vec::new();
        }

        ["npc", "building"]
            .iter()
            .chain(Species::get_words().iter())
            .chain(BuildingType::get_words().iter())
            .filter_map(|word| {
                if word.starts_with(partial) {
                    let mut suggestion = String::with_capacity(start.len() + partial.len());
                    suggestion.push_str(start);
                    suggestion.push_str(word);
                    Some(suggestion)
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_str_test() {
        let parsed_command = "building".parse();
        assert!(
            matches!(
                parsed_command,
                Ok(WorldCommand::Location {
                    location_type: LocationType::Building(None)
                }),
            ),
            "{:?}",
            parsed_command,
        );

        let parsed_command = "npc".parse();
        assert!(
            matches!(parsed_command, Ok(WorldCommand::Npc { species: None })),
            "{:?}",
            parsed_command,
        );

        let parsed_command = "elf".parse();
        assert!(
            matches!(
                parsed_command,
                Ok(WorldCommand::Npc {
                    species: Some(Species::Elf)
                }),
            ),
            "{:?}",
            parsed_command,
        );

        let parsed_command = "potato".parse::<WorldCommand>();
        assert!(matches!(parsed_command, Err(())), "{:?}", parsed_command);
    }

    #[test]
    fn autocomplete_test() {
        assert_eq!(vec!["npc"], WorldCommand::autocomplete("n"));
        assert_eq!(vec!["some npc"], WorldCommand::autocomplete("some n"));
        assert_eq!(
            vec!["potato half-elf", "potato half-orc", "potato halfling"],
            WorldCommand::autocomplete("potato half"),
        );

        // Non-ASCII whitespace
        assert_eq!(
            vec!["foo\u{2003}npc"],
            WorldCommand::autocomplete("foo\u{2003}n")
        );
    }

    #[test]
    fn autocomplete_test_all_words() {
        [
            "building",
            "npc",
            // Species
            "dragonborn",
            "dwarf",
            "elf",
            "gnome",
            "half-elf",
            "half-orc",
            "halfling",
            "human",
            "tiefling",
            "warforged",
            // BuildingType
            "inn",
            "residence",
            "shop",
            "temple",
            "warehouse",
        ]
        .iter()
        .for_each(|word| assert_eq!(vec![word.to_string()], WorldCommand::autocomplete(word)));
    }

    #[test]
    fn autocomplete_test_no_suggestions() {
        assert_eq!(Vec::<String>::new(), WorldCommand::autocomplete(""));
        assert_eq!(Vec::<String>::new(), WorldCommand::autocomplete("potato"));
        assert_eq!(Vec::<String>::new(), WorldCommand::autocomplete("npc "));
        assert_eq!(Vec::<String>::new(), WorldCommand::autocomplete("🥔"));
    }
}
