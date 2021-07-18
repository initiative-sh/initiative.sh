use super::{Age, Gender, Generate};
use rand::distributions::WeightedIndex;
use rand::prelude::*;

pub struct Ethnicity;

impl Ethnicity {
    const FEMININE_NAMES: &'static [&'static str] = &[
        "Aaliyah", "Aida", "Akilah", "Alia", "Amina", "Atefeh", "Chaima", "Dalia", "Ehsan",
        "Elham", "Farah", "Fatemah", "Gamila", "Iesha", "Inbar", "Kamaria", "Khadija", "Layla",
        "Lupe", "Nabila", "Nadine", "Naima", "Najila", "Najwa", "Kania", "Nashwa", "Nawra", "Nuha",
        "Nura", "Oma", "Qadira", "Qamar", "Qistina", "Rahima", "Rihanna", "Saadia", "Sabah",
        "Sada", "Saffron", "Sahar", "Salma", "Shatha", "Tahira", "Takisha", "Thana", "Yadira",
        "Zahra", "Zaida", "Zaina", "Zeinab",
    ];

    #[rustfmt::skip]
    const MASCULINE_NAMES: &'static [&'static str] = &[
        "Abbad", "Abdul", "Achmed", "Akeem", "Alif", "Amir", "Asim", "Bashir", "Bassam", "Fahim",
        "Farid", "Farouk", "Fayez", "Fayyaad", "Fazil", "Hakim", "Halil", "Hamid", "Hazim",
        "Heydar", "Hussein", "Jabari", "Jafar", "Jahid", "Jamal", "Kalim", "Karim", "Kazim",
        "Khadim", "Khalid", "Mahmud", "Mansour", "Musharraf", "Mustafa", "Nadir", "Nazim", "Omar",
        "Qadir", "Qusay", "Rafiq", "Rakim", "Rashad", "Rauf", "Saladin", "Sami", "Samir", "Talib",
        "Tamir", "Tariq", "Yazid",
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
            ["Abbad", "Saladin", "Qadira", "Layla", "Alif", "Shatha"],
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
        npc.ethnicity.replace(Ethnicity::Arabic);
        regenerate(rng, &mut npc);
        format!("{}", npc.name)
    }
}
