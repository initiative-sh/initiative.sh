use rand::Rng;

use super::{Age, Gender, Npc, Race as NpcRace};

pub fn regenerate(rng: &mut impl Rng, npc: &mut Npc) {
    if let Some(race) = npc.race.value {
        match race {
            NpcRace::Human => Human::regenerate(rng, npc),
        }
    }
}

trait Race {
    fn regenerate(rng: &mut impl Rng, npc: &mut Npc) {
        npc.gender.replace_with(|_| Self::gen_gender(rng));
        npc.age.replace_with(|_| Self::gen_age(rng));

        if let (Some(gender), Some(age)) = (&npc.gender.value, &npc.age.value) {
            let size = Self::gen_size(rng, age, gender);
            npc.name.replace_with(|_| Self::gen_name(rng, age, gender));
            npc.height.replace_with(|_| Self::gen_height(rng, size));
            npc.weight.replace_with(|_| Self::gen_weight(rng, size));
        }
    }

    fn gen_gender(rng: &mut impl Rng) -> Gender;

    fn gen_age(rng: &mut impl Rng) -> Age;

    fn gen_name(rng: &mut impl Rng, age: &Age, gender: &Gender) -> String;

    fn gen_size(rng: &mut impl Rng, age: &Age, gender: &Gender) -> u8;

    fn gen_height(rng: &mut impl Rng, size: u8) -> u16;

    fn gen_weight(rng: &mut impl Rng, size: u8) -> u16;
}

struct Human;

impl Race for Human {
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

    fn gen_size(_rng: &mut impl Rng, _age: &Age, _gender: &Gender) -> u8 {
        0
    }

    fn gen_height(_rng: &mut impl Rng, _size: u8) -> u16 {
        12 * 6
    }

    fn gen_weight(_rng: &mut impl Rng, _size: u8) -> u16 {
        180
    }
}
