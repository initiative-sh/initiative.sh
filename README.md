# Introducing initiative.sh

initiative.sh's design philosophy is to minimize the time and effort between the
question ("Is there a blacksmith nearby?") and the answer ("Yes, it's called
Frosthammer & Sons, and Fenrik Frosthammer is at the forge.").

* **Keyboard first:** All commands can be typed. Most can be run at any time, so
  you don't need to waste time navigating menus to find the tool you want.
* **Persistence:** The people and places you generate remain visible in your
  scrollback history and can be saved to your journal, so you don't need to
  worry about forgetting to scribble down a generated name that came up in
  conversation.

### Other features on the roadmap

The following features have not yet been implemented:

* [**Context:**](https://github.com/orgs/initiative-sh/projects/2) With your
  guidance, initiative.sh will track your party's location and the demographics
  in the area. If you're in a dwarvish settlement, the innkeeper and most of the
  patrons will probably be dwarves.
* [**Integrations:**](https://github.com/orgs/initiative-sh/projects/3)
  Integrate with Spotify to switch playlists as your players move through the
  world, with Home Assistant to dim the lights as the party beds down for the
  night, or use web hooks to build your own integrations.
* [**Cloud sync:**](https://github.com/initiative-sh/initiative.sh/issues/149)
  Keep your journal in sync between devices.

## Running the project

Dependencies:

* [Rust/Cargo](https://www.rust-lang.org/learn/get-started)
* [wasm-pack](https://rustwasm.github.io/) (for web)
* [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm) (for
  web)

Note that the project includes
[5e-database](https://github.com/5e-bits/5e-database) as a Git submodule, so
actions such as building and starting a dev server may fail until you run:

    git submodule update --init

### Web

1. Run `./dev-server.sh` from the project root at the command line.
2. Find the dev server URL in the command line output and open it in your
   browser. Typically, this will be [localhost:8080](http://localhost:8080/).

### Command line

*Note: The command line interface lost feature parity with the web version early
in the development process. Notably, it lacks autocomplete support, and the
"rich" version doesn't support scrolling or text formatting. For details, see
[#287](https://github.com/initiative-sh/initiative.sh/issues/287).*

#### Rich version

    cargo run

#### Light version

The light version is selected when the input is not from a tty, such as when you
pipe a command into `cargo run`.

    echo npc | cargo run

# Contributing to the project

Please see
[CONTRIBUTING.md](https://github.com/initiative-sh/initiative.sh/blob/main/CONTRIBUTING.md)
for details.
