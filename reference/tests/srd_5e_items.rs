//! | Case                             | Tested by         |
//! |----------------------------------|-------------------|
//! | name: armor w/ hard-coded suffix | leather_armor     |
//! | name: armor w/ no suffix         | chain_shirt       |
//! | name: one word                   | mastiff           |
//! | name: multiple words             | holy_water_flask  |
//! | name: one comma                  | light_crossbow    |
//! | armor_class: fixed               | splint_armor      |
//! | armor_class: with dex            | leather_armor     |
//! | armor_class: with limited dex    | chain_shirt       |
//! | armor_class: bonus               | shield            |
//! | capacity                         | mastiff           |
//! | cost                             | leather_armor     |
//! | damage                           | dagger            |
//! | desc                             | holy_water_flask  |
//! | item_category               | leather_armor     |
//! | name                             | leather_armor     |
//! | range                            | light_crossbow    |
//! | special                          | whip              |
//! | speed                            | mastiff           |
//! | stealth_disadvantage: false      | leather_armor     |
//! | stealth_disadvantage: true       | splint_armor      |
//! | str_minimum: none                | leather_armor     |
//! | str_minimum: some                | splint_armor      |
//! | throw_range                      | dagger            |
//! | weight                           | leather_armor     |

use initiative_reference::srd_5e::items;

#[test]
fn leather_armor() {
    let items = items().unwrap();
    let item = items.iter().find(|i| i.name() == "Leather Armor").unwrap();

    assert_eq!(
        "\
# Leather Armor
*Armor (Light)*

**Cost:** 10 gp\\
**Armor Class (AC):** 11 + Dex modifier\\
**Strength:** any\\
**Stealth:** no impact\\
**Weight:** 10 lbs",
        format!("{}", item.display_details()),
    );
}

#[test]
fn chain_shirt() {
    let items = items().unwrap();
    let item = items.iter().find(|i| i.name() == "Chain Shirt").unwrap();

    assert_eq!(
        "\
# Chain Shirt
*Armor (Medium)*

**Cost:** 50 gp\\
**Armor Class (AC):** 13 + Dex modifier (max 2)\\
**Strength:** any\\
**Stealth:** no impact\\
**Weight:** 20 lbs",
        format!("{}", item.display_details()),
    );
}

#[test]
fn splint_armor() {
    let items = items().unwrap();
    let item = items.iter().find(|i| i.name() == "Splint Armor").unwrap();

    assert_eq!(
        "\
# Splint Armor
*Armor (Heavy)*

**Cost:** 200 gp\\
**Armor Class (AC):** 17\\
**Strength:** 15+\\
**Stealth:** disadvantage\\
**Weight:** 60 lbs",
        format!("{}", item.display_details()),
    );
}

#[test]
fn shield() {
    let items = items().unwrap();
    let item = items.iter().find(|i| i.name() == "Shield").unwrap();

    assert_eq!(
        "\
# Shield
*Armor (Shield)*

**Cost:** 10 gp\\
**Armor Class (AC):** +2\\
**Strength:** any\\
**Stealth:** no impact\\
**Weight:** 6 lbs",
        format!("{}", item.display_details()),
    );
}

#[test]
fn trident() {
    let items = items().unwrap();
    let item = items.iter().find(|i| i.name() == "Trident").unwrap();

    assert_eq!(
        "\
# Trident
*Weapon (Martial Melee)*

**Cost:** 5 gp\\
**Damage:** 1d6 slashing\\
**Properties:** Thrown (range 20/60), versatile (1d8)\\
**Weight:** 4 lbs",
        format!("{}", item.display_details()),
    );
}

#[test]
fn light_crossbow() {
    let items = items().unwrap();
    let item = items.iter().find(|i| i.name() == "Light Crossbow").unwrap();

    assert_eq!(
        "\
# Light Crossbow
*Weapon (Simple Ranged)*

**Cost:** 25 gp\\
**Damage:** 1d8 piercing\\
**Properties:** Ammunition (range 80/320), loading, two-handed\\
**Weight:** 5 lbs",
        format!("{}", item.display_details()),
    );
}

#[test]
fn mastiff() {
    let items = items().unwrap();
    let item = items.iter().find(|i| i.name() == "Mastiff").unwrap();

    assert_eq!(
        "\
# Mastiff
*Mounts and Vehicles (Mounts and Other Animals)*

**Cost:** 25 gp\\
**Speed:** 40 ft/round\\
**Carrying Capacity:** 195 lb.",
        format!("{}", item.display_details()),
    );
}

#[test]
fn holy_water_flask() {
    let items = items().unwrap();
    let item = items
        .iter()
        .find(|i| i.name() == "Holy Water (Flask)")
        .unwrap();

    assert_eq!(
        "\
# Holy Water (Flask)
*Adventuring Gear (Standard Gear)*

**Cost:** 25 gp\\
**Weight:** 1 lbs

As an action, you can splash the contents of this flask onto a creature within 5 feet of you or throw it up to 20 feet, shattering it on impact. In either case, make a ranged attack against a target creature, treating the holy water as an improvised weapon.

If the target is a fiend or undead, it takes 2d6 radiant damage.

A cleric or paladin may create holy water by performing a special ritual.

The ritual takes 1 hour to perform, uses 25 gp worth of powdered silver, and requires the caster to expend a 1st-level spell slot.",
        format!("{}", item.display_details()),
    );
}

#[test]
fn lance() {
    let items = items().unwrap();
    let item = items.iter().find(|i| i.name() == "Lance").unwrap();

    assert_eq!("\
# Lance
*Weapon (Martial Melee)*

**Cost:** 10 gp\\
**Damage:** 1d12 piercing\\
**Properties:** Reach, special\\
**Weight:** 6 lbs

You have disadvantage when you use a lance to attack a target within 5 feet of you. Also, a lance requires two hands to wield when you aren't mounted.",
        format!("{}", item.display_details()),
    );
}
