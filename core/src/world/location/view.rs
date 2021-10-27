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

        match (location.subtype.value(), location.name.value()) {
            (Some(subtype), Some(name)) => {
                if subtype
                    .as_str()
                    .starts_with(&['a', 'e', 'i', 'o', 'u', 'y'][..])
                {
                    write!(f, "`{}`, an {}", name, subtype)
                } else {
                    write!(f, "`{}`, a {}", name, subtype)
                }
            }
            (Some(subtype), None) => write!(f, "{}", subtype),
            (None, Some(name)) => write!(f, "`{}`, a location", name),
            (None, None) => write!(f, "location"),
        }
    }
}

impl<'a> fmt::Display for DescriptionView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(subtype) = self.0.subtype.value() {
            write!(f, "{}", subtype)
        } else {
            write!(f, "location")
        }
    }
}

impl<'a> fmt::Display for DetailsView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let location = self.0;

        location
            .name
            .value()
            .map(|name| write!(f, "# {}", name))
            .unwrap_or_else(|| write!(f, "# Unnamed {}", location.display_description()))?;

        write!(f, "\n*{}*", location.display_description())?;

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
    use super::*;
    use crate::world::location::LocationType;

    #[test]
    fn view_test_empty() {
        let location = Location::default();
        assert_eq!("location", format!("{}", location.display_summary()));
        assert_eq!("location", format!("{}", location.display_description()));
        assert_eq!(
            "# Unnamed location\n*location*",
            format!("{}", location.display_details()),
        );
    }

    #[test]
    fn view_test_name_only() {
        let location = Location {
            name: "The Invulnerable Vagrant".into(),
            ..Default::default()
        };
        assert_eq!(
            "`The Invulnerable Vagrant`, a location",
            format!("{}", location.display_summary()),
        );
        assert_eq!("location", format!("{}", location.display_description()));
        assert_eq!(
            "# The Invulnerable Vagrant\n*location*",
            format!("{}", location.display_details()),
        );
    }

    #[test]
    fn view_test_subtype_only() {
        let location = Location {
            subtype: LocationType::Inn.into(),
            ..Default::default()
        };
        assert_eq!("inn", format!("{}", location.display_summary()));
        assert_eq!("inn", format!("{}", location.display_description()));
        assert_eq!(
            "# Unnamed inn\n*inn*",
            format!("{}", location.display_details()),
        );
    }

    #[test]
    fn view_test_description_only() {
        let location = Location {
            description: "A street with no name.".into(),
            ..Default::default()
        };
        assert_eq!("location", format!("{}", location.display_summary()));
        assert_eq!("location", format!("{}", location.display_description()));
        assert_eq!(
            "# Unnamed location\n*location*\n\nA street with no name.",
            format!("{}", location.display_details()),
        );
    }

    #[test]
    fn view_test_name_subtype() {
        let location = Location {
            subtype: LocationType::Inn.into(),
            name: "Oaken Mermaid Inn".into(),
            ..Default::default()
        };
        assert_eq!(
            "`Oaken Mermaid Inn`, an inn",
            format!("{}", location.display_summary()),
        );
        assert_eq!("inn", format!("{}", location.display_description()));
        assert_eq!(
            "# Oaken Mermaid Inn\n*inn*",
            format!("{}", location.display_details()),
        );
    }

    #[test]
    fn view_test_name_description() {
        let location = Location {
            name: "The Invulnerable Vagrant".into(),
            description: "Come in and see me, and me, and me!".into(),
            ..Default::default()
        };
        assert_eq!(
            "`The Invulnerable Vagrant`, a location",
            format!("{}", location.display_summary()),
        );
        assert_eq!("location", format!("{}", location.display_description()));
        assert_eq!(
            "# The Invulnerable Vagrant\n*location*\n\nCome in and see me, and me, and me!",
            format!("{}", location.display_details()),
        );
    }

    #[test]
    fn view_test_subtype_description() {
        let location = Location {
            subtype: LocationType::Inn.into(),
            description: "You can check out any time you like.".into(),
            ..Default::default()
        };
        assert_eq!("inn", format!("{}", location.display_summary()));
        assert_eq!("inn", format!("{}", location.display_description()));
        assert_eq!(
            "# Unnamed inn\n*inn*\n\nYou can check out any time you like.",
            format!("{}", location.display_details()),
        );
    }

    #[test]
    fn view_test_name_subtype_description() {
        let location = Location {
            subtype: LocationType::Inn.into(),
            name: "Oaken Mermaid Inn".into(),
            description: "I am Mordenkainen.".into(),
            ..Default::default()
        };
        assert_eq!(
            "`Oaken Mermaid Inn`, an inn",
            format!("{}", location.display_summary()),
        );
        assert_eq!("inn", format!("{}", location.display_description()));
        assert_eq!(
            "# Oaken Mermaid Inn\n*inn*\n\nI am Mordenkainen.",
            format!("{}", location.display_details()),
        );
    }
}
