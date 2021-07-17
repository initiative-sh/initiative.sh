pub use app::AppCommand;
pub use autocomplete::Autocomplete;
pub use storage::StorageCommand;
pub use world::WorldCommand;

mod app;
mod autocomplete;
mod storage;
mod world;

use autocomplete::autocomplete_words;
use std::str::FromStr;

#[derive(Debug)]
pub enum Command {
    App(AppCommand),
    // Context(ContextCommand),
    World(WorldCommand),
    Storage(StorageCommand),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if let Ok(command) = raw.parse() {
            Ok(Command::App(command))
        } else if let Ok(command) = raw.parse() {
            Ok(Command::Storage(command))
        } else if let Ok(command) = raw.parse() {
            Ok(Command::World(command))
        } else {
            Err(())
        }
    }
}
