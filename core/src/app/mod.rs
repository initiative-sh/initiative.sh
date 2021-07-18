pub use autocomplete::{autocomplete_phrase, Autocomplete};
pub use command::{AppCommand, Command};
pub use context::Context;

mod autocomplete;
mod command;
mod context;

use rand::prelude::*;
use rand::rngs::SmallRng;

pub struct App {
    context: Context,
    rng: SmallRng,
}

impl App {
    pub fn new(context: Context) -> App {
        App {
            context,
            rng: SmallRng::from_entropy(),
        }
    }

    pub fn command(&mut self, input: &str) -> String {
        if let Ok(command) = self
            .context
            .command_aliases
            .get(input)
            .map_or(input, |s| s.as_str())
            .parse::<Command>()
        {
            command.run(&mut self.context, &mut self.rng)
        } else {
            format!("Unknown command: \"{}\"", input)
        }
    }

    pub fn autocomplete(&self, input: &str) -> Vec<String> {
        Command::autocomplete(input, &self.context)
    }
}
