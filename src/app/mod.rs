use std::fmt::Display;
use std::io;

mod context;
mod interface;
mod parser;

pub use parser::syntax;
pub use parser::{AppCommand, Command, GenerateCommand, RawCommand};

use context::Context;

pub fn run() -> io::Result<()> {
    let context = context::Context::default();
    let app = App::new(context);
    interface::run(app)
}

pub struct App {
    context: Context,
}

impl App {
    fn new(context: Context) -> App {
        App { context }
    }

    fn command(&mut self, raw_command: &str) -> Box<dyn Display> {
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
