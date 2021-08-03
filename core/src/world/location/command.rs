use super::{Field, Generate, Location, LocationType};
use crate::app::AppMeta;
use rand::Rng;

pub fn command(location_type: &LocationType, app_meta: &mut AppMeta, rng: &mut impl Rng) -> String {
    let location = Location {
        subtype: Field::Locked(*location_type),
        ..Default::default()
    };

    let mut output = String::new();

    {
        let mut location = location.clone();
        location.regenerate(rng, &app_meta.demographics);
        output.push_str(&format!(
            "{}\n\n*Alternatives:* ",
            location.display_details(),
        ));
        app_meta.push_recent(location.into());
    }

    app_meta.batch_push_recent(
        (0..10)
            .map(|i| {
                let mut location = location.clone();
                location.regenerate(rng, &app_meta.demographics);
                output.push_str(&format!("\\\n{} {}", i, location.display_summary()));
                location.into()
            })
            .collect(),
    );

    output
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::world::location::BuildingType;
    use crate::world::Thing;
    use rand::rngs::mock::StepRng;
    use std::collections::HashMap;

    #[test]
    fn any_building_test() {
        let mut app_meta = AppMeta::default();
        let mut rng = StepRng::new(0, 0xDEADBEEF_DECAFBAD);
        let mut results: HashMap<_, u8> = HashMap::new();

        command(&LocationType::Building(None), &mut app_meta, &mut rng);

        app_meta.recent().iter().for_each(|thing| {
            if let Thing::Location(location) = thing {
                if let Some(location_type) = location.subtype.value() {
                    *results.entry(format!("{}", location_type)).or_default() += 1;
                }
            }
        });

        assert!(results.len() > 1, "{:?}\n{:?}", app_meta, results);
        assert_eq!(
            11,
            results.values().sum::<u8>(),
            "{:?}\n{:?}",
            app_meta,
            results,
        );
    }

    #[test]
    fn specific_building_test() {
        let mut app_meta = AppMeta::default();
        let mut rng = StepRng::new(0, 0xDEADBEEF_DECAFBAD);

        command(
            &LocationType::Building(Some(BuildingType::Inn)),
            &mut app_meta,
            &mut rng,
        );

        assert_eq!(
            11,
            app_meta
                .recent()
                .iter()
                .map(|thing| {
                    if let Thing::Location(location) = thing {
                        assert_eq!(
                            Some(&LocationType::Building(Some(BuildingType::Inn))),
                            location.subtype.value(),
                            "{:?}",
                            app_meta,
                        );
                    } else {
                        panic!("{:?}\n{:?}", thing, app_meta);
                    }
                })
                .count(),
            "{:?}",
            app_meta,
        );
    }
}
