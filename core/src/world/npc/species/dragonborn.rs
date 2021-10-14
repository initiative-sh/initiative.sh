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

    fn age_from_years(years: u16) -> Age {
        match years {
            i if i < 3 => Age::Child,
            i if i < 15 => Age::Adolescent,
            i if i < 25 => Age::YoungAdult,
            i if i < 40 => Age::Adult,
            i if i < 60 => Age::MiddleAged,
            i if i < 70 => Age::Elderly,
            _ => Age::Geriatric,
        }
    }

    fn gen_size(rng: &mut impl Rng, _age_years: u16, _gender: &Gender) -> Size {
        let size = rng.gen_range(1..=6) + rng.gen_range(1..=6);
        Size::Medium {
            height: 72 + size,
            weight: 220 + size * 6,
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
    fn age_from_years_test() {
        assert_eq!(Age::Child, Species::age_from_years(0));
        assert_eq!(Age::Child, Species::age_from_years(2));

        assert_eq!(Age::Adolescent, Species::age_from_years(3));
        assert_eq!(Age::Adolescent, Species::age_from_years(14));

        assert_eq!(Age::YoungAdult, Species::age_from_years(15));
        assert_eq!(Age::YoungAdult, Species::age_from_years(24));

        assert_eq!(Age::Adult, Species::age_from_years(25));
        assert_eq!(Age::Adult, Species::age_from_years(39));

        assert_eq!(Age::MiddleAged, Species::age_from_years(40));
        assert_eq!(Age::MiddleAged, Species::age_from_years(59));

        assert_eq!(Age::Elderly, Species::age_from_years(60));
        assert_eq!(Age::Elderly, Species::age_from_years(69));

        assert_eq!(Age::Geriatric, Species::age_from_years(70));
        assert_eq!(Age::Geriatric, Species::age_from_years(u16::MAX));
    }

    #[test]
    fn gen_size_test() {
        let mut rng = SmallRng::seed_from_u64(0);
        let t = Gender::NonBinaryThey;

        let size = |height, weight| Size::Medium { height, weight };

        assert_eq!(
            [
                size(78, 256),
                size(81, 274),
                size(84, 292),
                size(79, 262),
                size(80, 268),
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
