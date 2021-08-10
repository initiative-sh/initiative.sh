pub use command::{autocomplete_phrase, AppCommand, Command, Runnable};
pub use meta::AppMeta;

mod command;
mod meta;

use crate::storage::repository;
use initiative_macros::motd;

pub struct App {
    meta: AppMeta,
}

impl App {
    pub fn new(meta: AppMeta) -> App {
        App { meta }
    }

    pub async fn init(&mut self) -> &'static str {
        repository::init_cache(&mut self.meta).await;
        motd!()
    }

    pub async fn command(&mut self, input: &str) -> String {
        if let Some(command) = self.meta.command_aliases.get(input).cloned() {
            command.run(&mut self.meta).await
        } else if let Some(command) = Command::parse_input(input, &self.meta).first() {
            command.run(&mut self.meta).await
        } else {
            format!("Unknown command: \"{}\"", input)
        }
    }

    pub async fn autocomplete(&self, input: &str) -> Vec<(String, String)> {
        Command::autocomplete(input, &self.meta)
    }
}
