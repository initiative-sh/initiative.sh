use super::{Age, Gender, Generate};
use rand::distributions::WeightedIndex;
use rand::prelude::*;

pub struct Ethnicity;

impl Ethnicity {
    #[rustfmt::skip]
    const FEMININE_NAMES: &'static [&'static str] = &[
        "Aife", "Aina", "Alane", "Ardena", "Arienh", "Beatha", "Birgit", "Briann", "Caomh", "Cara",
        "Cinnia", "Cordelia", "Deheune", "Divone", "Donia", "Doreena", "Elsha", "Enid", "Ethne",
        "Evelina", "Fianna", "Genevieve", "Gilda", "Gitta", "Grania", "Gwyndolin", "Idelisa",
        "Isolde", "Keelin", "Kennocha", "Lavena", "Lesley", "Linnette", "Lyonesse", "Mabina",
        "Marvina", "Mavis", "Mirna", "Morgan", "Muriel", "Nareena", "Oriana", "Regan", "Ronat",
        "Rowena", "Selma", "Ula", "Venetia", "Wynne", "Yseult",
    ];

    #[rustfmt::skip]
    const MASCULINE_NAMES: &'static [&'static str] = &[
        "Airell", "Airic", "Alan", "Anghus", "Aodh", "Bardon", "Bearacb", "Bevyn", "Boden", "Bran",
        "Brasil", "Bredon", "Brian", "Bricriu", "Bryant", "Cadman", "Caradoc", "Cedric", "Conalt",
        "Conchobar", "Condon", "Darcy", "Devin", "Dillion", "Donaghy", "Donall", "Duer", "Eghan",
        "Ewyn", "Ferghus", "Galvyn", "Gildas", "Guy", "Harvey", "Iden", "Irven", "Karney", "Kayne",
        "Kelvyn", "Kunsgnos", "Leigh", "Maccus", "Moryn", "Neale", "Owyn", "Pryderi", "Reaghan",
        "Taliesin", "Tiernay", "Turi",
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
            ["Airell", "Neale", "Lavena", "Enid", "Aodh", "Oriana"],
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
        npc.ethnicity.replace(Ethnicity::Celtic);
        regenerate(rng, &mut npc);
        format!("{}", npc.name)
    }
}
