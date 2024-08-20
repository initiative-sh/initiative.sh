pub mod demographics;
pub mod npc;
pub mod place;
pub mod thing;

pub use command::{ParsedThing, WorldCommand};
pub use demographics::Demographics;
pub use field::Field;

mod command;
mod field;
mod word;

use rand::Rng;

pub trait Generate: Default {
    fn generate(rng: &mut impl Rng, demographics: &Demographics) -> Self {
        let mut result = Self::default();
        result.regenerate(rng, demographics);
        result
    }

    fn regenerate(&mut self, rng: &mut impl Rng, demographics: &Demographics);
}

fn weighted_index_from_tuple<'a, T>(rng: &mut impl Rng, input: &'a [(T, usize)]) -> &'a T {
    let total = input.iter().map(|(_, n)| n).sum();

    if total == 0 {
        panic!("Empty input.");
    }

    let target = rng.gen_range(0..total);
    let mut acc = 0;

    for (value, frequency) in input {
        acc += frequency;
        if acc > target {
            return value;
        }
    }

    unreachable!();
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn weighted_index_from_tuple_test() {
        let input = [('a', 1), ('b', 3), ('c', 5)];
        let mut rng = SmallRng::seed_from_u64(0);
        assert_eq!(
            vec!['c', 'c', 'c', 'c', 'c', 'c', 'b', 'c', 'b', 'a'],
            (0..10)
                .map(|_| weighted_index_from_tuple(&mut rng, &input[..]))
                .copied()
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn weighted_index_from_tuple_test_one() {
        let input = [(true, 1)];
        let mut rng = SmallRng::seed_from_u64(0);
        assert_eq!(
            vec![true, true, true],
            (0..3)
                .map(|_| weighted_index_from_tuple(&mut rng, &input[..]))
                .copied()
                .collect::<Vec<_>>()
        );
    }

    #[test]
    #[should_panic]
    fn weighted_index_from_tuple_test_empty() {
        let input: [(bool, usize); 0] = [];
        weighted_index_from_tuple(&mut SmallRng::seed_from_u64(0), &input[..]);
    }

    #[test]
    #[should_panic]
    fn weighted_index_from_tuple_test_zero() {
        let input = [(true, 0), (false, 0)];
        weighted_index_from_tuple(&mut SmallRng::seed_from_u64(0), &input[..]);
    }
}
