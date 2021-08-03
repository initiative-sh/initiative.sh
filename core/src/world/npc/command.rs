use super::{Npc, Species};
use crate::app::AppMeta;
use crate::storage::StorageCommand;
use crate::world::Generate;
use rand::Rng;

pub fn command(species: &Option<Species>, app_meta: &mut AppMeta, rng: &mut impl Rng) -> String {
    let demographics = if let Some(species) = species {
        app_meta.demographics.only_species(species)
    } else {
        app_meta.demographics.clone()
    };

    let mut output = String::new();
    let npc = Npc::generate(rng, &demographics);

    output.push_str(&format!("{}\n\n*Alternatives:* ", npc.display_details()));
    app_meta.push_recent(npc.into());

    let recent = (0..10)
        .map(|i| {
            let alt = Npc::generate(rng, &demographics);
            output.push_str(&format!("\\\n`{}` {}", i, alt.display_summary()));
            app_meta.command_aliases.insert(
                i.to_string(),
                StorageCommand::Load {
                    query: alt.name.value().unwrap().to_owned(),
                }
                .into(),
            );
            alt.into()
        })
        .collect();

    app_meta.batch_push_recent(recent);

    output
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::app::AppMeta;
    use crate::world::Thing;
    use rand::rngs::mock::StepRng;
    use std::collections::HashMap;

    #[test]
    fn any_species_test() {
        let mut app_meta = AppMeta::default();
        let mut rng = StepRng::new(0, 0xDEADBEEF_DECAFBAD);
        let mut results: HashMap<_, u8> = HashMap::new();

        command(&None, &mut app_meta, &mut rng);

        app_meta.recent().iter().for_each(|thing| {
            if let Thing::Npc(npc) = thing {
                if let Some(species) = npc.species.value() {
                    *results.entry(species).or_default() += 1;
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
    fn specific_species_test() {
        let mut app_meta = AppMeta::default();
        let mut rng = StepRng::new(0, 0xDEADBEEF_DECAFBAD);

        command(&Some(Species::Human), &mut app_meta, &mut rng);

        assert_eq!(
            11,
            app_meta
                .recent()
                .iter()
                .map(|thing| {
                    if let Thing::Npc(npc) = thing {
                        assert_eq!(Some(&Species::Human), npc.species.value(), "{:?}", app_meta);
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
