use super::{Age, Gender, Generate};
use rand::distributions::WeightedIndex;
use rand::prelude::*;

pub struct Ethnicity;

impl Ethnicity {
    #[rustfmt::skip]
    const FEMININE_NAMES: &'static [&'static str] = &[
        "Adrie", "Caelynn", "Chaedi", "Claira", "Dara", "Drusilia", "Elama", "Enna", "Faral",
        "Felosial", "Hatae", "Ielenia", "Ilanis", "Irann", "Jarsali", "Jelenneth", "Keyleth",
        "Leshanna", "Lia", "Maiathah", "Malquis", "Meriele", "Mialee", "Myathethil", "Naivara",
        "Quelenna", "Quillathe", "Ridaro", "Sariel", "Shanairla", "Shava", "Silaqui", "Sumnes",
        "Theirastra", "Thiala", "Tiaathque", "Traulam", "Vadania", "Vaina", "Valanthe", "Xanaphia",
    ];

    #[rustfmt::skip]
    const MASCULINE_NAMES: &'static [&'static str] = &[
        "Adran", "Aelar", "Aerdeth", "Ahvain", "Aramil", "Arannis", "Aust", "Azaki", "Beiro",
        "Berrian", "Caeldrim", "Carrie", "Dayereth", "Dreali", "Efferil", "Eiravel", "Enialis",
        "Erdan", "Erevan", "Fivin", "Galinndan", "Gennal", "Hadarai", "Halimath", "Heian", "Himo",
        "Immeral", "Ivellios", "Korfel", "Lamlis", "Laucian", "Lucan", "Mindartis", "Naal",
        "Nutae", "Paelias", "Peren", "Quarion", "Riardon", "Rolen", "Soveliss", "Suhnae",
        "Thamior", "Tharivol", "Theren", "Theriatis", "Thervan", "Uthemar", "Vanuath", "Varis",
    ];

    #[rustfmt::skip]
    const CHILD_NAMES: &'static [&'static str] = &[
        "Ael", "Ang", "Ara", "Ari", "Arn", "Aym", "Broe", "Bryn", "Cael", "Cy", "Dae", "Del",
        "Eli", "Eryn", "Faen", "Fera", "Gael", "Gar", "Innil", "Jar", "Kan", "Koeth", "Lael",
        "Lue", "Mai", "Mara", "Mella", "Mya", "Naeris", "Naill", "Nim", "Phann", "Py", "Rael",
        "Raer", "Ren", "Rinn", "Rua", "Sael", "Sai", "Sumi", "Syllin", "Ta", "Thia", "Tia",
        "Traki", "Vall", "Von", "Wil", "Za", 
    ];

    #[rustfmt::skip]
    const FAMILY_NAMES: &'static [&'static str] = &[
        "Aloro", "Amakiir", "Amastacia", "Ariessus", "Arnuanna", "Berevan", "Caerdonel",
        "Caphaxath", "Casilltenirra", "Cithreth", "Dalanthan", "Eathalena", "Erenaeth",
        "Ethanasath", "Fasharash", "Firahel", "Floshem", "Galanodel", "Goltorah", "Hanali",
        "Holimion", "Horineth", "Iathrana", "Ilphelkiir", "Iranapha", "Koehlanna", "Lathalas",
        "Liadon", "Meliamne", "Mellerelel", "Mystralath", "Nailo", "Netyoive", "Ofandrus",
        "Ostoroth", "Othronus", "Qualanthri", "Raethran", "Rothenel", "Selevarun", "Siannodel",
        "Suithrasas", "Sylvaranth", "Teinithra", "Tiltathana", "Wasanthi", "Withrethin",
        "Xiloscient", "Xistsrith", "Yaeldrin", 
    ];
}

impl Generate for Ethnicity {
    fn gen_name(rng: &mut impl Rng, age: &Age, gender: &Gender) -> String {
        let mut name = match (age, gender) {
            (Age::Infant(_), _) | (Age::Child(_), _) | (Age::Adolescent(_), _) => {
                Self::CHILD_NAMES[rng.gen_range(0..Self::CHILD_NAMES.len())].to_string()
            }
            (_, Gender::Masculine) => {
                Self::MASCULINE_NAMES[rng.gen_range(0..Self::MASCULINE_NAMES.len())].to_string()
            }
            (_, Gender::Feminine) => {
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

    #[test]
    fn gen_name_test() {
        let mut rng = SmallRng::seed_from_u64(0);
        let adult = Age::Adult(0);
        let m = Gender::Masculine;
        let f = Gender::Feminine;
        let t = Gender::Trans;

        assert_eq!(
            [
                "Lael Ilphelkiir",
                "Von Mellerelel",
                "Jar Erenaeth",
                "Mindartis Amastacia",
                "Thervan Eathalena",
                "Felosial Selevarun",
                "Hatae Teinithra",
                "Theriatis Xistsrith",
                "Gennal Sylvaranth",
            ],
            [
                gen_name(&mut rng, &Age::Infant(0), &m),
                gen_name(&mut rng, &Age::Child(0), &f),
                gen_name(&mut rng, &Age::Adolescent(0), &t),
                gen_name(&mut rng, &adult, &m),
                gen_name(&mut rng, &adult, &m),
                gen_name(&mut rng, &adult, &f),
                gen_name(&mut rng, &adult, &f),
                gen_name(&mut rng, &adult, &t),
                gen_name(&mut rng, &adult, &t),
            ],
        );
    }

    fn gen_name(rng: &mut impl Rng, age: &Age, gender: &Gender) -> String {
        let mut npc = Npc::default();
        npc.gender.replace(*gender);
        npc.age.replace(*age);
        npc.ethnicity.replace(Ethnicity::Elvish);
        regenerate(rng, &mut npc);
        format!("{}", npc.name)
    }
}
