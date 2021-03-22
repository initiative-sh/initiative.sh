use super::{Age, Gender, Race as NpcRace, Rng, Size};

pub struct Race;

impl NpcRace for Race {
    fn gen_gender(rng: &mut impl Rng) -> Gender {
        match rng.gen_range(1..=101) {
            1..=50 => Gender::Feminine,
            51..=100 => Gender::Masculine,
            101 => Gender::Trans,
            _ => unreachable!(),
        }
    }

    fn gen_age(rng: &mut impl Rng) -> Age {
        match rng.gen_range(0..=79) {
            i if i < 2 => Age::Infant(i),
            i if i < 10 => Age::Child(i),
            i if i < 20 => Age::Adolescent(i),
            i if i < 30 => Age::YoungAdult(i),
            i if i < 40 => Age::Adult(i),
            i if i < 60 => Age::MiddleAged(i),
            i if i < 70 => Age::Elderly(i),
            i => Age::Geriatric(i),
        }
    }

    fn gen_name(_rng: &mut impl Rng, _age: &Age, _gender: &Gender) -> String {
        String::from("Potato Johnson")
    }

    fn gen_size(_rng: &mut impl Rng, _age: &Age, _gender: &Gender) -> Size {
        Size::Medium {
            height: 72,
            weight: 180,
        }
    }
}
