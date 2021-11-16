use crate::common::{sync_app, sync_app_with_data_store};
use initiative_core::MemoryDataStore;

#[test]
fn time_is_initialized() {
    let mut app = sync_app();
    let result = app.command("now").unwrap();

    assert_eq!("It is currently day 1 at 8:00:00 am.", result);
    assert_eq!(result, app.command("time").unwrap());
    assert_eq!(result, app.command("date").unwrap());
}

#[test]
fn time_can_be_changed() {
    let mut app = sync_app();

    assert_eq!(
        "It is now day 3 at 8:00:00 am. Use `undo` to reverse.",
        app.command("+2d").unwrap(),
    );

    assert_eq!(
        "It is now day 4 at 8:00:00 am. Use `undo` to reverse.",
        app.command("+1d").unwrap(),
    );

    assert_eq!(
        "Successfully undid changing the time. Use `redo` to reverse this.",
        app.command("undo").unwrap(),
    );

    assert_eq!(
        "It is currently day 3 at 8:00:00 am.",
        app.command("now").unwrap(),
    );

    assert_eq!(
        "Successfully redid changing the time. Use `undo` to reverse this.",
        app.command("redo").unwrap(),
    );

    assert_eq!(
        "It is currently day 4 at 8:00:00 am.",
        app.command("now").unwrap(),
    );
}

#[test]
fn time_is_persisted() {
    let data_store = MemoryDataStore::default();

    {
        let mut app = sync_app_with_data_store(data_store.clone());
        assert_eq!(
            "It is currently day 1 at 8:00:00 am.",
            app.command("now").unwrap(),
        );
        assert_eq!(
            "It is now day 2 at 10:03:04 am. Use `undo` to reverse.",
            app.command("+1d2h3m4s").unwrap(),
        );
    }

    {
        let mut app = sync_app_with_data_store(data_store.clone());
        assert_eq!(
            "It is currently day 2 at 10:03:04 am.",
            app.command("now").unwrap(),
        );
    }
}
