use crate::common::sync_app;

#[test]
fn edit_npc() {
    let mut app = sync_app();

    app.command("man named Elvis").unwrap();

    {
        let output = app.command("Elvis is named Joe").unwrap();
        assert!(output.contains("# Joe"), "{}", output);
        assert!(
            output.ends_with("_Elvis was successfully edited. Use `undo` to reverse this._"),
            "{}",
            output,
        );
    }

    {
        let output = app.command("Joe").unwrap();
        assert!(output.contains("# Joe"), "{}", output);
    }

    {
        let output = app.command("undo").unwrap();
        assert!(output.contains("# Elvis"), "{}", output);
        assert!(
            output.ends_with("_Successfully undid editing Elvis. Use `redo` to reverse this._"),
            "{}",
            output,
        );
    }

    app.command("Elvis").unwrap();

    {
        let output = app.command("redo").unwrap();
        assert!(output.contains("# Joe"), "{}", output);
        assert!(
            output.ends_with("_Successfully redid editing Joe. Use `undo` to reverse this._"),
            "{}",
            output,
        );
    }

    app.command("Joe").unwrap();
}

#[test]
fn edit_place() {
    let mut app = sync_app();

    app.command("inn named Hotel California").unwrap();

    {
        let output = app
            .command("Hotel California is called Heaven Or Hell")
            .unwrap();
        assert!(output.contains("# Heaven Or Hell"), "{}", output);
        assert!(
            output.ends_with(
                "_Hotel California was successfully edited. Use `undo` to reverse this._"
            ),
            "{}",
            output,
        );
    }

    {
        let output = app.command("Heaven Or Hell").unwrap();
        assert!(output.contains("# Heaven Or Hell"), "{}", output);
    }

    {
        let output = app.command("undo").unwrap();
        assert!(output.contains("# Hotel California"), "{}", output);
        assert!(
            output.ends_with(
                "_Successfully undid editing Hotel California. Use `redo` to reverse this._"
            ),
            "{}",
            output,
        );
    }

    {
        let output = app.command("redo").unwrap();
        assert!(output.contains("# Heaven Or Hell"), "{}", output);
        assert!(
            output.ends_with(
                "_Successfully redid editing Heaven Or Hell. Use `undo` to reverse this._"
            ),
            "{}",
            output,
        );
    }
}

#[test]
fn edit_implicitly_saves() {
    let mut app = sync_app();

    let output = app.command("elf").unwrap();
    let name = output.lines().nth(2).unwrap().trim_start_matches("# ");

    {
        let output = app.command("journal").unwrap();
        assert!(output.contains("empty"), "{}", output);
    }

    {
        let output = app.command(&format!("{} is human", name)).unwrap();
        assert!(output.contains("**Species:** human"), "{}", output);
        assert!(
            output.ends_with(&format!("_{} was successfully edited and automatically saved to your `journal`. Use `undo` to reverse this._", name)),
            "{}",
            output,
        );
    }

    {
        let output = app.command("journal").unwrap();
        assert!(output.contains(&name), "{}", output);
    }
}

#[test]
fn edit_with_wrong_type() {
    let mut app = sync_app();
    app.command("inn named Place").unwrap();
    app.command("npc named Person").unwrap();

    assert_eq!(
        "There is no character named \"Place\".",
        app.command("Place is an elf").unwrap_err(),
    );
    assert_eq!(
        "There is no place named \"Person\".",
        app.command("Person is an inn").unwrap_err(),
    );

    assert_eq!(
        "There is no character named \"Blah\".",
        app.command("Blah is an elf").unwrap_err(),
    );
    assert_eq!(
        "There is no place named \"Blah\".",
        app.command("Blah is an inn").unwrap_err(),
    );
}

#[test]
fn edit_with_unknown_words() {
    let mut app = sync_app();
    app.command("npc named Spot").unwrap();

    let output = app.command("Spot is a good boy").unwrap();
    assert!(output.contains("# Spot"), "{}", output);
    assert!(
        output.ends_with(
            "! initiative.sh doesn't know some of those words, but it did its best.\n\
            \n\
            \\> Spot is a **good** boy\\\n\
            \u{a0}\u{a0}\u{a0}\u{a0}\u{a0}\u{a0}\u{a0}\u{a0}\u{a0}\u{a0}\u{a0}\u{a0}^^^^\\\n\
            Want to help improve its vocabulary? Join us [on Discord](https://discord.gg/ZrqJPpxXVZ) and suggest your new words!"
        ),
        "{}",
        output,
    );
}
