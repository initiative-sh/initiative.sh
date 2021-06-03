use std::io;

mod context;
mod interface;

pub fn run() -> io::Result<()> {
    let context = context::Context::default();
    interface::run(context)
}
