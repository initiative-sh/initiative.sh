use initiative_core::app;

#[test]
fn open_game_license() {
    assert_eq!(35, app().command("Open Game License").lines().count());
}

#[test]
fn spell() {
    assert_eq!(
        "\
Speak With Animals
1st-level divination (ritual)

Casting Time: 1 action
Range: Self
Components: V, S
Duration: 10 minutes

You gain the ability to comprehend and verbally communicate with beasts for the duration. The knowledge and awareness of many beasts is limited by their intelligence, but at a minimum, beasts can give you information about nearby locations and monsters, including whatever they can perceive or have perceived within the past day. You might be able to persuade a beast to perform a small favor for you, at the DM's discretion.

Speak With Animals is Open Game Content subject to the `Open Game License`.",
        app().command("Speak With Animals"),
    );
}
