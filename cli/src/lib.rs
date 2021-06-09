use std::io;

use initiative_core::app::App;

mod light;
mod rich;

pub fn run(app: App) -> io::Result<()> {
    if termion::is_tty(&io::stdin()) {
        rich::run(app)
    } else {
        light::run(app)
    }
}
