* **Bug:** Accessing the command history using the up and down arrows will no
  longer wrap around when you reach the beginning or end of the history.
* **Bug:** Clicking the right mouse button no longer runs commands or changes
  the selection. This was particularly annoying when trying to copy from the
  context menu.
* **New:** You can now `export` data from the `journal`! (There is not yet a
  corresponding import command, but that's coming next.)
* **Enhancement:** Changes to the `time` are now properly integrated into the
  `undo` history and available at all times.
* **Enhancement:** Refactored out the local cache. In practice, the only effect
  this should have right now is that if you have multiple tabs/windows open at
  once, the journal should remain in sync between all of them.
* **Enhancement:** All place types now include an emoji icon in summary view,
  such as in the `journal` listing.
* **Enhancement:** Just for good measure, added a wee favicon to the site.
* **Enhancement:** You can now navigate backwards and forwards through your
  command history with the up and down arrow keys while the autocomplete popup
  is closed. The history doesn't (yet!) persist between sessions.
* **Enhancement:** By popular request, autocomplete and command parsing are now
  completely case-insensitive!
* **Bug:** Fix the text colour of the input box when browsing in dark mode.
  Black on black is hard to read, I guess???
* **Enhancement:** Minor updates to vocabulary (somehow I missed `hotel` and
  `pub`, both of which I ran into during last night's game). I've also added a
  [Buy Me a Coffee](https://www.buymeacoffee.com/initiative) link for those who
  would like to support development, though I currently have no premium features
  to offer supporters. Just the same site with extra gratitude!
* **Enhancement:** Added tons of new place types, from `kingdom` to `pet-store`
  and everything in between (183 types and aliases in total). However, lacking
  name generators for all of these new types, their current purpose is just to
  make it easier to keep a coherent `journal`.
* **Bug:** Fixed the "more" command not being available when generating a
  `person` or `place` in a browser that lacks IndexedDB, such as an older
  browser or one with private browsing active.
* **Enhancement:** When you generate a `person` or `place`, alternative
  suggestions aren't offered automatically, but are instead available via the
  "more" command. Also updated the `tutorial` and `help` to cover the latest
  feature changes.
* **Enhancement:** Added box displays when generating a `character` or `place`,
  as well as emoji icons to differentiate people by age/gender.
* **Enhancement:** Creating and editing things will now proceed even if not all
  words are recognized, so long as the parser can make a reasonable guess about
  what you meant. If it guesses wrong, that's what `undo` is for.
* **Bug:** Fix broken text selection, which was preventing copy/paste
  operations. I wasn't trying to do the annoying "right click disabled" thing
  from the early '00s, but it sure felt like it.
* **New:** You can now edit things in your journal and recent items! Try
  `[name] is [description]` to get started.
* **Enhancement:** The `redo` command is now available at all times, not just
  immediately after `undo`.
* **Enhancement:** Improved readability by setting a max width to the page and
  increasing line spacing between user input and command output.
* **New:** Added `undo` to fix those little mistakes. Mostly this frees us to be
  more liberal with interpreting commands, since it removes the risk of changing
  something you don't want if your command is misinterpreted.
* **Enhancement:** Fixed the description of the `Detect Thoughts` spell.
* **New:** Substantially improved the process of creating NPCs and locations.
  Rather than being limited to simple terms like `inn` or `tiefling`, you can
  now generate `The Prancing Poodle, an inn`, an `old elvish woman`, or `a boy
  named Sue`. (Editing of existing things is coming next.)
* **Enhancement:** Things are now automatically saved upon creation if you
  include a name as one of the parameters. This should streamline the process of
  importing multiple things in sequence.
* **Enhancement:** NPCs, locations, etc. can no longer have the same names as
  one another, and the random generator should never suggest a thing with the
  same name as one that already exists. Sounds anticlimactic, but this is a
  substantial change under the hood, and an important stepping stone to creating
  and editing your own journal entries.
* **Enhancement:** Corrected terminology so that NPCs using "they/them" pronouns
  will be referred to as "non-binary" rather than "trans".
* **Enhancement:** The `tutorial` is now rather less brutal. Instead of killing
  the poor bartender, our hypothetical murderhobos just let the inn burn to the
  ground and delete that instead. (Spoilers, BTW.)
* **New:** Added a `tutorial`! This will be updated very occasionally as new
  features are added, but initiative.sh is now sufficiently complex and
  feature-complete to be worth the effort.
* **Enhancement:** On mobile, the keyboard will no longer pop up when clicking a
  link.
* **Enhancement:** Input field is now focused whenever you click or tap anywhere
  on the page, improving the "stickiness" of the experience, especially on
  mobile.
* **Enhancement:** Changes to the game `time` are now saved between sessions.
  There's still some work to do before this feature is worth featuring more
  prominently, but at least it's in a usable state now.
* **Enhancement:** Better behaviour if several names come into conflict, like in
  the case of `Shield`, or the automated test that failed because it decided to
  name a character `Lance`.
* **Enhancement:** Added support for more complex time expressions, like
  `+1d5h`.
* **New:** Added a privacy policy outlining what personal information we
  collect. Currently: we don't, which is why we haven't had a privacy policy
  until now.
* **Enhancement:** Added a loading message if the site is taking a while to load
  or crashes silently in the background.
* **New:** Rectified a major oversight for a D&D app: added a dice roller! Get
  your `Fireball` on with `roll 8d6` or use `roll (d4+1)^3` for `Magic Missile`.
* **Enhancement:** Rewrote name generators to work syllable by syllable. You
  might end up with some awkward names, but you'll have access to a much greater
  variety of names as a result.
* **Enhancement:** Inns now enjoy the same ergonomics as NPCs: you can reference
  alternative suggestions with numerals, and the "save" shortcut is now
  available where applicable.
* **"Enhancement":** Removed buildings such as residence and temple for the time
  being. They weren't really generating anything interesting and were blocking
  features that expect all Things to have a name.
* **Bug:** Fixed things being silently dropped from the recent journal if the
  save operation failed.
* **Bug:** Fixed formatting in error messages not being displayed.
* **New:** New additions to the changelog are now automatically announced to the
  #releases Discord channel!
* **Enhancement:** This is just a dummy update to make sure Discord handles
  multiple bullet points nicely.
* **Enhancement:** Tab key now fills as much text as possible. Try typing "weap"
  and pressing tab.
* **New:** There's a Discord server now! See the link in the welcome message to
  join.
* **Enhancement:** Tab key can be used to select an autocomplete option without
  submitting. If the selection includes a bracketed phrase like `load [name]`,
  the autofill will update to suggest ways to complete the phrase.
* **New:** Time! See the current time with `now`, advance and rewind time with
  `+1d`, `-1h`, etc. Time does not yet persist between sessions.
* **Enhancement:** You can now delete entries from your journal with
  `delete [name]`. This completes the basic journal functionality.
* **Enhancement:** Users that can't save to local storage (eg. private browsing,
  Firefox Focus) will see appropriate error messages instead of a broken site.
* **Enhancement:** You can now look up SRD magic items such as the inimitable
  `Deck Of Many Things`, as well as categories like `magic weapons` and
  `wondrous items`.
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
