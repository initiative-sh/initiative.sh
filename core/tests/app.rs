mod common;

use common::sync_app;

#[test]
fn about() {
    let output = sync_app().command("about");
    assert!(output.contains("initiative.sh"), "{}", output);
}

#[test]
fn autocomplete_command() {
    assert_eq!(
        [
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
    let output = app.command("npc");
    let npc_name = output.lines().next().unwrap().trim_start_matches("# ");
    let query = npc_name.split_whitespace().next().unwrap();
    let autocomplete_results = app.autocomplete(query);

    assert!(
        autocomplete_results.contains(&(npc_name.to_string(), "load generated NPC".to_string())),
        "Generator output:\n{}\n\nQuery: {}\nResults: {:?}",
        output,
        query,
        autocomplete_results,
    );
}

#[test]
fn debug() {
    let mut app = sync_app();

    let empty_output = app.command("debug");
    assert!(empty_output.starts_with("AppMeta { "), "{}", empty_output);

    app.command("npc");

    let populated_output = app.command("debug");
    assert!(
        populated_output.len() > empty_output.len(),
        "Empty:\n{}\n\nPopulated:\n{}",
        empty_output,
        populated_output,
    );
}

#[test]
fn help() {
    let output = sync_app().command("help");
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
fn unknown() {
    assert_eq!(
        "Unknown command: \"blah blah\"",
        sync_app().command("blah blah").as_str()
    );
}
