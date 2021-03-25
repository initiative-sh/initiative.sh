use rand::distributions::WeightedIndex;
use rand::prelude::*;

use super::{Age, Gender, Generate, Rng};

pub struct Ethnicity;

impl Ethnicity {
    #[rustfmt::skip]
    const FEMININE_NAMES: &'static [&'static str] = &[
        "Aalis", "Agatha", "Agnez", "Alberea", "Alips", "Amee", "Amelot", "Anne", "Avelina",
        "Blancha", "Cateline", "Cecilia", "Claricia", "Collette", "Denisete", "Dorian", "Edelina",
        "Emelina", "Emmelot", "Ermentrudis", "Gibelina", "Gila", "Gillette", "Guiburgis",
        "Guillemette", "Guoite", "Hecelina", "Heloysis", "Helyoudis", "Hodeardis", "Isabellis",
        "Jaquette", "Jehan", "Johanna", "Juliote", "Katerine", "Luciana", "Margot", "Marguerite",
        "Maria", "Marie", "Melisende", "Odelina", "Perrette", "Petronilla", "Sedilia", "Stephana",
        "Sybilla", "Ysabeau", "Ysabel",
    ];

    #[rustfmt::skip]
    const MASCULINE_NAMES: &'static [&'static str] = &[
        "Ambroys", "Ame", "Andri", "Andriet", "Anthoine", "Bernard", "Charles", "Charlot", "Colin",
        "Denis", "Durant", "Edouart", "Eremon", "Ernault", "Ethor", "Felix", "Floquart",
        "Galleren", "Gaultier", "Gilles", "Guy", "Henry", "Hugo", "Imbert", "Jacques", "Jacquot",
        "Jean", "Jehannin", "Louis", "Louys", "Loys", "Martin", "Michel", "Mille", "Morelet",
        "Nicolas", "Nicolle", "Oudart", "Perrin", "Phillippe", "Pierre", "Regnault", "Richart",
        "Robert", "Robinet", "Sauvage", "Simon", "Talbot", "Tanguy", "Vincent",
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
                "Ambroys",
                "Robert",
                "Isabellis",
                "Emelina",
                "Anthoine",
                "Melisende"
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
