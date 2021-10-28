mod common;

use common::{sync_app, sync_app_with_data_store};
use initiative_core::NullDataStore;

#[test]
fn results_are_random() {
    assert_ne!(
        sync_app().command("npc").unwrap(),
        sync_app().command("npc").unwrap(),
    );
}

#[test]
fn generated_content_is_limited_by_species() {
    [
        "dragonborn",
        "dwarf",
        "elf",
        "gnome",
        "halfling",
        "half-elf",
        "half-orc",
        "human",
        "tiefling",
    ]
    .iter()
    .for_each(|species| {
        let output = sync_app().command(species).unwrap();
        assert!(output.matches(species).count() >= 12, "{}", output);
    });
}

#[test]
fn generated_content_is_persisted() {
    let mut app = sync_app();
    let generated_output = app.command("npc").unwrap();

    // # Sybil
    // *elderly human, she/her*
    //
    // **Species:** human\
    // **Gender:** feminine\
    // **Age:** 64 years\
    // **Size:** 5'7", 112 lbs (medium)
    //
    // _Sybil has not yet been saved. Use ~save~ to save her to your `journal`._
    //
    // *Alternatives:* \
    // ~0~ `Mokosh` (middle-aged half-elf, she/her)\
    // ~1~ `Jaya` (middle-aged human, she/her)\
    // ~2~ `Harsha` (half-elf infant, he/him)\
    // ~3~ `Lucan Amakiir` (elderly half-elf, he/him)\
    // ~4~ `Germana` (middle-aged human, she/her)\
    // ~5~ `Akachi` (geriatric human, she/her)\
    // ~6~ `Callie Bigheart` (middle-aged halfling, she/her)\
    // ~7~ `Pratima` (young adult human, she/her)\
    // ~8~ `Laelia` (human infant, she/her)\
    // ~9~ `Pierre` (adult human, he/him)

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
        9,
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
                if let Some(pos) = s.find('(') {
                    let name = &s[5..(pos - 2)];
                    assert_eq!(
                        format!("# {}", name),
                        app.command(&format!("load {}", name))
                            .unwrap()
                            .lines()
                            .next()
                            .unwrap(),
                    );
                } else {
                    panic!("Missing ( in \"{}\"", s);
                }
            })
            .count(),
        "{}",
        generated_output,
    );
}

#[test]
fn numeric_aliases_exist_for_npcs() {
    let mut app = sync_app();

    // Generate a data set to potentially interfere with the one being tested.
    app.command("npc").unwrap();

    let generated_output = app.command("npc").unwrap();

    // Doing this in two steps due to borrowing issues.
    let mut outputs = generated_output
        .lines()
        .filter(|line| line.starts_with('~'))
        .map(|s| {
            if let Some(pos) = s.find('(') {
                let digit = &s[1..2];
                let digit_output = app.command(digit).unwrap();

                let name = &s[5..(pos - 2)];

                assert_eq!(format!("# {}", name), digit_output.lines().next().unwrap());

                (digit_output, name.to_string())
            } else {
                panic!("Missing ( in \"{}\"", s);
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
fn save_alias_exists_for_npcs() {
    let mut app = sync_app();

    {
        let output = app.command("npc").unwrap();
        let name = output.lines().next().unwrap().trim_start_matches("# ");

        let output = app.command(&format!("load {}", name)).unwrap();
        assert!(output.contains("has not yet been saved"), "{}", output);
    }

    {
        let output = app.command("npc").unwrap();
        let name = output.lines().next().unwrap().trim_start_matches("# ");

        let output = app.command("save").unwrap();
        assert!(output.contains("was successfully saved."), "{}", output);

        let output = app.command(&format!("load {}", name)).unwrap();
        assert!(!output.contains("has not yet been saved"), "{}", output);
    }
}

#[test]
fn npc_save_alias_does_not_exist_with_invalid_data_store() {
    let mut app = sync_app_with_data_store(NullDataStore::default());

    let output = app.command("npc").unwrap();
    assert!(!output.contains("has not yet been saved"), "{}", output);

    assert_eq!(
        "Unknown command: \"save\"",
        app.command("save").unwrap_err(),
    );
}

#[test]
fn create_npc_with_custom_attributes() {
    let mut app = sync_app();

    {
        let output = app.command("Sue, a young enby dwarvish elf").unwrap();
        assert!(
            output.starts_with("# Sue\n*young adult elf, they/them*"),
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
            "That name is already in use by `Sue` (young adult elf, they/them).",
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

    assert_eq!(
        "Successfully redid creating Sue. Use `undo` to reverse this.",
        app.command("redo").unwrap(),
    );

    {
        let output = app.command("journal").unwrap();
        assert!(output.contains("Sue"), "{}", output);
    }
}

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

    assert_eq!(
        "Successfully undid editing Elvis. Use `redo` to reverse this.",
        app.command("undo").unwrap(),
    );

    app.command("Elvis").unwrap();

    assert_eq!(
        "Successfully redid editing Joe. Use `undo` to reverse this.",
        app.command("redo").unwrap(),
    );

    app.command("Joe").unwrap();
}

#[test]
fn edit_npc_implicitly_saves() {
    let mut app = sync_app();

    let generated_output = app.command("elf").unwrap();

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
        let output = app.command(&format!("{} is human", name)).unwrap();
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

    assert_eq!(
        format!(
            "Successfully undid editing {}. Use `redo` to reverse this.",
            name,
        ),
        app.command("undo").unwrap(),
    );

    {
        let output = app.command(&name).unwrap();
        assert!(output.starts_with(&format!("# {}", name)), "{}", output);
        assert!(
            output.contains(&format!(
                "_{} has not yet been saved. Use ~save~ to save ",
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

    assert_eq!(
        format!(
            "Successfully redid editing {}. Use `undo` to reverse this.",
            name,
        ),
        app.command("redo").unwrap(),
    );

    {
        let output = app.command("journal").unwrap();
        assert!(output.contains(&name), "{}", output);
    }
}

#[test]
fn edit_npc_with_wrong_type() {
    let mut app = sync_app();
    app.command("elf named Foo").unwrap();

    assert_eq!(
        "Unknown command: \"Foo is an inn\"",
        app.command("Foo is an inn").unwrap_err(),
    );
}
