use std::collections::HashMap;

use rand::distributions::WeightedIndex;
use rand::prelude::*;

use super::npc::Race;

pub struct Demographics {
    races: HashMap<Race, u64>,
}

impl Demographics {
    pub fn gen_race(&self, rng: &mut impl Rng) -> Race {
        if self.races.is_empty() {
            Race::Human
        } else {
            let (races, race_weights): (Vec<Race>, Vec<u64>) = self.races.iter().unzip();
            let dist = WeightedIndex::new(&race_weights).unwrap();
            races[dist.sample(rng)]
        }
    }
}

impl Default for Demographics {
    fn default() -> Self {
        let mut races = HashMap::new();
        races.insert(Race::Human, 1_020_000);
        // races.insert(Race::HalfElf, 320_000);
        // races.insert(Race::Elf, 220_000);
        // races.insert(Race::Gnome, 220_000);
        // races.insert(Race::Halfling, 100_000);
        // races.insert(Race::Shifter, 60_000);
        // races.insert(Race::Changeling, 40_000);

        Self { races }
    }
}
