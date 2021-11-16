use crate::common::sync_app_with_dispatcher;
use initiative_core::Event;

static mut LAST_EVENT: Option<Event> = None;

fn event_dispatcher(event: Event) {
    unsafe {
        LAST_EVENT = Some(event);
    }
}

#[test]
fn export() {
    let mut app = sync_app_with_dispatcher(&event_dispatcher);
    app.command("inn named Foo").unwrap();
    app.command("npc named Bar").unwrap();
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
    assert!(data_json.contains(r#""name":"Bar""#), "{}", data_json);

    assert!(
        data_json.ends_with(r#"}],"keyValue":{"time":"2:08:00:00"}}"#),
        "{}",
        data_json,
    );
}

#[test]
fn import() {
    let mut app = sync_app_with_dispatcher(&event_dispatcher);
    app.init();

    assert_eq!(
        "The file upload popup should appear momentarily. Please select a compatible JSON file, such as that produced by the `export` command.",
        app.command("import").unwrap(),
    );

    let event = unsafe { &LAST_EVENT };
    assert!(matches!(event, Some(Event::Import)));
}
