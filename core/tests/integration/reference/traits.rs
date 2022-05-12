use crate::common::sync_app;

#[test]
fn stonecunning() {
    assert_eq!(
        "\
# Stonecunning

**Species:** Dwarf

Whenever you make an Intelligence (History) check related to the origin of stonework, you are considered proficient in the History skill and add double your proficiency bonus to the check, instead of your normal proficiency bonus.

*Stonecunning is Open Game Content subject to the `Open Game License`.*",
        sync_app().command("Stonecunning").unwrap(),
    );

    assert_eq!(
        vec![("Stonecunning".into(), "SRD trait".into())],
        sync_app().autocomplete("stonecunning"),
    );
}

#[test]
fn darkvision() {
    assert_eq!(
        "\
There are several possible interpretations of this command. Did you mean:

* `srd spell Darkvision`
* `srd trait Darkvision`",
        sync_app().command("Darkvision").unwrap_err(),
    );

    assert_eq!(
        "\
# Darkvision

**Species:** Dwarf, Elf, Gnome, Half-Elf, Half-Orc, Tiefling

You have superior vision in dark and dim conditions. You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You cannot discern color in darkness, only shades of gray.

*Darkvision is Open Game Content subject to the `Open Game License`.*",
        sync_app().command("srd trait Darkvision").unwrap(),
    );

    assert_eq!(2, sync_app().autocomplete("darkvision").len());
    assert!(sync_app()
        .autocomplete("darkvision")
        .iter()
        .any(|item| item == &("Darkvision".into(), "SRD trait".into())));
}

#[test]
fn draconic_ancestry() {
    assert_eq!(
        vec![("Draconic Ancestry".into(), "SRD trait".into())],
        sync_app().autocomplete("draconic ancestry"),
    );
}
