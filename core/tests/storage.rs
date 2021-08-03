mod common;

use common::{sync_app_with_data_store, MemoryDataStore};
use initiative_core::DataStore;

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

    let things = data_store.get_all();
    assert_eq!(1, things.len());
    assert_eq!(npc_name, things.first().unwrap().name().value().unwrap());
}
