use crate::world::{Demographics, Location};
use rand::Rng;

pub fn generate(location: &mut Location, rng: &mut impl Rng, _demographics: &Demographics) {
    location.name.clear();

    location.description.replace_with(|_| {
        match rng.gen_range(1..=20) {
            1..=2 => "Abandoned squat",
            3..=8 => "Middle-class home",
            9..=10 => "Upper-class home",
            11..=15 => "Crowded tenement",
            16..=17 => "Orphanage",
            18 => "Hidden slavers' den",
            19 => "Front for a secret cult",
            20 => "Lavish, guarded mansion",
            _ => unreachable!(),
        }
        .to_string()
    });
}
