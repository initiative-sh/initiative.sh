use super::{Place, PlaceType};
use std::fmt;

pub struct SummaryView<'a>(&'a Place);

pub struct DescriptionView<'a>(&'a Place);

pub struct DetailsView<'a>(&'a Place);

impl<'a> SummaryView<'a> {
    pub fn new(place: &'a Place) -> Self {
        Self(place)
    }
}

impl<'a> DescriptionView<'a> {
    pub fn new(place: &'a Place) -> Self {
        Self(place)
    }
}

impl<'a> DetailsView<'a> {
    pub fn new(place: &'a Place) -> Self {
        Self(place)
    }
}

impl<'a> fmt::Display for SummaryView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let place = self.0;

        write!(
            f,
            "{} ",
            place.subtype.value().unwrap_or(&PlaceType::Any).get_emoji(),
        )?;

        match (place.subtype.value(), place.name.value()) {
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
            (None, Some(name)) => write!(f, "`{}`, a place", name),
            (None, None) => write!(f, "place"),
        }
    }
}

impl<'a> fmt::Display for DescriptionView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(subtype) = self.0.subtype.value() {
            write!(f, "{}", subtype)
        } else {
            write!(f, "place")
        }
    }
}

impl<'a> fmt::Display for DetailsView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let place = self.0;

        place
            .name
            .value()
            .map(|name| write!(f, "# {}", name))
            .unwrap_or_else(|| write!(f, "# Unnamed {}", place.display_description()))?;

        write!(f, "\n*{}*", place.display_description())?;

        place
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
    use crate::world::place::PlaceType;

    #[test]
    fn view_test_empty() {
        let place = Place::default();
        assert_eq!("📍 place", format!("{}", place.display_summary()));
        assert_eq!("place", format!("{}", place.display_description()));
        assert_eq!(
            "# Unnamed place\n*place*",
            format!("{}", place.display_details()),
        );
    }

    #[test]
    fn view_test_name_only() {
        let place = Place {
            name: "The Invulnerable Vagrant".into(),
            ..Default::default()
        };
        assert_eq!(
            "📍 `The Invulnerable Vagrant`, a place",
            format!("{}", place.display_summary()),
        );
        assert_eq!("place", format!("{}", place.display_description()));
        assert_eq!(
            "# The Invulnerable Vagrant\n*place*",
            format!("{}", place.display_details()),
        );
    }

    #[test]
    fn view_test_subtype_only() {
        let place = Place {
            subtype: "inn".parse::<PlaceType>().ok().into(),
            ..Default::default()
        };
        assert_eq!("📍 inn", format!("{}", place.display_summary()));
        assert_eq!("inn", format!("{}", place.display_description()));
        assert_eq!(
            "# Unnamed inn\n*inn*",
            format!("{}", place.display_details()),
        );
    }

    #[test]
    fn view_test_description_only() {
        let place = Place {
            description: "A street with no name.".into(),
            ..Default::default()
        };
        assert_eq!("📍 place", format!("{}", place.display_summary()));
        assert_eq!("place", format!("{}", place.display_description()));
        assert_eq!(
            "# Unnamed place\n*place*\n\nA street with no name.",
            format!("{}", place.display_details()),
        );
    }

    #[test]
    fn view_test_name_subtype() {
        let place = Place {
            subtype: "inn".parse::<PlaceType>().ok().into(),
            name: "Oaken Mermaid Inn".into(),
            ..Default::default()
        };
        assert_eq!(
            "📍 `Oaken Mermaid Inn`, an inn",
            format!("{}", place.display_summary()),
        );
        assert_eq!("inn", format!("{}", place.display_description()));
        assert_eq!(
            "# Oaken Mermaid Inn\n*inn*",
            format!("{}", place.display_details()),
        );
    }

    #[test]
    fn view_test_name_description() {
        let place = Place {
            name: "The Invulnerable Vagrant".into(),
            description: "Come in and see me, and me, and me!".into(),
            ..Default::default()
        };
        assert_eq!(
            "📍 `The Invulnerable Vagrant`, a place",
            format!("{}", place.display_summary()),
        );
        assert_eq!("place", format!("{}", place.display_description()));
        assert_eq!(
            "# The Invulnerable Vagrant\n*place*\n\nCome in and see me, and me, and me!",
            format!("{}", place.display_details()),
        );
    }

    #[test]
    fn view_test_subtype_description() {
        let place = Place {
            subtype: "inn".parse::<PlaceType>().ok().into(),
            description: "You can check out any time you like.".into(),
            ..Default::default()
        };
        assert_eq!("📍 inn", format!("{}", place.display_summary()));
        assert_eq!("inn", format!("{}", place.display_description()));
        assert_eq!(
            "# Unnamed inn\n*inn*\n\nYou can check out any time you like.",
            format!("{}", place.display_details()),
        );
    }

    #[test]
    fn view_test_name_subtype_description() {
        let place = Place {
            subtype: "inn".parse::<PlaceType>().ok().into(),
            name: "Oaken Mermaid Inn".into(),
            description: "I am Mordenkainen.".into(),
            ..Default::default()
        };
        assert_eq!(
            "📍 `Oaken Mermaid Inn`, an inn",
            format!("{}", place.display_summary()),
        );
        assert_eq!("inn", format!("{}", place.display_description()));
        assert_eq!(
            "# Oaken Mermaid Inn\n*inn*\n\nI am Mordenkainen.",
            format!("{}", place.display_details()),
        );
    }
}
