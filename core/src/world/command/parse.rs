use crate::world::{Field, Location, Npc};
use std::str::FromStr;

impl FromStr for Location {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        input.parse().map(|location_type| Location {
            subtype: Field::new(location_type),
            ..Default::default()
        })
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
            let location: Location = "building".parse().unwrap();
            assert_eq!(
                Field::Locked(LocationType::Building(None)),
                location.subtype,
            );
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
