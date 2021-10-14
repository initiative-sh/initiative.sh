use super::{Age, Gender, Generate, Size};
use rand::prelude::*;

pub struct Species;

impl Generate for Species {
    fn gen_gender(rng: &mut impl Rng) -> Gender {
        match rng.gen_range(1..=101) {
            1..=50 => Gender::Feminine,
            51..=100 => Gender::Masculine,
            101 => Gender::NonBinaryThey,
            _ => unreachable!(),
        }
    }

    fn gen_age_years(rng: &mut impl Rng) -> u16 {
        rng.gen_range(0..=79)
    }

    fn age_from_years(years: u16) -> Age {
        match years {
            i if i < 2 => Age::Infant,
            i if i < 10 => Age::Child,
            i if i < 20 => Age::Adolescent,
            i if i < 30 => Age::YoungAdult,
            i if i < 40 => Age::Adult,
            i if i < 60 => Age::MiddleAged,
            i if i < 70 => Age::Elderly,
            _ => Age::Geriatric,
        }
    }

    fn gen_size(rng: &mut impl Rng, age_years: u16, gender: &Gender) -> Size {
        let is_female = match gender {
            Gender::Masculine => rng.gen_bool(0.01),
            Gender::Feminine => rng.gen_bool(0.99),
            _ => rng.gen_bool(0.5),
        };

        match (age_years, is_female) {
            (0, _) => {
                let size = rng.gen_range(0..=30);
                Size::Tiny {
                    height: 20 + size / 3,
                    weight: 7 + size / 2,
                }
            }
            (1, _) => {
                let size = rng.gen_range(0..=5);
                Size::Tiny {
                    height: 30 + size,
                    weight: 22 + size,
                }
            }
            (2..=9, _) => {
                let y = (age_years - 2) as f32 / 8.;
                let (height, weight) =
                    super::gen_height_weight(rng, (33. + y * 18.)..=(35. + y * 22.), 14.0..=17.0);
                Size::Small { height, weight }
            }
            (10..=19, true) => {
                let y = (age_years - 10) as f32;
                let (height, weight) = super::gen_height_weight(
                    rng,
                    (51. + y * 2.).min(61.)..=(65. + y * 2.).min(67.),
                    (15. + y * 2.5 / 5.).min(18.5)..=(19. + y * 4.5 / 5.).min(25.),
                );
                Size::Medium { height, weight }
            }
            (10..=19, false) => {
                let y = (age_years - 10) as f32 / 5.;
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
    use std::collections::HashMap;

    #[test]
    fn gen_age_years_test() {
        let mut rng = SmallRng::seed_from_u64(0);

        assert_eq!(
            [35, 35, 78, 36, 71],
            [
                Species::gen_age_years(&mut rng),
                Species::gen_age_years(&mut rng),
                Species::gen_age_years(&mut rng),
                Species::gen_age_years(&mut rng),
                Species::gen_age_years(&mut rng),
            ],
        );
    }

    #[test]
    fn age_from_years_test() {
        assert_eq!(Age::Infant, Species::age_from_years(0));
        assert_eq!(Age::Infant, Species::age_from_years(1));

        assert_eq!(Age::Child, Species::age_from_years(2));
        assert_eq!(Age::Child, Species::age_from_years(9));

        assert_eq!(Age::Adolescent, Species::age_from_years(10));
        assert_eq!(Age::Adolescent, Species::age_from_years(19));

        assert_eq!(Age::YoungAdult, Species::age_from_years(20));
        assert_eq!(Age::YoungAdult, Species::age_from_years(29));

        assert_eq!(Age::Adult, Species::age_from_years(30));
        assert_eq!(Age::Adult, Species::age_from_years(39));

        assert_eq!(Age::MiddleAged, Species::age_from_years(40));
        assert_eq!(Age::MiddleAged, Species::age_from_years(59));

        assert_eq!(Age::Elderly, Species::age_from_years(60));
        assert_eq!(Age::Elderly, Species::age_from_years(69));

        assert_eq!(Age::Geriatric, Species::age_from_years(70));
        assert_eq!(Age::Geriatric, Species::age_from_years(u16::MAX));
    }

    #[test]
    fn gen_gender_test() {
        let mut rng = SmallRng::seed_from_u64(0);
        let mut genders: HashMap<String, u16> = HashMap::new();

        for _ in 0..500 {
            let gender = Species::gen_gender(&mut rng);
            *genders.entry(format!("{}", gender)).or_default() += 1;
        }

        assert_eq!(3, genders.len());
        assert_eq!(Some(&3), genders.get("non-binary (they/them)"));
        assert_eq!(Some(&233), genders.get("feminine (she/her)"));
        assert_eq!(Some(&264), genders.get("masculine (he/him)"));
    }

    #[test]
    fn gen_size_male_test() {
        let mut rng = SmallRng::seed_from_u64(0);

        // (age, size, height, weight)
        assert_eq!(
            vec![
                (0, "tiny", 24, 13),
                (1, "tiny", 32, 24),
                (2, "small", 35, 28),
                (3, "small", 36, 30),
                (4, "small", 41, 39),
                (5, "small", 44, 38),
                (6, "small", 42, 43),
                (7, "small", 47, 44),
                (8, "small", 50, 55),
                (9, "small", 54, 56),
                (10, "medium", 53, 79),
                (11, "medium", 54, 82),
                (12, "medium", 57, 73),
                (13, "medium", 59, 83),
                (14, "medium", 67, 120),
                (15, "medium", 65, 130),
                (16, "medium", 66, 124),
                (17, "medium", 70, 125),
                (18, "medium", 71, 127),
                (19, "medium", 72, 197),
                (20, "medium", 67, 122),
            ],
            (0u16..=20)
                .map(move |y| {
                    let size = Species::gen_size(&mut rng, y, &Gender::Masculine);
                    (y, size.name(), size.height(), size.weight())
                })
                .collect::<Vec<_>>(),
        );
    }

    #[test]
    fn gen_size_female_test() {
        let mut rng = SmallRng::seed_from_u64(0);

        // (age, size, height, weight)
        assert_eq!(
            vec![
                (0, "tiny", 24, 13),
                (1, "tiny", 32, 24),
                (2, "small", 35, 28),
                (3, "small", 36, 30),
                (4, "small", 41, 39),
                (5, "small", 44, 38),
                (6, "small", 42, 43),
                (7, "small", 47, 44),
                (8, "small", 50, 55),
                (9, "small", 54, 56),
                (10, "medium", 56, 91),
                (11, "medium", 55, 87),
                (12, "medium", 58, 75),
                (13, "medium", 59, 82),
                (14, "medium", 67, 119),
                (15, "medium", 63, 122),
                (16, "medium", 62, 108),
                (17, "medium", 65, 108),
                (18, "medium", 66, 111),
                (19, "medium", 67, 160),
                (20, "medium", 62, 107),
            ],
            (0u16..=20)
                .map(move |y| {
                    let size = Species::gen_size(&mut rng, y, &Gender::Feminine);
                    (y, size.name(), size.height(), size.weight())
                })
                .collect::<Vec<_>>(),
        );
    }
}
