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
