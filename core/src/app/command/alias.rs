use super::{Autocomplete, Command, ContextAwareParse, Runnable};
use crate::app::AppMeta;
use crate::utils::CaseInsensitiveStr;
use async_trait::async_trait;
use std::borrow::Cow;
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
    StrictWildcard {
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

    pub fn strict_wildcard(command: Command) -> Self {
        Self::StrictWildcard {
            command: Box::new(command),
        }
    }

    pub fn get_command(&self) -> &Command {
        match self {
            Self::Literal { command, .. } => command,
            Self::StrictWildcard { command, .. } => command,
        }
    }
}

impl Hash for CommandAlias {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Literal { term, .. } => {
                if term.chars().any(char::is_uppercase) {
                    term.to_lowercase().hash(state);
                } else {
                    term.hash(state);
                }
            }
            Self::StrictWildcard { .. } => {}
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
            ) => term.eq_ci(other_term),
            (Self::StrictWildcard { .. }, Self::StrictWildcard { .. }) => true,
            _ => false,
        }
    }
}

impl Eq for CommandAlias {}

#[async_trait(?Send)]
impl Runnable for CommandAlias {
    async fn run(self, input: &str, app_meta: &mut AppMeta) -> Result<String, String> {
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
            Self::StrictWildcard { .. } => {
                app_meta.command_aliases.remove(&self);
                if let Self::StrictWildcard { command } = self {
                    command.run(input, app_meta).await
                } else {
                    unreachable!();
                }
            }
        }
    }
}

#[async_trait(?Send)]
impl ContextAwareParse for CommandAlias {
    async fn parse_input(input: &str, app_meta: &AppMeta) -> (Option<Self>, Vec<Self>) {
        (
            app_meta
                .command_aliases
                .iter()
                .find(|c| matches!(c, Self::StrictWildcard { .. }))
                .or_else(|| {
                    app_meta
                        .command_aliases
                        .iter()
                        .find(|command| match command {
                            Self::Literal { term, .. } => term.eq_ci(input),
                            Self::StrictWildcard { .. } => false,
                        })
                })
                .cloned(),
            Vec::new(),
        )
    }
}

#[async_trait(?Send)]
impl Autocomplete for CommandAlias {
    async fn autocomplete(
        input: &str,
        app_meta: &AppMeta,
        _include_aliases: bool,
    ) -> Vec<(Cow<'static, str>, Cow<'static, str>)> {
        app_meta
            .command_aliases
            .iter()
            .filter_map(|command| match command {
                Self::Literal { term, summary, .. } => {
                    if term.starts_with_ci(input) {
                        Some((term.clone().into(), summary.clone().into()))
                    } else {
                        None
                    }
                }
                Self::StrictWildcard { .. } => None,
            })
            .collect()
    }

    fn get_variant_name(&self) -> &'static str {
        match self {
            Self::Literal { .. } => "Literal",
            Self::StrictWildcard { .. } => "StrictWildcard",
        }
    }
}

impl fmt::Display for CommandAlias {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Literal { term, .. } => {
                write!(f, "{}", term)?;
            }
            Self::StrictWildcard { .. } => {}
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::{assert_autocomplete, AppCommand, Command, Event};
    use crate::storage::NullDataStore;
    use std::collections::HashSet;
    use tokio_test::block_on;

    #[test]
    fn literal_constructor_test() {
        let alias = CommandAlias::literal(
            "term".to_string(),
            "summary".to_string(),
            AppCommand::About.into(),
        );

        if let CommandAlias::Literal {
            term,
            summary,
            command,
        } = alias
        {
            assert_eq!("term", term);
            assert_eq!("summary", summary);
            assert_eq!(Box::new(Command::from(AppCommand::About)), command);
        } else {
            panic!("{:?}", alias);
        }
    }

    #[test]
    fn wildcard_constructor_test() {
        let alias = CommandAlias::strict_wildcard(AppCommand::About.into());

        if let CommandAlias::StrictWildcard { command } = alias {
            assert_eq!(Box::new(Command::from(AppCommand::About)), command);
        } else {
            panic!("{:?}", alias);
        }
    }

    #[test]
    fn eq_test() {
        assert_eq!(
            literal("foo", "foo", AppCommand::About.into()),
            literal("foo", "bar", AppCommand::Help.into()),
        );
        assert_ne!(
            literal("foo", "foo", AppCommand::About.into()),
            literal("bar", "foo", AppCommand::About.into()),
        );

        assert_eq!(
            strict_wildcard(AppCommand::About.into()),
            strict_wildcard(AppCommand::Help.into()),
        );
        assert_ne!(
            literal("", "", AppCommand::About.into()),
            strict_wildcard(AppCommand::About.into()),
        );
    }

    #[test]
    fn hash_test() {
        let mut set = HashSet::with_capacity(2);

        assert!(set.insert(literal("foo", "", AppCommand::About.into())));
        assert!(set.insert(literal("bar", "", AppCommand::About.into())));
        assert!(set.insert(strict_wildcard(AppCommand::About.into())));
        assert!(!set.insert(literal("foo", "", AppCommand::Help.into())));
        assert!(!set.insert(literal("FOO", "", AppCommand::Help.into())));
        assert!(!set.insert(strict_wildcard(AppCommand::Help.into())));
    }

    #[test]
    fn runnable_test_literal() {
        let about_alias = literal("about alias", "about summary", AppCommand::About.into());

        let mut app_meta = app_meta();
        app_meta.command_aliases.insert(about_alias.clone());
        app_meta.command_aliases.insert(literal(
            "help alias",
            "help summary",
            AppCommand::Help.into(),
        ));

        assert_autocomplete(
            &[("about alias", "about summary")][..],
            block_on(CommandAlias::autocomplete("a", &app_meta, true)),
        );

        assert_eq!(
            block_on(CommandAlias::autocomplete("a", &app_meta, true)),
            block_on(CommandAlias::autocomplete("A", &app_meta, true)),
        );

        assert_eq!(
            (None, Vec::new()),
            block_on(CommandAlias::parse_input("blah", &app_meta)),
        );

        {
            let (parsed_exact, parsed_fuzzy) =
                block_on(CommandAlias::parse_input("about alias", &app_meta));

            assert!(parsed_fuzzy.is_empty(), "{:?}", parsed_fuzzy);
            assert_eq!(about_alias, parsed_exact.unwrap());
        }

        {
            let (about_result, about_alias_result) = (
                block_on(AppCommand::About.run("about alias", &mut app_meta)),
                block_on(about_alias.run("about alias", &mut app_meta)),
            );

            assert!(about_result.is_ok(), "{:?}", about_result);
            assert_eq!(about_result, about_alias_result);

            assert!(!app_meta.command_aliases.is_empty());
        }
    }

    #[test]
    fn runnable_test_strict_wildcard() {
        let about_alias = strict_wildcard(AppCommand::About.into());

        let mut app_meta = app_meta();
        app_meta.command_aliases.insert(about_alias.clone());
        app_meta.command_aliases.insert(literal(
            "literal alias",
            "literally a summary",
            AppCommand::Help.into(),
        ));

        {
            // Should be caught by the wildcard, not the literal alias
            let (parsed_exact, parsed_fuzzy) =
                block_on(CommandAlias::parse_input("literal alias", &app_meta));

            assert!(parsed_fuzzy.is_empty(), "{:?}", parsed_fuzzy);
            assert_eq!(about_alias, parsed_exact.unwrap());
        }

        {
            assert_eq!(2, app_meta.command_aliases.len());

            let (about_result, about_alias_result) = (
                block_on(AppCommand::About.run("about", &mut app_meta)),
                block_on(about_alias.run("about", &mut app_meta)),
            );

            assert!(about_result.is_ok(), "{:?}", about_result);
            assert_eq!(about_result, about_alias_result);
            assert!(app_meta.command_aliases.is_empty());
        }
    }

    fn event_dispatcher(_event: Event) {}

    fn app_meta() -> AppMeta {
        AppMeta::new(NullDataStore::default(), &event_dispatcher)
    }

    fn literal(term: &str, summary: &str, command: Command) -> CommandAlias {
        CommandAlias::Literal {
            term: term.to_string(),
            summary: summary.to_string(),
            command: Box::new(command),
        }
    }

    fn strict_wildcard(command: Command) -> CommandAlias {
        CommandAlias::StrictWildcard {
            command: Box::new(command),
        }
    }
}
