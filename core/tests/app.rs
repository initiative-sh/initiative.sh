use initiative_core::app;

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
        "RawCommand { text: \"blah blah\", words: [Unknown(\"blah\"), Unknown(\"blah\")] }",
        format!("{}", app().command("blah blah")).as_str()
    );
}
