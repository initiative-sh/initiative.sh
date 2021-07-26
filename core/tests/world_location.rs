use initiative_core::app;

#[test]
fn results_are_random() {
    assert_ne!(app().command("building"), app().command("building"));
}

#[test]
fn generated_content_is_limited_by_building_type() {
    ["inn", "residence", "shop", "temple", "warehouse"]
        .iter()
        .for_each(|building_type| {
            let output = app().command(building_type);
            let building_type_capitalized: String = building_type
                .char_indices()
                .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
                .collect();

            assert!(
                output.matches(building_type_capitalized.as_str()).count() >= 11,
                "Input: {}\n\nOutput:\n{}",
                building_type,
                output,
            );
        });
}

#[test]
fn generated_content_is_persisted() {
    let mut app = app();
    let generated_output = app.command("inn");

    // # The Roaring Spirit
    // *Inn*
    //
    // Gathering place for a secret society
    //
    // *Alternatives:*\
    // 0 The Lonely Rose, an Inn\
    // 1 The Roaring Star, an Inn\
    // 2 The Howling Spirit, an Inn\
    // 3 The Lonely Dolphin, an Inn\
    // 4 The Prancing Lamb, an Inn\
    // 5 The Leering Star, an Inn\
    // 6 The Staggering Pegasus, an Inn\
    // 7 The Prancing Horde, an Inn\
    // 8 The Black Star, an Inn\
    // 9 The Prancing Pegasus, an Inn

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
            .filter(|line| line.starts_with(char::is_numeric))
            .map(|s| {
                if let Some(pos) = s.find(',') {
                    let name = &s[2..pos];
                    assert_eq!(
                        Some(format!("# {}", name).as_str()),
                        app.command(name).lines().next(),
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
