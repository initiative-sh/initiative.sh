use super::{Age, Gender, NpcData, NpcRelations, Uuid};
use std::fmt;

pub struct SummaryView<'a>(&'a NpcData);

pub struct DescriptionView<'a>(&'a NpcData);

pub struct DetailsView<'a> {
    npc: &'a NpcData,
    uuid: Uuid,
    relations: NpcRelations,
}

fn write_summary_details(npc: &NpcData, f: &mut fmt::Formatter) -> fmt::Result {
    if let Some(age) = npc.age.value() {
        age.fmt_with_species_ethnicity(npc.species.value(), npc.ethnicity.value(), f)?;
    } else if let Some(species) = npc.species.value() {
        write!(f, "{}", species)?;
    } else if let Some(ethnicity) = npc.ethnicity.value() {
        write!(f, "{} person", ethnicity)?;
    } else {
        write!(f, "person")?;
    }

    if let Some(gender) = npc.gender.value() {
        write!(f, ", {}", gender.pronouns())?;
    }

    Ok(())
}

impl<'a> SummaryView<'a> {
    pub fn new(npc: &'a NpcData) -> Self {
        Self(npc)
    }
}

impl<'a> DescriptionView<'a> {
    pub fn new(npc: &'a NpcData) -> Self {
        Self(npc)
    }
}

impl<'a> DetailsView<'a> {
    pub fn new(npc: &'a NpcData, uuid: Uuid, relations: NpcRelations) -> Self {
        Self {
            npc,
            uuid,
            relations,
        }
    }
}

impl<'a> fmt::Display for SummaryView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let npc = self.0;
        let has_details = npc.age.is_some()
            || npc.ethnicity.is_some()
            || npc.gender.is_some()
            || npc.species.is_some();

        write!(
            f,
            "{} ",
            match (npc.age.value(), npc.gender.value()) {
                (Some(Age::Infant), _) => '\u{1f476}',
                (Some(Age::Child | Age::Adolescent), Some(Gender::Feminine)) => '\u{1f467}',
                (Some(Age::Child | Age::Adolescent), Some(Gender::Masculine)) => '\u{1f466}',
                (Some(Age::Child | Age::Adolescent), _) => '\u{1f9d2}',
                (Some(Age::Elderly | Age::Geriatric), Some(Gender::Feminine)) => '\u{1f475}',
                (Some(Age::Elderly | Age::Geriatric), Some(Gender::Masculine)) => '\u{1f474}',
                (Some(Age::Elderly | Age::Geriatric), _) => '\u{1f9d3}',
                (_, Some(Gender::Feminine)) => '\u{1f469}',
                (_, Some(Gender::Masculine)) => '\u{1f468}',
                _ => '\u{1f9d1}',
            },
        )?;

        if let Some(name) = npc.name.value() {
            if has_details {
                write!(f, "`{}` (", name)?;
                write_summary_details(npc, f)?;
                write!(f, ")")?;
            } else {
                write!(f, "`{}`", name)?;
            }
        } else {
            write_summary_details(npc, f)?;
        }

        Ok(())
    }
}

impl<'a> fmt::Display for DescriptionView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_summary_details(self.0, f)
    }
}

impl<'a> fmt::Display for DetailsView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Self {
            npc,
            uuid,
            relations,
        } = self;

        writeln!(f, "<div class=\"thing-box npc\" data-uuid=\"{}\">\n", uuid)?;

        npc.name
            .value()
            .map(|name| write!(f, "# {}", name))
            .unwrap_or_else(|| write!(f, "# Unnamed NPC"))?;

        write!(f, "\n*")?;
        write_summary_details(npc, f)?;
        write!(f, "*")?;

        match (npc.species.value(), npc.ethnicity.value()) {
            (Some(species), Some(ethnicity)) if ethnicity != &species.default_ethnicity() => {
                write!(f, "\n\n**Species:** {} ({})", species, ethnicity)?
            }
            (Some(species), _) => write!(f, "\n\n**Species:** {}", species)?,
            (None, Some(ethnicity)) => write!(f, "\n\n**Ethnicity:** {}", ethnicity)?,
            (None, None) => write!(f, "\n\n**Species:** N/A")?,
        }

        npc.gender
            .value()
            .map(|gender| write!(f, "\\\n**Gender:** {}", gender.name()))
            .transpose()?;
        npc.age_years
            .value()
            .map(|age_years| write!(f, "\\\n**Age:** {} years", age_years))
            .transpose()?;
        npc.size
            .value()
            .map(|size| write!(f, "\\\n**Size:** {}", size))
            .transpose()?;

        relations
            .location
            .as_ref()
            .map(|(parent, grandparent)| {
                if let Some(grandparent) = grandparent {
                    write!(
                        f,
                        "\\\n**Location:** {}, {}",
                        parent.display_name(),
                        grandparent.display_name(),
                    )
                } else {
                    write!(f, "\\\n**Location:** {}", parent.display_summary())
                }
            })
            .transpose()?;

        write!(f, "\n\n</div>")?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::world::npc::{Age, Ethnicity, Gender, Npc, Size, Species};
    use crate::world::place::{Place, PlaceData, PlaceType};
    use crate::world::Field;

    const NAME: u8 = 0b1;
    const AGE: u8 = 0b10;
    const SPECIES: u8 = 0b100;
    const GENDER: u8 = 0b1000;
    const ETHNICITY: u8 = 0b10000;

    #[test]
    fn summary_view_test() {
        let (expected, actual): (Vec<String>, Vec<String>) = [
            "🧑 person",
            "🧑 `Potato Johnson`",
            "🧓 elderly person",
            "🧓 `Potato Johnson` (elderly person)",
            "🧑 human",
            "🧑 `Potato Johnson` (human)",
            "🧓 elderly human",
            "🧓 `Potato Johnson` (elderly human)",
            "👨 person, he/him",
            "👨 `Potato Johnson` (person, he/him)",
            "👴 elderly person, he/him",
            "👴 `Potato Johnson` (elderly person, he/him)",
            "👨 human, he/him",
            "👨 `Potato Johnson` (human, he/him)",
            "👴 elderly human, he/him",
            "👴 `Potato Johnson` (elderly human, he/him)",
            "🧑 elvish person",
            "🧑 `Potato Johnson` (elvish person)",
            "🧓 elderly elvish person",
            "🧓 `Potato Johnson` (elderly elvish person)",
            "🧑 human",
            "🧑 `Potato Johnson` (human)",
            "🧓 elderly human",
            "🧓 `Potato Johnson` (elderly human)",
            "👨 elvish person, he/him",
            "👨 `Potato Johnson` (elvish person, he/him)",
            "👴 elderly elvish person, he/him",
            "👴 `Potato Johnson` (elderly elvish person, he/him)",
            "👨 human, he/him",
            "👨 `Potato Johnson` (human, he/him)",
            "👴 elderly human, he/him",
            "👴 `Potato Johnson` (elderly human, he/him)",
        ]
        .iter()
        .enumerate()
        .map(|(i, s)| {
            (
                s.to_string(),
                gen_npc(i as u8).display_summary().to_string(),
            )
        })
        .unzip();

        assert_eq!(expected, actual);
    }

    #[test]
    fn details_view_test_filled() {
        let mut npc = NpcData::default();
        npc.name.replace("Potato Johnson".to_string());
        npc.species.replace(Species::Human);
        npc.ethnicity.replace(Ethnicity::Elvish);
        npc.gender.replace(Gender::NonBinaryThey);
        npc.age.replace(Age::Adult);
        npc.age_years.replace(30);
        npc.size.replace(Size::Medium {
            height: 71,
            weight: 140,
        });

        assert_eq!(
            r#"<div class="thing-box npc" data-uuid="00000000-0000-0000-0000-000000000000">

# Potato Johnson
*adult human, they/them*

**Species:** human (elvish)\
**Gender:** non-binary\
**Age:** 30 years\
**Size:** 5'11", 140 lbs (medium)

</div>"#,
            npc.display_details(Uuid::nil(), NpcRelations::default())
                .to_string(),
        );
    }

    #[test]
    fn details_view_test_species_ethnicity() {
        assert_eq!(
            r#"<div class="thing-box npc" data-uuid="00000000-0000-0000-0000-000000000000">

# Unnamed NPC
*human*

**Species:** human

</div>"#,
            gen_npc(SPECIES)
                .display_details(NpcRelations::default())
                .to_string(),
        );
        assert_eq!(
            r#"<div class="thing-box npc" data-uuid="00000000-0000-0000-0000-000000000000">

# Unnamed NPC
*elvish person*

**Ethnicity:** elvish

</div>"#,
            gen_npc(ETHNICITY)
                .display_details(NpcRelations::default())
                .to_string(),
        );
        assert_eq!(
            r#"<div class="thing-box npc" data-uuid="00000000-0000-0000-0000-000000000000">

# Unnamed NPC
*human*

**Species:** human (elvish)

</div>"#,
            gen_npc(ETHNICITY | SPECIES)
                .display_details(NpcRelations::default())
                .to_string(),
        );
    }

    #[test]
    fn details_view_test_empty() {
        assert_eq!(
            r#"<div class="thing-box npc" data-uuid="00000000-0000-0000-0000-000000000000">

# Unnamed NPC
*person*

**Species:** N/A

</div>"#,
            NpcData::default()
                .display_details(Uuid::nil(), NpcRelations::default())
                .to_string(),
        );
    }

    #[test]
    fn details_view_test_with_parent_location() {
        let npc = NpcData {
            name: "Frodo Baggins".into(),
            ..Default::default()
        };

        let relations = NpcRelations {
            location: Some((
                Place {
                    uuid: Uuid::nil(),
                    data: PlaceData {
                        name: "Mount Doom".into(),
                        subtype: "mountain".parse::<PlaceType>().unwrap().into(),
                        ..Default::default()
                    },
                },
                None,
            )),
        };

        assert_eq!(
            r#"<div class="thing-box npc" data-uuid="00000000-0000-0000-0000-000000000000">

# Frodo Baggins
*person*

**Species:** N/A\
**Location:** ⛰ `Mount Doom` (mountain)

</div>"#,
            DetailsView::new(&npc, Uuid::nil(), relations).to_string(),
        );
    }

    #[test]
    fn details_view_test_with_grandparent_location() {
        let npc = NpcData {
            name: "Frodo Baggins".into(),
            ..Default::default()
        };

        let relations = NpcRelations {
            location: Some((
                Place {
                    uuid: Uuid::nil(),
                    data: PlaceData {
                        name: "The Prancing Pony".into(),
                        subtype: "inn".parse::<PlaceType>().unwrap().into(),
                        ..Default::default()
                    },
                },
                Some(Place {
                    uuid: Uuid::nil(),
                    data: PlaceData {
                        name: "Bree".into(),
                        subtype: "town".parse::<PlaceType>().unwrap().into(),
                        ..Default::default()
                    },
                }),
            )),
        };

        assert_eq!(
            r#"<div class="thing-box npc" data-uuid="00000000-0000-0000-0000-000000000000">

# Frodo Baggins
*person*

**Species:** N/A\
**Location:** 🏨 `The Prancing Pony`, 🏘 `Bree`

</div>"#,
            DetailsView::new(&npc, Uuid::nil(), relations).to_string(),
        );
    }

    fn gen_npc(bitmask: u8) -> Npc {
        let mut npc_data = NpcData::default();

        if bitmask & NAME > 0 {
            npc_data.name = Field::new_generated("Potato Johnson".to_string());
        }
        if bitmask & AGE > 0 {
            npc_data.age = Field::new_generated(Age::Elderly);
            npc_data.age_years = Field::new_generated(60);
        }
        if bitmask & SPECIES > 0 {
            npc_data.species = Field::new_generated(Species::Human);
        }
        if bitmask & GENDER > 0 {
            npc_data.gender = Field::new_generated(Gender::Masculine);
        }
        if bitmask & ETHNICITY > 0 {
            npc_data.ethnicity = Field::new_generated(Ethnicity::Elvish);
        }

        Npc {
            uuid: Uuid::nil(),
            data: npc_data,
        }
    }
}
