use rand::distributions::WeightedIndex;
use rand::prelude::*;

use super::{Age, Gender, Generate};

pub struct Ethnicity;

impl Ethnicity {
    #[rustfmt::skip]
    const FEMININE_NAMES: &'static [&'static str] = &[
        "Alain", "Andry", "Anne", "Bella", "Blossom", "Bree", "Callie", "Chenna", "Cora", "Dee",
        "Dell", "Eida", "Eran", "Euphemia", "Georgina", "Gynnie", "Harriet", "Jasmine", "Jillian",
        "Jo", "Kithri", "Lavinia", "Lidda", "Maegan", "Marigold", "Merla", "Myria", "Nedda",
        "Nikki", "Nora", "Olivia", "Paela", "Pearl", "Pennie", "Philomena", "Portia", "Robbie",
        "Rose", "Saral", "Seraphina", "Shaena", "Stacee", "Tawna", "Thea", "Trym", "Tyna", "Vani",
        "Verna", "Wella", "Willow",
    ];

    #[rustfmt::skip]
    const MASCULINE_NAMES: &'static [&'static str] = &[
        "Alton", "Ander", "Bernie", "Bobbin", "Cade", "Callus", "Corrin", "Dannad", "Danniel",
        "Eddie", "Egart", "Eldon", "Errich", "Fildo", "Finnan", "Franklin", "Garret", "Garth",
        "Gilbert", "Gob", "Harol", "Igor", "Jasper", "Keith", "Kevin", "Lazam", "Lerry", "Lindal",
        "Lyle", "Merric", "Mican", "Milo", "Morrin", "Nebin", "Nevil", "Osborn", "Ostran",
        "Oswalt", "Perrin", "Poppy", "Reed", "Roscoe", "Sam", "Shardon", "Tye", "Ulmo", "Wellby",
        "Wendel", "Wenner", "Wes",
    ];

    #[rustfmt::skip]
    const FAMILY_NAMES: &'static [&'static str] = &[
        "Appleblossom", "Bigheart", "Brightmoon", "Brushgather", "Cherrycheeks", "Copperkettle",
        "Deephollow", "Elderberry", "Fastfoot", "Fatrabbit", "Glenfellow", "Goldfound",
        "Goodbarrel", "Goodearth", "Greenbottle", "Greenleaf", "Highhill", "Hilltopple",
        "Hogcollar", "Honeypot", "Jamjar", "Kettlewhistle", "Leagallow", "Littlefoot",
        "Nimblefingers", "Porridgepot", "Quickstep", "Reedfellow", "Shadowquick", "Silvereyes",
        "Smoothhands", "Stonebridge", "Stoutbridge", "Stoutman", "Strongbones", "Sunmeadow",
        "Swiftwhistle", "Tallfellow", "Tealeaf", "Tenpenny", "Thistletop", "Thorngage",
        "Tosscobble", "Underbough", "Underfoot", "Warmwater", "Whispermouse", "Wildcloak",
        "Wildheart", "Wiseacre",
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
        name.push_str(Self::FAMILY_NAMES[rng.gen_range(0..Self::FAMILY_NAMES.len())]);
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
                "Alton Underbough",
                "Mican Hilltopple",
                "Blossom Thorngage",
                "Nikki Greenleaf",
                "Bernie Tenpenny",
                "Myria Goodearth"
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
        npc.ethnicity.replace(Ethnicity::Halfling);
        regenerate(rng, &mut npc);
        format!("{}", npc.name)
    }
}
