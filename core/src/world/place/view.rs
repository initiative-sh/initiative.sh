use crate::world::place::{PlaceData, PlaceRelations, PlaceType};
use std::fmt;
use uuid::Uuid;

pub struct NameView<'a>(&'a PlaceData);

pub struct SummaryView<'a>(&'a PlaceData);

pub struct DescriptionView<'a>(&'a PlaceData);

pub struct DetailsView<'a> {
    place: &'a PlaceData,
    uuid: Uuid,
    relations: PlaceRelations,
}

impl<'a> NameView<'a> {
    pub fn new(place: &'a PlaceData) -> Self {
        Self(place)
    }
}

impl<'a> SummaryView<'a> {
    pub fn new(place: &'a PlaceData) -> Self {
        Self(place)
    }
}

impl<'a> DescriptionView<'a> {
    pub fn new(place: &'a PlaceData) -> Self {
        Self(place)
    }
}

impl<'a> DetailsView<'a> {
    pub fn new(place: &'a PlaceData, uuid: Uuid, relations: PlaceRelations) -> Self {
        Self {
            place,
            uuid,
            relations,
        }
    }
}

impl<'a> fmt::Display for NameView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let place = self.0;

        if let Some(name) = place.name.value() {
            write!(
                f,
                "{} `{}`",
                place.subtype.value().unwrap_or(&PlaceType::Any).get_emoji(),
                name,
            )
        } else {
            Ok(())
        }
    }
}

impl<'a> fmt::Display for SummaryView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let place = self.0;

        match (place.subtype.value(), place.name.is_some()) {
            (Some(subtype), true) => write!(f, "{} ({})", place.display_name(), subtype),
            (Some(subtype), false) => write!(f, "{} {}", subtype.get_emoji(), subtype),
            (None, true) => write!(f, "{} (place)", place.display_name()),
            (None, false) => write!(f, "{} place", PlaceType::Any.get_emoji()),
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
        let Self {
            place,
            uuid,
            relations,
        } = self;

        writeln!(
            f,
            "<div class=\"thing-box place\" data-uuid=\"{}\">\n",
            uuid
        )?;

        place
            .name
            .value()
            .map(|name| write!(f, "# {}", name))
            .unwrap_or_else(|| write!(f, "# Unnamed {}", place.display_description()))?;

        write!(f, "\n*{}*", place.display_description())?;

        relations
            .location
            .as_ref()
            .map(|(parent, grandparent)| {
                if let Some(grandparent) = grandparent {
                    write!(
                        f,
                        "\n\n**Location:** {}, {}",
                        parent.display_name(),
                        grandparent.display_name(),
                    )
                } else {
                    write!(f, "\n\n**Location:** {}", parent.display_summary())
                }
            })
            .transpose()?;

        place
            .description
            .value()
            .map(|description| write!(f, "\n\n{}", description))
            .transpose()?;

        write!(f, "\n\n</div>")?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::world::place::Place;

    #[test]
    fn view_test_empty() {
        let place = PlaceData::default();
        assert_eq!("", format!("{}", place.display_name()));
        assert_eq!("📍 place", format!("{}", place.display_summary()));
        assert_eq!("place", format!("{}", place.display_description()));
        assert_eq!(
            r#"<div class="thing-box place" data-uuid="00000000-0000-0000-0000-000000000000">

# Unnamed place
*place*

</div>"#,
            place
                .display_details(Uuid::nil(), PlaceRelations::default())
                .to_string(),
        );
    }

    #[test]
    fn view_test_name_only() {
        let place = PlaceData {
            name: "The Invulnerable Vagrant".into(),
            ..Default::default()
        };
        assert_eq!(
            "📍 `The Invulnerable Vagrant`",
            format!("{}", place.display_name()),
        );
        assert_eq!(
            "📍 `The Invulnerable Vagrant` (place)",
            format!("{}", place.display_summary()),
        );
        assert_eq!("place", format!("{}", place.display_description()));
        assert_eq!(
            r#"<div class="thing-box place" data-uuid="00000000-0000-0000-0000-000000000000">

# The Invulnerable Vagrant
*place*

</div>"#,
            place
                .display_details(Uuid::nil(), PlaceRelations::default())
                .to_string(),
        );
    }

    #[test]
    fn view_test_subtype_only() {
        let place = PlaceData {
            subtype: "inn".parse::<PlaceType>().unwrap().into(),
            ..Default::default()
        };
        assert_eq!("", format!("{}", place.display_name()));
        assert_eq!("🏨 inn", format!("{}", place.display_summary()));
        assert_eq!("inn", format!("{}", place.display_description()));
        assert_eq!(
            r#"<div class="thing-box place" data-uuid="00000000-0000-0000-0000-000000000000">

# Unnamed inn
*inn*

</div>"#,
            place
                .display_details(Uuid::nil(), PlaceRelations::default())
                .to_string(),
        );
    }

    #[test]
    fn view_test_description_only() {
        let place = PlaceData {
            description: "A street with no name.".into(),
            ..Default::default()
        };
        assert_eq!("", format!("{}", place.display_name()));
        assert_eq!("📍 place", format!("{}", place.display_summary()));
        assert_eq!("place", format!("{}", place.display_description()));
        assert_eq!(
            r#"<div class="thing-box place" data-uuid="00000000-0000-0000-0000-000000000000">

# Unnamed place
*place*

A street with no name.

</div>"#,
            place
                .display_details(Uuid::nil(), PlaceRelations::default())
                .to_string(),
        );
    }

    #[test]
    fn view_test_name_subtype() {
        let place = PlaceData {
            subtype: "inn".parse::<PlaceType>().unwrap().into(),
            name: "Oaken Mermaid Inn".into(),
            ..Default::default()
        };
        assert_eq!(
            "🏨 `Oaken Mermaid Inn`",
            format!("{}", place.display_name()),
        );
        assert_eq!(
            "🏨 `Oaken Mermaid Inn` (inn)",
            format!("{}", place.display_summary()),
        );
        assert_eq!("inn", format!("{}", place.display_description()));
        assert_eq!(
            r#"<div class="thing-box place" data-uuid="00000000-0000-0000-0000-000000000000">

# Oaken Mermaid Inn
*inn*

</div>"#,
            place
                .display_details(Uuid::nil(), PlaceRelations::default())
                .to_string(),
        );
    }

    #[test]
    fn view_test_name_description() {
        let place = PlaceData {
            name: "The Invulnerable Vagrant".into(),
            description: "Come in and see me, and me, and me!".into(),
            ..Default::default()
        };
        assert_eq!(
            "📍 `The Invulnerable Vagrant`",
            format!("{}", place.display_name()),
        );
        assert_eq!(
            "📍 `The Invulnerable Vagrant` (place)",
            format!("{}", place.display_summary()),
        );
        assert_eq!("place", format!("{}", place.display_description()));
        assert_eq!(
            r#"<div class="thing-box place" data-uuid="00000000-0000-0000-0000-000000000000">

# The Invulnerable Vagrant
*place*

Come in and see me, and me, and me!

</div>"#,
            place
                .display_details(Uuid::nil(), PlaceRelations::default())
                .to_string(),
        );
    }

    #[test]
    fn view_test_subtype_description() {
        let place = PlaceData {
            subtype: "inn".parse::<PlaceType>().unwrap().into(),
            description: "You can check out any time you like.".into(),
            ..Default::default()
        };
        assert_eq!("", format!("{}", place.display_name()));
        assert_eq!("🏨 inn", format!("{}", place.display_summary()));
        assert_eq!("inn", format!("{}", place.display_description()));
        assert_eq!(
            r#"<div class="thing-box place" data-uuid="00000000-0000-0000-0000-000000000000">

# Unnamed inn
*inn*

You can check out any time you like.

</div>"#,
            place
                .display_details(Uuid::nil(), PlaceRelations::default())
                .to_string(),
        );
    }

    #[test]
    fn view_test_name_subtype_description() {
        let place = PlaceData {
            subtype: "inn".parse::<PlaceType>().unwrap().into(),
            name: "Oaken Mermaid Inn".into(),
            description: "I am Mordenkainen.".into(),
            ..Default::default()
        };
        assert_eq!(
            "🏨 `Oaken Mermaid Inn`",
            format!("{}", place.display_name()),
        );
        assert_eq!(
            "🏨 `Oaken Mermaid Inn` (inn)",
            format!("{}", place.display_summary()),
        );
        assert_eq!("inn", format!("{}", place.display_description()));
        assert_eq!(
            r#"<div class="thing-box place" data-uuid="00000000-0000-0000-0000-000000000000">

# Oaken Mermaid Inn
*inn*

I am Mordenkainen.

</div>"#,
            place
                .display_details(Uuid::nil(), PlaceRelations::default())
                .to_string(),
        );
    }

    #[test]
    fn details_view_test_with_parent_location() {
        let place = PlaceData {
            name: "The Prancing Pony".into(),
            subtype: "inn".parse::<PlaceType>().unwrap().into(),
            ..Default::default()
        };

        let relations = PlaceRelations {
            location: Some((
                Place {
                    uuid: Uuid::nil(),
                    data: PlaceData {
                        name: "Bree".into(),
                        subtype: "town".parse::<PlaceType>().unwrap().into(),
                        ..Default::default()
                    },
                },
                None,
            )),
        };

        assert_eq!(
            r#"<div class="thing-box place" data-uuid="00000000-0000-0000-0000-000000000000">

# The Prancing Pony
*inn*

**Location:** 🏘 `Bree` (town)

</div>"#,
            DetailsView::new(&place, Uuid::nil(), relations).to_string(),
        );
    }

    #[test]
    fn details_view_test_with_grandparent_location() {
        let place = PlaceData {
            name: "The Prancing Pony".into(),
            subtype: "inn".parse::<PlaceType>().unwrap().into(),
            ..Default::default()
        };

        let relations = PlaceRelations {
            location: Some((
                Place {
                    uuid: Uuid::nil(),
                    data: PlaceData {
                        name: "Bree".into(),
                        subtype: "town".parse::<PlaceType>().unwrap().into(),
                        ..Default::default()
                    },
                },
                Some(Place {
                    uuid: Uuid::nil(),
                    data: PlaceData {
                        name: "The Shire".into(),
                        subtype: "region".parse::<PlaceType>().unwrap().into(),
                        ..Default::default()
                    },
                }),
            )),
        };

        assert_eq!(
            r#"<div class="thing-box place" data-uuid="00000000-0000-0000-0000-000000000000">

# The Prancing Pony
*inn*

**Location:** 🏘 `Bree`, 👑 `The Shire`

</div>"#,
            DetailsView::new(&place, Uuid::nil(), relations).to_string(),
        );
    }
}
