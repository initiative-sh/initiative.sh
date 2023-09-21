use crate::common::sync_app;
use initiative_core::app::AutocompleteSuggestion;

#[test]
fn weapons() {
    let output = sync_app().command("melee weapons").unwrap();

    assert_eq!(
        "\
 # Melee Weapons

| Name | Cost | Damage | Weight | Properties |
|---|--:|---|--:|---|
| `Battleaxe` | 10 gp | `1d8` slashing | 4 lb. | Versatile (`1d10`) |
| `Club` | 1 sp | `1d4` bludgeoning | 2 lb. | Light, monk |
| `Dagger` | 2 gp | `1d4` piercing | 1 lb. | Finesse, light, monk, thrown (range 20/60) |
| `Flail` | 10 gp | `1d8` bludgeoning | 2 lb. | — |
| `Glaive` | 20 gp | `1d10` slashing | 6 lb. | Heavy, reach, two-handed |
| `Greataxe` | 30 gp | `1d12` slashing | 7 lb. | Heavy, two-handed |
| `Greatclub` | 2 sp | `1d8` bludgeoning | 10 lb. | Two-Handed |
| `Greatsword` | 50 gp | `2d6` slashing | 6 lb. | Heavy, two-handed |
| `Halberd` | 20 gp | `1d10` slashing | 6 lb. | Heavy, reach, two-handed |
| `Handaxe` | 5 gp | `1d6` slashing | 2 lb. | Light, monk, thrown (range 20/60) |
| `Javelin` | 5 sp | `1d6` piercing | 2 lb. | Monk, thrown (range 30/120) |
| `Lance` | 10 gp | `1d12` piercing | 6 lb. | Reach, special |
| `Light hammer` | 2 gp | `1d4` bludgeoning | 2 lb. | Light, monk, thrown (range 20/60) |
| `Longsword` | 15 gp | `1d8` slashing | 3 lb. | Versatile (`1d10`) |
| `Mace` | 5 gp | `1d6` bludgeoning | 4 lb. | Monk |
| `Maul` | 10 gp | `2d6` bludgeoning | 10 lb. | Heavy, two-handed |
| `Morningstar` | 15 gp | `1d8` piercing | 4 lb. | — |
| `Pike` | 5 gp | `1d10` piercing | 18 lb. | Heavy, reach, two-handed |
| `Quarterstaff` | 2 sp | `1d6` bludgeoning | 4 lb. | Monk, versatile (`1d8`) |
| `Rapier` | 25 gp | `1d8` piercing | 2 lb. | Finesse |
| `Scimitar` | 25 gp | `1d6` slashing | 3 lb. | Finesse, light |
| `Shortsword` | 10 gp | `1d6` piercing | 2 lb. | Finesse, light, monk |
| `Sickle` | 1 gp | `1d4` slashing | 2 lb. | Light, monk |
| `Spear` | 1 gp | `1d6` piercing | 3 lb. | Monk, thrown (range 20/60), versatile (`1d8`) |
| `Trident` | 5 gp | `1d6` slashing | 4 lb. | Thrown (range 20/60), versatile (`1d8`) |
| `War pick` | 5 gp | `1d8` piercing | 2 lb. | — |
| `Warhammer` | 15 gp | `1d8` bludgeoning | 2 lb. | Versatile (`1d10`) |
| `Whip` | 2 gp | `1d4` slashing | 3 lb. | Finesse, reach |

*This listing is Open Game Content subject to the `Open Game License`.*",
        output,
    );

    assert_eq!(output, sync_app().command("weapons, melee").unwrap());
    assert_eq!(
        output,
        sync_app()
            .command("srd item category melee weapons")
            .unwrap(),
    );

    assert_eq!(
        vec![AutocompleteSuggestion::new(
            "melee weapons",
            "SRD item category",
        )],
        sync_app().autocomplete("melee weapons"),
    );
}

#[test]
fn magic_weapons() {
    let output = sync_app().command("magic weapons").unwrap();

    assert_eq!(
        "\
# Magic Weapons

* `Berserker Axe`
* `Dagger of Venom`
* `Dancing Sword`
* `Defender`
* `Dragon Slayer`
* `Dwarven Thrower`
* `Flame Tongue`
* `Frost Brand`
* `Giant Slayer`
* `Hammer of Thunderbolts`
* `Holy Avenger`
* `Javelin of Lightning`
* `Luck Blade`
* `Mace of Disruption`
* `Mace of Smiting`
* `Mace of Terror`
* `Nine Lives Stealer`
* `Oathbow`
* `Scimitar of Speed`
* `Sun Blade`
* `Sword of Life Stealing`
* `Sword of Sharpness`
* `Sword of Wounding`
* `Trident of Fish Command`
* `Vicious Weapon`
* `Vorpal Sword`
* `Weapon, +1`
* `Weapon, +2`
* `Weapon, +3`

*This listing is Open Game Content subject to the `Open Game License`.*",
        output,
    );

    assert_eq!(output, sync_app().command("weapons, magic").unwrap());
    assert_eq!(
        output,
        sync_app()
            .command("srd item category magic weapons")
            .unwrap(),
    );

    assert_eq!(
        vec![AutocompleteSuggestion::new(
            "magic weapons",
            "SRD item category",
        )],
        sync_app().autocomplete("magic weapons"),
    );
}
