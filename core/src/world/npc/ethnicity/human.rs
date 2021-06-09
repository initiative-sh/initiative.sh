use super::{Age, Gender, Generate, Rng};

pub struct Ethnicity;

impl Generate for Ethnicity {
    fn gen_name(rng: &mut impl Rng, age: &Age, gender: &Gender) -> String {
        match rng.gen_range(0..=16) {
            0 => super::arabic::Ethnicity::gen_name(rng, age, gender),
            1 => super::celtic::Ethnicity::gen_name(rng, age, gender),
            2 => super::chinese::Ethnicity::gen_name(rng, age, gender),
            3 => super::egyptian::Ethnicity::gen_name(rng, age, gender),
            4 => super::english::Ethnicity::gen_name(rng, age, gender),
            5 => super::french::Ethnicity::gen_name(rng, age, gender),
            6 => super::german::Ethnicity::gen_name(rng, age, gender),
            7 => super::greek::Ethnicity::gen_name(rng, age, gender),
            8 => super::indian::Ethnicity::gen_name(rng, age, gender),
            9 => super::japanese::Ethnicity::gen_name(rng, age, gender),
            10 => super::mesoamerican::Ethnicity::gen_name(rng, age, gender),
            11 => super::niger_congo::Ethnicity::gen_name(rng, age, gender),
            12 => super::norse::Ethnicity::gen_name(rng, age, gender),
            13 => super::polynesian::Ethnicity::gen_name(rng, age, gender),
            14 => super::roman::Ethnicity::gen_name(rng, age, gender),
            15 => super::slavic::Ethnicity::gen_name(rng, age, gender),
            16 => super::spanish::Ethnicity::gen_name(rng, age, gender),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test_generate_for_ethnicity {
    use super::*;
    use crate::world::npc::ethnicity::{regenerate, Ethnicity};
    use crate::world::Npc;
    use rand::rngs::mock::StepRng;

    #[test]
    fn gen_name_test() {
        let mut rng = StepRng::new(0, 0xDEADBEEF_DECAFBAD);
        let age = Age::Adult(0);
        let m = Gender::Masculine;
        let f = Gender::Feminine;
        let t = Gender::Trans;

        assert_eq!(
            [
                "Saladin",
                "Ilhicamina",
                "Rosario",
                "Agnez",
                "Kamakani",
                "Wei"
            ],
            [
                gen_name(&mut rng, &age, &m),
                gen_name(&mut rng, &age, &m),
                gen_name(&mut rng, &age, &f),
                gen_name(&mut rng, &age, &f),
                gen_name(&mut rng, &age, &t),
                gen_name(&mut rng, &age, &t),
            ]
        );
    }

    fn gen_name(rng: &mut impl Rng, age: &Age, gender: &Gender) -> String {
        let mut npc = Npc::default();
        npc.gender.replace(*gender);
        npc.age.replace(*age);
        npc.ethnicity.replace(Ethnicity::Human);
        regenerate(rng, &mut npc);
        npc.name.value.unwrap()
    }
}
