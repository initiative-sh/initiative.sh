//! | Case                            | Tested by            |
//! |---------------------------------|----------------------|
//! | Token: 1 word                   | alarm                |
//! | Token: 2+ words                 | acid_splash          |
//! | Token: Non-alphanumeric symbols | blindness_deafness   |
//! | Level: cantrip                  | acid_splash          |
//! | Level: 1                        | alarm                |
//! | Level: 2                        | blindness_deafness   |
//! | Level: 3                        | animate_dead         |
//! | Level: 4+                       | dispel_evil_and_good |
//! | Ritual                          | alarm                |
//! | Non-ritual                      | acid_splash          |
//! | AoE                             | alarm                |
//! | Non-AoE                         | acid_splash          |
//! | Components: 1                   | blindness_deafness   |
//! | Components: 2+                  | acid_splash          |
//! | Components: with materials      | alarm                |
//! | Concentration                   | dispel_evil_and_good |
//! | Non-concentration               | acid_splash          |
//! | Description: 1 line             | blindness_deafness   |
//! | Description: 2+ lines           | acid_splash          |
//! | Description: list               | augury               |
//! | At higher levels: none          | acid_splash          |
//! | At higher levels: 1 line        | blindness_deafness   |

use initiative_reference::srd_5e::spells;

#[test]
fn acid_splash() {
    let spells = spells().unwrap();
    let spell = spells.iter().find(|s| s.name() == "Acid Splash").unwrap();

    assert_eq!("AcidSplash", spell.token());
    assert_eq!(
        "`Acid Splash` (conjuration cantrip)",
        format!("{}", spell.display_summary()),
    );
    assert_eq!(
        "\
# Acid Splash
*Conjuration cantrip*

**Casting Time:** 1 action\\
**Range:** 60 feet\\
**Components:** V, S\\
**Duration:** Instantaneous

You hurl a bubble of acid. Choose one creature within range, or choose two creatures within range that are within 5 feet of each other. A target must succeed on a dexterity saving throw or take 1d6 acid damage.

This spell's damage increases by 1d6 when you reach 5th level (2d6), 11th level (3d6), and 17th level (4d6).",
        format!("{}", spell.display_details()),
    );
}

#[test]
fn alarm() {
    let spells = spells().unwrap();
    let spell = spells.iter().find(|s| s.name() == "Alarm").unwrap();

    assert_eq!("Alarm", spell.token());
    assert_eq!(
        "`Alarm` (1st-level abjuration)",
        format!("{}", spell.display_summary()),
    );
    assert_eq!(
        "\
# Alarm
*1st-level abjuration (ritual)*

**Casting Time:** 1 minute\\
**Range:** 30 feet (20' cube)\\
**Components:** V, S, M (a tiny bell and a piece of fine silver wire)\\
**Duration:** 8 hours

You set an alarm against unwanted intrusion. Choose a door, a window, or an area within range that is no larger than a 20-foot cube. Until the spell ends, an alarm alerts you whenever a Tiny or larger creature touches or enters the warded area. When you cast the spell, you can designate creatures that won't set off the alarm. You also choose whether the alarm is mental or audible.

A mental alarm alerts you with a ping in your mind if you are within 1 mile of the warded area. This ping awakens you if you are sleeping.

An audible alarm produces the sound of a hand bell for 10 seconds within 60 feet.",
        format!("{}", spell.display_details()),
    );
}

#[test]
fn blindness_deafness() {
    let spells = spells().unwrap();
    let spell = spells
        .iter()
        .find(|s| s.name() == "Blindness/Deafness")
        .unwrap();

    assert_eq!("BlindnessDeafness", spell.token());
    assert_eq!(
        "`Blindness/Deafness` (2nd-level necromancy)",
        format!("{}", spell.display_summary()),
    );
    assert_eq!(
        "\
# Blindness/Deafness
*2nd-level necromancy*

**Casting Time:** 1 action\\
**Range:** 30 feet\\
**Components:** V\\
**Duration:** 1 minute

You can blind or deafen a foe. Choose one creature that you can see within range to make a constitution saving throw. If it fails, the target is either blinded or deafened (your choice) for the duration. At the end of each of its turns, the target can make a constitution saving throw. On a success, the spell ends.

***At higher levels:*** When you cast this spell using a spell slot of 3rd level or higher, you can target one additional creature for each slot level above 2nd.",
        format!("{}", spell.display_details()),
    );
}

#[test]
fn animate_dead() {
    let spells = spells().unwrap();
    let spell = spells.iter().find(|s| s.name() == "Animate Dead").unwrap();

    assert_eq!("AnimateDead", spell.token());
    assert_eq!(
        "`Animate Dead` (3rd-level necromancy)",
        format!("{}", spell.display_summary()),
    );
    assert_eq!(
        "\
# Animate Dead
*3rd-level necromancy*

**Casting Time:** 1 minute\\
**Range:** 10 feet\\
**Components:** V, S, M (a drop of blood, a piece of flesh, and a pinch of bone dust)\\
**Duration:** Instantaneous

This spell creates an undead servant. Choose a pile of bones or a corpse of a Medium or Small humanoid within range. Your spell imbues the target with a foul mimicry of life, raising it as an undead creature. The target becomes a skeleton if you chose bones or a zombie if you chose a corpse (the DM has the creature's game statistics).

On each of your turns, you can use a bonus action to mentally command any creature you made with this spell if the creature is within 60 feet of you (if you control multiple creatures, you can command any or all of them at the same time, issuing the same command to each one). You decide what action the creature will take and where it will move during its next turn, or you can issue a general command, such as to guard a particular chamber or corridor. If you issue no commands, the creature only defends itself against hostile creatures. Once given an order, the creature continues to follow it until its task is complete.

The creature is under your control for 24 hours, after which it stops obeying any command you've given it. To maintain control of the creature for another 24 hours, you must cast this spell on the creature again before the current 24-hour period ends. This use of the spell reasserts your control over up to four creatures you have animated with this spell, rather than animating a new one.

***At higher levels:*** When you cast this spell using a spell slot of 4th level or higher, you animate or reassert control over two additional undead creatures for each slot level above 3rd. Each of the creatures must come from a different corpse or pile of bones.",
        format!("{}", spell.display_details()),
    );
}

#[test]
fn dispel_evil_and_good() {
    let spells = spells().unwrap();
    let spell = spells
        .iter()
        .find(|s| s.name() == "Dispel Evil And Good")
        .unwrap();

    assert_eq!("DispelEvilAndGood", spell.token());
    assert_eq!(
        "`Dispel Evil And Good` (5th-level abjuration)",
        format!("{}", spell.display_summary()),
    );
    assert_eq!(
        "\
# Dispel Evil And Good
*5th-level abjuration*

**Casting Time:** 1 action\\
**Range:** Self\\
**Components:** V, S, M (holy water or powdered silver and iron)\\
**Duration:** Concentration, up to 1 minute

Shimmering energy surrounds and protects you from fey, undead, and creatures originating from beyond the Material Plane. For the duration, celestials, elementals, fey, fiends, and undead have disadvantage on attack rolls against you.

You can end the spell early by using either of the following special functions.

Break Enchantment.

As your action, you touch a creature you can reach that is charmed, frightened, or possessed by a celestial, an elemental, a fey, a fiend, or an undead. The creature you touch is no longer charmed, frightened, or possessed by such creatures.

Dismissal.

As your action, make a melee spell attack against a celestial, an elemental, a fey, a fiend, or an undead you can reach. On a hit, you attempt to drive the creature back to its home plane. The creature must succeed on a charisma saving throw or be sent back to its home plane (if it isn't there already). If they aren't on their home plane, undead are sent to the Shadowfell, and fey are sent to the Feywild.",
        format!("{}", spell.display_details()),
    );
}

#[test]
fn augury() {
    let spells = spells().unwrap();
    let spell = spells.iter().find(|s| s.name() == "Augury").unwrap();

    assert_eq!("Augury", spell.token());
    assert_eq!(
        "`Augury` (2nd-level divination)",
        format!("{}", spell.display_summary())
    );
    assert_eq!(
        "\
# Augury
*2nd-level divination (ritual)*

**Casting Time:** 1 minute\\
**Range:** Self\\
**Components:** V, S, M (specially marked sticks, bones, or similar tokens worth at least 25gp)\\
**Duration:** Instantaneous

By casting gem-inlaid sticks, rolling dragon bones, laying out ornate cards, or employing some other divining tool, you receive an omen from an otherworldly entity about the results of a specific course of action that you plan to take within the next 30 minutes. The DM chooses from the following possible omens:

- Weal, for good results
- Woe, for bad results
- Weal and woe, for both good and bad results
- Nothing, for results that aren't especially good or bad

The spell doesn't take into account any possible circumstances that might change the outcome, such as the casting of additional spells or the loss or gain of a companion.

If you cast the spell two or more times before completing your next long rest, there is a cumulative 25 percent chance for each casting after the first that you get a random reading. The DM makes this roll in secret.",
        format!("{}", spell.display_details()),
    );
}
