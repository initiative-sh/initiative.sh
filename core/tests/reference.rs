mod common;

use common::sync_app;

#[test]
fn open_game_license() {
    assert_eq!(111, sync_app().command("Open Game License").lines().count());
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
        sync_app().command("Speak With Animals"),
    );
}

#[test]
fn spells() {
    let output = sync_app().command("spells");
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
    let output = sync_app().command("Light Crossbow");

    assert_eq!(
        "\
# Light Crossbow
*Weapon (Simple Ranged)*

**Cost:** 25 gp\\
**Damage:** 1d8 piercing\\
**Properties:** Ammunition (range 80/320), loading, two-handed\\
**Weight:** 5 lbs

*Light Crossbow is Open Game Content subject to the `Open Game License`.*",
        output,
    );

    assert_eq!(output, sync_app().command("Crossbow, Light"));
}

#[test]
fn weapons() {
    let output = sync_app().command("melee weapons");

    assert_eq!(
        "\
 # Melee Weapons

| Name | Cost | Damage | Weight | Properties |
|---|--:|---|--:|---|
| `Battleaxe` | 10 gp | 1d8 slashing | 4 lb. | Versatile (1d10) |
| `Club` | 1 sp | 1d4 bludgeoning | 2 lb. | Light, monk |
| `Dagger` | 2 gp | 1d4 piercing | 1 lb. | Finesse, light, monk, thrown (range 20/60) |
| `Flail` | 10 gp | 1d8 bludgeoning | 2 lb. | — |
| `Glaive` | 20 gp | 1d10 slashing | 6 lb. | Heavy, reach, two-handed |
| `Greataxe` | 30 gp | 1d12 slashing | 7 lb. | Heavy, two-handed |
| `Greatclub` | 2 sp | 1d8 bludgeoning | 10 lb. | Two-Handed |
| `Greatsword` | 50 gp | 2d6 slashing | 6 lb. | Heavy, two-handed |
| `Halberd` | 20 gp | 1d10 slashing | 6 lb. | Heavy, reach, two-handed |
| `Handaxe` | 5 gp | 1d6 slashing | 2 lb. | Light, monk, thrown (range 20/60) |
| `Javelin` | 5 sp | 1d6 piercing | 2 lb. | Monk, thrown (range 30/120) |
| `Lance` | 10 gp | 1d12 piercing | 6 lb. | Reach, special |
| `Light Hammer` | 2 gp | 1d4 bludgeoning | 2 lb. | Light, monk, thrown (range 20/60) |
| `Longsword` | 15 gp | 1d8 slashing | 3 lb. | Versatile (1d10) |
| `Mace` | 5 gp | 1d6 bludgeoning | 4 lb. | Monk |
| `Maul` | 10 gp | 2d6 bludgeoning | 10 lb. | Heavy, two-handed |
| `Morningstar` | 15 gp | 1d8 piercing | 4 lb. | — |
| `Pike` | 5 gp | 1d10 piercing | 18 lb. | Heavy, reach, two-handed |
| `Quarterstaff` | 2 sp | 1d6 bludgeoning | 4 lb. | Monk, versatile (1d8) |
| `Rapier` | 25 gp | 1d8 piercing | 2 lb. | Finesse |
| `Scimitar` | 25 gp | 1d6 slashing | 3 lb. | Finesse, light |
| `Shortsword` | 10 gp | 1d6 piercing | 2 lb. | Finesse, light, monk |
| `Sickle` | 1 gp | 1d4 slashing | 2 lb. | Light, monk |
| `Spear` | 1 gp | 1d6 piercing | 3 lb. | Monk, thrown (range 20/60), versatile (1d8) |
| `Trident` | 5 gp | 1d6 slashing | 4 lb. | Thrown (range 20/60), versatile (1d8) |
| `War Pick` | 5 gp | 1d8 piercing | 2 lb. | — |
| `Warhammer` | 15 gp | 1d8 bludgeoning | 2 lb. | Versatile (1d10) |
| `Whip` | 2 gp | 1d4 slashing | 3 lb. | Finesse, reach |

*This listing is Open Game Content subject to the `Open Game License`.*",
        output,
    );

    assert_eq!(output, sync_app().command("weapons, melee"));
}

#[test]
fn magic_item() {
    let output = sync_app().command("Rod Of Rulership");

    assert_eq!(
        "\
# Rod Of Rulership

*Rod, rare (requires attunement)*

You can use an action to present the rod and command obedience from each creature of your choice that you can see within 120 feet of you. Each target must succeed on a DC 15 Wisdom saving throw or be charmed by you for 8 hours. While charmed in this way, the creature regards you as its trusted leader. If harmed by you or your companions, or commanded to do something contrary to its nature, a target ceases to be charmed in this way. The rod can't be used again until the next dawn.

*Rod Of Rulership is Open Game Content subject to the `Open Game License`.*",
        output,
    );
}

#[test]
fn magic_weapons() {
    let output = sync_app().command("magic weapons");

    assert_eq!(
        "\
# Magic weapons

* `Berserker Axe`
* `Dagger Of Venom`
* `Dancing Sword`
* `Defender`
* `Dragon Slayer`
* `Dwarven Thrower`
* `Flame Tongue`
* `Frost Brand`
* `Giant Slayer`
* `Hammer Of Thunderbolts`
* `Holy Avenger`
* `Javelin Of Lightning`
* `Luck Blade`
* `Mace Of Disruption`
* `Mace Of Smiting`
* `Mace Of Terror`
* `Nine Lives Stealer`
* `Oathbow`
* `Scimitar Of Speed`
* `Sun Blade`
* `Sword Of Life Stealing`
* `Sword Of Sharpness`
* `Sword Of Wounding`
* `Trident Of Fish Command`
* `Vicious Weapon`
* `Vorpal Sword`
* `Weapon, +1, +2, Or +3`

*This listing is Open Game Content subject to the `Open Game License`.*",
        output,
    );

    assert_eq!(output, sync_app().command("weapons, magic"));
}
