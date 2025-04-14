use super::{CommandAlias, Event};
use crate::command::prelude::*;
use crate::storage::{DataStore, Repository};
use crate::world;
use rand::prelude::*;
use std::collections::HashSet;
use std::fmt;

pub struct AppMeta {
    pub command_aliases: HashSet<CommandAlias>,
    pub command_aliases_new: HashSet<Alias>,
    pub demographics: world::Demographics,
    pub event_dispatcher: &'static dyn Fn(Event),
    pub rng: SmallRng,
    pub repository: Repository,
}

impl AppMeta {
    pub fn new<F: Fn(Event)>(
        data_store: impl DataStore + 'static,
        event_dispatcher: &'static F,
    ) -> Self {
        Self {
            command_aliases: HashSet::default(),
            command_aliases_new: HashSet::default(),
            demographics: world::Demographics::default(),
            event_dispatcher,
            repository: Repository::new(data_store),
            rng: SmallRng::from_entropy(),
        }
    }
}

impl fmt::Debug for AppMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AppMeta {{ command_aliases: {:?}, demographics: {:?}, repository: {:?} }}",
            self.command_aliases, self.demographics, self.repository,
        )
    }
}

#[cfg(test)]
mod test {
    use crate::test_utils as test;
    use crate::world::Demographics;
    use std::collections::HashMap;

    #[test]
    fn debug_test() {
        let mut app_meta = test::app_meta();
        app_meta.demographics = Demographics::new(HashMap::new());

        assert_eq!(
            "AppMeta { command_aliases: {}, demographics: Demographics { groups: GroupMapWrapper({}) }, repository: Repository { data_store_enabled: false, recent: [] } }",
            format!("{:?}", app_meta),
        );
    }
}
