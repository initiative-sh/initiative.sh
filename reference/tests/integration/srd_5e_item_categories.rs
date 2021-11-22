use initiative_reference::srd_5e::{item_categories, items, magic_items};

#[test]
fn armor() {
    let categories = item_categories().unwrap();
    let category = categories.iter().find(|i| i.name() == "armor").unwrap();
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
    let category = categories.iter().find(|i| i.name() == "weapons").unwrap();
    let items = items().unwrap();

    assert_eq!(
        "\
# Weapons

| Name | Cost | Damage | Weight | Properties |
|---|--:|---|--:|---|
| `Battleaxe` | 10 gp | 1d8 slashing | 4 lb. | Versatile (1d10) |
| `Blowgun` | 10 gp | 1d1 piercing | 1 lb. | Ammunition (range 25/100), loading |
| `Club` | 1 sp | 1d4 bludgeoning | 2 lb. | Light, monk |
| `Crossbow, Hand` | 75 gp | 1d6 piercing | 3 lb. | Ammunition (range 30/120), light, loading |
| `Crossbow, Heavy` | 50 gp | 1d10 piercing | 18 lb. | Ammunition (range 100/400), heavy, loading, two-handed |
| `Crossbow, Light` | 25 gp | 1d8 piercing | 5 lb. | Ammunition (range 80/320), loading, two-handed |
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
| `Light Hammer` | 2 gp | 1d4 bludgeoning | 2 lb. | Light, monk, thrown (range 20/60) |
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
| `War Pick` | 5 gp | 1d8 piercing | 2 lb. | — |
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
        .find(|i| i.name() == "adventuring gear")
        .unwrap();
    let items = items().unwrap();

    assert_eq!(
        "\
# Adventuring Gear

| Name | Cost | Weight |
|---|--:|--:|
| `Abacus` | 2 gp | 2 lb. |
| `Acid (Vial)` | 25 gp | 1 lb. |
| `Alchemist's Fire (Flask)` | 50 gp | 1 lb. |
| `Alms Box` | 0 cp | 0 lb. |
| `Amulet` | 5 gp | 1 lb. |
| `Antitoxin (Vial)` | 50 gp | 0 lb. |
| `Arrow` | 1 gp | 1 lb. |
| `Backpack` | 2 gp | 5 lb. |
| `Ball Bearings (Bag Of 1,000)` | 1 gp | 2 lb. |
| `Barrel` | 2 gp | 70 lb. |
| `Basket` | 4 sp | 2 lb. |
| `Bedroll` | 1 gp | 7 lb. |
| `Bell` | 1 gp | 0 lb. |
| `Blanket` | 5 sp | 3 lb. |
| `Block And Tackle` | 1 gp | 5 lb. |
| `Block Of Incense` | 0 cp | 0 lb. |
| `Blowgun Needle` | 1 gp | 1 lb. |
| `Book` | 25 gp | 5 lb. |
| `Bottle, Glass` | 2 gp | 2 lb. |
| `Bucket` | 5 cp | 2 lb. |
| `Burglar's Pack` | 16 gp | — |
| `Caltrops` | 5 cp | 2 lb. |
| `Candle` | 1 cp | 0 lb. |
| `Case, Crossbow Bolt` | 1 gp | 1 lb. |
| `Case, Map Or Scroll` | 1 gp | 1 lb. |
| `Censer` | 0 cp | 0 lb. |
| `Chain (10 Feet)` | 5 gp | 10 lb. |
| `Chalk (1 Piece)` | 1 cp | 0 lb. |
| `Chest` | 5 gp | 25 lb. |
| `Climber's Kit` | 25 gp | 12 lb. |
| `Clothes, Common` | 5 sp | 3 lb. |
| `Clothes, Costume` | 5 gp | 4 lb. |
| `Clothes, Fine` | 15 gp | 6 lb. |
| `Clothes, Traveler's` | 2 gp | 4 lb. |
| `Component Pouch` | 25 gp | 2 lb. |
| `Crossbow Bolt` | 1 gp | 1.5 lb. |
| `Crowbar` | 2 gp | 5 lb. |
| `Crystal` | 10 gp | 1 lb. |
| `Diplomat's Pack` | 39 gp | — |
| `Disguise Kit` | 25 gp | 3 lb. |
| `Dungeoneer's Pack` | 12 gp | — |
| `Emblem` | 5 gp | 0 lb. |
| `Entertainer's Pack` | 40 gp | — |
| `Explorer's Pack` | 10 gp | — |
| `Fishing Tackle` | 1 gp | 4 lb. |
| `Flask Or Tankard` | 2 cp | 1 lb. |
| `Forgery Kit` | 15 gp | 5 lb. |
| `Grappling Hook` | 2 gp | 4 lb. |
| `Hammer` | 1 gp | 3 lb. |
| `Hammer, Sledge` | 2 gp | 10 lb. |
| `Healer's Kit` | 5 gp | 3 lb. |
| `Herbalism Kit` | 5 gp | 3 lb. |
| `Holy Water (Flask)` | 25 gp | 1 lb. |
| `Hourglass` | 25 gp | 1 lb. |
| `Hunting Trap` | 5 gp | 25 lb. |
| `Ink (1 Ounce Bottle)` | 10 gp | 0 lb. |
| `Ink Pen` | 2 cp | 0 lb. |
| `Jug Or Pitcher` | 2 cp | 4 lb. |
| `Ladder (10-Foot)` | 1 sp | 25 lb. |
| `Lamp` | 5 sp | 1 lb. |
| `Lantern, Bullseye` | 10 gp | 2 lb. |
| `Lantern, Hooded` | 5 gp | 2 lb. |
| `Little Bag Of Sand` | 0 cp | 0 lb. |
| `Lock` | 10 gp | 1 lb. |
| `Magnifying Glass` | 100 gp | 0 lb. |
| `Manacles` | 2 gp | 6 lb. |
| `Mess Kit` | 2 sp | 1 lb. |
| `Mirror, Steel` | 5 gp | 0.5 lb. |
| `Oil (Flask)` | 1 sp | 1 lb. |
| `Orb` | 20 gp | 3 lb. |
| `Paper (One Sheet)` | 2 sp | 0 lb. |
| `Parchment (One Sheet)` | 1 sp | 0 lb. |
| `Perfume (Vial)` | 5 gp | 0 lb. |
| `Pick, Miner's` | 2 gp | 10 lb. |
| `Piton` | 5 cp | 0.25 lb. |
| `Poison, Basic (Vial)` | 100 gp | 0 lb. |
| `Poisoner's Kit` | 50 gp | 2 lb. |
| `Pole (10-Foot)` | 5 cp | 7 lb. |
| `Pot, Iron` | 2 gp | 10 lb. |
| `Pouch` | 5 sp | 1 lb. |
| `Priest's Pack` | 19 gp | — |
| `Quiver` | 1 gp | 1 lb. |
| `Ram, Portable` | 4 gp | 35 lb. |
| `Rations (1 Day)` | 5 sp | 2 lb. |
| `Reliquary` | 5 gp | 2 lb. |
| `Robes` | 1 gp | 4 lb. |
| `Rod` | 10 gp | 2 lb. |
| `Rope, Hempen (50 Feet)` | 1 gp | 10 lb. |
| `Rope, Silk (50 Feet)` | 10 gp | 5 lb. |
| `Sack` | 1 cp | 0.5 lb. |
| `Scale, Merchant's` | 5 gp | 3 lb. |
| `Scholar's Pack` | 40 gp | — |
| `Sealing Wax` | 5 sp | 0 lb. |
| `Shovel` | 2 gp | 5 lb. |
| `Signal Whistle` | 5 cp | 0 lb. |
| `Signet Ring` | 5 gp | 0 lb. |
| `Sling Bullet` | 4 cp | 1.5 lb. |
| `Small Knife` | 0 cp | 0 lb. |
| `Soap` | 2 cp | 0 lb. |
| `Spellbook` | 50 gp | 3 lb. |
| `Spike, Iron` | 1 sp | 5 lb. |
| `Sprig Of Mistletoe` | 1 gp | 0 lb. |
| `Spyglass` | 1000 gp | 1 lb. |
| `Staff` | 5 gp | 4 lb. |
| `String (10 Feet)` | 0 cp | 0 lb. |
| `Tent, Two-Person` | 2 gp | 20 lb. |
| `Tinderbox` | 5 sp | 1 lb. |
| `Torch` | 1 cp | 1 lb. |
| `Totem` | 1 gp | 0 lb. |
| `Vestments` | 0 cp | 0 lb. |
| `Vial` | 1 gp | 0 lb. |
| `Wand` | 10 gp | 1 lb. |
| `Waterskin` | 2 sp | 5 lb. |
| `Whetstone` | 1 cp | 1 lb. |
| `Wooden Staff` | 5 gp | 4 lb. |
| `Yew Wand` | 10 gp | 1 lb. |",
        format!("{}", category.display_item_table(&items[..])),
    );
}

#[test]
fn tools() {
    let categories = item_categories().unwrap();
    let category = categories.iter().find(|i| i.name() == "tools").unwrap();
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
| `Cook's Utensils` | 1 gp | 8 lb. |
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
| `Pan Flute` | 12 gp | 2 lb. |
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
        .find(|i| i.name() == "mounts and vehicles")
        .unwrap();
    let items = items().unwrap();

    assert_eq!(
        "\
# Mounts And Vehicles

| Name | Cost | Speed |
|---|--:|--:|
| `Animal Feed (1 Day)` | 5 cp | — |
| `Barding: Breastplate` | 1600 gp | — |
| `Barding: Chain Mail` | 300 gp | — |
| `Barding: Chain Shirt` | 200 gp | — |
| `Barding: Half Plate` | 3000 gp | — |
| `Barding: Hide` | 40 gp | — |
| `Barding: Leather` | 40 gp | — |
| `Barding: Padded` | 20 gp | — |
| `Barding: Plate` | 6000 gp | — |
| `Barding: Ring Mail` | 12 gp | — |
| `Barding: Scale Mail` | 200 gp | — |
| `Barding: Splint` | 800 gp | — |
| `Barding: Studded Leather` | 180 gp | — |
| `Bit And Bridle` | 2 gp | — |
| `Carriage` | 100 gp | — |
| `Cart` | 15 gp | — |
| `Chariot` | 250 gp | — |
| `Elephant` | 200 gp | 40 ft/round |
| `Galley` | 30000 gp | 4 mph |
| `Horse, Draft` | 50 gp | 40 ft/round |
| `Horse, Riding` | 75 gp | 60 ft/round |
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
| `Sailing Ship` | 10000 gp | 2 mph |
| `Sled` | 20 gp | — |
| `Stabling (1 Day)` | 5 sp | — |
| `Wagon` | 35 gp | — |
| `Warhorse` | 400 gp | 60 ft/round |
| `Warship` | 25000 gp | 2.5 mph |",
        format!("{}", category.display_item_table(&items[..])),
    );
}

#[test]
fn potions() {
    let categories = item_categories().unwrap();
    let category = categories.iter().find(|i| i.name() == "potions").unwrap();
    let magic_items = magic_items().unwrap();

    assert_eq!(
        "\
# Potions

* `Oil Of Etherealness`
* `Oil Of Sharpness`
* `Oil Of Slipperiness`
* `Philter Of Love`
* `Potion Of Animal Friendship`
* `Potion Of Clairvoyance`
* `Potion Of Climbing`
* `Potion Of Diminution`
* `Potion Of Flying`
* `Potion Of Gaseous Form`
* `Potion Of Giant Strength`
* `Potion Of Growth`
* `Potion Of Healing`
* `Potion Of Heroism`
* `Potion Of Invisibility`
* `Potion Of Mind Reading`
* `Potion Of Poison`
* `Potion Of Resistance`
* `Potion Of Speed`
* `Potion Of Water Breathing`",
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
        .drain(..)
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
                vec!["adventuring gear", "gear, adventuring"],
            ),
            ("Ammunition", vec!["ammunition"]),
            ("ArcaneFoci", vec!["arcane foci", "foci, arcane"]),
            ("Armor", vec!["armor"]),
            ("ArtisansTools", vec!["artisan's tools", "tools, artisan's"]),
            ("DruidicFoci", vec!["druidic foci", "foci, druidic"]),
            (
                "EquipmentPacks",
                vec!["equipment packs", "packs, equipment"],
            ),
            ("GamingSets", vec!["gaming sets", "sets, gaming"]),
            ("HeavyArmor", vec!["heavy armor", "armor, heavy"]),
            ("HolySymbols", vec!["holy symbols", "symbols, holy"]),
            ("Kits", vec!["kits"]),
            ("LandVehicles", vec!["land vehicles", "vehicles, land"]),
            ("LightArmor", vec!["light armor", "armor, light"]),
            (
                "MartialMeleeWeapons",
                vec!["martial melee weapons", "weapons, martial melee"],
            ),
            (
                "MartialRangedWeapons",
                vec!["martial ranged weapons", "weapons, martial ranged"],
            ),
            (
                "MartialWeapons",
                vec!["martial weapons", "weapons, martial"],
            ),
            ("MediumArmor", vec!["medium armor", "armor, medium"]),
            ("MeleeWeapons", vec!["melee weapons", "weapons, melee"]),
            (
                "MountsAndOtherAnimals",
                vec!["mounts and other animals", "animals"],
            ),
            ("MountsAndVehicles", vec!["mounts and vehicles"]),
            (
                "MusicalInstruments",
                vec!["musical instruments", "instruments, musical"],
            ),
            ("OtherTools", vec!["other tools", "tools, other"]),
            ("Potion", vec!["potions"]),
            ("RangedWeapons", vec!["ranged weapons", "weapons, ranged"]),
            ("Ring", vec!["rings"]),
            ("Rod", vec!["rods"]),
            ("Scroll", vec!["scrolls"]),
            ("Shields", vec!["shields"]),
            (
                "SimpleMeleeWeapons",
                vec!["simple melee weapons", "weapons, simple melee"],
            ),
            (
                "SimpleRangedWeapons",
                vec!["simple ranged weapons", "weapons, simple ranged"],
            ),
            ("SimpleWeapons", vec!["simple weapons", "weapons, simple"]),
            ("Staff", vec!["staves"]),
            ("StandardGear", vec!["standard gear", "gear, standard"]),
            (
                "TackHarnessAndDrawnVehicles",
                vec!["tack, harness, and drawn vehicles"],
            ),
            ("Tools", vec!["tools"]),
            ("Wand", vec!["wands"]),
            (
                "WaterborneVehicles",
                vec![
                    "waterborne vehicles",
                    "vehicles, waterborne",
                    "ships",
                    "boats"
                ],
            ),
            ("Weapon", vec!["weapons"]),
            ("WondrousItems", vec!["wondrous items", "items, wondrous"]),
        ],
        categories_str
    );
}
