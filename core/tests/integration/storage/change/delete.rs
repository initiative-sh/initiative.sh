use crate::common::{sync_app, sync_app_with_data_store};
use initiative_core::NullDataStore;

#[test]
fn npc_can_be_deleted_from_temp() {
    let mut app = sync_app();

    let generated_output = app.command("npc").unwrap();
    let npc_name = generated_output
        .lines()
        .nth(2)
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

    {
        let output = app.command("undo").unwrap();
        assert!(output.contains(&format!("# {}", npc_name)), "{}", output);
        assert!(
            output.ends_with(&format!(
                "_Successfully undid deleting {}. Use `redo` to reverse this._",
                npc_name,
            )),
            "{}",
            output,
        );
    }

    assert_eq!(
        format!(
            "Successfully redid deleting {}. Use `undo` to reverse this.",
            npc_name,
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

    {
        let output = app.command("undo").unwrap();
        assert!(output.contains("# Potato Johnson"), "{}", output);
        assert!(
            output.ends_with(
                "_Successfully undid deleting Potato Johnson. Use `redo` to reverse this._"
            ),
            "{}",
            output,
        );
    }

    assert_eq!(
        "Successfully redid deleting Potato Johnson. Use `undo` to reverse this.",
        app.command("redo").unwrap(),
    );
}

#[test]
fn delete_works_with_unusable_data_store() {
    let mut app = sync_app_with_data_store(NullDataStore);

    app.command("npc named Potato Johnson").unwrap();

    assert_eq!(
        "Potato Johnson was successfully deleted. Use `undo` to reverse this.\n\n! Your browser does not support local storage. Any changes will not persist beyond this session.",
        app.command("delete Potato Johnson").unwrap(),
    );

    {
        let output = app.command("undo").unwrap();
        assert!(output.contains("# Potato Johnson"), "{}", output);
        assert!(
            output.ends_with(
                "_Successfully undid deleting Potato Johnson. Use `redo` to reverse this._\n\n! Your browser does not support local storage. Any changes will not persist beyond this session."
            ),
            "{}",
            output,
        );
    }

    assert_eq!(
        "Successfully redid deleting Potato Johnson. Use `undo` to reverse this.\n\n! Your browser does not support local storage. Any changes will not persist beyond this session.",
        app.command("redo").unwrap(),
    );
}
