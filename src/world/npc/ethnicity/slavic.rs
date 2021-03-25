use rand::distributions::WeightedIndex;
use rand::prelude::*;

use super::{Age, Gender, Generate, Rng};

pub struct Ethnicity;

impl Ethnicity {
    #[rustfmt::skip]
    const FEMININE_NAMES: &'static [&'static str] = &[
        "Agripina", "Anastasiya", "Bogdana", "Boleslava", "Bozhena", "Danica", "Darya",
        "Desislava", "Dragoslava", "Dunja", "Efrosinia", "Ekaterina", "Elena", "Faina", "Galina",
        "Irina", "Iskra", "Jasna", "Katarina", "Katya", "Kresimira", "Lyudmila", "Magda", "Mariya",
        "Militsa", "Miloslava", "Mira", "Miroslava", "Mokosh", "Morana", "Natasha", "Nika", "Olga",
        "Rada", "Radoslava", "Raisa", "Slavitsa", "Sofiya", "Stanislava", "Svetlana", "Tatyana",
        "Tomislava", "Veronika", "Vesna", "Vladimira", "Yaroslava", "Yelena", "Zaria", "Zarya",
        "Zoria", 
    ];

    #[rustfmt::skip]
    const MASCULINE_NAMES: &'static [&'static str] = &[
        "Aleksandru", "Berislav", "Blazh", "Bogumir", "Boguslav", "Borislav", "Bozhidar",
        "Bratomil", "Bratoslav", "Bronislav", "Chedomir", "Chestibor", "Chestirad", "Chestislav",
        "Desilav", "Dmitrei", "Dobromil", "Dobroslav", "Dragomir", "Dragutin", "Drazhan",
        "Gostislav", "Kazimir", "Kyrilu", "Lyubomir", "Mechislav", "Milivoj", "Milosh", "Mstislav",
        "Nikola", "Ninoslav", "Premislav", "Radomir", "Radovan", "Ratimir", "Rostislav",
        "Slavomir", "Stanislav", "Svetoslav", "Tomislav", "Vasili", "Velimir", "Vladimir",
        "Vladislav", "Vlastimir", "Volodimeru", "Vratislav", "Yarognev", "Yaromir", "Zbignev", 
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
                "Aleksandru",
                "Vladislav",
                "Natasha",
                "Jasna",
                "Boguslav",
                "Tomislava"
            ],
            [
                Ethnicity::gen_name(&mut rng, &age, &m),
                Ethnicity::gen_name(&mut rng, &age, &m),
                Ethnicity::gen_name(&mut rng, &age, &f),
                Ethnicity::gen_name(&mut rng, &age, &f),
                Ethnicity::gen_name(&mut rng, &age, &t),
                Ethnicity::gen_name(&mut rng, &age, &t),
            ]
        );
    }
}
