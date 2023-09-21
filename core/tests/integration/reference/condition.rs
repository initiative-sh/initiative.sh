use crate::common::sync_app;
use initiative_core::app::AutocompleteSuggestion;

#[test]
fn stunned() {
    let output = sync_app().command("Stunned").unwrap();

    assert_eq!(
        "\
# Stunned

- A stunned creature is incapacitated (see the condition), can't move, and can speak only falteringly.
- The creature automatically fails Strength and Dexterity saving throws.
- Attack rolls against the creature have advantage.

*Stunned is Open Game Content subject to the `Open Game License`.*",
        output,
    );

    assert_eq!(output, sync_app().command("srd condition Stunned").unwrap());

    assert_eq!(
        vec![AutocompleteSuggestion::new("Stunned", "SRD condition")],
        sync_app().autocomplete("stunned"),
    );
}
