use crate::common::sync_app_with_data_store;
use initiative_core::{MemoryDataStore, NullDataStore};

#[test]
fn npc_is_saved_to_storage() {
    let data_store = MemoryDataStore::default();
    let mut app = sync_app_with_data_store(data_store.clone());

    let generated_output = app.command("npc").unwrap();
    let npc_name = generated_output
        .lines()
        .nth(2)
        .unwrap()
        .trim_start_matches("# ");

    assert_eq!(
        format!(
            "{} was successfully saved. Use `undo` to reverse this.",
            npc_name,
        ),
        app.command(&format!("save {}", npc_name)).unwrap(),
    );

    let things = data_store.things.borrow();
    assert_eq!(1, things.len());
    assert_eq!(
        npc_name,
        things.values().next().unwrap().name().value().unwrap(),
    );
}

#[test]
fn npc_is_saved_to_storage_by_alias() {
    let data_store = MemoryDataStore::default();
    let mut app = sync_app_with_data_store(data_store.clone());

    let generated_output = app.command("npc").unwrap();
    let npc_name = generated_output
        .lines()
        .nth(2)
        .unwrap()
        .trim_start_matches("# ");

    assert_eq!(
        format!(
            "{} was successfully saved. Use `undo` to reverse this.",
            npc_name,
        ),
        app.command("save").unwrap(),
    );

    let things = data_store.things.borrow();
    assert_eq!(1, things.len());
    assert_eq!(
        npc_name,
        things.values().next().unwrap().name().value().unwrap(),
    );
}

#[test]
fn npc_can_be_saved_with_invalid_data_store() {
    let mut app = sync_app_with_data_store(NullDataStore);

    let generated_output = app.command("npc").unwrap();
    let npc_name = generated_output
        .lines()
        .nth(2)
        .unwrap()
        .trim_start_matches("# ");

    assert_eq!(
        format!(
            "{} was successfully saved. Use `undo` to reverse this.\n\n! Your browser does not support local storage. Any changes will not persist beyond this session.",
            npc_name,
        ),
        app.command(&format!("save {}", npc_name)).unwrap(),
    );

    assert_eq!(
        format!("# {}", npc_name),
        app.command(&format!("load {}", npc_name))
            .unwrap()
            .lines()
            .nth(2)
            .unwrap(),
    );
}

#[test]
fn npc_can_be_saved_by_alias_with_invalid_data_store() {
    let mut app = sync_app_with_data_store(NullDataStore);

    let generated_output = app.command("npc").unwrap();
    let npc_name = generated_output
        .lines()
        .nth(2)
        .unwrap()
        .trim_start_matches("# ");

    assert_eq!(
        format!(
            "{} was successfully saved. Use `undo` to reverse this.\n\n! Your browser does not support local storage. Any changes will not persist beyond this session.",
            npc_name,
        ),
        app.command("save").unwrap(),
    );

    assert_eq!(
        format!("# {}", npc_name),
        app.command(&format!("load {}", npc_name))
            .unwrap()
            .lines()
            .nth(2)
            .unwrap(),
    );
}
