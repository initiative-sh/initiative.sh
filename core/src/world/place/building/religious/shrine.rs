use crate::world::{Demographics, Place};
use rand::prelude::*;

pub fn generate(place: &mut Place, rng: &mut impl Rng, _demographics: &Demographics) {
    place.name.replace_with(|_| name(rng));
    place.description.replace_with(|_| description(rng));
}

fn name(_rng: &mut impl Rng) -> String {
    format!("New Shrine")
}
fn description(_rng: &mut impl Rng) -> String {
    format!("a new shrine")
}

// {place} of {purview}

//purview --> deity & description
//purview decides the deity the shrine is devoted to
//purview should also decide which offering types are possible.

