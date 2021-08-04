use initiative_core::app;
use initiative_core::app::App;
use tokio_test::block_on;

pub fn autocomplete(app: &App, input: &str) -> Vec<(String, String)> {
    block_on(app.autocomplete(input))
}

pub fn app_autocomplete(input: &str) -> Vec<(String, String)> {
    autocomplete(&app(), input)
}
