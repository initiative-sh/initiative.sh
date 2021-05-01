use rand::distributions::WeightedIndex;
use rand::prelude::*;

use super::{Age, Gender, Generate, Rng};

pub struct Ethnicity;

impl Ethnicity {
    #[rustfmt::skip]
    const FEMININE_NAMES: &'static [&'static str] = &[
        "Anbera", "Artin", "Audhild", "Balifra", "Barbena", "Bardryn", "Bolhild", "Dagnal",
        "Dariff", "Delre", "Diesa", "Eldeth", "Eridred", "Falkrunn", "Fallthra", "Finellen",
        "Gillydd", "Gunnloda", "Gurdis", "Helgret", "Helja", "Hlin", "Ilde", "Jarana", "Kathra",
        "Kilia", "Kristryd", "Liftrasa", "Marastyr", "Mardred", "Morana", "Nalaed", "Nora",
        "Nurkara", "Oriff", "Ovina", "Riswynn", "Sannl", "Therlin", "Thodris", "Torbera",
        "Tordrid", "Torgga", "Urshar", "Valida", "Vistra", "Vonana", "Werydd", "Whurdred",
        "Yurgunn",
    ];

    #[rustfmt::skip]
    const MASCULINE_NAMES: &'static [&'static str] = &[
        "Adrik", "Alberich", "Baern", "Barendd", "Beloril", "Brottor", "Dain", "Dalgal", "Darrak",
        "Delg", "Duergath", "Dworic", "Eberk", "Einkil", "Elaim", "Erias", "Fallond", "Fargrim",
        "Gardain", "Gilthur", "Gimgen", "Gimurt", "Harbek", "Kildrak", "Kilvar", "Morgran",
        "Morkral", "Nalral", "Nordak", "Nuraval", "Oloric", "Olunt", "Orsik", "Oskar", "Rangrim",
        "Reirak", "Rurik", "Taklinn", "Thoradin", "Thorin", "Thradal", "Tordek", "Traubon",
        "Travok", "Ulfgar", "Uraim", "Veit", "Vonbin", "Vondal", "Whurbin",
    ];

    #[rustfmt::skip]
    const CLAN_NAMES: &'static [&'static str] = &[
        "Aranore", "Balderk", "Battlehammer", "Bigtoe", "Bloodkith", "Bofdann", "Brawnanvil",
        "Brazzik", "Broodfist", "Burrowfound", "Caebrek", "Daerdahk", "Dankil", "Daraln",
        "Deepdelver", "Durthane", "Eversharp", "Fallack", "Fireforge", "Foamtankard", "Frostbeard",
        "Glanhig", "Goblinbane", "Goldfinder", "Gorunn", "Graybeard", "Hammerstone", "Helcral",
        "Holderhek", "Ironfist", "Loderr", "Lutgehr", "Morigak", "Orcfoe", "Rakankrak", "Ruby-Eye",
        "Rumnaheim", "Silveraxe", "Silverstone", "Steelfist", "Stoutale", "Strakeln",
        "Strongheart", "Thrahak", "Torevir", "Torunn", "Trollbleeder", "Trueanvil", "Trueblood",
        "Ungart",
    ];
}

impl Generate for Ethnicity {
    fn gen_name(rng: &mut impl Rng, age: &Age, gender: &Gender) -> String {
        let mut name = match gender {
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
                    return Self::gen_name(rng, age, &Gender::Masculine);
                } else {
                    return Self::gen_name(rng, age, &Gender::Feminine);
                }
            }
        };
        name.push(' ');
        name.push_str(Self::CLAN_NAMES[rng.gen_range(0..Self::CLAN_NAMES.len())]);
        name
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
                "Adrik Thrahak",
                "Oloric Fallack",
                "Barbena Strakeln",
                "Marastyr Durthane",
                "Baern Steelfist",
                "Kristryd Daraln"
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
        npc.ethnicity.replace(Ethnicity::Dwarvish);
        regenerate(rng, &mut npc);
        npc.name.value.unwrap()
    }
}
