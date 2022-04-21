//! | Case                  | Tested by          |
//! |-----------------------|--------------------|
//! | species (single)      | dwarven_resilience |
//! | species (multiple)    | darkvision         |
//! | subspecies (single)   | dwarven_toughness  |

use initiative_reference::srd_5e::traits;

#[test]
fn darkvision() {
    let species_traits = traits().unwrap();
    let species_trait = species_traits
        .iter()
        .find(|i| i.name == "Darkvision")
        .unwrap();

    assert_eq!("`Darkvision`", species_trait.display_summary().to_string());

    assert_eq!(
        "\
# Darkvision

**Species:** Dwarf, Elf, Gnome, Half-Elf, Half-Orc, Tiefling

You have superior vision in dark and dim conditions. You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You cannot discern color in darkness, only shades of gray.",
        species_trait.display_details().to_string(),
    );
}

#[test]
fn dwarven_resilience() {
    let species_traits = traits().unwrap();
    let species_trait = species_traits
        .iter()
        .find(|i| i.name == "Dwarven Resilience")
        .unwrap();

    assert_eq!(
        "`Dwarven Resilience`",
        species_trait.display_summary().to_string(),
    );

    assert_eq!(
        "\
# Dwarven Resilience

**Species:** Dwarf

You have advantage on saving throws against poison, and you have resistance against poison damage.",
        species_trait.display_details().to_string(),
    );
}

#[test]
fn dwarven_toughness() {
    let species_traits = traits().unwrap();
    let species_trait = species_traits
        .iter()
        .find(|i| i.name == "Dwarven Toughness")
        .unwrap();

    assert_eq!(
        "`Dwarven Toughness`",
        species_trait.display_summary().to_string(),
    );

    assert_eq!(
        "\
# Dwarven Toughness

**Subspecies:** Hill Dwarf

Your hit point maximum increases by 1, and it increases by 1 every time you gain a level.",
        species_trait.display_details().to_string(),
    );
}
