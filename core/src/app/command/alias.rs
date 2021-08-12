use super::{Command, Runnable};
use crate::app::AppMeta;
use async_trait::async_trait;

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
        self.command.run(app_meta).await
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
