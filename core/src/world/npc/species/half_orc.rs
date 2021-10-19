use super::{Age, Gender, Generate, Size};
use rand::prelude::*;

pub struct Species;

impl Generate for Species {
    fn gen_gender(rng: &mut impl Rng) -> Gender {
        match rng.gen_range(1..=101) {
            1..=50 => Gender::Feminine,
            51..=100 => Gender::Masculine,
            101 => Gender::NonBinaryThey,
            _ => unreachable!(),
        }
    }

    fn gen_age_years(rng: &mut impl Rng) -> u16 {
        rng.gen_range(0..=79)
    }

    fn gen_years_from_age(rng: &mut impl Rng, age: &Age) -> u16 {
        rng.gen_range(match age {
            Age::Infant => return 0,
            Age::Child => 1..=7,
            Age::Adolescent => 8..=14,
            Age::YoungAdult => 15..=19,
            Age::Adult => 20..=34,
            Age::MiddleAged => 35..=54,
            Age::Elderly => 55..=64,
            Age::Geriatric => 65..=79,
        })
    }

    fn age_from_years(years: u16) -> Age {
        match years {
            i if i < 1 => Age::Infant,
            i if i < 8 => Age::Child,
            i if i < 15 => Age::Adolescent,
            i if i < 20 => Age::YoungAdult,
            i if i < 35 => Age::Adult,
            i if i < 55 => Age::MiddleAged,
            i if i < 65 => Age::Elderly,
            _ => Age::Geriatric,
        }
    }

    fn gen_size(rng: &mut impl Rng, _age_years: u16, _gender: &Gender) -> Size {
        let size = rng.gen_range(1..=8) + rng.gen_range(1..=8);
        Size::Medium {
            height: 60 + size,
            weight: 130 + size * 6,
        }
    }
}

#[cfg(test)]
mod test_generate_for_species {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn gen_gender_test() {
        let mut rng = SmallRng::seed_from_u64(0);
        let mut genders: HashMap<String, u16> = HashMap::new();

        for _ in 0..500 {
            let gender = Species::gen_gender(&mut rng);
            *genders.entry(format!("{}", gender)).or_default() += 1;
        }

        assert_eq!(3, genders.len());
        assert_eq!(Some(&3), genders.get("non-binary (they/them)"));
        assert_eq!(Some(&233), genders.get("feminine (she/her)"));
        assert_eq!(Some(&264), genders.get("masculine (he/him)"));
    }

    #[test]
    fn gen_age_years_test() {
        let mut rng = SmallRng::seed_from_u64(0);

        assert_eq!(
            [35, 35, 78, 36, 71],
            [
                Species::gen_age_years(&mut rng),
                Species::gen_age_years(&mut rng),
                Species::gen_age_years(&mut rng),
                Species::gen_age_years(&mut rng),
                Species::gen_age_years(&mut rng),
            ],
        );
    }

    #[test]
    fn gen_years_from_age_test() {
        let ages = [
            Age::Infant,
            Age::Child,
            Age::Adolescent,
            Age::YoungAdult,
            Age::Adult,
            Age::MiddleAged,
            Age::Elderly,
            Age::Geriatric,
        ];

        for age in ages {
            let mut rng = SmallRng::seed_from_u64(0);

            for _ in 0..10 {
                let age_years = Species::gen_years_from_age(&mut rng, &age);
                assert_eq!(age, Species::age_from_years(age_years));
            }
        }
    }

    #[test]
    fn age_from_years_test() {
        assert_eq!(Age::Infant, Species::age_from_years(0));

        assert_eq!(Age::Child, Species::age_from_years(1));
        assert_eq!(Age::Child, Species::age_from_years(7));

        assert_eq!(Age::Adolescent, Species::age_from_years(8));
        assert_eq!(Age::Adolescent, Species::age_from_years(14));

        assert_eq!(Age::YoungAdult, Species::age_from_years(15));
        assert_eq!(Age::YoungAdult, Species::age_from_years(19));

        assert_eq!(Age::Adult, Species::age_from_years(20));
        assert_eq!(Age::Adult, Species::age_from_years(34));

        assert_eq!(Age::MiddleAged, Species::age_from_years(35));
        assert_eq!(Age::MiddleAged, Species::age_from_years(54));

        assert_eq!(Age::Elderly, Species::age_from_years(55));
        assert_eq!(Age::Elderly, Species::age_from_years(64));

        assert_eq!(Age::Geriatric, Species::age_from_years(65));
        assert_eq!(Age::Geriatric, Species::age_from_years(u16::MAX));
    }

    #[test]
    fn gen_size_test() {
        let mut rng = SmallRng::seed_from_u64(0);
        let t = Gender::NonBinaryThey;

        let size = |height, weight| Size::Medium { height, weight };

        assert_eq!(
            [
                size(68, 178),
                size(72, 202),
                size(76, 226),
                size(69, 184),
                size(71, 196),
            ],
            [
                Species::gen_size(&mut rng, 0, &t),
                Species::gen_size(&mut rng, 0, &t),
                Species::gen_size(&mut rng, 0, &t),
                Species::gen_size(&mut rng, 0, &t),
                Species::gen_size(&mut rng, 0, &t),
            ]
        );
    }
}
