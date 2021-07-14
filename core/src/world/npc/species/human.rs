use super::{Age, Gender, Generate, Rng, Size};

pub struct Species;

impl Species {
    fn age(years: u16) -> Age {
        match years {
            y if y < 2 => Age::Infant(y),
            y if y < 10 => Age::Child(y),
            y if y < 20 => Age::Adolescent(y),
            y if y < 30 => Age::YoungAdult(y),
            y if y < 40 => Age::Adult(y),
            y if y < 60 => Age::MiddleAged(y),
            y if y < 70 => Age::Elderly(y),
            y => Age::Geriatric(y),
        }
    }
}

#[cfg(test)]
mod test_species {
    use super::{Age, Species};

    #[test]
    fn age_test() {
        assert_eq!(Age::Infant(0), Species::age(0));
        assert_eq!(Age::Child(2), Species::age(2));
        assert_eq!(Age::Adolescent(10), Species::age(10));
        assert_eq!(Age::YoungAdult(20), Species::age(20));
        assert_eq!(Age::Adult(30), Species::age(30));
        assert_eq!(Age::MiddleAged(40), Species::age(40));
        assert_eq!(Age::Elderly(60), Species::age(60));
        assert_eq!(Age::Geriatric(70), Species::age(70));
    }
}

impl Generate for Species {
    fn gen_gender(rng: &mut impl Rng) -> Gender {
        match rng.gen_range(1..=101) {
            1..=50 => Gender::Feminine,
            51..=100 => Gender::Masculine,
            101 => Gender::Trans,
            _ => unreachable!(),
        }
    }

    fn gen_age(rng: &mut impl Rng) -> Age {
        Self::age(rng.gen_range(0..=79))
    }

    fn gen_size(rng: &mut impl Rng, age: &Age, gender: &Gender) -> Size {
        let is_female = match gender {
            Gender::Masculine => rng.gen_bool(0.01),
            Gender::Feminine => rng.gen_bool(0.99),
            _ => rng.gen_bool(0.5),
        };

        match (age, is_female) {
            (Age::Infant(0), _) => {
                let size = rng.gen_range(0..=30);
                Size::Tiny {
                    height: 20 + size / 3,
                    weight: 7 + size / 2,
                }
            }
            (Age::Infant(_), _) => {
                let size = rng.gen_range(0..=5);
                Size::Tiny {
                    height: 30 + size,
                    weight: 22 + size,
                }
            }
            (Age::Child(i), _) => {
                let y = (*i - 2) as f32 / 8.;
                let (height, weight) =
                    super::gen_height_weight(rng, (33. + y * 18.)..=(35. + y * 22.), 14.0..=17.0);
                Size::Small { height, weight }
            }
            (Age::Adolescent(i), true) => {
                let y = (*i - 10) as f32;
                let (height, weight) = super::gen_height_weight(
                    rng,
                    (51. + y * 2.).min(61.)..=(65. + y * 2.).min(67.),
                    (15. + y * 2.5 / 5.).min(18.5)..=(19. + y * 4.5 / 5.).min(25.),
                );
                Size::Medium { height, weight }
            }
            (Age::Adolescent(i), false) => {
                let y = (*i - 10) as f32 / 5.;
                let (height, weight) = super::gen_height_weight(
                    rng,
                    (51. + y * 12.).min(66.)..=(57. + y * 13.).min(72.),
                    (15. + y * 2.5).min(18.5)..=(18.5 + y * 4.5).min(29.),
                );
                Size::Medium { height, weight }
            }
            (_, true) => {
                let (height, weight) = super::gen_height_weight(rng, 61.0..=67.0, 19.0..=25.0);
                Size::Medium { height, weight }
            }
            (_, false) => {
                let (height, weight) = super::gen_height_weight(rng, 66.0..=72.0, 18.5..=29.0);
                Size::Medium { height, weight }
            }
        }
    }
}

#[cfg(test)]
mod test_generate_for_species {
    use super::*;
    use rand::rngs::mock::StepRng;
    use std::collections::HashMap;

    #[test]
    fn gen_gender_test() {
        let mut rng = StepRng::new(0, 0xDECAFBAD);
        let mut genders: HashMap<String, u16> = HashMap::new();

        for _ in 0..100 {
            let gender = Species::gen_gender(&mut rng);
            *genders.entry(format!("{}", gender)).or_default() += 1;
        }

        assert_eq!(3, genders.len());
        assert_eq!(Some(&2), genders.get("trans (they/them)"));
        assert_eq!(Some(&50), genders.get("feminine (she/her)"));
        assert_eq!(Some(&48), genders.get("masculine (he/him)"));
    }

    #[test]
    fn gen_age_test() {
        let mut rng = StepRng::new(0, 0xDECAFBAD);

        assert_eq!(Age::Infant(0), Species::gen_age(&mut rng));
        assert_eq!(Age::Elderly(69), Species::gen_age(&mut rng));
        assert_eq!(Age::MiddleAged(59), Species::gen_age(&mut rng));
        assert_eq!(Age::MiddleAged(48), Species::gen_age(&mut rng));
        assert_eq!(Age::Adult(38), Species::gen_age(&mut rng));
        assert_eq!(Age::YoungAdult(28), Species::gen_age(&mut rng));
        assert_eq!(Age::Adolescent(17), Species::gen_age(&mut rng));
        assert_eq!(Age::Child(7), Species::gen_age(&mut rng));
        assert_eq!(Age::Geriatric(76), Species::gen_age(&mut rng));
    }

    #[test]
    fn gen_size_male_test() {
        let mut rng = StepRng::new(0, 0xDEADBEEF_DECAFBAD);
        let mut iter = (0u16..=20).map(move |y| {
            let age = Species::age(y);
            let size = Species::gen_size(&mut rng, &age, &Gender::Masculine);
            (y, size.name(), size.height(), size.weight())
        });

        // (age, size, height, weight)
        assert_eq!(Some((0, "tiny", 28, 20)), iter.next());
        assert_eq!(Some((1, "tiny", 33, 25)), iter.next());
        assert_eq!(Some((2, "small", 33, 20)), iter.next());
        assert_eq!(Some((3, "small", 38, 37)), iter.next());
        assert_eq!(Some((4, "small", 39, 33)), iter.next());
        assert_eq!(Some((5, "small", 39, 27)), iter.next());
        assert_eq!(Some((6, "small", 45, 49)), iter.next());
        assert_eq!(Some((7, "small", 45, 41)), iter.next());
        assert_eq!(Some((8, "small", 45, 52)), iter.next());
        assert_eq!(Some((9, "small", 52, 60)), iter.next());
        assert_eq!(Some((10, "medium", 51, 53)), iter.next());
        assert_eq!(Some((11, "medium", 59, 94)), iter.next());
        assert_eq!(Some((12, "medium", 58, 81)), iter.next());
        assert_eq!(Some((13, "medium", 57, 106)), iter.next());
        assert_eq!(Some((14, "medium", 65, 120)), iter.next());
        assert_eq!(Some((15, "medium", 64, 100)), iter.next());
        assert_eq!(Some((16, "medium", 71, 169)), iter.next());
        assert_eq!(Some((17, "medium", 68, 137)), iter.next());
        assert_eq!(Some((18, "medium", 66, 173)), iter.next());
        assert_eq!(Some((19, "medium", 70, 164)), iter.next());
        assert_eq!(Some((20, "medium", 68, 125)), iter.next());

        assert_eq!(None, iter.next());
    }

    #[test]
    fn gen_size_female_test() {
        let mut rng = StepRng::new(0, 0xDEADBEEF_DECAFBAD);
        let mut iter = (0u16..=20).map(move |y| {
            let age = Species::age(y);
            let size = Species::gen_size(&mut rng, &age, &Gender::Feminine);
            (y, size.name(), size.height(), size.weight())
        });

        // (age, size, height, weight)
        assert_eq!(Some((0, "tiny", 28, 20)), iter.next());
        assert_eq!(Some((1, "tiny", 33, 25)), iter.next());
        assert_eq!(Some((2, "small", 33, 20)), iter.next());
        assert_eq!(Some((3, "small", 38, 37)), iter.next());
        assert_eq!(Some((4, "small", 39, 33)), iter.next());
        assert_eq!(Some((5, "small", 39, 27)), iter.next());
        assert_eq!(Some((6, "small", 45, 49)), iter.next());
        assert_eq!(Some((7, "small", 45, 41)), iter.next());
        assert_eq!(Some((8, "small", 45, 52)), iter.next());
        assert_eq!(Some((9, "small", 52, 60)), iter.next());
        assert_eq!(Some((10, "medium", 52, 54)), iter.next());
        assert_eq!(Some((11, "medium", 66, 121)), iter.next());
        assert_eq!(Some((12, "medium", 60, 86)), iter.next());
        assert_eq!(Some((13, "medium", 56, 104)), iter.next());
        assert_eq!(Some((14, "medium", 64, 119)), iter.next());
        assert_eq!(Some((15, "medium", 62, 93)), iter.next());
        assert_eq!(Some((16, "medium", 66, 149)), iter.next());
        assert_eq!(Some((17, "medium", 63, 118)), iter.next());
        assert_eq!(Some((18, "medium", 61, 143)), iter.next());
        assert_eq!(Some((19, "medium", 65, 136)), iter.next());
        assert_eq!(Some((20, "medium", 63, 109)), iter.next());

        assert_eq!(None, iter.next());
    }
}
