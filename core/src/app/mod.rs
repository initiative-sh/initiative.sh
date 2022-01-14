pub use command::{AppCommand, Autocomplete, Command, CommandAlias, ContextAwareParse, Runnable};
pub use meta::AppMeta;

#[cfg(test)]
pub use command::assert_autocomplete;

mod command;
mod meta;

use crate::storage::backup::{import, BackupData};
use crate::utils::CaseInsensitiveStr;
use initiative_macros::motd;
use std::borrow::Cow;

pub struct App {
    meta: AppMeta,
}

#[derive(Debug)]
pub enum Event {
    Export(BackupData),
    Import,
}

impl App {
    pub fn new(meta: AppMeta) -> App {
        App { meta }
    }

    pub async fn init(&mut self) -> &'static str {
        self.meta.repository.init().await;
        let (motd, motd_len) = motd!("! Local storage is not available in your browser. You will be able to use initiative.sh, but anything you save will not persist beyond this session.");

        if self.meta.repository.data_store_enabled() {
            &motd[..motd_len]
        } else {
            motd
        }
    }

    pub async fn command(&mut self, input: &str) -> Result<String, String> {
        Command::parse_input_irrefutable(input, &self.meta)
            .await
            .run(input, &mut self.meta)
            .await
    }

    pub async fn autocomplete(&self, input: &str) -> Vec<(Cow<'static, str>, Cow<'static, str>)> {
        let mut suggestions: Vec<_> = Command::autocomplete(input, &self.meta, true).await;
        suggestions.sort_by(|(a, _), (b, _)| a.cmp_ci(b));
        suggestions.truncate(10);
        suggestions
    }

    pub async fn bulk_import(&mut self, data: BackupData) -> Result<String, String> {
        import(&mut self.meta.repository, data)
            .await
            .map(|stats| stats.to_string())
            .map_err(|_| "Failed to import.".to_string())
    }
}
