use crate::utils::{capitalize, quoted_words};
use crate::world::{Field, Npc, Place};
use std::str::FromStr;

fn split_name(input: &str) -> Option<(&str, &str)> {
    let (named, comma) =
        quoted_words(input).fold((None, None), |(named, comma), word| match word.as_str() {
            "named" | "called" if named.is_none() => (Some(word), comma),
            s if s.ends_with(',') => (named, Some(word)),
            _ => (named, comma),
        });

    let (name, description) = if let Some(word) = named {
        // "a boy named Sue"
        (&input[word.range().end..], &input[..word.range().start])
    } else if let Some(word) = comma {
        // "Nott the Brave, a goblin"
        (
            input[..word.range().end].trim_end_matches(','),
            &input[word.range().end..],
        )
    } else {
        return None;
    };

    if let (Some(name_start), Some(name_end)) =
        quoted_words(name).fold((None, None), |(name_start, _), word| {
            (
                name_start.or_else(|| Some(word.range().start)),
                Some(word.range().end),
            )
        })
    {
        let name = &name[name_start..name_end];
        if let Some(name_stripped) = name.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
            Some((name_stripped, description))
        } else {
            Some((name, description))
        }
    } else {
        None
    }
}

impl FromStr for Place {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut place = Place::default();
        let mut is_explicit = false;

        let description = if let Some((name, description)) = split_name(input) {
            place.name = Field::new(capitalize(name));
            description
        } else {
            input
        };

        for word in quoted_words(description) {
            if ["a", "an"].contains(&word.as_str()) {
                // ignore
            } else if ["building", "place"].contains(&word.as_str()) {
                is_explicit = true;
            } else if let Ok(place_type) = word.as_str().parse() {
                place.subtype = Field::new(place_type);
            } else {
                return Err(());
            }
        }

        if is_explicit || place.subtype.is_some() {
            Ok(place)
        } else {
            Err(())
        }
    }
}

impl FromStr for Npc {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut npc = Npc::default();
        let mut is_explicit = false;

        let description = if let Some((name, description)) = split_name(input) {
            npc.name = Field::new(capitalize(name));
            description
        } else {
            input
        };

        for word in quoted_words(description) {
            let word_str = &word.as_str();

            if ["a", "an"].contains(word_str) {
                // ignore
            } else if ["character", "npc", "person"].contains(word_str) {
                is_explicit = true;
            } else if let Ok(gender) = word_str.parse() {
                npc.gender = Field::new(gender);

                if let Ok(age) = word_str.parse() {
                    // Terms like "boy" and "woman" imply both age and gender, although let's treat
                    // them as secondary to other specifiers. "Old boy" and "baby woman" sound a
                    // bit odd but are presumably elderly and infant, respectively.
                    npc.age.replace(age);
                    npc.age.lock();
                }
            } else if let Ok(age) = word_str.parse() {
                npc.age = Field::new(age);
            } else if let Ok(species) = word_str.parse() {
                npc.species = Field::new(species);

                if let Ok(ethnicity) = word_str.parse() {
                    npc.ethnicity.replace(ethnicity);
                    npc.ethnicity.lock();
                }
            } else if let Ok(ethnicity) = word_str.parse() {
                npc.ethnicity = Field::new(ethnicity);
            } else if let Some(Ok(age_years)) =
                word_str.strip_suffix("-year-old").map(|s| s.parse())
            {
                npc.age_years = Field::new(age_years);
            } else {
                return Err(());
            }
        }

        if is_explicit
            || npc.age.is_some()
            || npc.age_years.is_some()
            || npc.ethnicity.is_some()
            || npc.gender.is_some()
            || npc.species.is_some()
        {
            Ok(npc)
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::world::npc::{Age, Gender, Species};
    use crate::world::place::PlaceType;

    #[test]
    fn place_from_str_test() {
        {
            let place: Place = "inn".parse().unwrap();
            assert_eq!(Field::Locked(Some(PlaceType::Inn)), place.subtype);
        }

        {
            let place = "building named foo bar".parse::<Place>().unwrap();
            assert_eq!(Some("Foo bar"), place.name.value().map(|s| s.as_str()));
        }

        {
            let place: Place = "The Prancing Pony, an inn".parse().unwrap();
            assert_eq!(
                Field::Locked(Some("The Prancing Pony".to_string())),
                place.name,
            );
            assert_eq!(Field::Locked(Some(PlaceType::Inn)), place.subtype);
        }

        {
            let place: Place = "\"The Prancing Pony\", an inn".parse().unwrap();
            assert_eq!(
                Field::Locked(Some("The Prancing Pony".to_string())),
                place.name,
            );
            assert_eq!(Field::Locked(Some(PlaceType::Inn)), place.subtype);
        }

        {
            let place: Place = "a place called home".parse().unwrap();
            assert_eq!(Field::Locked(Some("Home".to_string())), place.name);
            assert!(place.subtype.is_none());
        }
    }

    #[test]
    fn npc_from_str_test() {
        {
            assert_eq!(Ok(Npc::default()), "npc".parse::<Npc>());
        }

        {
            let npc: Npc = "elf".parse().unwrap();
            assert_eq!(Field::Locked(Some(Species::Elf)), npc.species);
        }

        {
            let npc: Npc = "Potato Johnson, a non-binary elf".parse().unwrap();
            assert_eq!(Field::Locked(Some("Potato Johnson".to_string())), npc.name);
            assert_eq!(Field::Locked(Some(Species::Elf)), npc.species);
            assert_eq!(Field::Locked(Some(Gender::NonBinaryThey)), npc.gender);
        }

        {
            let npc: Npc = "37-year-old boy named sue".parse().unwrap();
            assert_eq!(Field::Locked(Some("Sue".to_string())), npc.name);
            assert_eq!(Field::Locked(Some(Gender::Masculine)), npc.gender);
            assert_eq!(Field::Locked(Some(Age::Child)), npc.age);
            assert_eq!(Field::Locked(Some(37)), npc.age_years);
        }

        {
            assert_eq!(Err(()), "potato".parse::<Npc>());
        }
    }
}
