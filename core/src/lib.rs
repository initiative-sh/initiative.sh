//! This crate forms the core of the initiative.sh application. It is a common dependency of the
//! `initiative_web` crate containing the web version of the app, and the `initiative_cli` crate
//! containing the (incomplete) terminal version.
//!
//! It communicates to the outside world through the [`app::App`] struct, which exposes essentially
//! the entirety of the crate's public API (constructed using the [`app()`] function). See the
//! documentation of these two entities for details on that API.

pub mod app;

pub use app::{App, Event};
pub use storage::backup::BackupData;
pub use storage::{DataStore, MemoryDataStore, NullDataStore};
pub use uuid::Uuid;
pub use world::thing::Thing;

mod command;
mod reference;
mod storage;
mod time;
mod utils;
mod world;

/// Creates a new instance of the application wrapper. The `data_store` is used to save and load
/// data from storage, and the `event_dispatcher` is a callback function invoked whenever an
/// event occurs in-app that may require special handling by the UI. See [`Event`] for details.
pub fn app<F: Fn(Event)>(
    data_store: impl DataStore + 'static,
    event_dispatcher: &'static F,
) -> app::App {
    /*
    Overflow(
        Match {
            token: Token {
                token_type: Phrase([
                    Token {
                        token_type: Keyword("Legolas"),
                        marker: Keyword,
                    },
                    Token {
                        token_type: AnyPhrase,
                        marker: AnyPhrase,
                    },
                    Token {
                        token_type: AnyWord,
                        marker: AnyWord,
                    },
                ]),
                marker: Phrase,
            },
            phrase: Word {
                phrase: " an elf",
                inner_range: 1..3,
                outer_range: 1..3,
            },
            meta: Sequence([
                Match {
                    token: Token {
                        token_type: Keyword("Legolas"),
                        marker: Keyword,
                    },
                    phrase: Word {
                        phrase: "Legolas is an elf",
                        inner_range: 0..7,
                        outer_range: 0..7,
                    },
                    meta: None,
                },
                Match {
                    token: Token {
                        token_type: AnyPhrase,
                        marker: AnyPhrase,
                    },
                    phrase: Word {
                        phrase: " is an elf",
                        inner_range: 1..3,
                        outer_range: 1..3,
                    },
                    meta: None,
                },
                Match {
                    token: Token {
                        token_type: AnyWord,
                        marker: AnyWord,
                    },
                    phrase: Word {
                        phrase: " an elf",
                        inner_range: 1..3,
                        outer_range: 1..3,
                    },
                    meta: None,
                },
            ]),
        },
        " elf",
    );
    */

    let app_meta = app::AppMeta::new(data_store, event_dispatcher);
    app::App::new(app_meta)
}
