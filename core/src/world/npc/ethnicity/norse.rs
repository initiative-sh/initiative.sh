use rand::distributions::WeightedIndex;
use rand::prelude::*;

use super::{Age, Gender, Generate, Rng};

pub struct Ethnicity;

impl Ethnicity {
    #[rustfmt::skip]
    const FEMININE_NAMES: &'static [&'static str] = &[
        "Alfhild", "Arnbjorg", "Ase", "Aslog", "Astrid", "Auda", "Audhid", "Bergljot", "Birghild",
        "Bodil", "Brenna", "Brynhild", "Dagmar", "Eerika", "Eira", "Gudrun", "Gunborg", "Gunhild",
        "Gunvor", "Helga", "Hertha", "Hilde", "Hillevi", "Ingrid", "Iona", "Jorunn", "Kari",
        "Kenna", "Magnhild", "Nanna", "Olga", "Ragna", "Ragnhild", "Ranveig", "Runa", "Saga",
        "Sigfrid", "Signe", "Sigrid", "Sigr.unn", "Solveg", "Svanhild", "Thora", "Torborg",
        "Torunn", "Tove", "Unn", "Vigdis", "Ylva", "Yngvild",
    ];

    #[rustfmt::skip]
    const MASCULINE_NAMES: &'static [&'static str] = &[
        "Agni", "Alaric", "Anvindr", "Arvid", "Asger", "Asmund", "Bjarte", "Bjorg", "Bjorn",
        "Brandr", "Brandt", "Brynjar", "Calder", "Colborn", "Cuyler", "Egil", "Einar", "Eric",
        "Erland", "Fiske", "Folkvar", "Fritjof", "Frede", "Geir", "Halvar", "Hemming", "Hjalmar",
        "Hjortr", "Ingimarr", "Ivar", "Knud", "Leif", "Liufr", "Manning", "Oddr", "Olin", "Ormr",
        "Ove", "Rannulfr", "Sigurd", "Skari", "Snorri", "Sten", "Stigandr", "Stigr", "Sven",
        "Trygve", "Ulf", "Vali", "Vidar",
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
            ["Agni", "Stigandr", "Olga", "Gunhild", "Asger", "Svanhild"],
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
        npc.ethnicity.replace(Ethnicity::Norse);
        regenerate(rng, &mut npc);
        npc.name.value.unwrap()
    }
}