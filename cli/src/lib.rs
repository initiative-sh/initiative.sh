mod light;
mod rich;

use initiative_core::App;
use std::io;

pub async fn run(app: App) -> io::Result<()> {
    if termion::is_tty(&io::stdin()) {
        rich::run(app).await
    } else {
        light::run(app).await
    }
}
