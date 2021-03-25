use rand::distributions::WeightedIndex;
use rand::prelude::*;

use super::{Age, Gender, Generate, Rng};

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
                "Abebe",
                "Sibonakaliso",
                "Nakato",
                "Efemena",
                "Adisa",
                "Temitope"
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
