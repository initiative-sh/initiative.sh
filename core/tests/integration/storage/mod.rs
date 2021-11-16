mod change;
mod export_import;
mod journal;
mod load;
mod undo_redo;

use crate::common::SyncApp;
use initiative_core::{Event, MemoryDataStore, NullDataStore};

fn event_dispatcher(_event: Event) {}

#[test]
fn startup_error_with_unusable_data_store() {
    {
        let mut app = SyncApp::new(NullDataStore::default(), &event_dispatcher);
        let output = app.init();
        assert!(
            output.contains("Local storage is not available in your browser."),
            "{}",
            output,
        );
    }

    {
        let mut app = SyncApp::new(MemoryDataStore::default(), &event_dispatcher);
        let output = app.init();
        assert!(
            !output.contains("Local storage is not available in your browser."),
            "{}",
            output,
        );
    }
}
