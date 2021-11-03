mod common;

use common::sync_app;

#[test]
fn about() {
    let output = sync_app().command("about").unwrap();
    assert!(output.contains("initiative.sh"), "{}", output);
}

#[test]
fn autocomplete_command() {
    assert_eq!(
        [
            ("date", "get the current time"),
            ("delete [name]", "remove an entry from journal"),
            ("desert", "create desert"),
            ("distillery", "create distillery"),
            ("district", "create district"),
            ("domain", "create domain"),
            ("dragonborn", "create dragonborn"),
            ("druidic foci", "SRD item category"),
            ("duchy", "create duchy"),
            ("duty-house", "create duty-house"),
            ("dwarf", "create dwarf"),
            ("dwarvish", "create dwarvish person"),
        ]
        .iter()
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect::<Vec<_>>(),
        sync_app().autocomplete("d"),
    );

    assert_eq!(
        Vec::<(String, String)>::new(),
        sync_app().autocomplete("potato")
    )
}

#[test]
fn autocomplete_proper_noun() {
    let mut app = sync_app();
    let output = app.command("npc").unwrap();
    let mut output_iter = output.lines();
    let npc_name = output_iter.nth(2).unwrap().trim_start_matches("# ");
    let npc_description = output_iter.next().unwrap().trim_matches('*');
    let query = npc_name.split_whitespace().next().unwrap();
    let autocomplete_results = app.autocomplete(query);

    assert!(
        autocomplete_results.contains(&(
            npc_name.to_string(),
            format!("{} (unsaved)", npc_description)
        )),
        "Generator output:\n{}\n\nQuery: {}\nResults: {:?}",
        output,
        query,
        autocomplete_results,
    );
}

#[test]
fn debug() {
    let mut app = sync_app();

    let empty_output = app.command("debug").unwrap();
    assert!(empty_output.starts_with("AppMeta { "), "{}", empty_output);

    app.command("npc").unwrap();

    let populated_output = app.command("debug").unwrap();
    assert!(
        populated_output.len() > empty_output.len(),
        "Empty:\n{}\n\nPopulated:\n{}",
        empty_output,
        populated_output,
    );
}

#[test]
fn help() {
    let output = sync_app().command("help").unwrap();
    assert!(output.contains("command"), "{}", output);
}

#[test]
fn init() {
    let output = sync_app().init();
    assert!(output.contains("initiative.sh"), "{}", output);
    assert!(output.contains("changelog"), "{}", output);
    assert!(output.contains("\n* "), "{}", output);
}

#[test]
fn roll() {
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

#[test]
fn unknown() {
    assert_eq!(
        "Unknown command: \"blah blah\"",
        sync_app().command("blah blah").unwrap_err(),
    );
}

#[test]
#[ignore]
fn command_conflict() {
    todo!();
}

#[test]
fn command_conflict_ambiguous() {
    assert_eq!(
        "There are several possible interpretations of this command. Did you mean:\n\n* `srd item Shield`\n* `srd spell Shield`",
        sync_app().command("Shield").unwrap_err(),
    );
}

#[test]
fn command_conflict_other_meanings() {
    let mut app = sync_app();

    app.command("character named Open Game License").unwrap();

    let output = app.command("Open Game License").unwrap();
    assert!(
        output.starts_with("# Open Game License Version"),
        "{}",
        output,
    );
    assert!(
        output.ends_with("\n\n! There are other possible interpretations of this command. Did you mean:\n\n* `load Open Game License`"),
        "{}",
        output,
    );
}
