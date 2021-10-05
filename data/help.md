# Help

Commands are intended to be typed, although underlined words are also clickable
to make it easier to discover new commands. They're also intended to be as
intuitive and succinct as possible.

To generate a random thing, simply type its name. Generators will include a list
of alternatives, in case the first suggestion isn't appropriate to your
situation. You can see more details about an alternative by typing its name or
the number listed next to it.

* `npc` (`human`, `elf`, etc.)
* `building` (`inn`)

Proper (capitalized) nouns will give you existing people, places, or reference
items from the 5th edition D&D Systems Reference Document. For instance:

* `spells` (`Acid Arrow` to `Zone Of Truth`)
* `weapons`, `adventuring gear`, `tools`, etc. (`Abacus` to `Yew Wand`)
* more to come

The journal allows you to save and load generated NPCs, locations, etc. Entries
in your journal are saved to your browser and will be available next time you
visit initiative.sh.

* `journal` lists all journal entries.
* `save [name]` saves a generated NPC, location, etc. to your journal.
* `load [name]` (or just `[name]`) loads the named entry from your journal or
  recently generated entries.
* `delete [name]` deletes a journal entry.

The journal also tracks the current time. When you start a game, the time is day
1 at 8:00 am.

* `now` shows the current time.
* `+[number][d, h, m, s, or r]` advances time by a given number of days, hours,
  minutes, seconds, or rounds.  For instance, `+8h` advances time by 8 hours.
* `-[number][d, h, m, s, or r]` rewinds time by the same.
* You can skip the number to advance or rewind time by a single unit, so `+d`
  advances to the next day.

Of course, no DM tool would be complete without a dice roller: `roll [formula]`
or simply `[formula]`.
Some examples to get you started:

* `8d6: Fireball`
* `d20+3: dexterity check with +3 bonus`
* `2d20k1+5: +5 attack roll with disadvantage` (k = keep low)
* `2d20d1+5: +5 attack roll with advantage` (d = drop low)
* `(d4+1)^3: magic missile` (rolls 3 times)
* See [the documentation](https://github.com/Geobert/caith/blob/v4.2.0/README.md#syntax)
  for more options.
