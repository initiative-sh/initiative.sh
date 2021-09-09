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
fn generated_content_is_limited_by_building_type() {
    ["inn"].iter().for_each(|building_type| {
        let output = sync_app().command(building_type).unwrap();

        assert!(
            output.matches(building_type).count() >= 11,
            "Input: {}\n\nOutput:\n{}",
            building_type,
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
fn numeric_aliases_exist_for_locations() {
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
fn save_alias_exists_for_locations() {
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
        assert!(output.contains(&format!("Saving `{}`", name)), "{}", output);

        let output = app.command(&format!("load {}", name)).unwrap();
        assert!(!output.contains("has not yet been saved"), "{}", output);
    }
}

#[test]
fn location_save_alias_does_not_exist_with_invalid_data_store() {
    let mut app = sync_app_with_data_store(NullDataStore::default());

    let output = app.command("building").unwrap();
    assert!(!output.contains("has not yet been saved"), "{}", output);

    assert_eq!(
        "Unknown command: \"save\"",
        app.command("save").unwrap_err(),
    );
}
