use crate::common::{get_name, sync_app, sync_app_with_data_store};
use initiative_core::NullDataStore;

#[test]
fn results_are_random() {
    assert_ne!(
        sync_app().command("npc").unwrap(),
        sync_app().command("npc").unwrap(),
    );

    assert_ne!(
        sync_app().command("inn").unwrap(),
        sync_app().command("inn").unwrap(),
    );
}

#[test]
fn save_alias() {
    let mut app = sync_app();

    let name = get_name(&app.command("npc").unwrap());

    assert_eq!(
        Ok(format!(
            "{} was successfully saved. Use `undo` to reverse this.",
            name,
        )),
        app.command("save"),
    );
}

#[test]
fn save_alias_exists_with_invalid_data_store() {
    let mut app = sync_app_with_data_store(NullDataStore::default());

    let name = get_name(&app.command("npc").unwrap());

    assert_eq!(
        Ok(format!(
            "{} was successfully saved. Use `undo` to reverse this.\n\n! Your browser does not support local storage. Any changes will not persist beyond this session.",
            name,
        )),
        app.command("save"),
    );
}

#[test]
fn create_npc_with_custom_attributes() {
    let mut app = sync_app();

    {
        let output = app.command("Sue, a young enby dwarvish elf").unwrap();
        assert!(
            output.contains("# Sue\n*young adult elf, they/them*"),
            "{}",
            output,
        );
        assert!(
            output.ends_with("_Because you specified a name, Sue has been automatically added to your `journal`. Use `undo` to remove them._"),
            "{}",
            output,
        );
        assert!(!output.contains("has not yet been saved"), "{}", output);
        assert!(!output.contains("Alternatives"), "{}", output);
    }

    {
        let output = app.command("journal").unwrap();
        assert!(output.contains("Sue"), "{}", output);
    }

    {
        let output = app.command("a boy named sue").unwrap_err();
        assert_eq!(
            "That name is already in use by 🧑 `Sue` (young adult elf, they/them).",
            output,
        );
    }

    assert_eq!(
        "Successfully undid creating Sue. Use `redo` to reverse this.",
        app.command("undo").unwrap(),
    );

    {
        let output = app.command("journal").unwrap();
        assert!(output.contains("empty"), "{}", output);
    }

    {
        let output = app.command("redo").unwrap();
        assert!(output.contains("# Sue"), "{}", output);
        assert!(
            output.ends_with("_Successfully redid creating Sue. Use `undo` to reverse this._"),
            "{}",
            output,
        );
    }

    {
        let output = app.command("journal").unwrap();
        assert!(output.contains("Sue"), "{}", output);
    }
}

#[test]
fn create_place_with_custom_attributes() {
    let mut app = sync_app();

    {
        let output = app.command("an inn called The Prancing Pony").unwrap();
        assert!(output.contains("# The Prancing Pony\n*inn*"), "{}", output);
        assert!(
            output.contains("has been automatically added to your `journal`."),
            "{}",
            output,
        );
        assert!(!output.contains("has not yet been saved"), "{}", output);
        assert!(!output.contains("Alternatives"), "{}", output);
    }

    {
        let output = app.command("journal").unwrap();
        assert!(output.contains("The Prancing Pony"), "{}", output);
    }

    {
        let output = app.command("a place named the prancing pony").unwrap_err();
        assert_eq!(
            "That name is already in use by 🏨 `The Prancing Pony` (inn).",
            output,
        );
    }
}

#[test]
fn create_with_unknown_words() {
    let mut app = sync_app();

    {
        let output = app
            .command("a male dragon turtle young adult named Smaug")
            .unwrap();

        assert!(output.contains("# Smaug"), "{}", output);
        assert!(output.contains("he/him"), "{}", output);
        assert!(
            output.ends_with(
                "! initiative.sh doesn't know some of those words, but it did its best.\n\
                \n\
                \\> a male **dragon** **turtle** young adult named Smaug\\\n\
                \u{a0}\u{a0}\u{a0}\u{a0}\u{a0}\u{a0}\u{a0}\u{a0}\u{a0}^^^^^^\u{a0}^^^^^^\\\n\
                Want to help improve its vocabulary? Join us [on Discord](https://discord.gg/ZrqJPpxXVZ) and suggest your new words!"
            ),
            "{}",
            output,
        );
    }
}

#[test]
fn generate_location_with_no_name_generator() {
    let mut app = sync_app();

    assert_eq!(
        "There is no name generator implemented for that type. You must specify your own name using `kingdom named [name]`.",
        app.command("kingdom").unwrap_err(),
    );

    {
        let output = app.command("kingdom named Narnia").unwrap();
        assert!(output.contains("# Narnia"), "{}", output);
    }

    {
        let output = app.command("Narnia").unwrap();
        assert!(output.contains("# Narnia"), "{}", output);
    }
}
