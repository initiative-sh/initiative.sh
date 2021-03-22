use rand::Rng;

use super::{Age, Gender, Npc, Race as NpcRace};

mod human;

pub fn regenerate(rng: &mut impl Rng, npc: &mut Npc) {
    if let Some(race) = npc.race.value {
        match race {
            NpcRace::Human => human::Human::regenerate(rng, npc),
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
