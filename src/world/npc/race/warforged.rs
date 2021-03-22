use super::{Age, Gender, RaceGenerate, Rng, Size};

pub struct Race;

impl Race {
    const NAMES: &'static [&'static str] = &[
        "Anchor", "Banner", "Bastion", "Blade", "Blue", "Bow", "Cart", "Church", "Grnch",
        "Crystal", "Dagger", "Dent", "Five", "Glaive", "Hammer", "Iron", "Lucky", "Mace", "Oak",
        "Onyx", "Pants", "Pierce", "Red", "Rod", "Rusty", "Scout", "Seven", "Shield", "Slash",
        "Smith", "Spike", "Temple", "Vault", "Wall",
    ];
}

impl RaceGenerate for Race {
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

    fn gen_name(rng: &mut impl Rng, _age: &Age, _gender: &Gender) -> String {
        Self::NAMES[rng.gen_range(0..Self::NAMES.len())].to_string()
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
mod test_race_generate_for_race {
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

        assert_eq!(4, genders.len());
        assert_eq!(Some(&59), genders.get("neuter (it)"));
        assert_eq!(Some(&15), genders.get("feminine (she/her)"));
        assert_eq!(Some(&16), genders.get("masculine (he/him)"));
        assert_eq!(Some(&10), genders.get("trans (they/them)"));
    }

    #[test]
    fn gen_age_test() {
        let mut rng = StepRng::new(0, 0xDECAFBAD);

        assert_eq!(Age::Adult(2), Race::gen_age(&mut rng));
        assert_eq!(Age::Adult(27), Race::gen_age(&mut rng));
        assert_eq!(Age::Adult(23), Race::gen_age(&mut rng));
        assert_eq!(Age::Adult(19), Race::gen_age(&mut rng));
        assert_eq!(Age::Adult(15), Race::gen_age(&mut rng));
    }

    #[test]
    fn gen_name_test() {
        let mut rng = StepRng::new(0, 0xDEADBEEF_DECAFBAD);
        let age = Age::Adult(0);
        let m = Gender::Masculine;

        assert_eq!("Anchor", Race::gen_name(&mut rng, &age, &m));
        assert_eq!("Scout", Race::gen_name(&mut rng, &age, &m));
        assert_eq!("Lucky", Race::gen_name(&mut rng, &age, &m));
        assert_eq!("Church", Race::gen_name(&mut rng, &age, &m));
        assert_eq!("Blade", Race::gen_name(&mut rng, &age, &m));
    }

    #[test]
    fn gen_size_test() {
        let mut rng = StepRng::new(0, 0xDECAFBAD);
        let age = Age::Adult(0);
        let t = Gender::Trans;

        let size = |height, weight| Size::Medium { height, weight };

        assert_eq!(size(77, 298), Race::gen_size(&mut rng, &age, &t));
        assert_eq!(size(79, 306), Race::gen_size(&mut rng, &age, &t));
        assert_eq!(size(76, 294), Race::gen_size(&mut rng, &age, &t));
        assert_eq!(size(73, 282), Race::gen_size(&mut rng, &age, &t));
        assert_eq!(size(81, 314), Race::gen_size(&mut rng, &age, &t));
    }
}
