mod common;

use common::{sync_app, sync_app_with_data_store};
use initiative_core::NullDataStore;

#[test]
fn results_are_random() {
    assert_ne!(
        sync_app().command("building").unwrap(),
        sync_app().command("building").unwrap(),
    );
}

#[test]
fn generated_content_is_limited_by_place_type() {
    ["inn"].iter().for_each(|place_type| {
        let output = sync_app().command(place_type).unwrap();

        assert!(
            output.matches(place_type).count() >= 11,
            "Input: {}\n\nOutput:\n{}",
            place_type,
            output,
        );
    });
}

#[test]
fn generated_content_is_persisted() {
    let mut app = sync_app();
    let generated_output = app.command("building").unwrap();

    // # The Roaring Spirit
    // *inn*
    //
    // _The Roaring Spirit has not yet been saved. Use ~save~ to save it to your `journal`._
    //
    // *Alternatives:*\
    // ~0~ `The Lonely Rose`, an inn\
    // ~1~ `The Roaring Star`, an inn\
    // ~2~ `The Howling Spirit`, an inn\
    // ~3~ `The Lonely Dolphin`, an inn\
    // ~4~ `The Prancing Lamb`, an inn\
    // ~5~ `The Leering Star`, an inn\
    // ~6~ `The Staggering Pegasus`, an inn\
    // ~7~ `The Prancing Horde`, an inn\
    // ~8~ `The Black Star`, an inn\
    // ~9~ `The Prancing Pegasus`, an inn

    // Ensure that the primary suggestion matches the generated content.
    let name = generated_output
        .lines()
        .next()
        .unwrap()
        .trim_start_matches("# ");
    let persisted_output = app.command(&format!("load {}", name)).unwrap();
    assert_eq!(
        format!("# {}", name),
        persisted_output.lines().next().unwrap(),
    );
    assert_eq!(
        4,
        generated_output
            .lines()
            .zip(persisted_output.lines())
            .map(|(generated, persisted)| assert_eq!(generated, persisted))
            .count(),
        "Generated:\n{}\n\nPersisted:\n{}",
        generated_output,
        persisted_output,
    );

    // Ensure that secondary suggestions have also been persisted.
    assert_eq!(
        10,
        generated_output
            .lines()
            .filter(|line| line.starts_with('~'))
            .map(|s| {
                if let Some(pos) = s.find(',') {
                    let name = &s[5..(pos - 1)];
                    assert_eq!(
                        format!("# {}", name),
                        app.command(&format!("load {}", name))
                            .unwrap()
                            .lines()
                            .next()
                            .unwrap(),
                    );
                } else {
                    panic!("Missing , in \"{}\"", s);
                }
            })
            .count(),
        "{}",
        generated_output,
    );
}

#[test]
fn numeric_aliases_exist_for_places() {
    let mut app = sync_app();

    // Generate a data set to potentially interfere with the one being tested.
    app.command("building").unwrap();

    let generated_output = app.command("building").unwrap();

    // Doing this in two steps due to borrowing issues.
    let mut outputs = generated_output
        .lines()
        .filter(|line| line.starts_with('~'))
        .map(|s| {
            if let Some(pos) = s.find(',') {
                let digit = &s[1..2];
                let digit_output = app.command(digit).unwrap();

                let name = &s[5..(pos - 1)];

                assert_eq!(format!("# {}", name), digit_output.lines().next().unwrap());

                (digit_output, name.to_string())
            } else {
                panic!("Missing , in \"{}\"", s);
            }
        })
        .collect::<Vec<_>>();

    assert_eq!(
        10,
        outputs
            .drain(..)
            .map(|(digit_output, name)| {
                let name_output = app.command(&format!("load {}", name)).unwrap();
                assert_eq!(digit_output, name_output);
            })
            .count(),
        "{}",
        generated_output,
    );
}

#[test]
fn save_alias_exists_for_places() {
    let mut app = sync_app();

    {
        let output = app.command("building").unwrap();
        let name = output.lines().next().unwrap().trim_start_matches("# ");

        let output = app.command(&format!("load {}", name)).unwrap();
        assert!(output.contains("has not yet been saved"), "{}", output);
    }

    {
        let output = app.command("building").unwrap();
        let name = output.lines().next().unwrap().trim_start_matches("# ");

        let output = app.command("save").unwrap();
        assert!(output.contains("was successfully saved."), "{}", output);

        let output = app.command(&format!("load {}", name)).unwrap();
        assert!(!output.contains("has not yet been saved"), "{}", output);
    }
}

#[test]
fn place_save_alias_does_not_exist_with_invalid_data_store() {
    let mut app = sync_app_with_data_store(NullDataStore::default());

    let output = app.command("building").unwrap();
    assert!(!output.contains("has not yet been saved"), "{}", output);

    assert_eq!(
        "Unknown command: \"save\"",
        app.command("save").unwrap_err(),
    );
}

#[test]
fn create_place_with_custom_attributes() {
    let mut app = sync_app();

    {
        let output = app.command("an inn called The Prancing Pony").unwrap();
        assert!(
            output.starts_with("# The Prancing Pony\n*inn*"),
            "{}",
            output,
        );
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
            "That name is already in use by `The Prancing Pony`, an inn.",
            output,
        );
    }
}

#[test]
fn edit_place() {
    let mut app = sync_app();

    app.command("inn named Hotel California").unwrap();

    {
        let output = app
            .command("Hotel California is called Heaven Or Hell")
            .unwrap();
        assert!(output.starts_with("# Heaven Or Hell"), "{}", output);
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
        assert!(output.starts_with("# Heaven Or Hell"), "{}", output);
    }

    {
        let output = app.command("undo").unwrap();
        assert!(output.starts_with("# Hotel California"), "{}", output);
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
        assert!(output.starts_with("# Heaven Or Hell"), "{}", output);
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
fn edit_place_implicitly_saves() {
    let mut app = sync_app();

    let generated_output = app.command("inn").unwrap();

    let name = generated_output
        .lines()
        .next()
        .unwrap()
        .trim_start_matches("# ");

    {
        let output = app.command("journal").unwrap();
        assert!(output.contains("empty"), "{}", output);
    }

    {
        let output = app.command(&format!("{} is called Desire", name)).unwrap();
        assert!(
            output.ends_with(&format!("_{} was successfully edited and automatically saved to your `journal`. Use `undo` to reverse this._", name)),
            "{}",
            output,
        );
    }

    {
        let output = app.command("journal").unwrap();
        assert!(output.contains("Desire"), "{}", output);
    }

    {
        let output = app.command("undo").unwrap();
        assert!(output.starts_with(&format!("# {}", name)), "{}", output);
        assert!(
            output.ends_with(&format!(
                "_Successfully undid editing {}. Use `redo` to reverse this._",
                name,
            )),
            "{}",
            output,
        );
    }

    {
        let output = app.command(&name).unwrap();
        assert!(output.starts_with(&format!("# {}", name)), "{}", output);
        assert!(
            output.ends_with(&format!(
                "_{} has not yet been saved. Use ~save~ to save it to your `journal`._",
                name,
            )),
            "{}",
            output,
        );
    }

    {
        let output = app.command("journal").unwrap();
        assert!(output.contains("empty"), "{}", output);
    }

    {
        let output = app.command("redo").unwrap();
        assert!(output.starts_with("# Desire"), "{}", output);
        assert!(
            output.ends_with("_Successfully redid editing Desire. Use `undo` to reverse this._"),
            "{}",
            output,
        );
    }
}

#[test]
fn edit_place_with_invalid_data_store() {
    let mut app = sync_app_with_data_store(NullDataStore::default());

    app.command("Oaken Mermaid Inn, an inn").unwrap();

    {
        let output = app
            .command("Oaken Mermaid Inn is named I Am Mordenkainen")
            .unwrap();
        assert!(output.starts_with("# I Am Mordenkainen"), "{}", output,);
        assert!(
            output.ends_with(
                "_Oaken Mermaid Inn was successfully edited. Use `undo` to reverse this._"
            ),
            "{}",
            output,
        );
    }

    {
        let output = app.command("I Am Mordenkainen").unwrap();
        assert!(output.starts_with("# I Am Mordenkainen"), "{}", output);
    }

    {
        let output = app.command("undo").unwrap();
        assert!(output.starts_with("# Oaken Mermaid Inn"), "{}", output);
        assert!(
            output.ends_with(
                "_Successfully undid editing Oaken Mermaid Inn. Use `redo` to reverse this._"
            ),
            "{}",
            output,
        );
    }

    {
        let output = app.command("redo").unwrap();
        assert!(output.starts_with("# I Am Mordenkainen"), "{}", output);
        assert!(
            output.ends_with(
                "_Successfully redid editing I Am Mordenkainen. Use `undo` to reverse this._"
            ),
            "{}",
            output,
        );
    }
}

#[test]
fn edit_place_with_wrong_type() {
    let mut app = sync_app();
    app.command("inn named Foo").unwrap();

    assert_eq!(
        "Unknown command: \"Foo is an elf\"",
        app.command("Foo is an elf").unwrap_err(),
    );
}
