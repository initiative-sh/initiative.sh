use initiative_core::app;

#[test]
fn autocomplete() {
    assert_eq!(
        vec!["debug", "dragonborn", "dwarf"],
        app().autocomplete("d"),
    );

    assert_eq!(Vec::<String>::new(), app().autocomplete("potato"))
}

#[test]
fn debug() {
    let mut app = app();

    let empty_output = format!("{}", app.command("debug"));
    assert!(empty_output.starts_with("Context { "), "{}", empty_output);

    app.command("npc");

    let populated_output = format!("{}", app.command("debug"));
    assert!(
        populated_output.len() > empty_output.len(),
        "Empty:\n{}\n\nPopulated:\n{}",
        empty_output,
        populated_output,
    );
}

#[test]
fn unknown() {
    assert_eq!(
        "Unknown command: \"blah blah\"",
        format!("{}", app().command("blah blah")).as_str()
    );
}
