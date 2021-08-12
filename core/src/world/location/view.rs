use super::Location;
use std::fmt;

pub struct SummaryView<'a>(&'a Location);

pub struct DescriptionView<'a>(&'a Location);

pub struct DetailsView<'a>(&'a Location);

impl<'a> SummaryView<'a> {
    pub fn new(location: &'a Location) -> Self {
        Self(location)
    }
}

impl<'a> DescriptionView<'a> {
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
                if subtype.starts_with(&['a', 'e', 'i', 'o', 'u'][..]) {
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

impl<'a> fmt::Display for DescriptionView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.subtype)
    }
}

impl<'a> fmt::Display for DetailsView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let location = self.0;

        location
            .name
            .value()
            .map(|name| write!(f, "# {}", name))
            .unwrap_or_else(|| {
                if let Some(subtype) = location.subtype.value() {
                    write!(f, "# Unnamed {}", subtype)
                } else {
                    write!(f, "# Unnamed building")
                }
            })?;
        location
            .subtype
            .value()
            .map(|subtype| write!(f, "\n*{}*", subtype))
            .transpose()?;
        location
            .description
            .value()
            .map(|description| write!(f, "\n\n{}", description))
            .transpose()?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::Location;

    use crate::world::location::{BuildingType, LocationType};
    use crate::world::Field;

    #[test]
    fn summary_view_test() {
        let mut location = Location::default();
        location.subtype = LocationType::from(BuildingType::Inn).into();
        location.name = "Oaken Mermaid Inn".into();
        location.description = "I am Mordenkainen".into();

        assert_eq!(
            "Oaken Mermaid Inn, an inn",
            format!("{}", location.display_summary()),
        );

        location.subtype = LocationType::from(BuildingType::Residence).into();
        assert_eq!(
            "Oaken Mermaid Inn, a residence",
            format!("{}", location.display_summary()),
        );

        location.name = Field::default();
        assert_eq!(
            "residence (I am Mordenkainen)",
            format!("{}", location.display_summary()),
        );

        location.description = Field::default();
        assert_eq!("residence", format!("{}", location.display_summary()));

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

    #[test]
    fn details_view_test() {
        let mut location = Location::default();
        location.subtype = LocationType::from(BuildingType::Inn).into();
        location.name = "Oaken Mermaid Inn".into();
        location.description = "I am Mordenkainen".into();
        assert_eq!(
            "\
# Oaken Mermaid Inn
*inn*

I am Mordenkainen",
            format!("{}", location.display_details()),
        );
    }
}
