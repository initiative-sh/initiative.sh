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
*Weapon (Simple Ranged)*

**Cost:** 25 gp\\
**Damage:** 1d8 piercing\\
**Range:** 80/320\\
**Weight:** 5 lbs

*Light Crossbow is Open Game Content subject to the `Open Game License`.*",
        output,
    );

    assert_eq!(output, app().command("Crossbow, Light"));
}

#[test]
fn weapons() {
    let output = app().command("melee weapons");

    assert_eq!(
        "\
 # Melee Weapons

| Name | Cost | Damage | Weight | Properties |
|---|--:|---|--:|---|
| `Battleaxe` | 10 gp | 1d8 slashing | 4 lb. | |
| `Club` | 1 sp | 1d4 bludgeoning | 2 lb. | |
| `Dagger` | 2 gp | 1d4 piercing | 1 lb. | |
| `Flail` | 10 gp | 1d8 bludgeoning | 2 lb. | |
| `Glaive` | 20 gp | 1d10 slashing | 6 lb. | |
| `Greataxe` | 30 gp | 1d12 slashing | 7 lb. | |
| `Greatclub` | 2 sp | 1d8 bludgeoning | 10 lb. | |
| `Greatsword` | 50 gp | 2d6 slashing | 6 lb. | |
| `Halberd` | 20 gp | 1d10 slashing | 6 lb. | |
| `Handaxe` | 5 gp | 1d6 slashing | 2 lb. | |
| `Javelin` | 5 sp | 1d6 piercing | 2 lb. | |
| `Lance` | 10 gp | 1d12 piercing | 6 lb. | |
| `Light Hammer` | 2 gp | 1d4 bludgeoning | 2 lb. | |
| `Longsword` | 15 gp | 1d8 slashing | 3 lb. | |
| `Mace` | 5 gp | 1d6 bludgeoning | 4 lb. | |
| `Maul` | 10 gp | 2d6 bludgeoning | 10 lb. | |
| `Morningstar` | 15 gp | 1d8 piercing | 4 lb. | |
| `Pike` | 5 gp | 1d10 piercing | 18 lb. | |
| `Quarterstaff` | 2 sp | 1d6 bludgeoning | 4 lb. | |
| `Rapier` | 25 gp | 1d8 piercing | 2 lb. | |
| `Scimitar` | 25 gp | 1d6 slashing | 3 lb. | |
| `Shortsword` | 10 gp | 1d6 piercing | 2 lb. | |
| `Sickle` | 1 gp | 1d4 slashing | 2 lb. | |
| `Spear` | 1 gp | 1d6 piercing | 3 lb. | |
| `Trident` | 5 gp | 1d6 slashing | 4 lb. | |
| `War Pick` | 5 gp | 1d8 piercing | 2 lb. | |
| `Warhammer` | 15 gp | 1d8 bludgeoning | 2 lb. | |
| `Whip` | 2 gp | 1d4 slashing | 3 lb. | |

*This listing is Open Game Content subject to the `Open Game License`.*",
        output,
    );

    assert_eq!(output, app().command("weapons, melee"));
}
