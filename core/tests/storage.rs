mod common;

use common::{sync_app_with_data_store, MemoryDataStore};

#[test]
fn npc_is_saved_to_storage() {
    let data_store = MemoryDataStore::default();
    let mut app = sync_app_with_data_store(data_store.clone());

    let generated_output = app.command("npc");
    let npc_name = generated_output
        .lines()
        .next()
        .unwrap()
        .trim_start_matches("# ");

    app.command(format!("save {}", npc_name).as_str());

    let things = data_store.things.borrow();
    assert_eq!(1, things.len());
    assert_eq!(npc_name, things.first().unwrap().name().value().unwrap());
}

#[test]
fn npc_can_be_loaded_from_storage() {
    let data_store = MemoryDataStore::default();

    let (npc_name, npc_output_from_temp) = {
        let mut app = sync_app_with_data_store(data_store.clone());

        let generated_output = app.command("npc");
        let npc_name = generated_output
            .lines()
            .next()
            .unwrap()
            .trim_start_matches("# ")
            .to_string();

        let npc_output = app.command(&npc_name);

        app.command(&format!("save {}", npc_name));
        (npc_name, npc_output)
    };

    let npc_output_from_data_store = {
        let mut app = sync_app_with_data_store(data_store.clone());
        app.init();
        app.command(&npc_name)
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
        )) && npc_output_from_temp.contains(" to your journal._"),
        "{}",
        npc_output_from_temp,
    );
}
