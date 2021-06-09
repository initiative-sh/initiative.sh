pub mod app;
mod world;

use app::syntax;

pub fn app() -> app::App {
    let context = app::context::Context::default();
    app::App::new(context)
}
