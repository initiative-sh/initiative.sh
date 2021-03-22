use std::collections::HashMap;

use rand::distributions::WeightedIndex;
use rand::prelude::*;

use super::npc::Race;

pub struct Demographics {
    races: HashMap<Race, u64>,
}

impl Demographics {
    pub fn shift_race(&self, race: &Race) -> Demographics {
        let population: u64 = self.races.values().sum();

        let mut races: HashMap<Race, u64> = self.races.iter().map(|(k, v)| (*k, v / 2)).collect();
        *races.entry(*race).or_default() += population / 2;

        Demographics { races }
    }

    pub fn only_race(&self, race: &Race) -> Demographics {
        let mut races = HashMap::with_capacity(1);
        races.insert(*race, self.races.values().sum());
        Demographics { races }
    }

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

#[cfg(test)]
mod test_demographics {
    use super::*;
    use rand::rngs::mock::StepRng;

    #[test]
    fn shift_race_test() {
        let mut races = HashMap::with_capacity(2);
        races.insert(Race::Human, 50);
        races.insert(Race::Warforged, 50);
        let demographics = Demographics { races }.shift_race(&Race::Human);

        assert_eq!(2, demographics.races.len());
        assert_eq!(Some(&75), demographics.races.get(&Race::Human));
        assert_eq!(Some(&25), demographics.races.get(&Race::Warforged));
    }

    #[test]
    fn only_race_test() {
        let mut races = HashMap::with_capacity(2);
        races.insert(Race::Human, 50);
        races.insert(Race::Warforged, 50);
        let demographics = Demographics { races }.only_race(&Race::Human);

        assert_eq!(1, demographics.races.len());
        assert_eq!(Some(&100), demographics.races.get(&Race::Human));
    }

    #[test]
    fn gen_race_test() {
        let mut races = HashMap::new();
        races.insert(Race::Human, 50);
        races.insert(Race::Warforged, 50);
        let demographics = Demographics { races };

        let mut rng = StepRng::new(0, 0xDEADBEEF_DECAFBAD);

        // HashMap ordering isn't deterministic, so we have to read back the
        // sequence to make our assertions correctly.
        let mut iter = demographics.races.keys();
        let (race1, race2) = (iter.next().unwrap(), iter.next().unwrap());

        assert_ne!(race1, race2);
        assert_eq!(race1, &demographics.gen_race(&mut rng));
        assert_eq!(race2, &demographics.gen_race(&mut rng));
        assert_eq!(race2, &demographics.gen_race(&mut rng));
        assert_eq!(race2, &demographics.gen_race(&mut rng));
        assert_eq!(race1, &demographics.gen_race(&mut rng));
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
