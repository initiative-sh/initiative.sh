use rand::distributions::WeightedIndex;
use rand::prelude::*;

use super::{Age, Gender, Generate, Rng};

pub struct Ethnicity;

impl Ethnicity {
    #[rustfmt::skip]
    const FEMININE_NAMES: &'static [&'static str] = &[
        "Ahuiliztli", "Atl", "Centehua", "Chalchiuitl", "Chipahua", "Cihuaton", "Citlali",
        "Citlalmina", "Coszcatl", "Cozamalotl", "Cuicatl", "Eleuia", "Eloxochitl", "Eztli",
        "Ichtaca", "Icnoyotl", "Ihuicatl", "Ilhuitl", "Itotia", "Iuitl", "Ixcatzin", "Izel",
        "Malinalxochitl", "Mecatl", "Meztli", "Miyaoaxochitl", "Mizquixaual", "Moyolehuani",
        "Nahuatl", "Necahual", "Nenetl", "Nochtli", "Noxochicoztli", "Ohtli", "Papan", "Patli",
        "Quetzalxochitl", "Sacnite", "Teicui", "Tepin", "Teuicui", "Teyacapan", "Tlaco",
        "Tlacoehua", "Tlacotl", "Tlalli", "Tlanextli", "Xihuitl", "Xiuhcoatl", "Xiuhtonal",
    ];

    #[rustfmt::skip]
    const MASCULINE_NAMES: &'static [&'static str] = &[
        "Achcauhtli", "Amoxtli", "Chicahua", "Chimalli", "Cipactli", "Coaxoch", "Coyotl", "Cualli",
        "Cuauhtémoc", "Cuetlachtilo", "Cuetzpalli", "Cuixtli", "Ehecatl", "Etalpalli", "Huemac",
        "Huitzilihuitl", "Iccauhtli", "Ilhicamina", "Itztli", "Ixtli", "Mahuizoh", "Manauia",
        "Matlal", "Matlalihuitl", "Mazati", "Mictlantecuhtli", "Milintica", "Momoztli", "Namacuix",
        "Necalli", "Necuametl", "Nezahualcoyotl", "Nexahualpilli", "Nochehuatl", "Nopaltzin",
        "Ollin", "Quauhtli", "Tenoch", "Teoxihuitl", "Tepiltzin", "Tezcacoatl", "Tlacaelel",
        "Tlacelel", "Tlaloc", "Tlanextic", "Tlazohtlaloni", "Tlazopillo", "Uetzcayotl", "Xipilli",
        "Yaoti", 
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
                "Achcauhtli",
                "Tlaloc",
                "Nenetl",
                "Ilhuitl",
                "Cipactli",
                "Teyacapan"
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
        npc.ethnicity.replace(Ethnicity::Mesoamerican);
        regenerate(rng, &mut npc);
        npc.name.value.unwrap()
    }
}
