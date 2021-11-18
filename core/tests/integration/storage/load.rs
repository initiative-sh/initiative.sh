use crate::common::{get_name, sync_app, sync_app_with_data_store};
use initiative_core::MemoryDataStore;

#[test]
fn npc_can_be_loaded_from_storage() {
    let data_store = MemoryDataStore::default();

    let (npc_name, npc_output_from_temp) = {
        let mut app = sync_app_with_data_store(data_store.clone());

        let npc_name = get_name(&app.command("npc").unwrap());

        let output = app.command(&npc_name).unwrap();

        app.command(&format!("save {}", npc_name)).unwrap();
        (npc_name, output)
    };

    let npc_output_from_data_store = {
        let mut app = sync_app_with_data_store(data_store.clone());
        app.init();
        app.command(&npc_name).unwrap()
    };

    assert!(
        npc_output_from_temp.lines().count() > 1,
        "{}",
        npc_output_from_temp,
    );
    assert!(
        npc_output_from_data_store.lines().count() > 1,
        "{}",
        npc_output_from_data_store,
    );
    assert!(
        npc_output_from_temp.starts_with(&npc_output_from_data_store),
        "{}\n\n{}",
        npc_output_from_temp,
        npc_output_from_data_store,
    );
    assert!(
        npc_output_from_temp.contains(&format!(
            "{} has not yet been saved. Use ~save~ to save ",
            npc_name,
        )) && npc_output_from_temp.contains(" to your `journal`._"),
        "{}",
        npc_output_from_temp,
    );
}

/// TODO: update test coverage to specify locations in-app instead of relying on imports
#[test]
fn npc_can_be_loaded_from_storage_with_location() {
    let mut app = sync_app();
    let backup_data = serde_json::from_str(include_str!("export_import/v2.json")).unwrap();

    app.bulk_import(backup_data).unwrap();

    assert_eq!(
        "<div class=\"thing-box npc\">

# Faman Halin
*middle-aged human, he/him*

**Species:** human\\
**Gender:** masculine\\
**Age:** 49 years\\
**Size:** 5'9\", 189 lbs (medium)\\
**Location:** 🏨 `The Moody Conjurer` (inn)

</div>",
        app.command("Faman Halin").unwrap(),
    );
}
