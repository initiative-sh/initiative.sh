use crate::common::sync_app;

#[test]
fn speak_with_animals() {
    assert_eq!(
        "\
# Speak With Animals
*1st-level divination (ritual)*

**Casting Time:** 1 action\\
**Range:** Self\\
**Components:** V, S\\
**Duration:** 10 minutes

You gain the ability to comprehend and verbally communicate with beasts for the duration. The knowledge and awareness of many beasts is limited by their intelligence, but at a minimum, beasts can give you information about nearby locations and monsters, including whatever they can perceive or have perceived within the past day. You might be able to persuade a beast to perform a small favor for you, at the DM's discretion.

*Speak With Animals is Open Game Content subject to the `Open Game License`.*",
        sync_app().command("Speak With Animals").unwrap(),
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
*2nd-level transmutation*

**Casting Time:** 1 action\\
**Range:** Touch\\
**Components:** V, S, M (either a pinch of dried carrot or an agate)\\
**Duration:** 8 hours

You touch a willing creature to grant it the ability to see in the dark. For the duration, that creature has darkvision out to a range of 60 feet.

*Darkvision is Open Game Content subject to the `Open Game License`.*",
        sync_app().command("srd spell Darkvision").unwrap(),
    );
}
