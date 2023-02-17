use crate::common::{sync_app, sync_app_with_dispatcher, SyncApp};
use initiative_core::Event;

static mut LAST_EVENT: Option<Event> = None;

fn event_dispatcher(event: Event) {
    unsafe {
        LAST_EVENT = Some(event);
    }
}

fn inspect_journal(app: &mut SyncApp) -> String {
    let mut result = String::new();
    result.push_str("> journal\n\n");
    let journal = app.command("journal").expect(&result);
    result.push_str(&journal);

    for command in journal
        .split('`')
        .skip(1)
        .step_by(2)
        .filter(|s| s != &"export")
    {
        result.push_str("\n\n> ");
        result.push_str(command);
        result.push_str("\n\n");
        result.push_str(&app.command(command).expect(&result));
    }

    result.push_str("\n\n> date\n\n");
    result.push_str(&app.command("date").expect(&result));

    result
}

#[test]
#[ignore]
fn export() {
    let mut app = sync_app_with_dispatcher(&event_dispatcher);
    app.command("inn named Foo").unwrap();
    app.command("npc named Blah").unwrap();
    app.command("+1d").unwrap();
    app.command("export").unwrap();

    let data = unsafe {
        if let Some(Event::Export(data)) = &LAST_EVENT {
            Some(data)
        } else {
            None
        }
    }
    .unwrap();

    assert_eq!(2, data.things.len());

    let data_json = serde_json::to_string(&data).unwrap();

    assert!(
        data_json.starts_with(r#"{"_":"This document is exported from initiative.sh. Please note that this format is currently undocumented and no guarantees of forward compatibility are provided, although a reasonable effort will be made to ensure that older backups can be safely imported.","things":[{"#),
        "{}",
        data_json,
    );

    assert!(data_json.contains(r#""name":"Foo""#), "{}", data_json);
    assert!(data_json.contains(r#""name":"Blah""#), "{}", data_json);

    assert!(
        data_json.ends_with(r#"}],"keyValue":{"time":"2:08:00:00"}}"#),
        "{}",
        data_json,
    );
}

#[test]
fn import_event() {
    let mut app = sync_app_with_dispatcher(&event_dispatcher);
    app.init();

    assert_eq!(
        "The file upload popup should appear momentarily. Please select a compatible JSON file, such as that produced by the `export` command.",
        app.command("import").unwrap(),
    );

    let event = unsafe { &LAST_EVENT };
    assert!(matches!(event, Some(Event::Import)));
}

#[test]
fn export_and_import() {
    let (backup_data, journal_before) = {
        let mut app = sync_app_with_dispatcher(&event_dispatcher);
        app.command("inn named Foo").unwrap();
        app.command("npc named Blah").unwrap();
        app.command("+1d").unwrap();
        app.command("export").unwrap();

        (
            unsafe {
                if let Some(Event::Export(data)) = LAST_EVENT.take() {
                    Some(data)
                } else {
                    None
                }
            }
            .unwrap(),
            inspect_journal(&mut app),
        )
    };

    let journal_after = {
        let mut app = sync_app_with_dispatcher(&event_dispatcher);
        assert_eq!(
            "Places: 1 created \\\nCharacters: 1 created \\\nKey/values: 1 created",
            app.bulk_import(backup_data).unwrap(),
        );
        inspect_journal(&mut app)
    };

    assert_eq!(journal_before, journal_after);
    assert_ne!(journal_before, inspect_journal(&mut sync_app()));
}

/// This is a backwards compatibility test. Do not update the source file.
#[test]
fn bulk_import_v1() {
    let mut app = sync_app();
    let backup_data = serde_json::from_str(include_str!("v1.json")).unwrap();

    assert_eq!(
        "Places: 5 created \\\nCharacters: 5 created \\\nKey/values: 1 created",
        app.bulk_import(backup_data).unwrap(),
    );

    assert_eq!(
        "# Journal

## NPCs
👨 `Faman Halin` (middle-aged human, he/him)\\
👧 `Halynn Mardeka` (adolescent human, she/her)\\
👴 `Losno Khayrysi` (elderly halfling, he/him)\\
👩 `Myrcia Haskyr` (middle-aged human, she/her)\\
👶 `Pino Nesgarth` (halfling infant, he/him)

## Places
🏨 `Book and Soldier` (inn)\\
🏨 `Five Millers` (inn)\\
🏨 `Raven and Fisherman` (inn)\\
🏨 `Ten Ghosts` (inn)\\
🏨 `The Moody Conjurer` (inn)

*To export the contents of your journal, use `export`.*",
        app.command("journal").unwrap(),
    );

    assert_eq!(
        "It is currently day 2 at 8:00:00 am.",
        app.command("time").unwrap(),
    );
}

/// This is a backwards compatibility test. Do not update the source file.
#[test]
fn bulk_import_v2() {
    let mut app = sync_app();
    let backup_data = serde_json::from_str(include_str!("v2.json")).unwrap();

    assert_eq!(
        "Places: 5 created \\\nCharacters: 5 created \\\nKey/values: 1 created",
        app.bulk_import(backup_data).unwrap(),
    );

    assert_eq!(
        "# Journal

## NPCs
👨 `Faman Halin` (middle-aged human, he/him)\\
👧 `Halynn Mardeka` (adolescent human, she/her)\\
👴 `Losno Khayrysi` (elderly halfling, he/him)\\
👩 `Myrcia Haskyr` (middle-aged human, she/her)\\
👶 `Pino Nesgarth` (halfling infant, he/him)

## Places
🏨 `Book and Soldier` (inn)\\
🏨 `Five Millers` (inn)\\
🏨 `Raven and Fisherman` (inn)\\
🏨 `Ten Ghosts` (inn)\\
🏨 `The Moody Conjurer` (inn)

*To export the contents of your journal, use `export`.*",
        app.command("journal").unwrap(),
    );

    assert_eq!(
        "It is currently day 2 at 8:00:00 am.",
        app.command("time").unwrap(),
    );
}
