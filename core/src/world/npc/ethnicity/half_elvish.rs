use super::{Age, Gender, Generate};
use rand::prelude::*;

pub struct Ethnicity;

impl Generate for Ethnicity {
    fn gen_name(rng: &mut impl Rng, age: &Age, gender: &Gender) -> String {
        if rng.gen_bool(0.5) {
            super::elvish::Ethnicity::gen_name(rng, age, gender)
        } else {
            super::human::Ethnicity::gen_name(rng, age, gender)
        }
    }
}

#[cfg(test)]
mod test_generate_for_ethnicity {
    use super::*;
    use crate::world::npc::ethnicity::{regenerate, Ethnicity};
    use crate::world::Npc;

    #[test]
    fn gen_name_test() {
        let mut rng = SmallRng::seed_from_u64(0);
        let adult = Age::Adult(0);
        let m = Gender::Masculine;
        let f = Gender::Feminine;
        let t = Gender::Trans;

        assert_eq!(
            [
                "Lue Xiloscient",
                "Jane",
                "Vall Eathalena",
                "Rolen Qualanthri",
                "Horwedja",
                "Zarya",
                "Theirastra Horineth",
                "Talib",
                "Efferil Ostoroth",
            ],
            [
                gen_name(&mut rng, &Age::Infant(0), &m),
                gen_name(&mut rng, &Age::Child(0), &f),
                gen_name(&mut rng, &Age::Adolescent(0), &t),
                gen_name(&mut rng, &adult, &m),
                gen_name(&mut rng, &adult, &m),
                gen_name(&mut rng, &adult, &f),
                gen_name(&mut rng, &adult, &f),
                gen_name(&mut rng, &adult, &t),
                gen_name(&mut rng, &adult, &t),
            ],
        );
    }

    fn gen_name(rng: &mut impl Rng, age: &Age, gender: &Gender) -> String {
        let mut npc = Npc::default();
        npc.gender.replace(*gender);
        npc.age.replace(*age);
        npc.ethnicity.replace(Ethnicity::HalfElvish);
        regenerate(rng, &mut npc);
        format!("{}", npc.name)
    }
}
