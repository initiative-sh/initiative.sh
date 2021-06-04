use std::io;

use super::App;

mod rich;

pub fn run(app: App) -> io::Result<()> {
    rich::run(app)
}
