use super::{Age, Gender, Generate, Rng, Size};

pub struct Race;

impl Generate for Race {
    fn gen_gender(rng: &mut impl Rng) -> Gender {
        match rng.gen_range(1..=101) {
            1..=50 => Gender::Feminine,
            51..=100 => Gender::Masculine,
            101 => Gender::Trans,
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
mod test_generate_for_race {
    use super::*;
    use rand::rngs::mock::StepRng;
    use std::collections::HashMap;

    #[test]
    fn gen_gender_test() {
        let mut rng = StepRng::new(0, 0xDECAFBAD);
        let mut genders: HashMap<String, u16> = HashMap::new();

        for _ in 0..100 {
            let gender = Race::gen_gender(&mut rng);
            *genders.entry(format!("{}", gender)).or_default() += 1;
        }

        assert_eq!(3, genders.len());
        assert_eq!(Some(&50), genders.get("feminine (she/her)"));
        assert_eq!(Some(&48), genders.get("masculine (he/him)"));
        assert_eq!(Some(&2), genders.get("trans (they/them)"));
    }

    #[test]
    fn gen_age_test() {
        let mut rng = StepRng::new(0, 0xDECAFBAD);

        assert_eq!(
            [
                Age::Infant(0),
                Age::Elderly(348),
                Age::MiddleAged(296),
                Age::MiddleAged(244),
                Age::Adult(192)
            ],
            [
                Race::gen_age(&mut rng),
                Race::gen_age(&mut rng),
                Race::gen_age(&mut rng),
                Race::gen_age(&mut rng),
                Race::gen_age(&mut rng)
            ]
        );
    }

    #[test]
    fn gen_size_test() {
        let mut rng = StepRng::new(0, 0xDECAFBAD);
        let age = Age::Adult(0);
        let t = Gender::Trans;

        let size = |height, weight| Size::Medium { height, weight };

        assert_eq!(
            [
                size(55, 153),
                size(57, 161),
                size(54, 149),
                size(51, 137),
                size(59, 169)
            ],
            [
                Race::gen_size(&mut rng, &age, &t),
                Race::gen_size(&mut rng, &age, &t),
                Race::gen_size(&mut rng, &age, &t),
                Race::gen_size(&mut rng, &age, &t),
                Race::gen_size(&mut rng, &age, &t)
            ]
        );
    }
}
