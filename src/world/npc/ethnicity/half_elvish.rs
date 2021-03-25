use super::{Age, Gender, Generate, Rng};

pub struct Ethnicity;

impl Generate for Ethnicity {
    fn gen_name(rng: &mut impl Rng, age: &Age, gender: &Gender) -> String {
        if rng.gen_bool(0.5) {
            super::elvish::Ethnicity::gen_name(rng, age, gender)
        } else {
            super::human::Ethnicity::gen_name(rng, age, gender)
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
        let adult = Age::Adult(0);
        let m = Gender::Masculine;
        let f = Gender::Feminine;
        let t = Gender::Trans;

        assert_eq!(
            [
                "Thia Mystralath",
                "Gar Arnuanna",
                "Horatia",
                "Eiravel Amastacia",
                "Laki",
                "Drusilia Aloro",
                "Indu",
                "Vanuath Suithrasas",
                "Huitzilihuitl"
            ],
            [
                Ethnicity::gen_name(&mut rng, &Age::Infant(0), &m),
                Ethnicity::gen_name(&mut rng, &Age::Child(0), &f),
                Ethnicity::gen_name(&mut rng, &Age::Adolescent(0), &t),
                Ethnicity::gen_name(&mut rng, &adult, &m),
                Ethnicity::gen_name(&mut rng, &adult, &m),
                Ethnicity::gen_name(&mut rng, &adult, &f),
                Ethnicity::gen_name(&mut rng, &adult, &f),
                Ethnicity::gen_name(&mut rng, &adult, &t),
                Ethnicity::gen_name(&mut rng, &adult, &t),
            ]
        );
    }
}
