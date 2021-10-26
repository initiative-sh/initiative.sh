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
        assert!(output.contains("was successfully saved."), "{}", output);
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
        "The journal is not supported by your browser.",
        app.command(&format!("save {}", npc_name)).unwrap_err(),
    );

    // The NPC should still be in the recent store.
    assert_eq!(
        format!("# {}", npc_name),
        app.command(&format!("load {}", npc_name))
            .unwrap()
            .lines()
            .next()
            .unwrap(),
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
fn npc_can_be_deleted_from_temp() {
    let mut app = sync_app();

    let generated_output = app.command("npc").unwrap();
    let npc_name = generated_output
        .lines()
        .next()
        .unwrap()
        .trim_start_matches("# ")
        .to_string();

    assert_eq!(
        format!(
            "{} was successfully deleted. Use `undo` to reverse this.",
            npc_name,
        ),
        app.command(&format!("delete {}", npc_name)).unwrap(),
    );

    assert_eq!(
        format!("There is no entity named \"{}\".", npc_name),
        app.command(&format!("delete {}", npc_name)).unwrap_err(),
    );

    assert_eq!(
        format!(
            "Successfully undid deleting {}. Use ~redo~ to reverse this.",
            npc_name,
        ),
        app.command("undo").unwrap(),
    );

    assert_eq!(
        format!(
            "{} was successfully deleted. Use `undo` to reverse this.",
            npc_name
        ),
        app.command("redo").unwrap(),
    );
}

#[test]
fn npc_can_be_deleted_from_data_store() {
    let mut app = sync_app();

    let generated_output = app.command("male character named Potato Johnson").unwrap();

    assert!(
        generated_output.ends_with("\n\n_Because you specified a name, Potato Johnson has been automatically added to your `journal`. Use `undo` to remove him._"),
        "{}",
        generated_output,
    );

    assert_eq!(
        "Potato Johnson was successfully deleted. Use `undo` to reverse this.",
        app.command("delete Potato Johnson").unwrap(),
    );

    assert_eq!(
        "There is no entity named \"Potato Johnson\".",
        app.command("delete Potato Johnson").unwrap_err(),
    );

    assert_eq!(
        "Successfully undid deleting Potato Johnson. Use ~redo~ to reverse this.",
        app.command("undo").unwrap(),
    );

    assert_eq!(
        "Potato Johnson was successfully deleted. Use `undo` to reverse this.",
        app.command("redo").unwrap(),
    );
}

#[test]
fn delete_works_with_unusable_data_store() {
    let mut app = sync_app_with_data_store(NullDataStore::default());

    app.command("npc named Potato Johnson").unwrap();

    assert_eq!(
        "Potato Johnson was successfully deleted. Use `undo` to reverse this.",
        app.command("delete Potato Johnson").unwrap(),
    );

    assert_eq!(
        "Successfully undid deleting Potato Johnson. Use ~redo~ to reverse this.",
        app.command("undo").unwrap(),
    );

    assert_eq!(
        "Potato Johnson was successfully deleted. Use `undo` to reverse this.",
        app.command("redo").unwrap(),
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
    println!("{}", npc_list);
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
    println!("{}", inn_list);
    let mut inns: Vec<&str> = inn_list
        .lines()
        .skip_while(|s| !s.starts_with('~'))
        .map(|s| s[4..].trim_end_matches('\\'))
        .map(|line| {
            println!(
                "{}",
                app.command(&format!(
                    "save {}",
                    line.find(',').map(|pos| &line[1..pos - 1]).unwrap()
                ))
                .unwrap(),
            );
            line
        })
        .collect();

    inns.sort();

    let output = app.command("journal").unwrap();
    println!("{}", output);
    let mut output_iter = output.lines();

    assert_eq!(Some("# Journal"), output_iter.next());
    assert_eq!(Some(""), output_iter.next());
    assert_eq!(Some("## NPCs"), output_iter.next());

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

    assert_eq!(Some(""), output_iter.next());
    assert_eq!(Some("## Locations"), output_iter.next());

    inns.drain(..)
        .zip(output_iter.by_ref())
        .enumerate()
        .for_each(|(i, (a, b))| {
            if i == 9 {
                assert_eq!(a, b)
            } else {
                assert_eq!(format!("{}\\", a), b)
            }
        });

    assert!(output_iter.next().is_none());
}
