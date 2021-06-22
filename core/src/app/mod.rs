use rand::prelude::*;
use rand::rngs::SmallRng;
use std::fmt::Display;

mod context;
mod parser;

pub use context::Context;
pub use parser::syntax;
pub use parser::{AppCommand, Command, RawCommand, StorageCommand, WorldCommand};

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

    pub fn command(&mut self, raw_command: &str) -> Box<dyn Display> {
        let command_subtype: Command = raw_command.parse().unwrap();

        match command_subtype {
            Command::App(c) => command(&c, &mut self.context),
            Command::Storage(c) => crate::storage::command(&c, &mut self.context),
            Command::World(c) => crate::world::command(&c, &mut self.context, &mut self.rng),
            Command::Unknown(c) => Box::new(format!("{:?}", c)),
        }
    }
}

pub fn command(command: &AppCommand, context: &mut Context) -> Box<dyn Display> {
    match command {
        AppCommand::Debug(_) => Box::new(format!("{:?}", context)),
        app_command => Box::new(format!("{:?}", app_command)),
    }
}
