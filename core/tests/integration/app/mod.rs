mod app;
mod tutorial;

use crate::common::{get_name, sync_app};
use std::borrow::Cow;

#[test]
fn autocomplete_command() {
    assert_eq!(
        [
            ("Dancing Lights", "SRD spell"),
            ("Darkness", "SRD spell"),
            ("Darkvision", "SRD spell"),
            ("date", "get the current time"),
            ("Daylight", "SRD spell"),
            ("Death Ward", "SRD spell"),
            ("Delayed Blast Fireball", "SRD spell"),
            ("delete [name]", "remove an entry from journal"),
            ("Demiplane", "SRD spell"),
            ("desert", "create desert"),
        ]
        .into_iter()
        .map(|(a, b)| (a.into(), b.into()))
        .collect::<Vec<_>>(),
        sync_app().autocomplete("d"),
    );

    assert_eq!(
        Vec::<(Cow<'static, str>, Cow<'static, str>)>::new(),
        sync_app().autocomplete("potato")
    )
}

#[test]
fn autocomplete_proper_noun() {
    let mut app = sync_app();
    let output = app.command("npc").unwrap();
    let npc_name = get_name(&output);
    let npc_description = output.lines().nth(3).unwrap().trim_matches('*');
    let query = npc_name.split_whitespace().next().unwrap();
    let autocomplete_results = app.autocomplete(query);

    assert!(
        autocomplete_results.contains(&(
            npc_name.to_string().into(),
            format!("{} (unsaved)", npc_description).into(),
        )),
        "Generator output:\n{}\n\nQuery: {}\nResults: {:?}",
        output,
        query,
        autocomplete_results,
    );
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
        sync_app().command("blah blah").unwrap_err(),
    );
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
