# Help

_For a hands-on overview of these commands, why not try the `tutorial`?_

Commands are intended to be typed, although underlined words are also clickable
to make it easier to discover new commands. They're also intended to be as
intuitive and succinct as possible.

To generate a random thing, simply describe it. This can be simple, as in typing
the name of the thing you're looking for, or more complex, describing details of
that thing.

* `character` (`human`, `elf`, etc.)
* `inn` (currently the only Place for which a name generator exists)
* `a human boy named Roger`
* `Nevermoor, a moor`

Existing things can be edited by describing them with "is", for instance:

* once you have created `a character named Roger`, you can say that
  `Roger is a halfling`

You can invoke terms from the 5th edition D&D Systems Reference Document to pull
up the relevant details or rule reference. For instance:

* `spells` (from `Acid Arrow` to `Zone of Truth`)
* `weapons`, `adventuring gear`, `tools`, etc. (from `Abacus` to `Yew Wand`)
* conditions (`exhaustion`, `paralyzed`, etc.)
* traits (`stonecunning`, `lucky`, `hellish resistance`)
* more to come

The journal allows you to save and load generated characters, places, etc.
Entries in your journal are saved locally in your browser and will be available
next time you visit initiative.sh.

* `journal` lists all journal entries.
* `save [name]` saves a generated character, place, etc. to your journal.
* `[name]` (or `load [name]`) loads the named entry from your journal or
  recently generated entries.
* `delete [name]` deletes a journal entry.
* `export` and `import` journal backups.

The journal also tracks the current time. When you start a game, the time is day
1 at 8:00 am.

* `now` shows the current time.
* `+[number][d, h, m, s, or r]` advances time by a given number of days, hours,
  minutes, seconds, or rounds.  For instance, `+8h` advances time by 8 hours.
* `-[number][d, h, m, s, or r]` rewinds time by the same.
* You can skip the number to advance or rewind time by a single unit, so `+d`
  advances to the next day.

Of course, no DM tool would be complete without a dice roller: `roll [formula]`
or simply `[formula]`. Here are some examples to get you started:

* `8d6: Fireball`
* `d20+3: dexterity check with +3 bonus`
* `2d20k1+5: +5 attack roll with disadvantage` (k = keep low)
* `2d20d1+5: +5 attack roll with advantage` (d = drop low)
* `(d4+1)^3: magic missile` (rolls 3 times)
