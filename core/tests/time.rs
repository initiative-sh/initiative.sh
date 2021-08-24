mod common;

use common::sync_app;

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
        "It is now day 3 at 8:00:00 am. Use ~undo~ to reverse.",
        app.command("+2d").unwrap(),
    );
    assert_eq!(
        "It is now day 1 at 8:00:00 am. Use ~undo~ to reverse.",
        app.command("undo").unwrap(),
    );
    assert_eq!(
        "It is now day 3 at 8:00:00 am. Use ~undo~ to reverse.",
        app.command("undo").unwrap(),
    );
    assert_eq!(
        "It is now day 2 at 8:00:00 am. Use ~undo~ to reverse.",
        app.command("-d").unwrap(),
    );
    assert_eq!(
        "It is currently day 2 at 8:00:00 am.",
        app.command("now").unwrap(),
    );
}
