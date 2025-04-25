use crate::command::prelude::*;
use crate::world::thing::ThingData;

use super::{create, load, save};

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
    CreateMore { thing_data: ThingData },
    Save { uuid: Uuid },
    Load { uuid: Uuid },
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
        app_meta: &mut AppMeta,
    ) -> Result<impl std::fmt::Display, impl std::fmt::Display> {
        fn out(
            input: Result<impl std::fmt::Display, impl std::fmt::Display>,
        ) -> Result<String, String> {
            input.map(|s| s.to_string()).map_err(|s| s.to_string())
        }

        match &self.command {
            AliasCommand::CreateMore { thing_data } => {
                out(create::Create.more(thing_data, app_meta).await)
            }
            AliasCommand::Save { uuid } => out(save::Save.run_with_uuid(uuid, app_meta).await),
            AliasCommand::Load { uuid } => out(load::Load.run_with_uuid(uuid, app_meta).await),
        }
    }
}

impl std::hash::Hash for Alias {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.token.hash(state);
    }
}
