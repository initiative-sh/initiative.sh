use super::{Age, Gender, Generate};
use rand::distributions::WeightedIndex;
use rand::prelude::*;

pub struct Ethnicity;

impl Ethnicity {
    #[rustfmt::skip]
    const FEMININE_NAMES: &'static [&'static str] = &[
        "Ahulani", "Hi'ilei", "Airini", "Alani", "Aluala", "Anahera", "Anuhea", "Aolani", "Elenoa",
        "Emele", "Fetia", "Fiva", "Halona", "Hina", "Hinatea", "Huali", "Inia", "Inina", "Iolani",
        "Isa", "Ka'ana'ana", "Ka'ena", "Kaamia", "Kahula", "Kailani", "Kamaile", "Kamakani",
        "Kamea", "Latai", "Liona", "Lokelani", "Marva", "Mehana", "Millawa", "Moana", "Ngana",
        "Nohea", "Pelika", "Sanoe", "Satina", "Tahia", "Tasi", "Tiaho", "Tihani", "Toroa",
        "Ulanni", "Uluwehi", "Vaina", "Waiola", "Waitara",
    ];

    #[rustfmt::skip]
    const MASCULINE_NAMES: &'static [&'static str] = &[
        "Afa", "Ahohako", "Aisake", "Aleki", "Anewa", "Anitelu", "Aputi", "Ariki", "Butat",
        "Enele", "Fef", "Fuifui", "Ha'aheo", "Hanohano", "Haunui", "Hekili", "Hiapo", "Hikawera",
        "Hanano", "Ho'onani", "Hoku", "Hû'eu", "Ina", "Itu", "Ka'aukai", "Kaelani", "Laki", "Pono",
        "Ka'eo", "Kahale", "Kaiea", "Kaikoa", "Kana'l", "Koamalu", "Ka", "Makai", "Manu", "Manuka",
        "Nui", "Popoki", "Ruru", "Tahu", "Taurau", "Tuala", "Turoa", "Tusitala", "Uaine", "Waata",
        "Waipuna", "Zamar",
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

    #[test]
    fn gen_name_test() {
        let mut rng = SmallRng::seed_from_u64(0);
        let age = Age::Adult(0);
        let m = Gender::Masculine;
        let f = Gender::Feminine;
        let t = Gender::Trans;

        assert_eq!(
            ["Ina", "Itu", "Vaina", "Liona", "Ho'onani", "Halona"],
            [
                gen_name(&mut rng, &age, &m),
                gen_name(&mut rng, &age, &m),
                gen_name(&mut rng, &age, &f),
                gen_name(&mut rng, &age, &f),
                gen_name(&mut rng, &age, &t),
                gen_name(&mut rng, &age, &t),
            ],
        );
    }

    fn gen_name(rng: &mut impl Rng, age: &Age, gender: &Gender) -> String {
        let mut npc = Npc::default();
        npc.gender.replace(*gender);
        npc.age.replace(*age);
        npc.ethnicity.replace(Ethnicity::Polynesian);
        regenerate(rng, &mut npc);
        format!("{}", npc.name)
    }
}
