//! This is the incomplete terminal interface for initiative.sh. It was the only version of the
//! application for the first several months of development, so the core app functionality ran and
//! continues to run fine in a command line context. However, the `initiative_web` crate has since
//! surpassed the command line in terms of features, specifically:
//!
//! * No autocomplete suggestions are displayed
//! * Markdown output is displayed literally rather than being formatted
//! * No scrolling is present in the rich interface
//! * No data storage is available
//! * Import/export don't work

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
