mod common;

use common::sync_app;

#[test]
fn results_are_random() {
    assert_ne!(sync_app().command("npc"), sync_app().command("npc"));
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
        let output = sync_app().command(species);
        assert_eq!(12, output.matches(species).count(), "{}", output);
    });

    [("half elf", "half-elf"), ("half orc", "half-orc")]
        .iter()
        .for_each(|(input, species)| {
            let output = sync_app().command(input);
            assert_eq!(12, output.matches(species).count(), "{}", output);
        });
}

#[test]
fn generated_content_is_persisted() {
    let mut app = sync_app();
    let generated_output = app.command("npc");

    // # Sybil
    // *elderly human, she/her*
    //
    // **Species:** human\
    // **Gender:** feminine\
    // **Age:** 64 years\
    // **Size:** 5'7", 112 lbs (medium)
    //
    // *Alternatives:* \
    // `0` Mokosh (middle-aged half-elf, she/her)\
    // `1` Jaya (middle-aged human, she/her)\
    // `2` Harsha (half-elf infant, he/him)\
    // `3` Lucan Amakiir (elderly half-elf, he/him)\
    // `4` Germana (middle-aged human, she/her)\
    // `5` Akachi (geriatric human, she/her)\
    // `6` Callie Bigheart (middle-aged halfling, she/her)\
    // `7` Pratima (young adult human, she/her)\
    // `8` Laelia (human infant, she/her)\
    // `9` Pierre (adult human, he/him)

    // Ensure that the primary suggestion matches the generated content.
    let name = generated_output
        .lines()
        .next()
        .unwrap()
        .trim_start_matches("# ");
    let persisted_output = app.command(name);
    assert_eq!(
        Some(format!("# {}", name).as_str()),
        persisted_output.lines().next(),
    );
    assert_eq!(
        7,
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
            .filter(|line| line.starts_with('`'))
            .map(|s| {
                if let Some(pos) = s.find('(') {
                    let name = &s[4..(pos - 1)];
                    assert_eq!(
                        Some(format!("# {}", name).as_str()),
                        app.command(name).lines().next(),
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
    app.command("npc");

    let generated_output = app.command("npc");

    assert_eq!(
        10,
        generated_output
            .lines()
            .filter(|line| line.starts_with('`'))
            .map(|s| {
                if let Some(pos) = s.find('(') {
                    let digit = &s[1..2];
                    let digit_output = app.command(digit);

                    let name = &s[4..(pos - 1)];
                    let name_output = app.command(name);

                    assert_eq!(
                        Some(format!("# {}", name).as_str()),
                        digit_output.lines().next(),
                    );
                    assert_eq!(digit_output, name_output);
                } else {
                    panic!("Missing ( in \"{}\"", s);
                }
            })
            .count(),
        "{}",
        generated_output,
    );
}
