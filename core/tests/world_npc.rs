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
        assert_eq!(12, output.matches(species).count(), "{}", output);
    });

    [("half elf", "half-elf"), ("half orc", "half-orc")]
        .iter()
        .for_each(|(input, species)| {
            let output = sync_app().command(input).unwrap();
            assert_eq!(12, output.matches(species).count(), "{}", output);
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
    let persisted_output = app.command(name).unwrap();
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
                        app.command(name).unwrap().lines().next().unwrap(),
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
                let name_output = app.command(&name).unwrap();
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

        let output = app.command(&name).unwrap();
        assert!(output.contains("has not yet been saved"), "{}", output);
    }

    {
        let output = app.command("npc").unwrap();
        let name = output.lines().next().unwrap().trim_start_matches("# ");

        let output = app.command("save").unwrap();
        assert!(output.contains(&format!("Saving `{}`", name)), "{}", output);

        let output = app.command(&name).unwrap();
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
