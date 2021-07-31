//! | Case                             | Tested by         |
//! |----------------------------------|-------------------|
//! | name: armor w/ hard-coded suffix | leather_armor     |
//! | name: armor w/ no suffix         | chain_shirt       |
//! | name: one word                   | mastiff           |
//! | name: multiple words             | potion_of_healing |
//! | name: one comma                  | light_crossbow    |
//! | armor_class: fixed               | splint_armor      |
//! | armor_class: with dex            | leather_armor     |
//! | armor_class: with limited dex    | chain_shirt       |
//! | armor_class: bonus               | shield            |
//! | capacity                         | mastiff           |
//! | cost                             | leather_armor     |
//! | damage                           | dagger            |
//! | desc                             | potion_of_healing |
//! | equipment_category               | leather_armor     |
//! | name                             | leather_armor     |
//! | range                            | light_crossbow    |
//! | speed                            | mastiff           |
//! | stealth_disadvantage: false      | leather_armor     |
//! | stealth_disadvantage: true       | splint_armor      |
//! | str_minimum: none                | leather_armor     |
//! | str_minimum: some                | splint_armor      |
//! | throw_range                      | dagger            |
//! | weight                           | leather_armor     |

use initiative_reference::srd_5e::equipment;

#[test]
fn leather_armor() {
    let equipment = equipment().unwrap();
    let item = equipment
        .iter()
        .find(|i| i.name() == "Leather Armor")
        .unwrap();

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
    let equipment = equipment().unwrap();
    let item = equipment
        .iter()
        .find(|i| i.name() == "Chain Shirt")
        .unwrap();

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
    let equipment = equipment().unwrap();
    let item = equipment
        .iter()
        .find(|i| i.name() == "Splint Armor")
        .unwrap();

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
    let equipment = equipment().unwrap();
    let item = equipment.iter().find(|i| i.name() == "Shield").unwrap();

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
    let equipment = equipment().unwrap();
    let item = equipment.iter().find(|i| i.name() == "Trident").unwrap();

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
    let equipment = equipment().unwrap();
    let item = equipment
        .iter()
        .find(|i| i.name() == "Light Crossbow")
        .unwrap();

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
    let equipment = equipment().unwrap();
    let item = equipment.iter().find(|i| i.name() == "Mastiff").unwrap();

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
fn potion_of_healing() {
    let equipment = equipment().unwrap();
    let item = equipment
        .iter()
        .find(|i| i.name() == "Potion Of Healing")
        .unwrap();

    assert_eq!(
        "\
# Potion Of Healing
*Adventuring Gear (Standard Gear)*

**Cost:** 50 gp\\
**Weight:** 0.5 lbs

A character who drinks the magical red fluid in this vial regains 2d4 + 2 hit points. Drinking or administering a potion takes an action.",
        format!("{}", item.display_details()),
    );
}
