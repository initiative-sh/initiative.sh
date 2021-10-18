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

    fn gen_age(rng: &mut impl Rng) -> Age {
        match rng.gen_range(0..=400) {
            i if i < 2 => Age::Infant(i),
            i if i < 10 => Age::Child(i),
            i if i < 20 => Age::Adolescent(i),
            i if i < 40 => Age::YoungAdult(i),
            i if i < 200 => Age::Adult(i),
            i if i < 300 => Age::MiddleAged(i),
            i if i < 350 => Age::Elderly(i),
            i => Age::Geriatric(i),
        }
    }

    fn gen_size(rng: &mut impl Rng, _age: &Age, _gender: &Gender) -> Size {
        let size = rng.gen_range(1..=6) + rng.gen_range(1..=6);
        Size::Medium {
            height: 48 + size,
            weight: 125 + size * 4,
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
    fn gen_age_test() {
        let mut rng = SmallRng::seed_from_u64(0);

        assert_eq!(
            [
                Age::Adult(179),
                Age::Adult(176),
                Age::Geriatric(392),
                Age::Adult(185),
                Age::Geriatric(359),
            ],
            [
                Species::gen_age(&mut rng),
                Species::gen_age(&mut rng),
                Species::gen_age(&mut rng),
                Species::gen_age(&mut rng),
                Species::gen_age(&mut rng),
            ],
        );
    }

    #[test]
    fn gen_size_test() {
        let mut rng = SmallRng::seed_from_u64(0);
        let age = Age::Adult(0);
        let t = Gender::NonBinaryThey;

        let size = |height, weight| Size::Medium { height, weight };

        assert_eq!(
            [
                size(54, 149),
                size(57, 161),
                size(60, 173),
                size(55, 153),
                size(56, 157),
            ],
            [
                Species::gen_size(&mut rng, &age, &t),
                Species::gen_size(&mut rng, &age, &t),
                Species::gen_size(&mut rng, &age, &t),
                Species::gen_size(&mut rng, &age, &t),
                Species::gen_size(&mut rng, &age, &t),
            ]
        );
    }
}
