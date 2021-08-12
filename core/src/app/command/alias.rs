use super::{Command, Runnable};
use crate::app::AppMeta;
use async_trait::async_trait;
use std::mem;

#[derive(Clone, Debug, PartialEq)]
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

#[async_trait(?Send)]
impl Runnable for CommandAlias {
    async fn run(&self, app_meta: &mut AppMeta) -> String {
        let mut temp_aliases = mem::take(&mut app_meta.command_aliases);

        let result = self.command.run(app_meta).await;

        if app_meta.command_aliases.is_empty() {
            app_meta.command_aliases = temp_aliases;
        } else {
            temp_aliases.drain().for_each(|(alias, command)| {
                app_meta.command_aliases.entry(alias).or_insert(command);
            });
        }

        result
    }

    fn parse_input(input: &str, app_meta: &AppMeta) -> Vec<Self> {
        app_meta
            .command_aliases
            .get(input)
            .iter()
            .map(|&c| c.clone())
            .collect()
    }

    fn autocomplete(input: &str, app_meta: &AppMeta) -> Vec<(String, String)> {
        app_meta
            .command_aliases
            .keys()
            .filter(|s| s.starts_with(input))
            .filter_map(|s| {
                Self::parse_input(s, app_meta)
                    .drain(..)
                    .next()
                    .map(|c| (c.term, c.summary))
            })
            .collect()
    }
}
