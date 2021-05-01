use rand::distributions::WeightedIndex;
use rand::prelude::*;

use super::{Age, Gender, Generate, Rng};

pub struct Ethnicity;

impl Ethnicity {
    #[rustfmt::skip]
    const FEMININE_NAMES: &'static [&'static str] = &[
        "Aelia", "Antonia", "Aemilia", "Agrippina", "Alba", "Aquila", "Augusta", "Aurelia",
        "Balbina", "Blandina", "Caelia", "Camilla", "Casia", "Claudia", "Cloelia", "Domitia",
        "Drusa", "Fabia", "Fabricia", "Fausta", "Flavia", "Floriana", "Fulvia", "Germana",
        "Glaucia", "Gratiana", "Hadriana", "Hermina", "Horatia", "Hortensia", "Iovita", "Iulia",
        "Laelia", "Lucretia", "Laurentia", "Livia", "Longina", "Lucilla", "Marcella", "Marcia",
        "Maxima", "Nona", "Octavia", "Paulina", "Petronia", "Porcia", "Tacita", "Tullia",
        "Verginia", "Vita",
    ];

    #[rustfmt::skip]
    const MASCULINE_NAMES: &'static [&'static str] = &[
        "Aelius", "Antonius", "Aetius", "Agrippa", "Albanus", "Albus", "Appius", "Aquilinus",
        "Atilus", "Augustus", "Aurelius", "Avitus", "Balbus", "Blandus", "Blasius", "Brutus",
        "Caelius", "Caius", "Casian", "Cassius", "Cato", "Celsus", "Claudius", "Cloelius",
        "Cnaeus", "Crispus", "Cyprianus", "Diocletianus", "Egnatius", "Ennius", "Fabricius",
        "Faustus", "Gaius", "Germanus", "Gnaeus", "Horatius", "Iovianus", "Iulius", "Lucilius",
        "Manius", "Marcus", "Marius", "Maximus", "Octavius", "Paulus", "Quintilian", "Regulus",
        "Servius", "Tacitus", "Varius",
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
            ["Aelius", "Octavius", "Iovita", "Fabia", "Albanus", "Nona"],
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
        npc.ethnicity.replace(Ethnicity::Roman);
        regenerate(rng, &mut npc);
        npc.name.value.unwrap()
    }
}
