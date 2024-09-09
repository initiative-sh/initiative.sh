use crate::utils::{capitalize, quoted_words, CaseInsensitiveStr};
use crate::world::command::ParsedThing;
use crate::world::npc::NpcData;
use crate::world::place::PlaceData;
use crate::world::Field;
use std::str::FromStr;

fn split_name(input: &str) -> Option<(&str, &str)> {
    let (named, comma) = quoted_words(input).fold((None, None), |(named, comma), word| {
        if named.is_none() && word.as_str().in_ci(&["named", "called"]) {
            (Some(word), comma)
        } else if word.as_str().ends_with(',') {
            (named, Some(word))
        } else {
            (named, comma)
        }
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

impl FromStr for ParsedThing<PlaceData> {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut place = PlaceData::default();
        let mut unknown_words = Vec::new();
        let mut word_count = 0;

        let description = if let Some((name, description)) = split_name(input) {
            place.name = Field::new(capitalize(name));
            description
        } else {
            input
        };

        for word in quoted_words(description) {
            let word_str = &word.as_str();
            word_count += 1;

            if word_str.in_ci(&["a", "an"]) {
                word_count -= 1;
            } else if let Ok(place_type) = word_str.parse() {
                place.subtype = Field::new(place_type);
            } else {
                unknown_words.push(word.range().to_owned());
            }
        }

        if unknown_words.is_empty() || unknown_words.len() <= word_count / 2 {
            Ok(ParsedThing {
                thing_data: place,
                unknown_words,
                word_count,
            })
        } else {
            Err(())
        }
    }
}

impl FromStr for ParsedThing<NpcData> {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut npc = NpcData::default();
        let mut unknown_words = Vec::new();
        let mut word_count = 0;

        let description = if let Some((name, description)) = split_name(input) {
            npc.name = Field::new(capitalize(name));
            description
        } else {
            input
        };

        for word in quoted_words(description) {
            let word_str = &word.as_str();
            word_count += 1;

            if word_str.in_ci(&["a", "an"]) {
                word_count -= 1;
            } else if word_str.in_ci(&["character", "npc", "person"]) {
                // ignore
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
                word_str.strip_suffix_ci("-year-old").map(|s| s.parse())
            {
                npc.age_years = Field::new(age_years);
            } else {
                unknown_words.push(word.range().to_owned());
            }
        }

        if unknown_words.is_empty() || unknown_words.len() <= word_count / 2 {
            Ok(ParsedThing {
                thing_data: npc,
                unknown_words,
                word_count,
            })
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
            let place: ParsedThing<PlaceData> = "inn".parse().unwrap();
            assert_eq!(
                Field::Locked("inn".parse::<PlaceType>().ok()),
                place.thing_data.subtype,
            );
            assert_eq!(0, place.unknown_words.len());
            assert_eq!(1, place.word_count);
        }

        {
            let place = "building named foo bar"
                .parse::<ParsedThing<PlaceData>>()
                .unwrap();
            assert_eq!(
                Some("Foo bar"),
                place.thing_data.name.value().map(|s| s.as_str()),
            );
            assert_eq!(0, place.unknown_words.len());
            assert_eq!(1, place.word_count);
        }

        {
            let place: ParsedThing<PlaceData> = "The Prancing Pony, an inn".parse().unwrap();
            assert_eq!(
                Field::Locked(Some("The Prancing Pony".to_string())),
                place.thing_data.name,
            );
            assert_eq!(
                Field::Locked("inn".parse::<PlaceType>().ok()),
                place.thing_data.subtype,
            );
            assert_eq!(0, place.unknown_words.len());
            assert_eq!(1, place.word_count);
        }

        {
            let place: ParsedThing<PlaceData> = "\"The Prancing Pony\", an inn".parse().unwrap();
            assert_eq!(
                Field::Locked(Some("The Prancing Pony".to_string())),
                place.thing_data.name,
            );
            assert_eq!(
                Field::Locked("inn".parse::<PlaceType>().ok()),
                place.thing_data.subtype,
            );
            assert_eq!(0, place.unknown_words.len());
            assert_eq!(1, place.word_count);
        }

        {
            let place: ParsedThing<PlaceData> = "a place called home".parse().unwrap();
            assert_eq!(
                Field::Locked(Some("Home".to_string())),
                place.thing_data.name
            );
            assert_eq!(Some(&PlaceType::Any), place.thing_data.subtype.value());
            assert_eq!(0, place.unknown_words.len());
            assert_eq!(1, place.word_count);
        }
    }

    #[test]
    fn npc_from_str_test() {
        {
            let npc: ParsedThing<NpcData> = "npc".parse().unwrap();
            assert_eq!(NpcData::default(), npc.thing_data);
            assert_eq!(0, npc.unknown_words.len());
            assert_eq!(1, npc.word_count);
        }
        assert_eq!(
            "npc".parse::<ParsedThing<NpcData>>().unwrap(),
            "NPC".parse::<ParsedThing<NpcData>>().unwrap(),
        );

        {
            let npc: ParsedThing<NpcData> = "elf".parse().unwrap();
            assert_eq!(Field::Locked(Some(Species::Elf)), npc.thing_data.species);
            assert_eq!(0, npc.unknown_words.len());
            assert_eq!(1, npc.word_count);
        }
        assert_eq!(
            "elf".parse::<ParsedThing<NpcData>>().unwrap(),
            "ELF".parse::<ParsedThing<NpcData>>().unwrap(),
        );

        {
            let npc: ParsedThing<NpcData> = "Potato Johnson, a non-binary elf".parse().unwrap();
            assert_eq!(
                Field::Locked(Some("Potato Johnson".to_string())),
                npc.thing_data.name,
            );
            assert_eq!(Field::Locked(Some(Species::Elf)), npc.thing_data.species);
            assert_eq!(
                Field::Locked(Some(Gender::NonBinaryThey)),
                npc.thing_data.gender
            );
            assert_eq!(0, npc.unknown_words.len());
            assert_eq!(2, npc.word_count);
        }
        assert_eq!(
            "Potato Johnson, a non-binary elf"
                .parse::<ParsedThing<NpcData>>()
                .unwrap(),
            "Potato Johnson, A NON-BINARY ELF"
                .parse::<ParsedThing<NpcData>>()
                .unwrap(),
        );

        {
            let npc: ParsedThing<NpcData> = "37-year-old boy named sue".parse().unwrap();
            assert_eq!(Field::Locked(Some("Sue".to_string())), npc.thing_data.name);
            assert_eq!(
                Field::Locked(Some(Gender::Masculine)),
                npc.thing_data.gender
            );
            assert_eq!(Field::Locked(Some(Age::Child)), npc.thing_data.age);
            assert_eq!(Field::Locked(Some(37)), npc.thing_data.age_years);
            assert_eq!(0, npc.unknown_words.len());
            assert_eq!(2, npc.word_count);
        }
        assert_eq!(
            "37-year-old boy named sue"
                .parse::<ParsedThing<NpcData>>()
                .unwrap(),
            "37-YEAR-OLD BOY NAMED sue"
                .parse::<ParsedThing<NpcData>>()
                .unwrap(),
        );

        {
            assert!("potato".parse::<ParsedThing<NpcData>>().is_err());
        }
    }
}
