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
        rng.gen_range(0..=500)
    }

    fn gen_years_from_age(rng: &mut impl Rng, age: &Age) -> u16 {
        rng.gen_range(match age {
            Age::Infant => 0..=1,
            Age::Child => 2..=9,
            Age::Adolescent => 10..=19,
            Age::YoungAdult => 20..=29,
            Age::Adult => 30..=99,
            Age::MiddleAged => 100..=249,
            Age::Elderly => 250..=399,
            Age::Geriatric => 400..=500,
        })
    }

    fn age_from_years(years: u16) -> Age {
        match years {
            i if i < 2 => Age::Infant,
            i if i < 10 => Age::Child,
            i if i < 20 => Age::Adolescent,
            i if i < 30 => Age::YoungAdult,
            i if i < 100 => Age::Adult,
            i if i < 250 => Age::MiddleAged,
            i if i < 400 => Age::Elderly,
            _ => Age::Geriatric,
        }
    }

    fn gen_size(rng: &mut impl Rng, _age_years: u16, _gender: &Gender) -> Size {
        let size = rng.gen_range(1..=4) + rng.gen_range(1..=4);
        Size::Small {
            height: 36 + size,
            weight: 32 + size * 2,
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
            [224, 220, 490, 231, 449],
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
        assert_eq!(Age::Infant, Species::age_from_years(1));

        assert_eq!(Age::Child, Species::age_from_years(2));
        assert_eq!(Age::Child, Species::age_from_years(9));

        assert_eq!(Age::Adolescent, Species::age_from_years(10));
        assert_eq!(Age::Adolescent, Species::age_from_years(19));

        assert_eq!(Age::YoungAdult, Species::age_from_years(20));
        assert_eq!(Age::YoungAdult, Species::age_from_years(29));

        assert_eq!(Age::Adult, Species::age_from_years(30));
        assert_eq!(Age::Adult, Species::age_from_years(99));

        assert_eq!(Age::MiddleAged, Species::age_from_years(100));
        assert_eq!(Age::MiddleAged, Species::age_from_years(249));

        assert_eq!(Age::Elderly, Species::age_from_years(250));
        assert_eq!(Age::Elderly, Species::age_from_years(399));

        assert_eq!(Age::Geriatric, Species::age_from_years(400));
        assert_eq!(Age::Geriatric, Species::age_from_years(u16::MAX));
    }

    #[test]
    fn gen_size_test() {
        let mut rng = SmallRng::seed_from_u64(0);
        let t = Gender::NonBinaryThey;

        let size = |height, weight| Size::Small { height, weight };

        assert_eq!(
            [
                size(40, 40),
                size(42, 44),
                size(44, 48),
                size(41, 42),
                size(42, 44),
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
