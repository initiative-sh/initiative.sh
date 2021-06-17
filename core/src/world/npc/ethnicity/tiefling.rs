use rand::distributions::WeightedIndex;
use rand::prelude::*;

use super::{Age, Gender, Generate};

pub struct Ethnicity;

impl Ethnicity {
    #[rustfmt::skip]
    const FEMININE_NAMES: &'static [&'static str] = &[
        "Akta", "Anakis", "Armara", "Astaro", "Aym", "Azza", "Beleth", "Bryseis", "Bune",
        "Criella", "Damaia", "Decarabia", "Ea", "Gadreel", "Gomory", "Hecat", "Ishte", "Jezebeth",
        "Kali", "Kallista", "Kasdeya", "Lerissa", "Lilith", "Makaria", "Manea", "Markosian",
        "Mastema", "Naamah", "Nemeia", "Nija", "Orianna", "Osah", "Phelaia", "Prosperine", "Purah",
        "Pyra", "Rieta", "Ronobe", "Ronwe", "Seddit", "Seere", "Sekhmet", "Semyaza", "Shava",
        "Shax", "Sorath", "Uzza", "Vapula", "Vepar", "Verin",
    ];

    #[rustfmt::skip]
    const MASCULINE_NAMES: &'static [&'static str] = &[
        "Abad", "Ahrim", "Akmen", "Amnon", "Andram", "Astar", "Balam", "Barakas", "Bathin",
        "Cairn", "Chem", "Cimer", "Cressel", "Damakos", "Ekemon", "Euron", "Fenriz", "Forcas",
        "Habor", "Iados", "Kairon", "Leucis", "Mamnen", "Mantus", "Marbas", "Melech", "Merihim",
        "Modean", "Mordai", "Mormo", "Morthos", "Nicor", "Nirgel", "Oriax", "Paymon", "Pelaios",
        "Purson", "Qemuel", "Raam", "Rimmon", "Sammal", "Skamos", "Tethren", "Thamuz", "Therai",
        "Valafar", "Vassago", "Xappan", "Zepar", "Zephan",
    ];

    #[rustfmt::skip]
    const VIRTUE_NAMES: &'static [&'static str] = &[
        "Ambition", "Art", "Carrion", "Chant", "Creed", "Death", "Debauchery", "Despair", "Doom",
        "Doubt", "Dread", "Ecstasy", "Ennui", "Entropy", "Excellence", "Fear", "Glory", "Gluttony",
        "Grief", "Hate", "Hope", "Horror", "Ideal", "Ignominy", "Laughter", "Love", "Lust",
        "Mayhem", "Mockery", "Murder", "Muse", "Music", "Mystery", "Nowhere", "Open", "Pain",
        "Passion", "Poetry", "Quest", "Random", "Reverence", "Revulsion", "Sorrow", "Temerity",
        "Torment", "Tragedy", "Vice", "Virtue", "Weary", "Wit",
    ];
}

impl Generate for Ethnicity {
    fn gen_name(rng: &mut impl Rng, age: &Age, gender: &Gender) -> String {
        let use_virtue_name = match (rng.gen_range(1..=10), age, gender) {
            (n, _, Gender::Trans) | (n, _, Gender::Neuter) if n < 9 => true,
            (_, Age::Infant(_), _) => false,
            (n, Age::Adolescent(_), _) | (n, Age::YoungAdult(_), _) if n < 5 => true,
            (n, _, _) if n < 2 => true,
            _ => false,
        };

        match (use_virtue_name, age, gender) {
            (true, _, _) => {
                Self::VIRTUE_NAMES[rng.gen_range(0..Self::VIRTUE_NAMES.len())].to_string()
            }
            (_, _, Gender::Masculine) => {
                Self::MASCULINE_NAMES[rng.gen_range(0..Self::MASCULINE_NAMES.len())].to_string()
            }
            (_, _, Gender::Feminine) => {
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
        let adult = Age::Adult(0);
        let m = Gender::Masculine;
        let f = Gender::Feminine;
        let t = Gender::Trans;

        assert_eq!(
            [
                "Thamuz", "Orianna", "Creed", "Sekhmet", "Mordai", "Euron", "Random", "Gadreel",
                "Ambition", "Laughter"
            ],
            [
                gen_name(&mut rng, &Age::Infant(0), &m),
                gen_name(&mut rng, &Age::Child(0), &f),
                gen_name(&mut rng, &Age::Adolescent(0), &m),
                gen_name(&mut rng, &Age::YoungAdult(0), &f),
                gen_name(&mut rng, &adult, &m),
                gen_name(&mut rng, &adult, &m),
                gen_name(&mut rng, &adult, &f),
                gen_name(&mut rng, &adult, &f),
                gen_name(&mut rng, &adult, &t),
                gen_name(&mut rng, &adult, &t)
            ]
        );
    }

    fn gen_name(rng: &mut impl Rng, age: &Age, gender: &Gender) -> String {
        let mut npc = Npc::default();
        npc.gender.replace(*gender);
        npc.age.replace(*age);
        npc.ethnicity.replace(Ethnicity::Tiefling);
        regenerate(rng, &mut npc);
        npc.name.value.unwrap()
    }
}
