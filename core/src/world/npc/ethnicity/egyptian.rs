use rand::distributions::WeightedIndex;
use rand::prelude::*;

use super::{Age, Gender, Generate};

pub struct Ethnicity;

impl Ethnicity {
    #[rustfmt::skip]
    const FEMININE_NAMES: &'static [&'static str] = &[
        "A\'at", "Ahset", "Amunet", "Aneksi", "Atet", "Baketamon", "Betrest", "Bunefer", "Dedyet",
        "Hatshepsut", "Hentie", "Herit", "Hetepheres", "Intakaes", "Ipwet", "Itet", "Joba",
        "Kasmut", "Kemanub", "Khemut", "Kiya", "Maia", "Menhet", "Merit", "Meritamen", "Merneith",
        "Merseger", "Muyet", "Nebet", "Nebetah", "Nedjemmut", "Nefertiti", "Neferu", "Neithotep",
        "Nit", "Nofret", "Nubemiunu", "Peseshet", "Pypuy", "Qalhata", "Rai", "Redji", "Sadeh",
        "Sadek", "Sitamun", "Sitre", "Takhat", "Tarset", "Taweret", "Werenro",
    ];

    #[rustfmt::skip]
    const MASCULINE_NAMES: &'static [&'static str] = &[
        "Ahmose", "Akhom", "Amasis", "Amenemhet", "Anen", "Banefre", "Bek", "Djedefre", "Djoser",
        "Hekaib", "Henenu", "Horemheb", "Horwedja", "Huya", "Ibebi", "Idu", "Imhotep", "Ineni",
        "Ipuki", "Irsu", "Kagemni", "Kawab", "Kenamon", "Kewap", "Khaemwaset", "Khafra",
        "Khusebek", "Masaharta", "Meketre", "Menkhaf", "Merenre", "Metjen", "Nebamun", "Nebetka",
        "Nehi", "Nekure", "Nessumontu", "Pakhom", "Pawah", "Pawero", "Ramose", "Rudjek", "Sabaf",
        "Sebek-khu", "Sebni", "Senusret", "Shabaka", "Somintu", "Thaneni", "Thethi",
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
                "Ahmose",
                "Sebek-khu",
                "Nedjemmut",
                "Kasmut",
                "Anen",
                "Redji"
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
        npc.ethnicity.replace(Ethnicity::Egyptian);
        regenerate(rng, &mut npc);
        format!("{}", npc.name)
    }
}
