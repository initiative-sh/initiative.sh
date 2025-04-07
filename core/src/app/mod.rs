pub use command::{
    AppCommand, Autocomplete, AutocompleteSuggestion, Command, CommandAlias, CommandMatches,
    ContextAwareParse, Runnable,
};
pub use meta::AppMeta;

mod command;
mod meta;

use crate::storage::backup::{import, BackupData};
use crate::utils::CaseInsensitiveStr;
use initiative_macros::motd;

/// The application wrapper. Its inner [`AppMeta`] object holds metadata associated with the
/// application, including ephemeral storage of journal entries and the object representing the
/// underlying data storage.
///
/// The main methods of interest are [`App::command`], called when a command is run by the user,
/// and [`App::autocomplete`], called to get a list of suggestions for a given input.
#[derive(Debug)]
pub struct App {
    meta: AppMeta,
}

/// An event that can occur while the app is running that may require special handling by the UI.
#[derive(Debug)]
pub enum Event {
    /// The user typed the `export` command and the journal backup is ready to download.
    Export(BackupData),

    /// The user typed the `import` command and should be prompted to select a file to import.
    Import,
}

impl App {
    pub fn new(meta: AppMeta) -> App {
        App { meta }
    }

    /// Initialize a running application. This is done as a separate step from the constructor
    /// because it runs asynchronously. Its purpose, in turn, is to trigger the underlying data
    /// store to initialize, which may involve opening a database connection.
    pub async fn init(&mut self) -> &'static str {
        self.meta.repository.init().await;
        let (motd, motd_len) = motd!("! Local storage is not available in your browser. You will be able to use initiative.sh, but anything you save will not persist beyond this session.");

        if self.meta.repository.data_store_enabled() {
            &motd[..motd_len]
        } else {
            motd
        }
    }

    /// The user typed an input and pressed Enter. What happens?
    ///
    /// On success or failure, returns a String that can be displayed back to the user.
    pub async fn command(&mut self, input: &str) -> Result<String, String> {
        Command::parse_input_irrefutable(input, &self.meta)
            .await
            .run(input, &mut self.meta)
            .await
    }

    /// The user has updated their input and a new set of suggestions should be populated. This
    /// consists of a `Vec` of tuples; the first entry being the text that the user is suggested to
    /// type, the second being a brief (1-3--word) description of what that input will do. `Cow` is
    /// used here to allow either `String` or `&'static str`, whatever is appropriate to a given
    /// case.
    ///
    /// Returns a maximum of 10 results.
    pub async fn autocomplete(&self, input: &str) -> Vec<AutocompleteSuggestion> {
        let mut suggestions: Vec<_> = Command::autocomplete(input, &self.meta).await;
        suggestions.sort_by(|a, b| a.term.cmp_ci(&b.term));
        suggestions.truncate(10);
        suggestions
    }

    /// The part of the import flow that occurs after the user selects a file in response to the
    /// [`Event::Import`].
    pub async fn bulk_import(&mut self, data: BackupData) -> Result<String, String> {
        import(&mut self.meta.repository, data)
            .await
            .map(|stats| stats.to_string())
            .map_err(|_| "Failed to import.".to_string())
    }
}
