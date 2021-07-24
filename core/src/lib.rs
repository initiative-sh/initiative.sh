pub mod app;

mod reference;
mod storage;
mod world;

pub fn app() -> app::App {
    let context = app::Context::default();
    app::App::new(context)
}
