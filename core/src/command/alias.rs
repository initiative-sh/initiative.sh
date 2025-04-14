use crate::command::prelude::*;

use std::borrow::Cow;
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Alias {
    token: Token,
    autocomplete_description: Cow<'static, str>,
    command: AliasCommand,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AliasCommand {
    Save { uuid: Uuid },
}

impl Alias {
    pub fn new<S>(token: Token, autocomplete_description: S, command: AliasCommand) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        Alias {
            token,
            autocomplete_description: autocomplete_description.into(),
            command,
        }
    }
}

impl Command for Alias {
    fn token(&self) -> Token {
        self.token.clone()
    }

    fn autocomplete(
        &self,
        fuzzy_match_list: FuzzyMatchList,
        _input: &str,
    ) -> Option<AutocompleteSuggestion> {
        Some(
            (
                fuzzy_match_list.autocomplete_term()?,
                self.autocomplete_description.clone(),
            )
                .into(),
        )
    }

    fn get_priority(&self, _: &MatchList) -> Option<CommandPriority> {
        Some(CommandPriority::Canonical)
    }

    fn get_canonical_form_of(&self, match_list: &MatchList) -> Option<String> {
        Some(match_list.parts().next()?.term()?.to_string())
    }

    async fn run(
        &self,
        _match_list: MatchList<'_>,
        _app_meta: &mut AppMeta,
    ) -> Result<impl std::fmt::Display, impl std::fmt::Display> {
        match self.command {
            AliasCommand::Save { .. } => todo!(),
        }

        #[expect(unreachable_code)]
        Ok::<_, &str>("")
    }
}

impl std::hash::Hash for Alias {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.token.hash(state);
    }
}
