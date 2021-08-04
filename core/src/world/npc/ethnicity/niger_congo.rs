use super::{Age, Gender, Generate};
use rand::distributions::WeightedIndex;
use rand::prelude::*;

pub struct Ethnicity;

impl Ethnicity {
    #[rustfmt::skip]
    const FEMININE_NAMES: &'static [&'static str] = &[
        "Abebi", "Abena", "Abimbola", "Akoko", "Akachi", "Alaba", "Anuli", "Ayo", "Bolanle",
        "Bosede", "Chiamaka", "Chidi", "Chidimma", "Chinyere", "Chioma", "Dada", "Ebele",
        "Efemena", "Ejiro", "Ekundayo", "Enitan", "Funanya", "Ifunanya", "Ige", "Ime", "Kunto",
        "Lesedi", "Lumusi", "Mojisola", "Monifa", "Nakato", "Ndidi", "Ngozi", "Nkiruka", "Nneka",
        "Ogechi", "Olamide", "Oluchi", "Omolara", "Onyeka", "Simisola", "Temitope", "Thema",
        "Titlayo", "Udo", "Uduak", "Ufuoma", "Yaa", "Yejide", "Yewande",
    ];

    #[rustfmt::skip]
    const MASCULINE_NAMES: &'static [&'static str] = &[
        "Abebe", "Abel", "Abidemi", "Abrafo", "Adisa", "Amadi", "Amara", "Anyim", "Azubuike",
        "Bapoto", "Baraka", "Bohlale", "Bongani", "Bujune", "Buziba", "Chakide", "Chibuzo",
        "Chika", "Chimola", "Chiratidzo", "Dabulamanzi", "Dumisa", "Dwanh", "Emeka", "Folami",
        "Gatura", "Gebhuza", "Gero", "Isoba", "Kagiso", "Kamau", "Katlego", "Masego", "Matata",
        "Nthanda", "Ogechi", "Olwenyo", "Osumare", "Paki", "Qinisela", "Quanda", "Samanya",
        "Shanika", "Sibonakaliso", "Tapiwa", "Thabo", "Themba", "Uzoma", "Zuberi", "Zuri", 
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
            ["Dwanh", "Emeka", "Yaa", "Monifa", "Chiratidzo", "Chidimma"],
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
        npc.ethnicity.replace(Ethnicity::NigerCongo);
        regenerate(rng, &mut npc);
        format!("{}", npc.name)
    }
}
