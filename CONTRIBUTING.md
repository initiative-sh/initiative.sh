# Contributing to initiative.sh

Awesome! Thank you for your interest in contributing to the project.

initiative.sh is a unique kind of web app that has a unique set of design
principles and constraints. **For this reason, unapproved contributions are
likely to be regretfully declined.** Please follow the contribution process
detailed below to avoid wasted effort.

### How to contribute

1. Create or comment on an [open
   issue](https://github.com/initiative-sh/initiative.sh/issues). Wait for
   consensus and approval before beginning work, especially with regard to
   command syntax and autocomplete behaviour.
2. For nontrivial work, post a work-in-progress pull request with a rough draft
   of your change.
3. Complete implementation and add test coverage for your change. Include a
   brief summary of your change to the top of
   [data/changelog.md](https://github.com/initiative-sh/initiative.sh/blob/main/data/changelog.md),
   which will be displayed to users upon login, in the `changelog` command, and
   announced on Discord.

When in doubt, ask! I try to be as responsive as possible [on
Discord](https://discord.gg/ZrqJPpxXVZ) and [by
email](mailto:support@initiative.sh).

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
