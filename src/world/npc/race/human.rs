use super::{Age, Gender, RaceGenerate, Rng, Size};

pub struct Race;

impl RaceGenerate for Race {
    fn gen_gender(rng: &mut impl Rng) -> Gender {
        match rng.gen_range(1..=101) {
            1..=50 => Gender::Feminine,
            51..=100 => Gender::Masculine,
            101 => Gender::Trans,
            _ => unreachable!(),
        }
    }

    fn gen_age(rng: &mut impl Rng) -> Age {
        match rng.gen_range(0..=79) {
            i if i < 2 => Age::Infant(i),
            i if i < 10 => Age::Child(i),
            i if i < 20 => Age::Adolescent(i),
            i if i < 30 => Age::YoungAdult(i),
            i if i < 40 => Age::Adult(i),
            i if i < 60 => Age::MiddleAged(i),
            i if i < 70 => Age::Elderly(i),
            i => Age::Geriatric(i),
        }
    }

    fn gen_name(_rng: &mut impl Rng, _age: &Age, _gender: &Gender) -> String {
        String::from("Potato Johnson")
    }

    fn gen_size(_rng: &mut impl Rng, _age: &Age, _gender: &Gender) -> Size {
        Size::Medium {
            height: 72,
            weight: 180,
        }
    }
}

#[cfg(test)]
mod test_race {
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
        assert_eq!(Some(&2), genders.get("trans (they/them)"));
        assert_eq!(Some(&50), genders.get("feminine (she/her)"));
        assert_eq!(Some(&48), genders.get("masculine (he/him)"));
    }

    #[test]
    fn gen_age_test() {
        let mut rng = StepRng::new(0, 0xDECAFBAD);

        assert_eq!(Age::Infant(0), Race::gen_age(&mut rng));
        assert_eq!(Age::Elderly(69), Race::gen_age(&mut rng));
        assert_eq!(Age::MiddleAged(59), Race::gen_age(&mut rng));
        assert_eq!(Age::MiddleAged(48), Race::gen_age(&mut rng));
        assert_eq!(Age::Adult(38), Race::gen_age(&mut rng));
        assert_eq!(Age::YoungAdult(28), Race::gen_age(&mut rng));
        assert_eq!(Age::Adolescent(17), Race::gen_age(&mut rng));
        assert_eq!(Age::Child(7), Race::gen_age(&mut rng));
        assert_eq!(Age::Geriatric(76), Race::gen_age(&mut rng));
    }

    #[test]
    fn gen_name_test() {
        let mut rng = StepRng::new(0, 0xDECAFBAD);
        let age = Age::Adult(0);
        let m = Gender::Masculine;

        assert_eq!("Potato Johnson", Race::gen_name(&mut rng, &age, &m));
    }

    #[test]
    fn gen_size_test() {
        let mut rng = StepRng::new(0, 0xDECAFBAD);
        let age = Age::Adult(0);
        let t = Gender::Trans;

        assert_eq!(
            Size::Medium {
                height: 72,
                weight: 180
            },
            Race::gen_size(&mut rng, &age, &t),
        );
    }
}
