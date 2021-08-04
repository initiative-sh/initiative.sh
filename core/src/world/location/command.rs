use super::{Field, Generate, Location, LocationType};
use crate::app::Context;
use rand::Rng;

pub fn command(location_type: &LocationType, context: &mut Context, rng: &mut impl Rng) -> String {
    let location = Location {
        subtype: Field::Locked(*location_type),
        ..Default::default()
    };

    let mut output = String::new();

    {
        let mut location = location.clone();
        location.regenerate(rng, &context.demographics);
        output.push_str(&format!(
            "{}\n\n*Alternatives:* ",
            location.display_details(),
        ));
        context.push_recent(location.into());
    }

    context.batch_push_recent(
        (0..10)
            .map(|i| {
                let mut location = location.clone();
                location.regenerate(rng, &context.demographics);
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
        let mut context = Context::default();
        let mut rng = StepRng::new(0, 0xDEADBEEF_DECAFBAD);
        let mut results: HashMap<_, u8> = HashMap::new();

        command(&LocationType::Building(None), &mut context, &mut rng);

        context.recent().iter().for_each(|thing| {
            if let Thing::Location(location) = thing {
                if let Some(location_type) = location.subtype.value() {
                    *results.entry(format!("{}", location_type)).or_default() += 1;
                }
            }
        });

        assert!(results.len() > 1, "{:?}\n{:?}", context, results);
        assert_eq!(
            11,
            results.values().sum::<u8>(),
            "{:?}\n{:?}",
            context,
            results,
        );
    }

    #[test]
    fn specific_building_test() {
        let mut context = Context::default();
        let mut rng = StepRng::new(0, 0xDEADBEEF_DECAFBAD);

        command(
            &LocationType::Building(Some(BuildingType::Inn)),
            &mut context,
            &mut rng,
        );

        assert_eq!(
            11,
            context
                .recent()
                .iter()
                .map(|thing| {
                    if let Thing::Location(location) = thing {
                        assert_eq!(
                            Some(&LocationType::Building(Some(BuildingType::Inn))),
                            location.subtype.value(),
                            "{:?}",
                            context,
                        );
                    } else {
                        panic!("{:?}\n{:?}", thing, context);
                    }
                })
                .count(),
            "{:?}",
            context,
        );
    }
}
