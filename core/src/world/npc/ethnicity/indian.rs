use rand::distributions::WeightedIndex;
use rand::prelude::*;

use super::{Age, Gender, Generate, Rng};

pub struct Ethnicity;

impl Ethnicity {
    #[rustfmt::skip]
    const FEMININE_NAMES: &'static [&'static str] = &[
        "Abha", "Aishwarya", "Amala", "Ananda", "Ankita", "Archana", "Avani", "Chandana",
        "Chandrakanta", "Chetan", "Darshana", "Devi", "Dipti", "Esha", "Gauro", "Gita", "Indira",
        "Indu", "Jaya", "Kala", "Kalpana", "Kamala", "Kanta", "Kashi", "Kishori", "Lalita", "Lina",
        "Madhur", "Manju", "Meera", "Mohana", "Mukta", "Nisha", "Nitya", "Padma", "Pratima",
        "Priya", "Rani", "Sarala", "Shakti", "Shanta", "Shobha", "Sima", "Sonal", "Sumana",
        "Sunita", "Tara", "Valli", "Vijaya", "Vimala",
    ];

    #[rustfmt::skip]
    const MASCULINE_NAMES: &'static [&'static str] = &[
        "Abhay", "Ahsan", "Ajay", "Ajit", "Akhil", "Amar", "Amit", "Ananta", "Aseem", "Ashok",
        "Bahadur", "Basu", "Chand", "Chandra", "Damodar", "Darhsan", "Devdan", "Dinesh", "Dipak",
        "Gopal", "Govind", "Harendra", "Harsha", "Ila", "Isha", "Johar", "Kalyan", "Kiran",
        "Kumar", "Lakshmana", "Mahavir", "Narayan", "Naveen", "Nirav", "Prabhakar", "Prasanna",
        "Raghu", "Rajanikant", "Rakesh", "Ranjeet", "Rishi", "Sanjay", "Sekar", "Shandar",
        "Sumantra", "Vijay", "Vikram", "Vimal", "Vishal", "Yash",
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
            ["Abhay", "Shandar", "Mohana", "Indu", "Akhil", "Shobha"],
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
        npc.ethnicity.replace(Ethnicity::Indian);
        regenerate(rng, &mut npc);
        npc.name.value.unwrap()
    }
}
