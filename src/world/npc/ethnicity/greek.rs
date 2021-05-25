use rand::distributions::WeightedIndex;
use rand::prelude::*;

use super::{Age, Gender, Generate, Rng};

pub struct Ethnicity;

impl Ethnicity {
    #[rustfmt::skip]
    const FEMININE_NAMES: &'static [&'static str] = &[
        "Acantha", "Aella", "Alektos", "Alkippe", "Andromeda", "Antigone", "Ariadne", "Astraea",
        "Chloros", "Chryseos", "Daphne", "Despoina", "Dione", "Eileithyia", "Elektra", "Euadne",
        "Eudora", "Eunomia", "Hekabe", "Helene", "Hermoione", "Hippolyte", "Ianthe", "Iokaste",
        "Iole", "Iphigenia", "Ismene", "Kalliope", "Kallisto", "Kalypso", "Karme", "Kassandra",
        "Kassiopeia", "Kirke", "Kleio", "Klotho", "Klytie", "Kynthia", "Leto", "Megaera",
        "Melaina", "Melpomene", "Nausikaa", "Nemesis", "Niobe", "Ourania", "Phaenna", "Polymnia",
        "Semele", "Theia",
    ];

    #[rustfmt::skip]
    const MASCULINE_NAMES: &'static [&'static str] = &[
        "Adonis", "Adrastos", "Aeson", "Aias", "Aineias", "Aiolos", "Alekto", "Name", "Alkeides",
        "Argos", "Brontes", "Damazo", "Dardanos", "Deimos", "Diomedes", "Endymion", "Epimetheus",
        "Erebos", "Euandros", "Ganymedes", "Glaukos", "Hektor", "Heros", "Hippolytos", "Iacchus",
        "Iason", "Kadmos", "Kastor", "Kephalos", "Kepheus", "Koios", "Kreios", "Laios", "Leandros",
        "Linos", "Lykos", "Melanthios", "Menelaus", "Mentor", "Neoptolemus", "Okeanos", "Orestes",
        "Pallas", "Patroklos", "Philandros", "Phoibos", "Phrixus", "Priamos", "Pyrrhos", "Xanthos",
        "Zephyros",
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
                "Adonis",
                "Philandros",
                "Karme",
                "Eunomia",
                "Aineias",
                "Melpomene"
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
        npc.ethnicity.replace(Ethnicity::Greek);
        regenerate(rng, &mut npc);
        npc.name.value.unwrap()
    }
}
