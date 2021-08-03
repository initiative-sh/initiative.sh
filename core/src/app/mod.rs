pub use command::{autocomplete_phrase, AppCommand, Command, Runnable};
pub use meta::AppMeta;

mod command;
mod meta;

use crate::storage::DataStore;
use initiative_macros::motd;
use rand::prelude::*;

pub struct App<DS: DataStore> {
    meta: AppMeta,
    _data_store: DS,
    rng: SmallRng,
}

impl<DS: DataStore> App<DS> {
    pub fn new(meta: AppMeta, data_store: DS) -> App<DS> {
        App {
            meta,
            _data_store: data_store,
            rng: SmallRng::from_entropy(),
        }
    }

    pub fn motd(&self) -> &'static str {
        motd!()
    }

    pub async fn command(&mut self, input: &str) -> String {
        if let Some(command) = self.meta.command_aliases.get(input).cloned() {
            command.run(&mut self.meta, &mut self.rng)
        } else if let Some(command) = Command::parse_input(input, &self.meta).first() {
            command.run(&mut self.meta, &mut self.rng)
        } else {
            format!("Unknown command: \"{}\"", input)
        }
    }

    pub async fn autocomplete(&self, input: &str) -> Vec<(String, String)> {
        Command::autocomplete(input, &self.meta)
            .drain(..)
            .map(|(s, c)| (s, c.summarize().to_string()))
            .collect()
    }
}
