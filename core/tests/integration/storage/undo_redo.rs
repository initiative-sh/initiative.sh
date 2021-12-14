use crate::common::{sync_app, SyncApp};
use std::borrow::Cow;

fn undo_redo_test(
    app: &mut SyncApp,
    test: &str,
    change: &str,
    expect_undo_autocomplete: &str,
    expect_undo_message: &str,
    expect_redo_autocomplete: &str,
    expect_redo_message: &str,
) -> [Result<String, String>; 4] {
    let a = app.command(test);
    app.command(change).unwrap();
    let b = app.command(test);

    {
        assert_eq!(
            Some(&(Cow::from("undo"), Cow::from(expect_undo_autocomplete))),
            app.autocomplete("undo").first(),
        );

        let output = app.command("undo").unwrap();
        assert_eq!(
            expect_undo_message,
            output.lines().last().unwrap().trim_matches('_'),
            "{}",
            output,
        );
    }

    let c = app.command(test);

    {
        assert_eq!(
            Some(&(Cow::from("redo"), Cow::from(expect_redo_autocomplete))),
            app.autocomplete("redo").first(),
        );

        let output = app.command("redo").unwrap();
        assert_eq!(
            expect_redo_message,
            output.lines().last().unwrap().trim_matches('_'),
            "{}",
            output,
        );
    }

    let d = app.command(test);

    [a, b, c, d]
}

#[test]
fn nothing() {
    assert_eq!("Nothing to undo.", sync_app().command("undo").unwrap_err());
    assert_eq!(
        Some(&(Cow::from("undo"), Cow::from("Nothing to undo."))),
        sync_app().autocomplete("undo").first(),
    );

    assert_eq!("Nothing to redo.", sync_app().command("redo").unwrap_err());
    assert_eq!(
        Some(&(Cow::from("redo"), Cow::from("Nothing to redo."))),
        sync_app().autocomplete("redo").first(),
    );
}

#[test]
fn create_and_save() {
    let [a, b, c, d] = undo_redo_test(
        &mut sync_app(),
        "load Potato Johnson",
        "character named Potato Johnson",
        "undo creating Potato Johnson",
        "Successfully undid creating Potato Johnson. Use `redo` to reverse this.",
        "redo creating Potato Johnson",
        "Successfully redid creating Potato Johnson. Use `undo` to reverse this.",
    );
    assert_ne!(a, b);
    assert_eq!(a, c);
    assert_eq!(b, d);
}

#[test]
fn delete() {
    let mut app = sync_app();
    app.command("character named Potato Johnson").unwrap();

    let [a, b, c, d] = undo_redo_test(
        &mut app,
        "load Potato Johnson",
        "delete Potato Johnson",
        "undo deleting Potato Johnson",
        "Successfully undid deleting Potato Johnson. Use `redo` to reverse this.",
        "redo deleting Potato Johnson",
        "Successfully redid deleting Potato Johnson. Use `undo` to reverse this.",
    );
    assert_ne!(a, b);
    assert_eq!(a, c);
    assert_eq!(b, d);
}

#[test]
fn edit() {
    let mut app = sync_app();
    app.command("human named Potato Johnson").unwrap();

    let [a, b, c, d] = undo_redo_test(
        &mut app,
        "load Potato Johnson",
        "Potato Johnson is an elf",
        "undo editing Potato Johnson",
        "Successfully undid editing Potato Johnson. Use `redo` to reverse this.",
        "redo editing Potato Johnson",
        "Successfully redid editing Potato Johnson. Use `undo` to reverse this.",
    );
    assert_ne!(a, b);
    assert_eq!(a, c);
    assert_eq!(b, d);
}

#[test]
fn set_key_value() {
    let [a, b, c, d] = undo_redo_test(
        &mut sync_app(),
        "now",
        "+1d",
        "undo changing the time",
        "Successfully undid changing the time. Use `redo` to reverse this.",
        "redo changing the time",
        "Successfully redid changing the time. Use `undo` to reverse this.",
    );
    assert_ne!(a, b);
    assert_eq!(a, c);
    assert_eq!(b, d);
}
