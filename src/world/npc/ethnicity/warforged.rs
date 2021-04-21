use super::{Age, Gender, Generate, Rng};

pub struct Ethnicity;

impl Ethnicity {
    const NAMES: &'static [&'static str] = &[
        "Anchor", "Banner", "Bastion", "Blade", "Blue", "Bow", "Cart", "Church", "Crunch",
        "Crystal", "Dagger", "Dent", "Five", "Glaive", "Hammer", "Iron", "Lucky", "Mace", "Oak",
        "Onyx", "Pants", "Pierce", "Red", "Rod", "Rusty", "Scout", "Seven", "Shield", "Slash",
        "Smith", "Spike", "Temple", "Vault", "Wall",
    ];
}

impl Generate for Ethnicity {
    fn gen_name(rng: &mut impl Rng, _age: &Age, _gender: &Gender) -> String {
        Self::NAMES[rng.gen_range(0..Self::NAMES.len())].to_string()
    }
}

#[cfg(test)]
mod test_generate_for_ethnicity {
    use super::*;
    use rand::rngs::mock::StepRng;

    #[test]
    fn gen_name_test() {
        let mut rng = StepRng::new(0, 0xDEADBEEF_DECAFBAD);
        let age = Age::Adult(0);
        let m = Gender::Masculine;

        assert_eq!("Anchor", Ethnicity::gen_name(&mut rng, &age, &m));
        assert_eq!("Scout", Ethnicity::gen_name(&mut rng, &age, &m));
        assert_eq!("Lucky", Ethnicity::gen_name(&mut rng, &age, &m));
        assert_eq!("Church", Ethnicity::gen_name(&mut rng, &age, &m));
        assert_eq!("Blade", Ethnicity::gen_name(&mut rng, &age, &m));
    }
}
