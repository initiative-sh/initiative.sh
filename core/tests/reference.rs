use initiative_core::app;

#[test]
fn open_game_license() {
    assert_eq!(111, app().command("Open Game License").lines().count());
}

#[test]
fn spell() {
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
        app().command("Speak With Animals"),
    );
}

#[test]
fn spells() {
    let output = app().command("spells");
    assert_eq!(
        "\
# Spells
* `Acid Arrow` (2nd-level evocation)
* `Acid Splash` (conjuration cantrip)
* `Aid` (2nd-level abjuration)
* `Alarm` (1st-level abjuration)
",
        output.split_inclusive('\n').take(5).collect::<String>(),
    );

    assert_eq!(322, output.lines().count(), "{}", output);
}

#[test]
fn item() {
    let output = app().command("Light Crossbow");

    assert_eq!(
        "\
# Light Crossbow
*Weapon*

**Cost:** 25 gp\\
**Damage:** 1d8 piercing\\
**Range:** 80/320\\
**Weight:** 5 lbs

*Light Crossbow is Open Game Content subject to the `Open Game License`.*",
        output,
    );

    assert_eq!(output, app().command("Crossbow, Light"));
}
