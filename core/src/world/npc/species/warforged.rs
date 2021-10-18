use super::{Age, Gender, Generate, Size};
use rand::prelude::*;

pub struct Species;

impl Generate for Species {
    fn gen_gender(rng: &mut impl Rng) -> Gender {
        match rng.gen_range(1..=100) {
            1..=60 => Gender::Neuter,
            61..=75 => Gender::Masculine,
            76..=90 => Gender::Feminine,
            91..=100 => Gender::NonBinaryThey,
            _ => unreachable!(),
        }
    }

    fn gen_age(rng: &mut impl Rng) -> Age {
        Age::Adult(rng.gen_range(2..=30))
    }

    fn gen_size(rng: &mut impl Rng, _age: &Age, _gender: &Gender) -> Size {
        let size = rng.gen_range(1..=6) + rng.gen_range(1..=6);
        Size::Medium {
            height: 70 + size,
            weight: 270 + size * 4,
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

        for _ in 0..100 {
            let gender = Species::gen_gender(&mut rng);
            *genders.entry(format!("{}", gender)).or_default() += 1;
        }

        assert_eq!(4, genders.len());
        assert_eq!(Some(&59), genders.get("neuter (it)"));
        assert_eq!(Some(&15), genders.get("feminine (she/her)"));
        assert_eq!(Some(&16), genders.get("masculine (he/him)"));
        assert_eq!(Some(&10), genders.get("non-binary (they/them)"));
    }

    #[test]
    fn gen_age_test() {
        let mut rng = SmallRng::seed_from_u64(0);

        assert_eq!(
            [
                Age::Adult(2),
                Age::Adult(27),
                Age::Adult(23),
                Age::Adult(19),
                Age::Adult(15),
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
                size(77, 298),
                size(79, 306),
                size(76, 294),
                size(73, 282),
                size(81, 314),
            ],
            [
                Species::gen_size(&mut rng, &age, &t),
                Species::gen_size(&mut rng, &age, &t),
                Species::gen_size(&mut rng, &age, &t),
                Species::gen_size(&mut rng, &age, &t),
                Species::gen_size(&mut rng, &age, &t),
            ],
        );
    }
}
