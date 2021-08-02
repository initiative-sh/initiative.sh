use super::{Npc, Species};
use crate::app::Context;
use crate::storage::StorageCommand;
use crate::world::Generate;
use rand::Rng;

pub fn command(species: &Option<Species>, context: &mut Context, rng: &mut impl Rng) -> String {
    let demographics = if let Some(species) = species {
        context.demographics.only_species(species)
    } else {
        context.demographics.clone()
    };

    let mut output = String::new();
    let npc = Npc::generate(rng, &demographics);

    output.push_str(&format!("{}\n\n*Alternatives:* ", npc.display_details()));
    context.push_recent(npc.into());

    let recent = (0..10)
        .map(|i| {
            let alt = Npc::generate(rng, &demographics);
            output.push_str(&format!("\\\n`{}` {}", i, alt.display_summary()));
            context.command_aliases.insert(
                i.to_string(),
                StorageCommand::Load {
                    query: alt.name.value().unwrap().to_owned(),
                }
                .into(),
            );
            alt.into()
        })
        .collect();

    context.batch_push_recent(recent);

    output
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::app::Context;
    use crate::world::Thing;
    use rand::rngs::mock::StepRng;
    use std::collections::HashMap;

    #[test]
    fn any_species_test() {
        let mut context = Context::default();
        let mut rng = StepRng::new(0, 0xDEADBEEF_DECAFBAD);
        let mut results: HashMap<_, u8> = HashMap::new();

        command(&None, &mut context, &mut rng);

        context.recent().iter().for_each(|thing| {
            if let Thing::Npc(npc) = thing {
                if let Some(species) = npc.species.value() {
                    *results.entry(species).or_default() += 1;
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
    fn specific_species_test() {
        let mut context = Context::default();
        let mut rng = StepRng::new(0, 0xDEADBEEF_DECAFBAD);

        command(&Some(Species::Human), &mut context, &mut rng);

        assert_eq!(
            11,
            context
                .recent()
                .iter()
                .map(|thing| {
                    if let Thing::Npc(npc) = thing {
                        assert_eq!(Some(&Species::Human), npc.species.value(), "{:?}", context);
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
