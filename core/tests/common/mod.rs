use initiative_core::{app, App, DataStore, Event, MemoryDataStore};
use tokio_test::block_on;

#[allow(dead_code)]
pub fn sync_app() -> SyncApp {
    sync_app_with_data_store(MemoryDataStore::default())
}

#[allow(dead_code)]
pub fn sync_app_with_data_store(data_store: impl DataStore + 'static) -> SyncApp {
    let mut app = SyncApp::new(data_store, &event_dispatcher);
    app.init();
    app
}

pub struct SyncApp(App);

fn event_dispatcher(_event: Event) {}

#[allow(dead_code)]
impl SyncApp {
    pub fn new<F: Fn(Event)>(
        data_store: impl DataStore + 'static,
        event_dispatcher: &'static F,
    ) -> Self {
        Self(app(data_store, event_dispatcher))
    }

    pub fn init(&mut self) -> &'static str {
        block_on(self.0.init())
    }

    pub fn command(&mut self, input: &str) -> Result<String, String> {
        block_on(self.0.command(input))
    }

    pub fn autocomplete(&self, input: &str) -> Vec<(String, String)> {
        block_on(self.0.autocomplete(input))
    }
}
