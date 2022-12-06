# Contributing to initiative.sh

Everyone's experience as a game master is different. Thank you for bringing your
unique perspective to this project.

initiative.sh is a particular kind of tool for a particular kind of game master,
who favours off-the-cuff world building over extensive prep and isn't
intimidated by a command line (but doesn't necessarily relish reading docs). If
that's you, great! Welcome aboard. If not, that's totally okay - the ecosystem
surrounding D&D and pen-and-paper RPGs in general is truly mind-boggling, and
you'll find tons of other tools that might suit your play style better.

I've deliberately resisted installing any sort of metric tracking on the
website, so feedback in the form of bug reports and feature requests is
invaluable. If you're motivated to fix a bug or implement a feature yourself,
that's even better, and of course the [GitHub
issues](https://github.com/initiative-sh/initiative.sh/issues) are kept curated
to provide plenty of opportunities to dive into the code if you're so inclined.

Optimizing for user experience is by far the hardest aspect of this project.
Due to the reliance on discoverability over documentation, it's important to
keep that experience as tight and consistent as possible. For this reason, I
prefer to maintain creative control over project features and direction. One UX
choice or priority isn't necessarily objectively better than another, but by
keeping decision-making centralized, I hope to foster a more coherent, usable
product for everyone.

### How to contribute

1. Create or comment on an [open
   issue](https://github.com/initiative-sh/initiative.sh/issues). Wait for
   approval before beginning work, especially with regard to command syntax and
   autocomplete behaviour.
2. For nontrivial work, post a work-in-progress pull request with a rough draft
   of your change for review and discussion.
3. Complete implementation and add test coverage for your change. Include a
   brief summary of your change to the top of
   [data/changelog.md](https://github.com/initiative-sh/initiative.sh/blob/main/data/changelog.md),
   which will be displayed to users upon login, in the `changelog` command, and
   announced on Discord.

When in doubt, ask! I try to be as responsive as possible [on
Discord](https://discord.gg/ZrqJPpxXVZ) and [by
email](mailto:support@initiative.sh). I'm also happy to jump into a pair
programming or brainstorming session as time permits.

### UX design principles

One of the hardest challenges to introducing new features is doing so in a way
that is consistent with the unique design possibilities and constraints of the
text UI.

* **Keyboard first:** All actions can be executed using the keyboard only.
* **Discoverable:** Links in output text introduce new commands that can be run.
  Autocomplete suggestions introduce new command subtypes, eg. by suggesting
  that `elf` can be modified with `young`.
* **Intuitive:** New commands should be as intuitive as possible, more
  reminiscent of a MUD or other text-based RPG than of a traditional command
  line. This means using plain English syntax (even if it might be unnecessarily
  verbose), multiple synonyms, and forgiving grammar parsing (eg. ignoring words
  like "a" and "the").

These principles sometimes conflict with new features. I'm reluctant to adopt
new features (of my own or someone else's conception) until I have a clear idea
of how the user can interact with them according to those design principles.
Building an intuitive command line that does not require thorough reading of
docs is a delicate balancing act.

## Development

*For dependencies and instructions to run the project, see
[README.md](https://github.com/initiative-sh/initiative.sh#running-the-project).*

**Note:** Webpack should automatically reload when you make changes to the
Javascript code, but you will need to manually rebuild the WebAssembly module in
order to see your changes. I recommend re-running `./dev-server.sh` to refresh;
it will not spawn additional servers if one is already active, but it will
rebuild the module.

### Coding standards

All Rust code should conform to `rustfmt` and `cargo clippy`. This is enforced
by the local test script as well as upon posting pull requests.

## Testing

Run `./test.sh` at the command line from the project root to run all automated
tests, including `cargo clippy` and `rustfmt` checks.

### Rust unit tests

Code should be covered by unit tests wherever possible. Testing follows the Rust
convention of using a `mod test` within each module.

Don't forget when running tests manually that the project uses a
[workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html), which
means that `cargo test` *won't* automatically cover the entire application.
Instead, use:

    cargo test --workspace

Or better yet, run the full suite:

    ./test.sh

### Rust integration tests

The integration tests primarily verify that a certain text input or set of text
inputs result in an expected output. They can be found in `core/tests`. All
features should be covered in broad strokes in the integration tests. Corner
cases can be covered at a more granular level in unit tests.

### Browser tests

There are no in-browser tests yet! If you're interested in working on that, see
[#61](https://github.com/initiative-sh/initiative.sh/issues/61).

## Helpful links

* [initiative.sh Discord](https://discord.gg/ZrqJPpxXVZ)
* [Project
  tracker](https://github.com/initiative-sh/initiative.sh/projects?query=is%3Aopen)
* [Technical docs](https://initiative.sh/doc/initiative_core/)

## Maintainers

* [Mikkel Paulson](https://mikkel.ca/): initiative@email.mikkel.ca
