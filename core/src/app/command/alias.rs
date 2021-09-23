use super::{Command, Runnable};
use crate::app::AppMeta;
use async_trait::async_trait;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem;

#[derive(Clone, Debug)]
pub struct CommandAlias {
    term: String,
    summary: String,
    command: Box<Command>,
}

impl CommandAlias {
    pub fn new(term: String, summary: String, command: Command) -> Self {
        Self {
            term,
            summary,
            command: Box::new(command),
        }
    }
}

impl Hash for CommandAlias {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.term.hash(state);
    }
}

impl PartialEq for CommandAlias {
    fn eq(&self, other: &Self) -> bool {
        self.term == other.term
    }
}

impl Eq for CommandAlias {}

#[async_trait(?Send)]
impl Runnable for CommandAlias {
    async fn run(&self, app_meta: &mut AppMeta) -> Result<String, String> {
        let mut temp_aliases = mem::take(&mut app_meta.command_aliases);

        let result = self.command.run(app_meta).await;

        if app_meta.command_aliases.is_empty() {
            app_meta.command_aliases = temp_aliases;
        } else {
            temp_aliases.drain().for_each(|command| {
                if !app_meta.command_aliases.contains(&command) {
                    app_meta.command_aliases.insert(command);
                }
            });
        }

        result
    }

    fn parse_input(input: &str, app_meta: &AppMeta) -> (Option<Self>, Vec<Self>) {
        (
            app_meta
                .command_aliases
                .iter()
                .find(|c| c.term == input)
                .cloned(),
            Vec::new(),
        )
    }

    fn autocomplete(input: &str, app_meta: &AppMeta) -> Vec<(String, String)> {
        app_meta
            .command_aliases
            .iter()
            .filter(|c| c.term.starts_with(input))
            .map(|c| (c.term.clone(), c.summary.clone()))
            .collect()
    }
}

impl fmt::Display for CommandAlias {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.term)
    }
}
