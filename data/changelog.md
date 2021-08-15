* **Enhancement:** You can now look up SRD magic items such as the inimitable
  `Deck Of Many Things`.
* **Enhancement:** Rewrite inn name generator to provide a greater variety of
  results.
* **New:** New command `journal` lists everything you've saved to your journal.
* **Enhancement:** Autocomplete suggestions when loading a thing will include
  a brief summary of that thing, eg. "adult human, they/them".
* **Enhancement:** Attempting to save a thing that has already been saved will
  give a distinct error from saving a thing that doesn't exist.
* **Enhancement:** Autocomplete now properly suggests save and load commands.
* **Enhancement:** When viewing an NPC that has not yet been saved to your
  journal, a note will appear reminding you to save.
* **Enhancement:** Temporary commands (those that are only valid in the current
  context) now have a ~dashed underline~, distinguishing them from permanent
  commands (that are always valid) like `help`.
* **Enhancement:** Autocomplete suggestions now provide more detail about the
  command being suggested.
* **New:** You can now save generated content with `save [name]`. Saved content
  will be available on future page loads. A lot of work remains to refine the
  usability of this feature.
* **Bug:** Fix fallback font for browsers that don't support or haven't yet
  loaded the default Source Code Pro font.
* **Bug:** Fixed `changelog` output being truncated and not always displaying 10
  entries as intended.
* **Enhancement:** Adopted Source Code Pro for the interface font. Most
  importantly, the common font ensures that the blocks in the logo are always
  the same width.
* **Bug:** Fixed the page not scrolling predictably. Scrolling behaviour should
  also respect browsers configured to prefer reduced motion.
* **Enhancement:** Added equipment, all of the essentials like `Dagger` and
  `Bagpipes`, as well as categories like `weapons` and `ships`.
* **Enhancement:** New command: `spells`.
* **Bug:** Fixed rendering inline lists in spell descriptions.
* **Bug:** Fixed capitalization of possessives: `Arcanist's Magic Aura` is no
  longer capitalized as "Arcanist'S".
* **Enhancement:** Added content formatting to improve readability, while
  remaining within the bounds of what is possible with an actual terminal.
* **Enhancement/bug:** Improve mobile experience, including styling and
  bugfixes.
* **Enhancement:** Keywords are now clickable, like this: `npc`.
* **New:** Added commands `about`, `help`, and `changelog`.
* **New:** Added "message of the day" and intro text.
* **Enhancement:** Improved experience for users with JavaScript disabled.
* **New:** SRD spells have been added. Try `Acid Arrow`.
* **Enhancement:** Included at-a-glance summary of autocomplete suggestions.
* **New:** An autocomplete popup is now available in the web UI.
* **Enhancement:** Restyled to use the Solarized colour scheme.
* **New:** Aliases are now available for newly-generated NPCs, eg. `0` to get
  alternative 0.
* **Enhancement:** Proper nouns are now included in autocomplete suggestions.
* **Enhancement:** Basic commands are now included in autocomplete suggestions.
* **Enhancement:** Commands now parse more strictly, which ironically enables
  more flexibility in the future.
* **Enhancement:** All occurrences of "race" have been changed to "species".
* **New:** Previously-generated content is now saved to memory and can be loaded
  by saying its name.
* **Bug:** Specifying a species would not restrict suggestions to that species.
* **New:** Implemented a basic web frontend.
