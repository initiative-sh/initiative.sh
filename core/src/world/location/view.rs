use std::fmt;

use super::Location;

pub struct SummaryView<'a>(&'a Location);

pub struct DetailsView<'a>(&'a Location);

impl<'a> SummaryView<'a> {
    pub fn new(location: &'a Location) -> Self {
        Self(location)
    }
}

impl<'a> DetailsView<'a> {
    pub fn new(location: &'a Location) -> Self {
        Self(location)
    }
}

impl<'a> fmt::Display for SummaryView<'a> {
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

#[cfg(test)]
mod test_display_for_location_summary_view {
    use super::Location;

    use crate::world::location::{BuildingType, LocationType};
    use crate::world::Field;

    #[test]
    fn fmt_test() {
        let mut location = Location::default();
        location.subtype = LocationType::from(BuildingType::Inn).into();
        location.name = "Oaken Mermaid Inn".into();
        location.description = "I am Mordenkainen".into();

        assert_eq!(
            "Oaken Mermaid Inn, an Inn",
            format!("{}", location.display_summary()),
        );

        location.subtype = LocationType::from(BuildingType::Residence).into();
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

impl<'a> fmt::Display for DetailsView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let location = self.0;

        location
            .name
            .value()
            .map(|name| writeln!(f, "{}", name))
            .transpose()?;
        location
            .subtype
            .value()
            .map(|subtype| writeln!(f, "Type: {}", subtype))
            .transpose()?;
        location
            .description
            .value()
            .map(|description| writeln!(f, "{}", description))
            .transpose()?;
        Ok(())
    }
}

#[cfg(test)]
mod test_display_for_location_details_view {
    use super::Location;

    use crate::world::location::{BuildingType, LocationType};

    #[test]
    fn fmt_test() {
        let mut location = Location::default();
        location.subtype = LocationType::from(BuildingType::Inn).into();
        location.name = "Oaken Mermaid Inn".into();
        location.description = "I am Mordenkainen".into();
        assert_eq!(
            "Oaken Mermaid Inn\n\
            Type: Inn\n\
            I am Mordenkainen\n",
            format!("{}", location.display_details()),
        );
    }
}
