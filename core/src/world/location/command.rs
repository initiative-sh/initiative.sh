use super::{Field, Generate, Location, LocationType};
use crate::app::AppMeta;

pub fn command(location_type: &LocationType, app_meta: &mut AppMeta) -> String {
    let location = Location {
        subtype: Field::Locked(*location_type),
        ..Default::default()
    };

    let mut output = String::new();

    {
        let mut location = location.clone();
        location.regenerate(&mut app_meta.rng, &app_meta.demographics);
        output.push_str(&format!(
            "{}\n\n*Alternatives:* ",
            location.display_details(),
        ));
        app_meta.push_recent(location.into());
    }

    let recent = (0..10)
        .map(|i| {
            let mut location = location.clone();
            location.regenerate(&mut app_meta.rng, &app_meta.demographics);
            output.push_str(&format!("\\\n{} {}", i, location.display_summary()));
            location.into()
        })
        .collect();

    app_meta.batch_push_recent(recent);

    output
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::storage::NullDataStore;
    use crate::world::location::BuildingType;
    use crate::world::Thing;
    use rand::prelude::*;
    use std::collections::HashMap;

    #[test]
    fn any_building_test() {
        let mut app_meta = AppMeta::new(NullDataStore::default());
        app_meta.rng = SmallRng::seed_from_u64(0);
        let mut results: HashMap<_, u8> = HashMap::new();

        command(&LocationType::Building(None), &mut app_meta);

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
        let mut app_meta = AppMeta::new(NullDataStore::default());
        app_meta.rng = SmallRng::seed_from_u64(0);

        command(
            &LocationType::Building(Some(BuildingType::Inn)),
            &mut app_meta,
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
