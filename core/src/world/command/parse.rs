use crate::utils::{capitalize, quoted_words};
use crate::world::{Field, Location, Npc};
use std::str::FromStr;

fn split_name(input: &str) -> Option<(&str, &str)> {
    let (named, comma) =
        quoted_words(input).fold((None, None), |(named, comma), word| match word.as_str() {
            "named" | "called" if named.is_none() => (Some(word), comma),
            s if s.ends_with(',') => (named, Some(word)),
            _ => (named, comma),
        });

    let (name, description) = if let Some(word) = named {
        // "a boy named Sue"
        (&input[word.range().end..], &input[..word.range().start])
    } else if let Some(word) = comma {
        // "Nott the Brave, a goblin"
        (
            input[..word.range().end].trim_end_matches(','),
            &input[word.range().end..],
        )
    } else {
        return None;
    };

    if let (Some(name_start), Some(name_end)) =
        quoted_words(name).fold((None, None), |(name_start, _), word| {
            (
                name_start.or_else(|| Some(word.range().start)),
                Some(word.range().end),
            )
        })
    {
        let name = &name[name_start..name_end];
        if let Some(name_stripped) = name.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
            Some((name_stripped, description))
        } else {
            Some((name, description))
        }
    } else {
        None
    }
}

impl FromStr for Location {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut location = Location::default();
        let mut is_explicit = false;

        let description = if let Some((name, description)) = split_name(input) {
            location.name = Field::new(capitalize(name));
            description
        } else {
            input
        };

        for word in quoted_words(description) {
            if ["a", "an"].contains(&word.as_str()) {
                // ignore
            } else if ["location", "place"].contains(&word.as_str()) {
                is_explicit = true;
            } else if let Ok(location_type) = word.as_str().parse() {
                location.subtype = Field::new(location_type);
            } else {
                return Err(());
            }
        }

        if is_explicit || location.subtype.is_some() {
            Ok(location)
        } else {
            Err(())
        }
    }
}

impl FromStr for Npc {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if ["character", "npc", "person"].contains(&input) {
            Ok(Npc::default())
        } else if let Ok(species) = input.parse() {
            Ok(Npc {
                species: Field::new(species),
                ..Default::default()
            })
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::world::location::{BuildingType, LocationType};
    use crate::world::npc::Species;

    #[test]
    fn location_from_str_test() {
        {
            let location: Location = "inn".parse().unwrap();
            assert_eq!(
                Field::Locked(LocationType::Building(Some(BuildingType::Inn))),
                location.subtype,
            );
        }

        {
            let location = "building named foo bar".parse::<Location>().unwrap();
            assert_eq!(Some("Foo bar"), location.name.value().map(|s| s.as_str()));
        }

        {
            let location: Location = "The Prancing Pony, an inn".parse().unwrap();
            assert_eq!(
                Field::Locked("The Prancing Pony".to_string()),
                location.name,
            );
        }

        {
            let location: Location = "building".parse().unwrap();
            assert_eq!(
                Field::Locked(LocationType::Building(None)),
                location.subtype,
            );
        }

        {
            let location: Location = "\"The Prancing Pony\", an inn".parse().unwrap();
            assert_eq!(
                Field::Locked("The Prancing Pony".to_string()),
                location.name,
            );
            assert_eq!(
                Field::Locked(LocationType::Building(Some(BuildingType::Inn))),
                location.subtype,
            );
        }

        {
            let location: Location = "a place called home".parse().unwrap();
            assert_eq!(Field::Locked("Home".to_string()), location.name);
            assert!(location.subtype.is_none());
        }
    }

    #[test]
    fn npc_from_str_test() {
        {
            assert_eq!(Ok(Npc::default()), "npc".parse::<Npc>());
        }

        {
            let npc: Npc = "elf".parse().unwrap();
            assert_eq!(Field::Locked(Species::Elf), npc.species);
        }

        {
            assert_eq!(Err(()), "potato".parse::<Npc>());
        }
    }
}
