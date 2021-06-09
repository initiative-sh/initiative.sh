use std::fmt::Display;

pub mod context;
mod parser;

pub use parser::syntax;
pub use parser::{AppCommand, Command, GenerateCommand, RawCommand};

use context::Context;

pub struct App {
    context: Context,
}

impl App {
    pub fn new(context: Context) -> App {
        App { context }
    }

    pub fn command(&mut self, raw_command: &str) -> Box<dyn Display> {
        let command: Command = raw_command.parse().unwrap();
        let demographics = &self.context.demographics;

        match command {
            Command::App(app_command) => Box::new(format!("{:?}", app_command)),
            Command::Generate(generate_command) => {
                crate::world::command(&generate_command, demographics)
            }
            Command::Unknown(raw_command) => Box::new(format!("{:?}", raw_command)),
        }
    }
}
