mod common;

use common::{sync_app, sync_app_with_data_store, MemoryDataStore};
use initiative_core::NullDataStore;

#[test]
fn npc_is_saved_to_storage() {
    let data_store = MemoryDataStore::default();
    let mut app = sync_app_with_data_store(data_store.clone());

    let generated_output = app.command("npc").unwrap();
    let npc_name = generated_output
        .lines()
        .next()
        .unwrap()
        .trim_start_matches("# ");

    {
        let output = app.command(&format!("save {}", npc_name)).unwrap();
        let start = format!("Saving `{}`", npc_name);
        assert!(
            output.starts_with(&start),
            "\"{}\" should start with \"{}\"",
            output,
            start,
        );
    }

    let things = data_store.things.borrow();
    assert_eq!(1, things.len());
    assert_eq!(npc_name, things.first().unwrap().name().value().unwrap());
}

#[test]
fn npc_is_saved_to_storage_by_alias() {
    let data_store = MemoryDataStore::default();
    let mut app = sync_app_with_data_store(data_store.clone());

    let generated_output = app.command("npc").unwrap();
    let npc_name = generated_output
        .lines()
        .next()
        .unwrap()
        .trim_start_matches("# ");

    app.command("save").unwrap();

    let things = data_store.things.borrow();
    assert_eq!(1, things.len());
    assert_eq!(npc_name, things.first().unwrap().name().value().unwrap());
}

#[test]
fn npc_cannot_be_saved_with_invalid_data_store() {
    let mut app = sync_app_with_data_store(NullDataStore::default());

    let generated_output = app.command("npc").unwrap();
    let npc_name = generated_output
        .lines()
        .next()
        .unwrap()
        .trim_start_matches("# ");

    assert_eq!(
        format!("Couldn't save `{}`", npc_name),
        app.command(&format!("save {}", npc_name)).unwrap_err(),
    );
}

#[test]
fn npc_cannot_be_saved_by_alias_with_invalid_data_store() {
    let mut app = sync_app_with_data_store(NullDataStore::default());

    let generated_output = app.command("npc").unwrap();
    let npc_name = generated_output
        .lines()
        .next()
        .unwrap()
        .trim_start_matches("# ");

    let output = app.command(&npc_name).unwrap();

    assert!(!output.contains("has not yet been saved"), "{}", output);
    assert_eq!(
        "Unknown command: \"save\"",
        app.command("save").unwrap_err(),
    );
}

#[test]
fn npc_can_be_loaded_from_storage() {
    let data_store = MemoryDataStore::default();

    let (npc_name, npc_output_from_temp) = {
        let mut app = sync_app_with_data_store(data_store.clone());

        let generated_output = app.command("npc").unwrap();
        let npc_name = generated_output
            .lines()
            .next()
            .unwrap()
            .trim_start_matches("# ")
            .to_string();

        let npc_output = app.command(&npc_name).unwrap();

        app.command(&format!("save {}", npc_name)).unwrap();
        (npc_name, npc_output)
    };

    let npc_output_from_data_store = {
        let mut app = sync_app_with_data_store(data_store.clone());
        app.init();
        app.command(&npc_name).unwrap()
    };

    assert!(
        npc_output_from_temp.lines().count() > 1,
        "{}",
        npc_output_from_temp,
    );
    assert!(
        npc_output_from_data_store.lines().count() > 1,
        "{}",
        npc_output_from_data_store,
    );
    assert!(
        npc_output_from_temp.starts_with(&npc_output_from_data_store),
        "{}\n\n{}",
        npc_output_from_temp,
        npc_output_from_data_store,
    );
    assert!(
        npc_output_from_temp.contains(&format!(
            "{} has not yet been saved. Use ~save~ to save ",
            npc_name,
        )) && npc_output_from_temp.contains(" to your `journal`._"),
        "{}",
        npc_output_from_temp,
    );
}

#[test]
fn startup_error_with_unusable_data_store() {
    {
        let mut app = sync_app_with_data_store(NullDataStore::default());
        let output = app.init();
        assert!(
            output.contains("Local storage is not available in your browser."),
            "{}",
            output,
        );
    }

    {
        let mut app = sync_app_with_data_store(MemoryDataStore::default());
        let output = app.init();
        assert!(
            !output.contains("Local storage is not available in your browser."),
            "{}",
            output,
        );
    }
}

#[test]
fn journal_has_empty_error_message() {
    assert_eq!(
        "\
# Journal

*Your journal is currently empty.*",
        sync_app().command("journal").unwrap(),
    );
}

#[test]
fn journal_with_invalid_data_store_shows_error_message() {
    let mut app = sync_app_with_data_store(NullDataStore::default());
    assert_eq!(
        "The journal is not supported by your browser.",
        app.command("journal").unwrap_err(),
    );
}

#[test]
fn journal_shows_alphabetized_results() {
    let mut app = sync_app();

    let npc_list = app.command("npc").unwrap();
    let mut npcs: Vec<&str> = npc_list
        .lines()
        .skip_while(|s| !s.starts_with('~'))
        .map(|s| s[4..].trim_end_matches('\\'))
        .map(|line| {
            println!(
                "{}",
                app.command(&format!(
                    "save {}",
                    line.find('(').map(|pos| &line[1..(pos - 2)]).unwrap()
                ))
                .unwrap(),
            );
            line
        })
        .collect();

    npcs.sort();

    let inn_list = app.command("inn").unwrap();
    let mut inns: Vec<&str> = inn_list
        .lines()
        .skip_while(|s| !s.starts_with(char::is_numeric))
        .map(|s| s[2..].trim_end_matches('\\'))
        .map(|line| {
            println!(
                "{}",
                app.command(&format!(
                    "save {}",
                    line.find(',').map(|pos| &line[..pos]).unwrap()
                ))
                .unwrap(),
            );
            line
        })
        .collect();

    inns.sort();

    let output = app.command("journal").unwrap();
    let mut output_iter = output.lines();

    assert_eq!(Some("# Journal"), output_iter.next(), "{}", output);
    assert_eq!(Some(""), output_iter.next(), "{}", output);
    assert_eq!(Some("## NPCs"), output_iter.next(), "{}", output);

    npcs.drain(..)
        .zip(output_iter.by_ref())
        .enumerate()
        .for_each(|(i, (a, b))| {
            if i == 9 {
                assert_eq!(a, b, "{}", output)
            } else {
                assert_eq!(format!("{}\\", a), b, "{}", output)
            }
        });

    assert_eq!(Some(""), output_iter.next(), "{}", output);
    assert_eq!(Some("## Locations"), output_iter.next(), "{}", output);

    inns.drain(..)
        .zip(output_iter.by_ref())
        .enumerate()
        .for_each(|(i, (a, b))| {
            if i == 9 {
                assert_eq!(a, b, "{}", output)
            } else {
                assert_eq!(format!("{}\\", a), b, "{}", output)
            }
        });

    assert!(output_iter.next().is_none(), "{}", output);
}
