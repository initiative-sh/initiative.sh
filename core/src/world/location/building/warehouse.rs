use crate::world::{Demographics, Location};
use rand::Rng;

pub fn generate(location: &mut Location, rng: &mut impl Rng, _demographics: &Demographics) {
    location.name.clear();

    location.description.replace_with(|_| {
        match rng.gen_range(1..=20) {
            1..=4 => "Empty or abandoned",
            5..=6 => "Heavily guarded, expensve goods",
            7..=10 => "Cheap goods",
            11..=14 => "Bulk goods",
            15 => "Live animals",
            16..=17 => "Weapons/armor",
            18..=19 => "Goods from a distant land",
            20 => "Secret smuggler's den",
            _ => unreachable!(),
        }
        .to_string()
    });
}
