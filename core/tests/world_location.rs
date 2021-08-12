mod common;

use common::sync_app;

#[test]
fn results_are_random() {
    assert_ne!(
        sync_app().command("building"),
        sync_app().command("building")
    );
}

#[test]
fn generated_content_is_limited_by_building_type() {
    ["inn", "residence", "shop", "temple", "warehouse"]
        .iter()
        .for_each(|building_type| {
            let output = sync_app().command(building_type);

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
    let generated_output = app.command("inn");

    // # The Roaring Spirit
    // *inn*
    //
    // _The Roaring Spirit has not yet been saved. Use ~save~ to save it to your journal._
    //
    // *Alternatives:*\
    // 0 The Lonely Rose, an inn\
    // 1 The Roaring Star, an inn\
    // 2 The Howling Spirit, an inn\
    // 3 The Lonely Dolphin, an inn\
    // 4 The Prancing Lamb, an inn\
    // 5 The Leering Star, an inn\
    // 6 The Staggering Pegasus, an inn\
    // 7 The Prancing Horde, an inn\
    // 8 The Black Star, an inn\
    // 9 The Prancing Pegasus, an inn

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
            .enumerate()
            .map(|(i, (generated, persisted))| {
                if i == 3 {
                    assert_eq!("*Alternatives:* \\", generated);
                    assert_eq!(
                        format!(
                            "_{} has not yet been saved. Use ~save~ to save it to your `journal`._",
                            name,
                        ),
                        persisted,
                    );
                } else {
                    assert_eq!(generated, persisted)
                }
            })
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
