use crate::common::sync_app;
use initiative_core::app::AutocompleteSuggestion;

#[test]
fn rod_of_rulership() {
    let output = sync_app().command("Rod Of Rulership").unwrap();

    assert_eq!(
        "\
# Rod of Rulership

*Rod, rare (requires attunement)*

You can use an action to present the rod and command obedience from each creature of your choice that you can see within 120 feet of you. Each target must succeed on a DC 15 Wisdom saving throw or be charmed by you for 8 hours. While charmed in this way, the creature regards you as its trusted leader. If harmed by you or your companions, or commanded to do something contrary to its nature, a target ceases to be charmed in this way. The rod can't be used again until the next dawn.

*Rod of Rulership is Open Game Content subject to the `Open Game License`.*",
        output,
    );

    assert_eq!(
        output,
        sync_app()
            .command("srd magic item Rod of Rulership")
            .unwrap(),
    );

    assert_eq!(
        vec![AutocompleteSuggestion::new(
            "Rod of Rulership",
            "SRD magic item",
        )],
        sync_app().autocomplete("rod of rulership"),
    );
}
