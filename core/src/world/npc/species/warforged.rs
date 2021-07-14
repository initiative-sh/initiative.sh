use super::{Age, Gender, Generate, Rng, Size};

pub struct Species;

impl Generate for Species {
    fn gen_gender(rng: &mut impl Rng) -> Gender {
        match rng.gen_range(1..=100) {
            1..=60 => Gender::Neuter,
            61..=75 => Gender::Masculine,
            76..=90 => Gender::Feminine,
            91..=100 => Gender::Trans,
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
    use rand::rngs::mock::StepRng;
    use std::collections::HashMap;

    #[test]
    fn gen_gender_test() {
        let mut rng = StepRng::new(0, 0xDECAFBAD);
        let mut genders: HashMap<String, u16> = HashMap::new();

        for _ in 0..100 {
            let gender = Species::gen_gender(&mut rng);
            *genders.entry(format!("{}", gender)).or_default() += 1;
        }

        assert_eq!(4, genders.len());
        assert_eq!(Some(&59), genders.get("neuter (it)"));
        assert_eq!(Some(&15), genders.get("feminine (she/her)"));
        assert_eq!(Some(&16), genders.get("masculine (he/him)"));
        assert_eq!(Some(&10), genders.get("trans (they/them)"));
    }

    #[test]
    fn gen_age_test() {
        let mut rng = StepRng::new(0, 0xDECAFBAD);

        assert_eq!(Age::Adult(2), Species::gen_age(&mut rng));
        assert_eq!(Age::Adult(27), Species::gen_age(&mut rng));
        assert_eq!(Age::Adult(23), Species::gen_age(&mut rng));
        assert_eq!(Age::Adult(19), Species::gen_age(&mut rng));
        assert_eq!(Age::Adult(15), Species::gen_age(&mut rng));
    }

    #[test]
    fn gen_size_test() {
        let mut rng = StepRng::new(0, 0xDECAFBAD);
        let age = Age::Adult(0);
        let t = Gender::Trans;

        let size = |height, weight| Size::Medium { height, weight };

        assert_eq!(size(77, 298), Species::gen_size(&mut rng, &age, &t));
        assert_eq!(size(79, 306), Species::gen_size(&mut rng, &age, &t));
        assert_eq!(size(76, 294), Species::gen_size(&mut rng, &age, &t));
        assert_eq!(size(73, 282), Species::gen_size(&mut rng, &age, &t));
        assert_eq!(size(81, 314), Species::gen_size(&mut rng, &age, &t));
    }
}
