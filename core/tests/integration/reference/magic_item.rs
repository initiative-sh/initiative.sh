use crate::common::sync_app;

#[test]
fn rod_of_rulership() {
    assert_eq!(
        "\
# Rod Of Rulership

*Rod, rare (requires attunement)*

You can use an action to present the rod and command obedience from each creature of your choice that you can see within 120 feet of you. Each target must succeed on a DC 15 Wisdom saving throw or be charmed by you for 8 hours. While charmed in this way, the creature regards you as its trusted leader. If harmed by you or your companions, or commanded to do something contrary to its nature, a target ceases to be charmed in this way. The rod can't be used again until the next dawn.

*Rod Of Rulership is Open Game Content subject to the `Open Game License`.*",
        sync_app().command("Rod Of Rulership").unwrap(),
    );
}
