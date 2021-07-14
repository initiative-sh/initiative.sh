use super::human::Species as Human;
use super::{Age, Gender, Generate, Rng, Size};

pub struct Species;

impl Generate for Species {
    fn gen_gender(rng: &mut impl Rng) -> Gender {
        Human::gen_gender(rng)
    }

    fn gen_age(rng: &mut impl Rng) -> Age {
        match rng.gen_range(0..=99) {
            i if i < 2 => Age::Infant(i),
            i if i < 10 => Age::Child(i),
            i if i < 20 => Age::Adolescent(i),
            i if i < 30 => Age::YoungAdult(i),
            i if i < 40 => Age::Adult(i),
            i if i < 70 => Age::MiddleAged(i),
            i if i < 85 => Age::Elderly(i),
            i => Age::Geriatric(i),
        }
    }

    fn gen_size(rng: &mut impl Rng, age: &Age, gender: &Gender) -> Size {
        Human::gen_size(rng, age, gender)
    }
}

#[cfg(test)]
mod test_generate_for_species {
    use super::*;
    use rand::rngs::mock::StepRng;

    #[test]
    fn gen_gender_test() {
        let (mut rng1, mut rng2) = (StepRng::new(0, 0xDECAFBAD), StepRng::new(0, 0xDECAFBAD));

        for _ in 0..10 {
            assert_eq!(Species::gen_gender(&mut rng1), Human::gen_gender(&mut rng2));
        }
    }

    #[test]
    fn gen_age_test() {
        let mut rng = StepRng::new(0, 0xDECAFBAD);

        assert_eq!(
            [
                Age::Infant(0),
                Age::Geriatric(87),
                Age::Elderly(74),
                Age::MiddleAged(61),
                Age::MiddleAged(48)
            ],
            [
                Species::gen_age(&mut rng),
                Species::gen_age(&mut rng),
                Species::gen_age(&mut rng),
                Species::gen_age(&mut rng),
                Species::gen_age(&mut rng)
            ]
        );
    }

    #[test]
    fn gen_size_test() {
        let (mut rng1, mut rng2) = (StepRng::new(0, 0xDECAFBAD), StepRng::new(0, 0xDECAFBAD));

        for _ in 0..10 {
            assert_eq!(
                Species::gen_size(&mut rng1, &Age::Adult(0), &Gender::Trans),
                Human::gen_size(&mut rng2, &Age::Adult(0), &Gender::Trans)
            );
        }
    }
}
