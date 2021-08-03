use initiative_core::{app, App, NullDataStore};
use tokio_test::block_on;

pub fn sync_app() -> SyncApp {
    SyncApp(app())
}

pub struct SyncApp(App<NullDataStore>);

#[allow(dead_code)]
impl SyncApp {
    pub fn motd(&self) -> &'static str {
        self.0.motd()
    }

    pub fn command(&mut self, input: &str) -> String {
        block_on(self.0.command(input))
    }

    pub fn autocomplete(&self, input: &str) -> Vec<(String, String)> {
        block_on(self.0.autocomplete(input))
    }
}
