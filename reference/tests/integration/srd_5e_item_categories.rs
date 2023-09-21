use initiative_reference::srd_5e::{item_categories, items, magic_items};

#[test]
fn armor() {
    let categories = item_categories().unwrap();
    let category = categories.iter().find(|i| i.name() == "Armor").unwrap();
    let items = items().unwrap();

    assert_eq!(
        "\
# Armor

| Name | Cost | Armor Class (AC) | Strength | Stealth | Weight |
|---|--:|---|---|---|--:|
| `Breastplate` | 400 gp | 14 + Dex modifier (max 2) | — | — | 20 lb. |
| `Chain Mail` | 75 gp | 16 | Str 13 | disadvantage | 55 lb. |
| `Chain Shirt` | 50 gp | 13 + Dex modifier (max 2) | — | — | 20 lb. |
| `Half Plate Armor` | 750 gp | 15 + Dex modifier (max 2) | — | disadvantage | 40 lb. |
| `Hide Armor` | 10 gp | 12 + Dex modifier (max 2) | — | — | 12 lb. |
| `Leather Armor` | 10 gp | 11 + Dex modifier | — | — | 10 lb. |
| `Padded Armor` | 5 gp | 11 + Dex modifier | — | disadvantage | 8 lb. |
| `Plate Armor` | 1500 gp | 18 | Str 15 | disadvantage | 65 lb. |
| `Ring Mail` | 30 gp | 14 | — | disadvantage | 40 lb. |
| `Scale Mail` | 50 gp | 14 + Dex modifier (max 2) | — | disadvantage | 45 lb. |
| `Shield` | 10 gp | +2 | — | — | 6 lb. |
| `Splint Armor` | 200 gp | 17 | Str 15 | disadvantage | 60 lb. |
| `Studded Leather Armor` | 45 gp | 12 + Dex modifier | — | — | 13 lb. |",
        format!("{}", category.display_item_table(&items[..])),
    );
}

#[test]
fn weapons() {
    let categories = item_categories().unwrap();
    let category = categories.iter().find(|i| i.name() == "Weapons").unwrap();
    let items = items().unwrap();

    assert_eq!(
        "\
# Weapons

| Name | Cost | Damage | Weight | Properties |
|---|--:|---|--:|---|
| `Battleaxe` | 10 gp | 1d8 slashing | 4 lb. | Versatile (1d10) |
| `Blowgun` | 10 gp | 1d1 piercing | 1 lb. | Ammunition (range 25/100), loading |
| `Club` | 1 sp | 1d4 bludgeoning | 2 lb. | Light, monk |
| `Crossbow, hand` | 75 gp | 1d6 piercing | 3 lb. | Ammunition (range 30/120), light, loading |
| `Crossbow, heavy` | 50 gp | 1d10 piercing | 18 lb. | Ammunition (range 100/400), heavy, loading, two-handed |
| `Crossbow, light` | 25 gp | 1d8 piercing | 5 lb. | Ammunition (range 80/320), loading, two-handed |
| `Dagger` | 2 gp | 1d4 piercing | 1 lb. | Finesse, light, monk, thrown (range 20/60) |
| `Dart` | 5 cp | 1d4 piercing | 0.25 lb. | Finesse, thrown (range 20/60) |
| `Flail` | 10 gp | 1d8 bludgeoning | 2 lb. | — |
| `Glaive` | 20 gp | 1d10 slashing | 6 lb. | Heavy, reach, two-handed |
| `Greataxe` | 30 gp | 1d12 slashing | 7 lb. | Heavy, two-handed |
| `Greatclub` | 2 sp | 1d8 bludgeoning | 10 lb. | Two-Handed |
| `Greatsword` | 50 gp | 2d6 slashing | 6 lb. | Heavy, two-handed |
| `Halberd` | 20 gp | 1d10 slashing | 6 lb. | Heavy, reach, two-handed |
| `Handaxe` | 5 gp | 1d6 slashing | 2 lb. | Light, monk, thrown (range 20/60) |
| `Javelin` | 5 sp | 1d6 piercing | 2 lb. | Monk, thrown (range 30/120) |
| `Lance` | 10 gp | 1d12 piercing | 6 lb. | Reach, special |
| `Light hammer` | 2 gp | 1d4 bludgeoning | 2 lb. | Light, monk, thrown (range 20/60) |
| `Longbow` | 50 gp | 1d8 piercing | 2 lb. | Ammunition (range 150/600), heavy, two-handed |
| `Longsword` | 15 gp | 1d8 slashing | 3 lb. | Versatile (1d10) |
| `Mace` | 5 gp | 1d6 bludgeoning | 4 lb. | Monk |
| `Maul` | 10 gp | 2d6 bludgeoning | 10 lb. | Heavy, two-handed |
| `Morningstar` | 15 gp | 1d8 piercing | 4 lb. | — |
| `Net` | 1 gp | — | 3 lb. | Special, thrown (range 5/15) |
| `Pike` | 5 gp | 1d10 piercing | 18 lb. | Heavy, reach, two-handed |
| `Quarterstaff` | 2 sp | 1d6 bludgeoning | 4 lb. | Monk, versatile (1d8) |
| `Rapier` | 25 gp | 1d8 piercing | 2 lb. | Finesse |
| `Scimitar` | 25 gp | 1d6 slashing | 3 lb. | Finesse, light |
| `Shortbow` | 25 gp | 1d6 piercing | 2 lb. | Ammunition (range 80/320), two-handed |
| `Shortsword` | 10 gp | 1d6 piercing | 2 lb. | Finesse, light, monk |
| `Sickle` | 1 gp | 1d4 slashing | 2 lb. | Light, monk |
| `Sling` | 1 sp | 1d4 bludgeoning | 0 lb. | Ammunition (range 30/120) |
| `Spear` | 1 gp | 1d6 piercing | 3 lb. | Monk, thrown (range 20/60), versatile (1d8) |
| `Trident` | 5 gp | 1d6 slashing | 4 lb. | Thrown (range 20/60), versatile (1d8) |
| `War pick` | 5 gp | 1d8 piercing | 2 lb. | — |
| `Warhammer` | 15 gp | 1d8 bludgeoning | 2 lb. | Versatile (1d10) |
| `Whip` | 2 gp | 1d4 slashing | 3 lb. | Finesse, reach |",
        format!("{}", category.display_item_table(&items[..])),
    );
}

#[test]
fn adventuring_gear() {
    let categories = item_categories().unwrap();
    let category = categories
        .iter()
        .find(|i| i.name() == "Adventuring Gear")
        .unwrap();
    let items = items().unwrap();

    assert_eq!(
        "\
# Adventuring Gear

| Name | Cost | Weight |
|---|--:|--:|
| `Abacus` | 2 gp | 2 lb. |
| `Acid (vial)` | 25 gp | 1 lb. |
| `Alchemist's fire (flask)` | 50 gp | 1 lb. |
| `Alms box` | 0 cp | 0 lb. |
| `Amulet` | 5 gp | 1 lb. |
| `Antitoxin (vial)` | 50 gp | 0 lb. |
| `Arrow` | 1 gp | 1 lb. |
| `Backpack` | 2 gp | 5 lb. |
| `Ball bearings (bag of 1,000)` | 1 gp | 2 lb. |
| `Barrel` | 2 gp | 70 lb. |
| `Basket` | 4 sp | 2 lb. |
| `Bedroll` | 1 gp | 7 lb. |
| `Bell` | 1 gp | 0 lb. |
| `Blanket` | 5 sp | 3 lb. |
| `Block and tackle` | 1 gp | 5 lb. |
| `Block of incense` | 0 cp | 0 lb. |
| `Blowgun needle` | 1 gp | 1 lb. |
| `Book` | 25 gp | 5 lb. |
| `Bottle, glass` | 2 gp | 2 lb. |
| `Bucket` | 5 cp | 2 lb. |
| `Burglar's Pack` | 16 gp | — |
| `Caltrops` | 5 cp | 2 lb. |
| `Candle` | 1 cp | 0 lb. |
| `Case, crossbow bolt` | 1 gp | 1 lb. |
| `Case, map or scroll` | 1 gp | 1 lb. |
| `Censer` | 0 cp | 0 lb. |
| `Chain (10 feet)` | 5 gp | 10 lb. |
| `Chalk (1 piece)` | 1 cp | 0 lb. |
| `Chest` | 5 gp | 25 lb. |
| `Climber's Kit` | 25 gp | 12 lb. |
| `Clothes, common` | 5 sp | 3 lb. |
| `Clothes, costume` | 5 gp | 4 lb. |
| `Clothes, fine` | 15 gp | 6 lb. |
| `Clothes, traveler's` | 2 gp | 4 lb. |
| `Component pouch` | 25 gp | 2 lb. |
| `Crossbow bolt` | 1 gp | 1.5 lb. |
| `Crowbar` | 2 gp | 5 lb. |
| `Crystal` | 10 gp | 1 lb. |
| `Diplomat's Pack` | 39 gp | — |
| `Disguise Kit` | 25 gp | 3 lb. |
| `Dungeoneer's Pack` | 12 gp | — |
| `Emblem` | 5 gp | 0 lb. |
| `Entertainer's Pack` | 40 gp | — |
| `Explorer's Pack` | 10 gp | — |
| `Fishing tackle` | 1 gp | 4 lb. |
| `Flask or tankard` | 2 cp | 1 lb. |
| `Forgery Kit` | 15 gp | 5 lb. |
| `Grappling hook` | 2 gp | 4 lb. |
| `Hammer` | 1 gp | 3 lb. |
| `Hammer, sledge` | 2 gp | 10 lb. |
| `Healer's Kit` | 5 gp | 3 lb. |
| `Herbalism Kit` | 5 gp | 3 lb. |
| `Holy water (flask)` | 25 gp | 1 lb. |
| `Hourglass` | 25 gp | 1 lb. |
| `Hunting trap` | 5 gp | 25 lb. |
| `Ink (1 ounce bottle)` | 10 gp | 0 lb. |
| `Ink pen` | 2 cp | 0 lb. |
| `Jug or pitcher` | 2 cp | 4 lb. |
| `Ladder (10-foot)` | 1 sp | 25 lb. |
| `Lamp` | 5 sp | 1 lb. |
| `Lantern, bullseye` | 10 gp | 2 lb. |
| `Lantern, hooded` | 5 gp | 2 lb. |
| `Little bag of sand` | 0 cp | 0 lb. |
| `Lock` | 10 gp | 1 lb. |
| `Magnifying glass` | 100 gp | 0 lb. |
| `Manacles` | 2 gp | 6 lb. |
| `Mess Kit` | 2 sp | 1 lb. |
| `Mirror, steel` | 5 gp | 0.5 lb. |
| `Oil (flask)` | 1 sp | 1 lb. |
| `Orb` | 20 gp | 3 lb. |
| `Paper (one sheet)` | 2 sp | 0 lb. |
| `Parchment (one sheet)` | 1 sp | 0 lb. |
| `Perfume (vial)` | 5 gp | 0 lb. |
| `Pick, miner's` | 2 gp | 10 lb. |
| `Piton` | 5 cp | 0.25 lb. |
| `Poison, basic (vial)` | 100 gp | 0 lb. |
| `Poisoner's Kit` | 50 gp | 2 lb. |
| `Pole (10-foot)` | 5 cp | 7 lb. |
| `Pot, iron` | 2 gp | 10 lb. |
| `Pouch` | 5 sp | 1 lb. |
| `Priest's Pack` | 19 gp | — |
| `Quiver` | 1 gp | 1 lb. |
| `Ram, portable` | 4 gp | 35 lb. |
| `Rations (1 day)` | 5 sp | 2 lb. |
| `Reliquary` | 5 gp | 2 lb. |
| `Robes` | 1 gp | 4 lb. |
| `Rod` | 10 gp | 2 lb. |
| `Rope, hempen (50 feet)` | 1 gp | 10 lb. |
| `Rope, silk (50 feet)` | 10 gp | 5 lb. |
| `Sack` | 1 cp | 0.5 lb. |
| `Scale, merchant's` | 5 gp | 3 lb. |
| `Scholar's Pack` | 40 gp | — |
| `Sealing wax` | 5 sp | 0 lb. |
| `Shovel` | 2 gp | 5 lb. |
| `Signal whistle` | 5 cp | 0 lb. |
| `Signet ring` | 5 gp | 0 lb. |
| `Sling bullet` | 4 cp | 1.5 lb. |
| `Small knife` | 0 cp | 0 lb. |
| `Soap` | 2 cp | 0 lb. |
| `Spellbook` | 50 gp | 3 lb. |
| `Spike, iron` | 1 sp | 5 lb. |
| `Sprig of mistletoe` | 1 gp | 0 lb. |
| `Spyglass` | 1000 gp | 1 lb. |
| `Staff` | 5 gp | 4 lb. |
| `String (10 feet)` | 0 cp | 0 lb. |
| `Tent, two-person` | 2 gp | 20 lb. |
| `Tinderbox` | 5 sp | 1 lb. |
| `Torch` | 1 cp | 1 lb. |
| `Totem` | 1 gp | 0 lb. |
| `Vestments` | 0 cp | 0 lb. |
| `Vial` | 1 gp | 0 lb. |
| `Wand` | 10 gp | 1 lb. |
| `Waterskin` | 2 sp | 5 lb. |
| `Whetstone` | 1 cp | 1 lb. |
| `Wooden staff` | 5 gp | 4 lb. |
| `Yew wand` | 10 gp | 1 lb. |",
        format!("{}", category.display_item_table(&items[..])),
    );
}

#[test]
fn tools() {
    let categories = item_categories().unwrap();
    let category = categories.iter().find(|i| i.name() == "Tools").unwrap();
    let items = items().unwrap();

    assert_eq!(
        "\
# Tools

| Name | Cost | Weight |
|---|--:|--:|
| `Alchemist's Supplies` | 50 gp | 8 lb. |
| `Bagpipes` | 30 gp | 6 lb. |
| `Brewer's Supplies` | 20 gp | 9 lb. |
| `Calligrapher's Supplies` | 10 gp | 5 lb. |
| `Carpenter's Tools` | 8 gp | 6 lb. |
| `Cartographer's Tools` | 15 gp | 6 lb. |
| `Cobbler's Tools` | 5 gp | 5 lb. |
| `Cook's utensils` | 1 gp | 8 lb. |
| `Dice Set` | 1 sp | 0 lb. |
| `Drum` | 6 gp | 3 lb. |
| `Dulcimer` | 25 gp | 10 lb. |
| `Flute` | 2 gp | 1 lb. |
| `Glassblower's Tools` | 30 gp | 5 lb. |
| `Horn` | 3 gp | 2 lb. |
| `Jeweler's Tools` | 25 gp | 2 lb. |
| `Leatherworker's Tools` | 5 gp | 5 lb. |
| `Lute` | 35 gp | 2 lb. |
| `Lyre` | 30 gp | 2 lb. |
| `Mason's Tools` | 10 gp | 8 lb. |
| `Navigator's Tools` | 25 gp | 2 lb. |
| `Painter's Supplies` | 10 gp | 5 lb. |
| `Pan flute` | 12 gp | 2 lb. |
| `Playing Card Set` | 5 sp | 0 lb. |
| `Potter's Tools` | 10 gp | 3 lb. |
| `Shawm` | 2 gp | 1 lb. |
| `Smith's Tools` | 20 gp | 8 lb. |
| `Thieves' Tools` | 25 gp | 1 lb. |
| `Tinker's Tools` | 50 gp | 10 lb. |
| `Viol` | 30 gp | 1 lb. |
| `Weaver's Tools` | 1 gp | 5 lb. |
| `Woodcarver's Tools` | 1 gp | 5 lb. |",
        format!("{}", category.display_item_table(&items[..])),
    );
}

#[test]
fn mounts_and_vehicles() {
    let categories = item_categories().unwrap();
    let category = categories
        .iter()
        .find(|i| i.name() == "Mounts and Vehicles")
        .unwrap();
    let items = items().unwrap();

    assert_eq!(
        "\
# Mounts and Vehicles

| Name | Cost | Speed |
|---|--:|--:|
| `Animal Feed (1 day)` | 5 cp | — |
| `Barding: Breastplate` | 1600 gp | — |
| `Barding: Chain mail` | 300 gp | — |
| `Barding: Chain shirt` | 200 gp | — |
| `Barding: Half plate` | 3000 gp | — |
| `Barding: Hide` | 40 gp | — |
| `Barding: Leather` | 40 gp | — |
| `Barding: Padded` | 20 gp | — |
| `Barding: Plate` | 6000 gp | — |
| `Barding: Ring mail` | 12 gp | — |
| `Barding: Scale mail` | 200 gp | — |
| `Barding: Splint` | 800 gp | — |
| `Barding: Studded Leather` | 180 gp | — |
| `Bit and bridle` | 2 gp | — |
| `Carriage` | 100 gp | — |
| `Cart` | 15 gp | — |
| `Chariot` | 250 gp | — |
| `Elephant` | 200 gp | 40 ft/round |
| `Galley` | 30000 gp | 4 mph |
| `Horse, draft` | 50 gp | 40 ft/round |
| `Horse, riding` | 75 gp | 60 ft/round |
| `Keelboat` | 3000 gp | 1 mph |
| `Longship` | 10000 gp | 3 mph |
| `Mastiff` | 25 gp | 40 ft/round |
| `Mule` | 8 gp | 40 ft/round |
| `Pony` | 30 gp | 40 ft/round |
| `Rowboat` | 50 gp | 1.5 mph |
| `Saddle, Exotic` | 60 gp | — |
| `Saddle, Military` | 20 gp | — |
| `Saddle, Pack` | 5 gp | — |
| `Saddle, Riding` | 10 gp | — |
| `Saddlebags` | 4 gp | — |
| `Sailing ship` | 10000 gp | 2 mph |
| `Sled` | 20 gp | — |
| `Stabling (1 day)` | 5 sp | — |
| `Wagon` | 35 gp | — |
| `Warhorse` | 400 gp | 60 ft/round |
| `Warship` | 25000 gp | 2.5 mph |",
        format!("{}", category.display_item_table(&items[..])),
    );
}

#[test]
fn potions() {
    let categories = item_categories().unwrap();
    let category = categories.iter().find(|i| i.name() == "Potions").unwrap();
    let magic_items = magic_items().unwrap();

    assert_eq!(
        "\
# Potions

* `Oil of Etherealness`
* `Oil of Sharpness`
* `Oil of Slipperiness`
* `Philter of Love`
* `Potion of Acid Resistance`
* `Potion of Animal Friendship`
* `Potion of Clairvoyance`
* `Potion of Climbing`
* `Potion of Cloud Giant Strength`
* `Potion of Cold Resistance`
* `Potion of Diminution`
* `Potion of Fire Giant Strength`
* `Potion of Fire Resistance`
* `Potion of Flying`
* `Potion of Force Resistance`
* `Potion of Frost Giant Strength`
* `Potion of Gaseous Form`
* `Potion of Greater Healing`
* `Potion of Growth`
* `Potion of Healing`
* `Potion of Heroism`
* `Potion of Hill Giant Strength`
* `Potion of Invisibility`
* `Potion of Lightning Resistance`
* `Potion of Mind Reading`
* `Potion of Necrotic Resistance`
* `Potion of Poison`
* `Potion of Poison Resistance`
* `Potion of Psychic Resistance`
* `Potion of Radiant Resistance`
* `Potion of Speed`
* `Potion of Stone Giant Strength`
* `Potion of Storm Giant Strength`
* `Potion of Superior Healing`
* `Potion of Supreme Healing`
* `Potion of Thunder Resistance`
* `Potion of Water Breathing`",
        format!(
            "{}",
            category.display_magic_item_list(&magic_items[..], "Potions"),
        ),
    );
}

#[test]
fn list_all_categories() {
    let mut categories: Vec<(String, Vec<String>)> = item_categories()
        .unwrap()
        .into_iter()
        .map(|category| {
            let mut names = vec![category.name()];
            names.append(&mut category.alt_names());
            (category.token(), names)
        })
        .collect();

    categories.sort_by(|(a, _), (b, _)| a.cmp(b));

    let categories_str: Vec<(&str, Vec<&str>)> = categories
        .iter()
        .map(|(a, b)| (a.as_str(), b.iter().map(|b| b.as_str()).collect()))
        .collect();

    assert_eq!(
        vec![
            (
                "AdventuringGear",
                vec!["Adventuring Gear", "Gear, Adventuring"],
            ),
            ("Ammunition", vec!["Ammunition"]),
            ("ArcaneFoci", vec!["Arcane Foci", "Foci, Arcane"]),
            ("Armor", vec!["Armor"]),
            ("ArtisansTools", vec!["Artisan's Tools", "Tools, Artisan's"]),
            ("DruidicFoci", vec!["Druidic Foci", "Foci, Druidic"]),
            (
                "EquipmentPacks",
                vec!["Equipment Packs", "Packs, Equipment"],
            ),
            ("GamingSets", vec!["Gaming Sets", "Sets, Gaming"]),
            ("HeavyArmor", vec!["Heavy Armor", "Armor, Heavy"]),
            ("HolySymbols", vec!["Holy Symbols", "Symbols, Holy"]),
            ("Kits", vec!["Kits"]),
            ("LandVehicles", vec!["Land Vehicles", "Vehicles, Land"]),
            ("LightArmor", vec!["Light Armor", "Armor, Light"]),
            (
                "MartialMeleeWeapons",
                vec!["Martial Melee Weapons", "Weapons, Martial Melee"],
            ),
            (
                "MartialRangedWeapons",
                vec!["Martial Ranged Weapons", "Weapons, Martial Ranged"],
            ),
            (
                "MartialWeapons",
                vec!["Martial Weapons", "Weapons, Martial"],
            ),
            ("MediumArmor", vec!["Medium Armor", "Armor, Medium"]),
            ("MeleeWeapons", vec!["Melee Weapons", "Weapons, Melee"]),
            (
                "MountsAndOtherAnimals",
                vec!["Mounts and Other Animals", "Animals"],
            ),
            ("MountsAndVehicles", vec!["Mounts and Vehicles"]),
            (
                "MusicalInstruments",
                vec!["Musical Instruments", "Instruments, Musical"],
            ),
            ("OtherTools", vec!["Other Tools", "Tools, Other"]),
            ("Potion", vec!["Potions"]),
            ("RangedWeapons", vec!["Ranged Weapons", "Weapons, Ranged"]),
            ("Ring", vec!["Rings"]),
            ("Rod", vec!["Rods"]),
            ("Scroll", vec!["Scrolls"]),
            ("Shields", vec!["Shields"]),
            (
                "SimpleMeleeWeapons",
                vec!["Simple Melee Weapons", "Weapons, Simple Melee"],
            ),
            (
                "SimpleRangedWeapons",
                vec!["Simple Ranged Weapons", "Weapons, Simple Ranged"],
            ),
            ("SimpleWeapons", vec!["Simple Weapons", "Weapons, Simple"]),
            ("Staff", vec!["Staves"]),
            ("StandardGear", vec!["Standard Gear", "Gear, Standard"]),
            (
                "TackHarnessAndDrawnVehicles",
                vec!["Tack, Harness, and Drawn Vehicles"],
            ),
            ("Tools", vec!["Tools"]),
            ("Wand", vec!["Wands"]),
            (
                "WaterborneVehicles",
                vec![
                    "Waterborne Vehicles",
                    "Vehicles, Waterborne",
                    "Ships",
                    "Boats"
                ],
            ),
            ("Weapon", vec!["Weapons"]),
            ("WondrousItems", vec!["Wondrous Items", "Items, Wondrous"]),
        ],
        categories_str
    );
}
