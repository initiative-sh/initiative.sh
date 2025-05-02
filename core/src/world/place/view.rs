use crate::world::place::{PlaceData, PlaceRelations, PlaceType};
use std::fmt;
use uuid::Uuid;

pub struct NameView<'a>(&'a PlaceData);

pub struct SummaryView<'a>(&'a PlaceData);

pub struct DescriptionView<'a>(&'a PlaceData);

pub struct DetailsView<'a> {
    place: &'a PlaceData,
    uuid: Uuid,
    relations: Option<&'a PlaceRelations>,
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
    pub fn new(place: &'a PlaceData, uuid: Uuid, relations: Option<&'a PlaceRelations>) -> Self {
        Self {
            place,
            uuid,
            relations,
        }
    }
}

impl fmt::Display for NameView<'_> {
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

impl fmt::Display for SummaryView<'_> {
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

impl fmt::Display for DescriptionView<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(subtype) = self.0.subtype.value() {
            write!(f, "{}", subtype)
        } else {
            write!(f, "place")
        }
    }
}

impl fmt::Display for DetailsView<'_> {
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
            .and_then(|relations| relations.location.as_ref())
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
    use crate::test_utils as test;

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
            place.display_details(Uuid::nil(), None).to_string(),
        );
    }

    #[test]
    fn view_test_name_only() {
        let place = test::place().name("Olympus").build();
        assert_eq!("📍 `Olympus`", format!("{}", place.display_name()));
        assert_eq!(
            "📍 `Olympus` (place)",
            format!("{}", place.display_summary()),
        );
        assert_eq!("place", format!("{}", place.display_description()));
        assert_eq!(
            r#"<div class="thing-box place" data-uuid="00000000-0000-0000-0000-000000000000">

# Olympus
*place*

</div>"#,
            place.display_details(Uuid::nil(), None).to_string(),
        );
    }

    #[test]
    fn view_test_subtype_only() {
        let place = test::place()
            .subtype("inn".parse::<PlaceType>().unwrap())
            .build();
        assert_eq!("", format!("{}", place.display_name()));
        assert_eq!("🏨 inn", format!("{}", place.display_summary()));
        assert_eq!("inn", format!("{}", place.display_description()));
        assert_eq!(
            r#"<div class="thing-box place" data-uuid="00000000-0000-0000-0000-000000000000">

# Unnamed inn
*inn*

</div>"#,
            place.display_details(Uuid::nil(), None).to_string(),
        );
    }

    #[test]
    fn view_test_description_only() {
        let place = test::place().description("A street with no name.").build();
        assert_eq!("", format!("{}", place.display_name()));
        assert_eq!("📍 place", format!("{}", place.display_summary()));
        assert_eq!("place", format!("{}", place.display_description()));
        assert_eq!(
            r#"<div class="thing-box place" data-uuid="00000000-0000-0000-0000-000000000000">

# Unnamed place
*place*

A street with no name.

</div>"#,
            place.display_details(Uuid::nil(), None).to_string(),
        );
    }

    #[test]
    fn view_test_name_subtype() {
        let place = test::place::ithaca();
        assert_eq!("🏝 `Ithaca`", format!("{}", place.display_name()));
        assert_eq!(
            "🏝 `Ithaca` (island)",
            format!("{}", place.display_summary()),
        );
        assert_eq!("island", format!("{}", place.display_description()));
        assert_eq!(
            r#"<div class="thing-box place" data-uuid="00000000-0000-0000-0000-000000000001">

# Ithaca
*island*

</div>"#,
            place.display_details(None).to_string(),
        );
    }

    #[test]
    fn view_test_name_description() {
        let place = test::place()
            .name("The Invulnerable Vagrant")
            .description("Come in and see me, and me, and me!")
            .build();
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
            place.display_details(Uuid::nil(), None).to_string(),
        );
    }

    #[test]
    fn view_test_subtype_description() {
        let place = test::place()
            .subtype("inn".parse::<PlaceType>().unwrap())
            .description("You can check out any time you like.")
            .build();
        assert_eq!("", format!("{}", place.display_name()));
        assert_eq!("🏨 inn", format!("{}", place.display_summary()));
        assert_eq!("inn", format!("{}", place.display_description()));
        assert_eq!(
            r#"<div class="thing-box place" data-uuid="00000000-0000-0000-0000-000000000000">

# Unnamed inn
*inn*

You can check out any time you like.

</div>"#,
            place.display_details(Uuid::nil(), None).to_string(),
        );
    }

    #[test]
    fn view_test_name_subtype_description() {
        let place = test::place::greece();
        assert_eq!("👑 `Greece`", format!("{}", place.display_name()));
        assert_eq!(
            "👑 `Greece` (territory)",
            format!("{}", place.display_summary()),
        );
        assert_eq!("territory", format!("{}", place.display_description()));
        assert_eq!(
            r#"<div class="thing-box place" data-uuid="00000000-0000-0000-0000-000000000002">

# Greece
*territory*

You're cruisin' for a bruisin'.

</div>"#,
            place.display_details(None).to_string(),
        );
    }

    #[test]
    fn details_view_test_with_parent_location() {
        assert_eq!(
            r#"<div class="thing-box place" data-uuid="00000000-0000-0000-0000-000000000001">

# Ithaca
*island*

**Location:** 👑 `Greece` (territory)

</div>"#,
            test::place::ithaca()
                .display_details(Some(&test::place::ithaca::relations()))
                .to_string(),
        );
    }

    #[test]
    fn details_view_test_with_grandparent_location() {
        assert_eq!(
            r#"<div class="thing-box place" data-uuid="00000000-0000-0000-0000-000000000000">

# Chez Penelope
*castle*

**Location:** 🏝 `Ithaca`, 👑 `Greece`

</div>"#,
            test::place()
                .name("Chez Penelope")
                .subtype("castle".parse::<PlaceType>().unwrap())
                .build()
                .display_details(
                    Uuid::nil(),
                    Some(
                        &test::place::relations()
                            .location(test::place::ithaca())
                            .location(test::place::greece())
                            .build()
                    ),
                )
                .to_string(),
        );
    }
}
