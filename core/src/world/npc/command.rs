use super::{Gender, Npc, Species};
use crate::app::{AppMeta, CommandAlias};
use crate::storage::StorageCommand;
use crate::world::Generate;

pub fn command(species: &Option<Species>, app_meta: &mut AppMeta) -> String {
    let demographics = if let Some(species) = species {
        app_meta.demographics.only_species(species)
    } else {
        app_meta.demographics.clone()
    };

    let mut output = String::new();
    let npc = Npc::generate(&mut app_meta.rng, &demographics);

    output.push_str(&format!(
        "\
{}

_{} has not yet been saved. Use ~save~ to save {} to your journal._

*Alternatives:* ",
        npc.display_details(),
        npc.name,
        npc.gender.value().unwrap_or(&Gender::Trans).them(),
    ));

    if let Some(name) = npc.name.value() {
        app_meta.command_aliases.insert(
            "save".to_string(),
            CommandAlias::new(
                "save".to_string(),
                format!("save {}", npc.name),
                StorageCommand::Save {
                    name: name.to_string(),
                }
                .into(),
            ),
        );
    }

    app_meta.push_recent(npc.into());

    let recent = (0..10)
        .map(|i| {
            let alt = Npc::generate(&mut app_meta.rng, &demographics);
            output.push_str(&format!("\\\n~{}~ {}", i, alt.display_summary()));
            app_meta.command_aliases.insert(
                i.to_string(),
                CommandAlias::new(
                    i.to_string(),
                    format!("load {}", alt.name),
                    StorageCommand::Load {
                        name: alt.name.value().unwrap().to_owned(),
                    }
                    .into(),
                ),
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
    use crate::storage::NullDataStore;
    use crate::world::Thing;
    use rand::prelude::*;
    use std::collections::HashMap;

    #[test]
    fn any_species_test() {
        let mut app_meta = AppMeta::new(NullDataStore::default());
        app_meta.rng = SmallRng::seed_from_u64(0);
        let mut results: HashMap<_, u8> = HashMap::new();

        command(&None, &mut app_meta);

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
        let mut app_meta = AppMeta::new(NullDataStore::default());
        app_meta.rng = SmallRng::seed_from_u64(0);

        command(&Some(Species::Human), &mut app_meta);

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
