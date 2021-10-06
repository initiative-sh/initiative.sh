use super::{Command, Runnable};
use crate::app::AppMeta;
use async_trait::async_trait;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem;

#[derive(Clone, Debug)]
pub enum CommandAlias {
    Literal {
        term: String,
        summary: String,
        command: Box<Command>,
    },
}

impl CommandAlias {
    pub fn literal(term: String, summary: String, command: Command) -> Self {
        Self::Literal {
            term,
            summary,
            command: Box::new(command),
        }
    }
}

impl Hash for CommandAlias {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Literal { term, .. } => term.hash(state),
        }
    }
}

impl PartialEq for CommandAlias {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Literal { term, .. },
                Self::Literal {
                    term: other_term, ..
                },
            ) => term == other_term,
        }
    }
}

impl Eq for CommandAlias {}

#[async_trait(?Send)]
impl Runnable for CommandAlias {
    async fn run(&self, input: &str, app_meta: &mut AppMeta) -> Result<String, String> {
        match self {
            Self::Literal { command, .. } => {
                let mut temp_aliases = mem::take(&mut app_meta.command_aliases);

                let result = command.run(input, app_meta).await;

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
        }
    }

    fn parse_input(input: &str, app_meta: &AppMeta) -> (Option<Self>, Vec<Self>) {
        (
            app_meta
                .command_aliases
                .iter()
                .find(|command| match command {
                    Self::Literal { term, .. } => term == input,
                })
                .cloned(),
            Vec::new(),
        )
    }

    fn autocomplete(input: &str, app_meta: &AppMeta) -> Vec<(String, String)> {
        app_meta
            .command_aliases
            .iter()
            .filter_map(|command| match command {
                Self::Literal { term, summary, .. } => {
                    if term.starts_with(input) {
                        Some((term.clone(), summary.clone()))
                    } else {
                        None
                    }
                }
            })
            .collect()
    }
}

impl fmt::Display for CommandAlias {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Literal { term, .. } => write!(f, "{}", term),
        }
    }
}
