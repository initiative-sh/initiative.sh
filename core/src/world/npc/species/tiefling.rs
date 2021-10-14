use super::human::Species as Human;
use super::{Age, Gender, Generate, Size};
use rand::prelude::*;

pub struct Species;

impl Generate for Species {
    fn gen_gender(rng: &mut impl Rng) -> Gender {
        Human::gen_gender(rng)
    }

    fn gen_age_years(rng: &mut impl Rng) -> u16 {
        rng.gen_range(0..=99)
    }

    fn age_from_years(years: u16) -> Age {
        match years {
            i if i < 2 => Age::Infant,
            i if i < 10 => Age::Child,
            i if i < 20 => Age::Adolescent,
            i if i < 30 => Age::YoungAdult,
            i if i < 40 => Age::Adult,
            i if i < 70 => Age::MiddleAged,
            i if i < 85 => Age::Elderly,
            _ => Age::Geriatric,
        }
    }

    fn gen_size(rng: &mut impl Rng, age_years: u16, gender: &Gender) -> Size {
        Human::gen_size(rng, age_years, gender)
    }
}

#[cfg(test)]
mod test_generate_for_species {
    use super::*;

    #[test]
    fn gen_gender_test() {
        let (mut rng1, mut rng2) = (SmallRng::seed_from_u64(0), SmallRng::seed_from_u64(0));

        for _ in 0..10 {
            assert_eq!(Species::gen_gender(&mut rng1), Human::gen_gender(&mut rng2));
        }
    }

    #[test]
    fn gen_age_years_test() {
        let mut rng = SmallRng::seed_from_u64(0);

        assert_eq!(
            [44, 43, 97, 46, 89],
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
        assert_eq!(Age::Infant, Species::age_from_years(0));
        assert_eq!(Age::Infant, Species::age_from_years(1));

        assert_eq!(Age::Child, Species::age_from_years(2));
        assert_eq!(Age::Child, Species::age_from_years(9));

        assert_eq!(Age::Adolescent, Species::age_from_years(10));
        assert_eq!(Age::Adolescent, Species::age_from_years(19));

        assert_eq!(Age::YoungAdult, Species::age_from_years(20));
        assert_eq!(Age::YoungAdult, Species::age_from_years(29));

        assert_eq!(Age::Adult, Species::age_from_years(30));
        assert_eq!(Age::Adult, Species::age_from_years(39));

        assert_eq!(Age::MiddleAged, Species::age_from_years(40));
        assert_eq!(Age::MiddleAged, Species::age_from_years(69));

        assert_eq!(Age::Elderly, Species::age_from_years(70));
        assert_eq!(Age::Elderly, Species::age_from_years(84));

        assert_eq!(Age::Geriatric, Species::age_from_years(85));
        assert_eq!(Age::Geriatric, Species::age_from_years(u16::MAX));
    }

    #[test]
    fn gen_size_test() {
        let (mut rng1, mut rng2) = (SmallRng::seed_from_u64(0), SmallRng::seed_from_u64(0));

        for _ in 0..10 {
            assert_eq!(
                Species::gen_size(&mut rng1, 0, &Gender::NonBinaryThey),
                Human::gen_size(&mut rng2, 0, &Gender::NonBinaryThey),
            );
        }
    }
}
