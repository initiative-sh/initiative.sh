mod light;
mod rich;

use initiative_core::app::App;
use std::io;

pub fn run(app: App) -> io::Result<()> {
    if termion::is_tty(&io::stdin()) {
        rich::run(app)
    } else {
        light::run(app)
    }
}
