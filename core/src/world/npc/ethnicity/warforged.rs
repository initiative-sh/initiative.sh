use super::{Age, Gender, Generate};
use rand::prelude::*;

pub struct Ethnicity;

impl Ethnicity {
    #[rustfmt::skip]
    const NAMES: &'static [&'static str] = &[
        "Anchor", "Angel", "Banner", "Bastion", "Biscuit", "Blade", "Blue", "Bones", "Bow",
        "Brains", "Bridgehead", "Caboose", "Candle", "Cart", "Casper", "Chappie", "Church",
        "Craft", "Crunch", "Crystal", "Curly", "Dagger", "Dash", "Dent", "Digger", "Five", "Flash",
        "Foggy", "Four", "Furball", "Ghost", "Giggles", "Gilligan", "Ginger", "Glaive", "Gramps",
        "Green", "Gunner", "Hammer", "Happy", "Hurricane", "Hyde", "Iron", "Juggernaut", "Junior",
        "Keep", "Leaky", "Leatherman", "Lucky", "Mace", "Maverick", "Midas", "Mini", "Mumbles",
        "Nitro", "Nugget", "Oak", "Onyx", "Pants", "Pierce", "Pump", "Raven", "Reaper", "Red",
        "Rock", "Rod", "Rusty", "Scout", "Scratch", "Seven", "Shield", "Shrimp", "Six", "Slash",
        "Slate", "Smith", "Snake", "Spear", "Spike", "Sword", "Temple", "Ten", "Three", "Turret",
        "Twiggy", "Vault", "Wall",
    ];
}

impl Generate for Ethnicity {
    fn gen_name(rng: &mut impl Rng, _age: &Age, _gender: &Gender) -> String {
        Self::NAMES[rng.gen_range(0..Self::NAMES.len())].to_string()
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
        let age = Age::Adult;
        let m = Gender::Masculine;

        assert_eq!(
            ["Anchor", "Smith", "Rock", "Mumbles", "Ghost", "Crystal"],
            [
                gen_name(&mut rng, &age, &m),
                gen_name(&mut rng, &age, &m),
                gen_name(&mut rng, &age, &m),
                gen_name(&mut rng, &age, &m),
                gen_name(&mut rng, &age, &m),
                gen_name(&mut rng, &age, &m),
            ],
        );
    }

    fn gen_name(rng: &mut impl Rng, age: &Age, gender: &Gender) -> String {
        let mut npc = Npc::default();
        npc.gender.replace(*gender);
        npc.age.replace(*age);
        npc.ethnicity.replace(Ethnicity::Warforged);
        regenerate(rng, &mut npc);
        format!("{}", npc.name)
    }
}
