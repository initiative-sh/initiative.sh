mod create;
mod create_multiple;
mod edit;

use crate::common::{get_name, sync_app};

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
    let name = get_name(&generated_output);
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
