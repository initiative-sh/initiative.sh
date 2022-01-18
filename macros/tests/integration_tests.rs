mod app {
    pub use initiative_core::app::{AppMeta, Autocomplete, ContextAwareParse, Event};
}

mod utils {
    pub use initiative_core::utils::{CaseInsensitiveStr, QuotedWordChunk, QuotedWords};
}

mod integration;

use initiative_core::app::{AppMeta, Event};
use initiative_core::NullDataStore;

fn get_app_meta() -> AppMeta {
    AppMeta::new(NullDataStore::default(), &null_dispatcher)
}

fn null_dispatcher(_event: Event) {}
