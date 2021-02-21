use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use rand::Rng;

use super::region::Uuid as RegionUuid;
use super::{Demographics, Field, Generate};
use crate::Noun;

pub use building::*;

mod building;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Uuid(uuid::Uuid);

#[derive(Debug, Default)]
pub struct Location {
    pub uuid: Option<Rc<Uuid>>,
    pub parent_uuid: Option<Rc<RegionUuid>>,
    pub subtype: Field<LocationType>,

    pub name: Field<String>,
    pub description: Field<String>,
    // pub architecture: Option<String>,
    // pub floors: Field<u8>,
    // pub owner: Field<Vec<NpcUuid>>,
    // pub staff: Field<Vec<NpcUuid>>,
    // pub occupants: Field<Vec<NpcUuid>>,
    // pub services: Option<String>,
    // pub worship: Field<String>,
    // pub quality: something
    // pub price: something
}

pub struct LocationSummaryView<'a>(&'a Location);

pub struct LocationDetailsView<'a>(&'a Location);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LocationType {
    Building(BuildingType),
}

impl Deref for Uuid {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<uuid::Uuid> for Uuid {
    fn from(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }
}

impl Location {
    pub fn display_summary(&self) -> LocationSummaryView {
        LocationSummaryView(self)
    }

    pub fn display_details(&self) -> LocationDetailsView {
        LocationDetailsView(self)
    }

    pub fn generate_subtype(
        subtype: LocationType,
        rng: &mut impl Rng,
        demographics: &Demographics,
    ) -> Self {
        let mut location = Self::default();
        location.subtype = Field::new(subtype);
        location.regenerate(rng, demographics);
        location
    }
}

impl Generate for Location {
    fn regenerate(&mut self, rng: &mut impl Rng, demographics: &Demographics) {
        self.subtype.replace_with(|location_type| {
            if let Some(mut location_type) = location_type {
                location_type.regenerate(rng, demographics);
                location_type
            } else {
                LocationType::generate(rng, demographics)
            }
        });

        if let Some(value) = self.subtype.as_ref() {
            match value {
                LocationType::Building(building_type) => match building_type {
                    BuildingType::Inn => generate_inn(self, rng, demographics),
                    BuildingType::Residence => generate_residence(self, rng, demographics),
                    BuildingType::Shop => generate_shop(self, rng, demographics),
                    BuildingType::Temple => generate_temple(self, rng, demographics),
                    BuildingType::Warehouse => generate_warehouse(self, rng, demographics),
                },
            }
        }
    }
}

#[cfg(test)]
mod test_location {
    use super::{BuildingType, Demographics, Field, Generate, Location, LocationType};
    use rand::rngs::mock::StepRng;

    #[test]
    fn generate_test() {
        let demographics = Demographics::default();

        let mut rng = StepRng::new(0, 10_000_000_000);
        assert_ne!(
            Location::generate(&mut rng, &demographics).subtype,
            Location::generate(&mut rng, &demographics).subtype,
        );

        let mut rng = StepRng::new(0, 0);
        assert_eq!(
            Location::generate(&mut rng, &demographics).subtype,
            Location::generate(&mut rng, &demographics).subtype,
        );
    }

    #[test]
    fn generate_type_test() {
        let demographics = Demographics::default();
        let mut rng = StepRng::new(0, 0);

        assert_eq!(
            Location::generate_subtype(BuildingType::Inn.into(), &mut rng, &demographics).name,
            Field::new_generated("The Silver Eel".into()),
        );

        assert_eq!(
            Location::generate_subtype(BuildingType::Residence.into(), &mut rng, &demographics)
                .description,
            Field::new_generated("Abandoned squat".into()),
        );

        assert_eq!(
            Location::generate_subtype(BuildingType::Shop.into(), &mut rng, &demographics)
                .description,
            Field::new_generated("Pawnshop".into()),
        );

        assert_eq!(
            Location::generate_subtype(BuildingType::Temple.into(), &mut rng, &demographics)
                .description,
            Field::new_generated("Temple to a good or neutral deity".into()),
        );

        assert_eq!(
            Location::generate_subtype(BuildingType::Warehouse.into(), &mut rng, &demographics)
                .description,
            Field::new_generated("Empty or abandoned".into()),
        );
    }

    #[test]
    fn display_details_test() {
        let mut location = Location::default();
        location.subtype = Field::new(BuildingType::Inn.into());
        location.name = Field::new("Oaken Mermaid Inn".into());
        location.description = Field::new("I am Mordenkainen".into());
        assert_eq!(
            "Oaken Mermaid Inn\n\
            Type: Inn\n\
            I am Mordenkainen\n",
            format!("{}", location.display_details()),
        );
    }

    #[test]
    fn display_summary_test() {
        let mut location = Location::default();
        location.subtype = LocationType::Building(BuildingType::Inn).into();
        location.name = "Oaken Mermaid Inn".into();
        location.description = "I am Mordenkainen".into();

        assert_eq!(
            "Oaken Mermaid Inn, an Inn",
            format!("{}", location.display_summary()),
        );

        location.subtype = LocationType::Building(BuildingType::Residence).into();
        assert_eq!(
            "Oaken Mermaid Inn, a Residence",
            format!("{}", location.display_summary()),
        );

        location.name = Field::default();
        assert_eq!(
            "Residence (I am Mordenkainen)",
            format!("{}", location.display_summary()),
        );

        location.description = Field::default();
        assert_eq!("Residence", format!("{}", location.display_summary()));

        location.subtype = Field::default();
        assert_eq!("", format!("{}", location.display_summary()));

        location.name = "The Invulnerable Vagrant".into();
        assert_eq!(
            "The Invulnerable Vagrant",
            format!("{}", location.display_summary()),
        );

        location.description = "Come in and see me, and me, and me!".into();
        assert_eq!(
            "The Invulnerable Vagrant (Come in and see me, and me, and me!)",
            format!("{}", location.display_summary()),
        );

        location.name = Field::default();
        assert_eq!(
            "Come in and see me, and me, and me!",
            format!("{}", location.display_summary()),
        );
    }
}

impl<'a> fmt::Display for LocationSummaryView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let location = self.0;

        match (
            location.subtype.is_some(),
            location.name.is_some(),
            location.description.is_some(),
        ) {
            (true, true, _) => {
                let subtype = format!("{}", location.subtype);
                if subtype.starts_with(&['A', 'E', 'I', 'O', 'U'][..]) {
                    write!(f, "{}, an {}", location.name, subtype)
                } else {
                    write!(f, "{}, a {}", location.name, subtype)
                }
            }
            (true, false, true) => write!(f, "{} ({})", location.subtype, location.description),
            (true, false, false) => write!(f, "{}", location.subtype),
            (false, true, true) => write!(f, "{} ({})", location.name, location.description),
            (false, true, false) => write!(f, "{}", location.name),
            (false, false, true) => write!(f, "{}", location.description),
            (false, false, false) => Ok(()),
        }
    }
}

impl<'a> fmt::Display for LocationDetailsView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let location = self.0;

        location
            .name
            .as_ref()
            .map(|name| writeln!(f, "{}", name))
            .transpose()?;
        location
            .subtype
            .as_ref()
            .map(|subtype| writeln!(f, "Type: {}", subtype))
            .transpose()?;
        location
            .description
            .as_ref()
            .map(|description| writeln!(f, "{}", description))
            .transpose()?;
        Ok(())
    }
}

impl Default for LocationType {
    fn default() -> Self {
        Self::Building(Default::default())
    }
}

impl Generate for LocationType {
    fn regenerate(&mut self, rng: &mut impl Rng, demographics: &Demographics) {
        *self = Self::Building(BuildingType::generate(rng, demographics))
    }
}

impl fmt::Display for LocationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Building(building_type) => write!(f, "{}", building_type),
        }
    }
}

impl TryFrom<Noun> for LocationType {
    type Error = ();

    fn try_from(value: Noun) -> Result<Self, Self::Error> {
        if let Ok(building_type) = value.try_into() {
            Ok(LocationType::Building(building_type))
        } else {
            Err(())
        }
    }
}
