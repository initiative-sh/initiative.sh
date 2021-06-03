use std::io;

use super::context::Context;

mod rich;

pub fn run(context: Context) -> io::Result<()> {
    rich::run(context)
}
