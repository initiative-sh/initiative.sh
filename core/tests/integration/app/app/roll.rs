use crate::common::sync_app;

#[test]
fn it_works() {
    let mut app = sync_app();

    let output = app.command("roll d1").unwrap();
    assert_eq!("[1] = **1**", output);

    let output = app.command("(d1)^2").unwrap();
    assert_eq!("[1] = **1**\\\n[1] = **1**", output);

    let output = app.command("roll banana").unwrap_err();
    assert_eq!(
        "\"banana\" is not a valid dice formula. See `help` for some examples.",
        output,
    );

    assert_ne!(app.command("roll 100d1000"), app.command("roll 100d1000"));
}
