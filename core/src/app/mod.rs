pub use command::{autocomplete_phrase, AppCommand, Command, Runnable};
pub use context::Context;

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
        let commands = Command::parse_input(
            self.context
                .command_aliases
                .get(input)
                .map_or(input, |s| s.as_str()),
            &self.context,
        );

        if let Some(command) = commands.first() {
            command.run(&mut self.context, &mut self.rng)
        } else {
            format!("Unknown command: \"{}\"", input)
        }
    }

    pub fn autocomplete(&self, input: &str) -> Vec<String> {
        Command::autocomplete(input, &self.context)
            .drain(..)
            .map(|(s, _)| s)
            .collect()
    }
}
