use std::collections::HashMap;
use std::iter;

use rand::distributions::WeightedIndex;
use rand::prelude::*;

use super::npc::{Ethnicity, Race};

#[derive(Clone, Debug)]
pub struct Demographics {
    groups: HashMap<(Race, Ethnicity), u64>,
}

impl Demographics {
    pub fn shift_race(&self, race: &Race, amount: f64) -> Demographics {
        if amount < 0. || amount > 1. {
            panic!("Invalid input: {}", amount);
        }

        let population: u64 = self.groups.values().sum();
        let race_population: u64 = self
            .groups
            .iter()
            .filter_map(|((r, _), n)| if r == race { Some(n) } else { None })
            .sum();

        let groups: HashMap<(Race, Ethnicity), u64> = if race_population > 0 {
            self.groups
                .iter()
                .map(|((r, e), &v)| {
                    (
                        (*r, *e),
                        if r == race {
                            (v as f64 * (1. - amount)
                                + (v as f64 * amount * population as f64 / race_population as f64))
                                as u64
                        } else {
                            (v as f64 * (1. - amount)) as u64
                        },
                    )
                })
                .filter(|(_, v)| *v > 0)
                .collect()
        } else {
            self.groups
                .iter()
                .map(|(&k, &v)| (k, (v as f64 * (1. - amount)) as u64))
                .chain(iter::once((
                    (*race, race.default_ethnicity()),
                    (population as f64 * amount) as u64,
                )))
                .filter(|(_, v)| *v > 0)
                .collect()
        };

        Demographics { groups }
    }

    pub fn only_race(&self, race: &Race) -> Demographics {
        self.shift_race(race, 1.)
    }

    pub fn gen_race_ethnicity(&self, rng: &mut impl Rng) -> (Race, Ethnicity) {
        if self.groups.is_empty() {
            (Race::Human, Race::Human.default_ethnicity())
        } else {
            let (groups, weights): (Vec<(Race, Ethnicity)>, Vec<u64>) = self.groups.iter().unzip();
            let dist = WeightedIndex::new(&weights).unwrap();
            groups[dist.sample(rng)]
        }
    }
}

#[cfg(test)]
mod test_demographics {
    use super::*;
    use rand::rngs::mock::StepRng;

    #[test]
    fn shift_race_test_existing() {
        let mut groups = HashMap::with_capacity(2);
        groups.insert((Race::Human, Ethnicity::Arabic), 30);
        groups.insert((Race::Human, Ethnicity::Celtic), 20);
        groups.insert((Race::Warforged, Ethnicity::Warforged), 50);
        let demographics = Demographics { groups }.shift_race(&Race::Human, 0.3);

        assert_eq!(3, demographics.groups.len());
        assert_eq!(
            Some(&39),
            demographics.groups.get(&(Race::Human, Ethnicity::Arabic)),
            "{:?}",
            demographics
        );
        assert_eq!(
            Some(&26),
            demographics.groups.get(&(Race::Human, Ethnicity::Celtic))
        );
        assert_eq!(
            Some(&35),
            demographics
                .groups
                .get(&(Race::Warforged, Ethnicity::Warforged))
        );
    }

    #[test]
    fn shift_race_test_new() {
        let mut groups = HashMap::with_capacity(1);
        groups.insert((Race::Human, Ethnicity::Arabic), 100);
        let demographics = Demographics { groups }.shift_race(&Race::Warforged, 0.4);

        assert_eq!(2, demographics.groups.len());
        assert_eq!(
            Some(&60),
            demographics.groups.get(&(Race::Human, Ethnicity::Arabic))
        );
        assert_eq!(
            Some(&40),
            demographics
                .groups
                .get(&(Race::Warforged, Ethnicity::Warforged))
        );
    }

    #[test]
    fn only_race_test() {
        let mut groups = HashMap::with_capacity(2);
        groups.insert((Race::Human, Ethnicity::Arabic), 30);
        groups.insert((Race::Human, Ethnicity::Celtic), 20);
        groups.insert((Race::Warforged, Ethnicity::Warforged), 50);
        let demographics = Demographics { groups }.only_race(&Race::Human);

        assert_eq!(2, demographics.groups.len());
        assert_eq!(
            Some(&60),
            demographics.groups.get(&(Race::Human, Ethnicity::Arabic))
        );
        assert_eq!(
            Some(&40),
            demographics.groups.get(&(Race::Human, Ethnicity::Celtic))
        );
    }

    #[test]
    fn gen_race_ethnicity_test() {
        let mut groups = HashMap::new();
        groups.insert((Race::Human, Ethnicity::Arabic), 50);
        groups.insert((Race::Warforged, Ethnicity::Warforged), 50);
        let demographics = Demographics { groups };

        let mut rng = StepRng::new(0, 0xDEADBEEF_DECAFBAD);
        let mut counts: HashMap<(Race, Ethnicity), u8> = HashMap::with_capacity(2);

        for i in 0..10 {
            let race_ethnicity = &demographics.gen_race_ethnicity(&mut rng);
            *counts.entry(*race_ethnicity).or_default() += 1;
            println!("{}: {:?}", i, counts);
        }

        assert_eq!(Some(&5), counts.get(&(Race::Human, Ethnicity::Arabic)));
        assert_eq!(
            Some(&5),
            counts.get(&(Race::Warforged, Ethnicity::Warforged))
        );
    }
}

impl Default for Demographics {
    fn default() -> Self {
        let mut groups = HashMap::new();
        groups.insert((Race::Human, Ethnicity::Arabic), 1_020_000);
        // groups.insert(Race::HalfElf, 320_000);
        // groups.insert(Race::Elf, 220_000);
        // groups.insert(Race::Gnome, 220_000);
        // groups.insert(Race::Halfling, 100_000);
        // groups.insert(Race::Shifter, 60_000);
        // groups.insert(Race::Changeling, 40_000);

        Self { groups }
    }
}
