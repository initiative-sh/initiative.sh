use std::fmt::Display;
use std::io;

mod context;
mod interface;
mod parser;

pub use parser::syntax;

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

    fn command(&mut self, command: &str) -> Box<impl Display> {
        self.context.run(&command.parse().unwrap())
    }
}
