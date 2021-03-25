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
    pub fn shift_race(&self, race: &Race, amount: f64) -> Self {
        self.shift_by(|r, _| r == race, amount, (*race, race.default_ethnicity()))
    }

    pub fn only_race(&self, race: &Race) -> Self {
        self.shift_race(race, 1.)
    }

    pub fn shift_ethnicity(&self, ethnicity: &Ethnicity, amount: f64) -> Self {
        self.shift_by(
            |_, e| e == ethnicity,
            amount,
            (ethnicity.default_race(), *ethnicity),
        )
    }

    pub fn only_ethnicity(&self, ethnicity: &Ethnicity) -> Self {
        self.shift_ethnicity(ethnicity, 1.)
    }

    pub fn shift_race_ethnicity(&self, race: &Race, ethnicity: &Ethnicity, amount: f64) -> Self {
        self.shift_by(
            |r, e| r == race && e == ethnicity,
            amount,
            (*race, *ethnicity),
        )
    }

    pub fn only_race_ethnicity(&self, race: &Race, ethnicity: &Ethnicity) -> Self {
        self.shift_race_ethnicity(race, ethnicity, 1.)
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

    fn shift_by<F: Fn(&Race, &Ethnicity) -> bool>(
        &self,
        f: F,
        amount: f64,
        default: (Race, Ethnicity),
    ) -> Self {
        if amount < 0. || amount > 1. {
            panic!("Invalid input: {}", amount);
        }

        let population: u64 = self.groups.values().sum();
        let race_population: u64 = self
            .groups
            .iter()
            .filter_map(|((r, e), n)| if f(r, e) { Some(n) } else { None })
            .sum();

        let groups: HashMap<(Race, Ethnicity), u64> = if race_population > 0 {
            self.groups
                .iter()
                .map(|((r, e), &v)| {
                    (
                        (*r, *e),
                        if f(r, e) {
                            (v as f64 * (1. - amount)
                                + (v as f64 * amount * population as f64 / race_population as f64))
                                .round() as u64
                        } else {
                            (v as f64 * (1. - amount)).round() as u64
                        },
                    )
                })
                .filter(|(_, v)| *v > 0)
                .collect()
        } else {
            self.groups
                .iter()
                .map(|(&k, &v)| (k, (v as f64 * (1. - amount)).round() as u64))
                .chain(iter::once((
                    default,
                    (population as f64 * amount).round() as u64,
                )))
                .filter(|(_, v)| *v > 0)
                .collect()
        };

        Self { groups }
    }
}

#[cfg(test)]
mod test_demographics {
    use super::*;
    use rand::rngs::mock::StepRng;

    #[test]
    fn shift_race_test_existing() {
        let demographics = demographics().shift_race(&Race::Human, 0.3);

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
                .get(&(Race::Warforged, Ethnicity::Celtic))
        );
    }

    #[test]
    fn shift_ethnicity_test_existing() {
        let demographics = demographics().shift_ethnicity(&Ethnicity::Celtic, 0.3);

        assert_eq!(3, demographics.groups.len());
        assert_eq!(
            Some(&21),
            demographics.groups.get(&(Race::Human, Ethnicity::Arabic)),
            "{:?}",
            demographics
        );
        assert_eq!(
            Some(&23),
            demographics.groups.get(&(Race::Human, Ethnicity::Celtic))
        );
        assert_eq!(
            Some(&56),
            demographics
                .groups
                .get(&(Race::Warforged, Ethnicity::Celtic))
        );
    }

    #[test]
    fn shift_race_ethnicity_test_existing() {
        let demographics =
            demographics().shift_race_ethnicity(&Race::Warforged, &Ethnicity::Celtic, 0.3);

        assert_eq!(3, demographics.groups.len());
        assert_eq!(
            Some(&21),
            demographics.groups.get(&(Race::Human, Ethnicity::Arabic)),
            "{:?}",
            demographics
        );
        assert_eq!(
            Some(&14),
            demographics.groups.get(&(Race::Human, Ethnicity::Celtic))
        );
        assert_eq!(
            Some(&65),
            demographics
                .groups
                .get(&(Race::Warforged, Ethnicity::Celtic))
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
    fn shift_ethnicity_test_new() {
        let mut groups = HashMap::with_capacity(1);
        groups.insert((Race::Human, Ethnicity::Arabic), 100);
        let demographics = Demographics { groups }.shift_ethnicity(&Ethnicity::Warforged, 0.4);

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
    fn shift_race_ethnicity_test_new() {
        let mut groups = HashMap::with_capacity(1);
        groups.insert((Race::Human, Ethnicity::Arabic), 100);
        let demographics =
            Demographics { groups }.shift_race_ethnicity(&Race::Warforged, &Ethnicity::Celtic, 0.4);

        assert_eq!(2, demographics.groups.len());
        assert_eq!(
            Some(&60),
            demographics.groups.get(&(Race::Human, Ethnicity::Arabic))
        );
        assert_eq!(
            Some(&40),
            demographics
                .groups
                .get(&(Race::Warforged, Ethnicity::Celtic))
        );
    }

    #[test]
    fn only_race_test() {
        let demographics = demographics().only_race(&Race::Human);

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
    fn only_ethnicity_test() {
        let demographics = demographics().only_ethnicity(&Ethnicity::Celtic);

        assert_eq!(2, demographics.groups.len());
        assert_eq!(
            Some(&29),
            demographics.groups.get(&(Race::Human, Ethnicity::Celtic))
        );
        assert_eq!(
            Some(&71),
            demographics
                .groups
                .get(&(Race::Warforged, Ethnicity::Celtic))
        );
    }

    #[test]
    fn only_race_ethnicity_test() {
        let demographics = demographics().only_race_ethnicity(&Race::Warforged, &Ethnicity::Celtic);

        assert_eq!(1, demographics.groups.len());
        assert_eq!(
            Some(&100),
            demographics
                .groups
                .get(&(Race::Warforged, Ethnicity::Celtic))
        );
    }

    #[test]
    fn gen_race_ethnicity_test() {
        let mut groups = HashMap::new();
        groups.insert((Race::Human, Ethnicity::Arabic), 50);
        groups.insert((Race::Warforged, Ethnicity::Celtic), 50);
        let demographics = Demographics { groups };

        let mut rng = StepRng::new(0, 0xDEADBEEF_DECAFBAD);
        let mut counts: HashMap<(Race, Ethnicity), u8> = HashMap::with_capacity(2);

        for i in 0..10 {
            let race_ethnicity = &demographics.gen_race_ethnicity(&mut rng);
            *counts.entry(*race_ethnicity).or_default() += 1;
            println!("{}: {:?}", i, counts);
        }

        assert_eq!(Some(&5), counts.get(&(Race::Human, Ethnicity::Arabic)));
        assert_eq!(Some(&5), counts.get(&(Race::Warforged, Ethnicity::Celtic)));
    }

    fn demographics() -> Demographics {
        let mut groups = HashMap::with_capacity(3);
        groups.insert((Race::Human, Ethnicity::Arabic), 30);
        groups.insert((Race::Human, Ethnicity::Celtic), 20);
        groups.insert((Race::Warforged, Ethnicity::Celtic), 50);
        Demographics { groups }
    }
}

impl Default for Demographics {
    fn default() -> Self {
        let mut groups = HashMap::new();
        groups.insert((Race::Human, Ethnicity::Human), 1_020_000);
        groups.insert((Race::HalfElf, Ethnicity::HalfElvish), 320_000);
        groups.insert((Race::Elf, Ethnicity::Elvish), 220_000);
        // groups.insert(Race::Gnome, 220_000);
        groups.insert((Race::Halfling, Ethnicity::Halfling), 100_000);
        // groups.insert(Race::Shifter, 60_000);
        // groups.insert(Race::Changeling, 40_000);

        Self { groups }
    }
}
