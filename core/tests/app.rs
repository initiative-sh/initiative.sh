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
            ("dragonborn", "generate NPC species"),
            ("druidic foci", "SRD item category"),
            ("dwarf", "generate NPC species"),
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
    let npc_name = output_iter.next().unwrap().trim_start_matches("# ");
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
