use crate::world::{Field, Location};
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::world::location::{BuildingType, LocationType};

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
}
