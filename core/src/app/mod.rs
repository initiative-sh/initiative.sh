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

    pub fn motd(&self) -> &'static str {
        include_str!("../../../data/motd.md").trim_end()
    }

    pub fn command(&mut self, input: &str) -> String {
        if let Some(command) = self.context.command_aliases.get(input).cloned() {
            command.run(&mut self.context, &mut self.rng)
        } else if let Some(command) = Command::parse_input(&input, &self.context).first() {
            command.run(&mut self.context, &mut self.rng)
        } else {
            format!("Unknown command: \"{}\"", input)
        }
    }

    pub fn autocomplete(&self, input: &str) -> Vec<(String, String)> {
        Command::autocomplete(input, &self.context)
            .drain(..)
            .map(|(s, c)| (s, c.summarize().to_string()))
            .collect()
    }
}
