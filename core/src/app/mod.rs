pub use command::{AppCommand, Command, StorageCommand, WorldCommand};
pub use context::Context;

mod command;
mod context;

use rand::prelude::*;
use rand::rngs::SmallRng;
use std::fmt::Display;

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

    pub fn command(&mut self, input: &str) -> Box<dyn Display> {
        match input.parse() {
            Ok(Command::App(c)) => command(&c, &mut self.context),
            Ok(Command::Storage(c)) => crate::storage::command(&c, &mut self.context),
            Ok(Command::World(c)) => crate::world::command(&c, &mut self.context, &mut self.rng),
            Err(()) => Box::new(format!("Unknown command: \"{}\"", input)),
        }
    }
}

pub fn command(command: &AppCommand, context: &mut Context) -> Box<dyn Display> {
    match command {
        AppCommand::Debug => Box::new(format!("{:?}", context)),
    }
}
