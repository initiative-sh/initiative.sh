use rand::distributions::WeightedIndex;
use rand::prelude::*;

use super::{Age, Gender, Generate};

pub struct Ethnicity;

impl Ethnicity {
    #[rustfmt::skip]
    const FEMININE_NAMES: &'static [&'static str] = &[
        "Adelaide", "Agatha", "Agnes", "Alice", "Aline", "Anne", "Avelina", "Avice", "Beatrice",
        "Cecily", "Egelina", "Eleanor", "Elizabeth", "Ella", "Eloise", "Elysande", "Emeny", "Emma",
        "Emmeline", "Ermina", "Eva", "Galiena", "Geva", "Giselle", "Griselda", "Hadwisa", "Helen",
        "Herleva", "Hugolina", "Ida", "Isabella", "jacoba", "Jane", "Joan", "Juliana", "Katherine",
        "Margery", "Mary", "Matilda", "Maynild", "Millicent", "Oriel", "Rohesia", "Rosalind",
        "Rosamund", "Sarah", "Susannah", "Sybil", "Williamina", "Yvonne",
    ];

    #[rustfmt::skip]
    const MASCULINE_NAMES: &'static [&'static str] = &[
        "Adam", "Adelard", "Aldous", "Anselm", "Arnold", "Bernard", "Bertram", "Charles",
        "Clerebold", "Conrad", "Diggory", "Drogo", "Everard", "Frederick", "Geoffrey", "Gerald",
        "Gilbert", "Godfrey", "Gunter", "Guy", "Henry", "Heward", "Hubert", "Hugh", "Jocelyn",
        "john", "Lance", "Manfred", "Miles", "Nicholas", "Norman", "Odo", "Percival", "Peter",
        "Ralf", "Randal", "Raymond", "Reynard", "Richard", "Robert", "Roger", "Roland", "Rolf",
        "Simon", "Theobald", "Theodoric", "Thomas", "Timm", "William", "Wymar",
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
            ["Adam", "Simon", "Isabella", "Emma", "Arnold", "Oriel"],
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
        npc.ethnicity.replace(Ethnicity::English);
        regenerate(rng, &mut npc);
        format!("{}", npc.name)
    }
}
