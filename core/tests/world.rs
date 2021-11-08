mod common;

use common::{sync_app, sync_app_with_data_store};
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
fn generated_npcs_are_limited_by_species() {
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
        let mut app = sync_app();

        let output = app.command(species).unwrap();
        assert!(
            output.contains(species),
            "Input: {}\n\nOutput:\n{}",
            species,
            output,
        );

        let output = app.command("more").unwrap();
        assert!(
            output.matches(species).count() >= 11,
            "Input: {}\n\nOutput:\n{}",
            species,
            output,
        );

        let output = app.command("more").unwrap();
        assert!(
            output.matches(species).count() >= 11,
            "Input: {}\n\nOutput:\n{}",
            species,
            output,
        );
    });
}

#[test]
fn generated_locations_are_limited_by_place_type() {
    ["inn"].iter().for_each(|place_type| {
        let mut app = sync_app();

        let output = app.command(place_type).unwrap();
        assert!(
            output.contains(place_type),
            "Input: {}\n\nOutput:\n{}",
            place_type,
            output,
        );

        let output = app.command("more").unwrap();
        assert!(
            output.matches(place_type).count() >= 11,
            "Input: {}\n\nOutput:\n{}",
            place_type,
            output,
        );

        let output = app.command("more").unwrap();
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

    // # Sybil
    // *elderly human, she/her*
    //
    // **Species:** human\
    // **Gender:** feminine\
    // **Age:** 64 years\
    // **Size:** 5'7", 112 lbs (medium)
    //
    // _Sybil has not yet been saved. Use ~save~ to save her to your `journal`. For more
    // suggestions, type ~more~._
    let generated_output = app.command("npc").unwrap();

    // Ensure that the primary suggestion matches the generated content.
    let name = generated_output
        .lines()
        .nth(2)
        .unwrap()
        .trim_start_matches("# ");
    let persisted_output = app.command(&format!("load {}", name)).unwrap();
    assert_eq!(
        format!("# {}", name),
        persisted_output.lines().nth(2).unwrap(),
    );
    assert_eq!(
        12,
        generated_output
            .lines()
            .zip(persisted_output.lines())
            .filter(|(generated, _)| !generated.starts_with('_'))
            .map(|(generated, persisted)| assert_eq!(generated, persisted))
            .count(),
        "Generated:\n{}\n\nPersisted:\n{}",
        generated_output,
        persisted_output,
    );
}

#[test]
fn more_alias() {
    let mut app = sync_app();
    let mut output = app.command("npc").unwrap();

    for i in 1..2 {
        assert!(output.contains("~more~"), "Iteration {}\n\n{}", i, output);
        // # Alternative suggestions for "person":
        //
        // ~1~ `Jaya` (middle-aged human, she/her)\
        // ~2~ `Harsha` (half-elf infant, he/him)\
        // ~3~ `Lucan Amakiir` (elderly half-elf, he/him)\
        // ~4~ `Germana` (middle-aged human, she/her)\
        // ~5~ `Akachi` (geriatric human, she/her)\
        // ~6~ `Callie Bigheart` (middle-aged halfling, she/her)\
        // ~7~ `Pratima` (young adult human, she/her)\
        // ~8~ `Laelia` (human infant, she/her)\
        // ~9~ `Pierre` (adult human, he/him)\
        // ~0~ `Mokosh` (middle-aged half-elf, she/her)
        //
        // _For even more suggestions, type ~more~.
        output = app.command("more").unwrap();

        // Ensure that secondary suggestions have also been persisted.
        assert_eq!(
            10,
            output
                .lines()
                .filter(|line| line.starts_with('~'))
                .map(|s| {
                    if let Some(pos) = s.find('(') {
                        let name = &s[10..(pos - 2)];
                        assert_eq!(
                            format!("# {}", name),
                            app.command(&format!("load {}", name))
                                .unwrap()
                                .lines()
                                .nth(2)
                                .unwrap(),
                            "Iteration {}",
                            i,
                        );
                    } else {
                        panic!("Missing ( in \"{}\"", s);
                    }
                })
                .count(),
            "Iteration {}\n\n{}",
            i,
            output,
        );
    }
}

#[test]
fn more_alias_exists_with_invalid_data_store() {
    let mut app = sync_app_with_data_store(NullDataStore::default());

    let output = app.command("npc").unwrap();
    assert!(output.contains("~more~"), "{}", output);
    app.command("more").unwrap();
}

#[test]
fn more_alias_does_not_exist_with_name() {
    {
        let mut app = sync_app();
        let output = app.command("place called Home").unwrap();
        assert!(!output.contains("~more~"), "{}", output);
        app.command("more").unwrap_err();
    }

    {
        let mut app = sync_app_with_data_store(NullDataStore::default());
        let output = app.command("place called Home").unwrap();
        assert!(!output.contains("~more~"), "{}", output);
        app.command("more").unwrap_err();
    }
}

#[test]
fn numeric_aliases() {
    let mut app = sync_app();

    // Generate a data set to potentially interfere with the one being tested.
    app.command("npc").unwrap();
    app.command("npc").unwrap();

    let generated_output = app.command("more").unwrap();

    // Doing this in two steps due to borrowing issues.
    let mut outputs = generated_output
        .lines()
        .filter(|line| line.starts_with('~'))
        .map(|s| {
            if let Some(pos) = s.find('(') {
                let digit = &s[1..2];
                let digit_output = app.command(digit).unwrap();

                let name = &s[10..(pos - 2)];

                assert_eq!(format!("# {}", name), digit_output.lines().nth(2).unwrap());

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
fn save_alias() {
    let mut app = sync_app();

    {
        let output = app.command("npc").unwrap();
        let name = output.lines().nth(2).unwrap().trim_start_matches("# ");

        let output = app.command(&format!("load {}", name)).unwrap();
        assert!(output.contains("has not yet been saved"), "{}", output);
    }

    {
        let output = app.command("npc").unwrap();
        let name = output.lines().nth(2).unwrap().trim_start_matches("# ");

        let output = app.command("save").unwrap();
        assert!(output.contains("was successfully saved."), "{}", output);

        let output = app.command(&format!("load {}", name)).unwrap();
        assert!(!output.contains("has not yet been saved"), "{}", output);
    }
}

#[test]
fn save_alias_does_not_exist_with_invalid_data_store() {
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
        assert!(output.contains("# The Prancing Pony\n*inn*"), "{}", output,);
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
            "That name is already in use by 🏨 `The Prancing Pony`, an inn.",
            output,
        );
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

#[test]
fn generate_location_with_no_name_generator() {
    let mut app = sync_app();

    assert_eq!(
        "The only place name generator currently implemented is `inn`. For other types, you must specify a name using `kingdom named [name]`.",
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
