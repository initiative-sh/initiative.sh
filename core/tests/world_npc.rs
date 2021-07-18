use initiative_core::app;

#[test]
fn results_are_random() {
    assert_ne!(
        format!("{}", app().command("npc")),
        format!("{}", app().command("npc")),
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
        "warforged",
    ]
    .iter()
    .for_each(|species| {
        let output = format!("{}", app().command(species));
        assert_eq!(11, output.matches(species).count(), "{}", output);
    });

    [("half elf", "half-elf"), ("half orc", "half-orc")]
        .iter()
        .for_each(|(input, species)| {
            let output = format!("{}", app().command(input));
            assert_eq!(11, output.matches(species).count(), "{}", output);
        });
}

#[test]
fn generated_content_is_persisted() {
    let mut app = app();
    let generated_output = format!("{}", app.command("npc"));

    // Naal Tiltathana
    // Species: half-elf (Half-Elvish)
    // Gender: masculine (he/him)
    // Age: young adult (24 years)
    // Size: 5'10", 132 lbs (medium)
    //
    // Alternatives:
    // 0 Amadi (elderly human, he/him)
    // 1 Daiki (adult half-elf, he/him)
    // 2 Rostislav (young adult human, he/him)
    // 3 Gang (middle-aged human, he/him)
    // 4 Laucian Caerdonel (middle-aged elf, he/him)
    // 5 Philandros (middle-aged human, he/him)
    // 6 Makai (adult half-elf, he/him)
    // 7 Bapoto (elderly human, he/him)
    // 8 Gebhuza (elderly half-elf, he/him)
    // 9 Marguerite (middle-aged human, she/her)

    // Ensure that the primary suggestion matches the generated content.
    let name = generated_output.lines().next().unwrap();
    let persisted_output = format!("{}", app.command(name));
    assert_eq!(Some(name), persisted_output.lines().next());
    assert_eq!(
        5,
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
            .filter(|line| line.starts_with(char::is_numeric))
            .map(|s| {
                if let Some(pos) = s.find('(') {
                    let name = &s[2..(pos - 1)];
                    assert_eq!(Some(name), format!("{}", app.command(name)).lines().next());
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
    let mut app = app();

    // Generate a data set to potentially interfere with the one being tested.
    app.command("npc");

    let generated_output = app.command("npc");

    assert_eq!(
        10,
        generated_output
            .lines()
            .filter(|line| line.starts_with(char::is_numeric))
            .map(|s| {
                if let Some(pos) = s.find('(') {
                    let digit = &s[0..1];
                    let digit_output = app.command(digit);

                    let name = &s[2..(pos - 1)];
                    let name_output = app.command(name);

                    assert_eq!(Some(name), digit_output.lines().next());
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
