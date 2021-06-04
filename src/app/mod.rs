use std::io;

mod context;
mod interface;
mod parser;

pub use parser::syntax;

pub fn run() -> io::Result<()> {
    let context = context::Context::default();
    interface::run(context)
}
