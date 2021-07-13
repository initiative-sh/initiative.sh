use std::fmt::Display;

mod context;
mod parser;

pub use context::Context;
pub use parser::syntax;
pub use parser::{AppCommand, Command, GenerateCommand, RawCommand};

pub struct App {
    context: Context,
}

impl App {
    pub fn new(context: Context) -> App {
        App { context }
    }

    pub fn command(&mut self, raw_command: &str) -> Box<dyn Display> {
        let command_subtype: Command = raw_command.parse().unwrap();

        match command_subtype {
            Command::App(app_command) => command(&app_command, &mut self.context),
            Command::Generate(generate_command) => {
                crate::world::command(&generate_command, &mut self.context)
            }
            Command::Unknown(raw_command) => Box::new(format!("{:?}", raw_command)),
        }
    }
}

pub fn command(command: &AppCommand, context: &mut Context) -> Box<dyn Display> {
    match command {
        AppCommand::Debug(_) => Box::new(format!("{:?}", context)),
        app_command => Box::new(format!("{:?}", app_command)),
    }
}
