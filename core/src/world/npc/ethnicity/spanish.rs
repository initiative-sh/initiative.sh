use rand::distributions::WeightedIndex;
use rand::prelude::*;

use super::{Age, Gender, Generate, Rng};

pub struct Ethnicity;

impl Ethnicity {
    #[rustfmt::skip]
    const FEMININE_NAMES: &'static [&'static str] = &[
        "Abella", "Adalina", "Adora", "Adriana", "Ana", "Antonia", "Basilia", "Beatriz", "Bonita",
        "Camila", "Cande", "Carmen", "Catlina", "Dolores", "Dominga", "Dorotea", "Elena", "Elicia",
        "Esmerelda", "Felipina", "Francisca", "Gabriela", "Imelda", "Ines", "Isabel", "Juana",
        "Leocadia", "Leonor", "Leta", "Lucinda", "Maresol", "Maria", "Maricela", "Matilde",
        "Melania", "Monica", "Neva", "Nilda", "Petrona", "Rafaela", "Ramira", "Rosario", "Sofia",
        "Suelo", "Teresa", "Tomasa", "Valentia", "Veronica", "Ynes", "Ysabel",
    ];

    #[rustfmt::skip]
    const MASCULINE_NAMES: &'static [&'static str] = &[
        "Alexandre", "Alfonso", "Alonso", "Anthon", "Arcos", "Arnaut", "Arturo", "Bartoleme",
        "Benito", "Bernat", "Blasco", "Carlos", "Damian", "Diego", "Domingo", "Enrique", "Escobar",
        "Etter", "Fernando", "Franciso", "Gabriel", "Garcia", "Gaspar", "Gil", "Gomes", "Goncalo",
        "Gostantin", "Jayme", "Joan", "Jorge", "Jose", "Juan", "Machin", "Martin", "Mateu",
        "Miguel", "Nicolas", "Pascual", "Pedro", "Perico", "Ramiro", "Ramon", "Rodrigo",
        "Sabastian", "Salvador", "Simon", "Tomas", "Tristan", "Valeriano", "Ynigo",
    ];
}

impl Generate for Ethnicity {
    fn gen_name(rng: &mut impl Rng, age: &Age, gender: &Gender) -> String {
        match gender {
            Gender::Masculine => {
                Self::MASCULINE_NAMES[rng.gen_range(0..Self::MASCULINE_NAMES.len())].to_string()
            }
            Gender::Feminine => {
                Self::FEMININE_NAMES[rng.gen_range(0..Self::FEMININE_NAMES.len())].to_string()
            }
            _ => {
                let dist =
                    WeightedIndex::new(&[Self::MASCULINE_NAMES.len(), Self::FEMININE_NAMES.len()])
                        .unwrap();
                if dist.sample(rng) == 0 {
                    Self::gen_name(rng, age, &Gender::Masculine)
                } else {
                    Self::gen_name(rng, age, &Gender::Feminine)
                }
            }
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
                "Alexandre",
                "Sabastian",
                "Maresol",
                "Elicia",
                "Arcos",
                "Rosario"
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
        npc.ethnicity.replace(Ethnicity::Spanish);
        regenerate(rng, &mut npc);
        npc.name.value.unwrap()
    }
}
