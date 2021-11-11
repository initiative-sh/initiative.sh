use super::CommandAlias;
use crate::storage::{DataStore, Repository};
use crate::world;
use rand::prelude::*;
use std::collections::HashSet;
use std::fmt;

pub struct AppMeta {
    pub command_aliases: HashSet<CommandAlias>,
    pub demographics: world::Demographics,
    pub rng: SmallRng,
    pub repository: Repository,
}

impl AppMeta {
    pub fn new(data_store: impl DataStore + 'static) -> Self {
        Self {
            command_aliases: HashSet::default(),
            demographics: world::Demographics::default(),
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
    use super::*;
    use crate::storage::NullDataStore;
    use crate::world::Demographics;
    use std::collections::HashMap;

    #[test]
    fn debug_test() {
        let mut app_meta = AppMeta::new(NullDataStore::default());
        app_meta.demographics = Demographics::new(HashMap::new().into());

        assert_eq!(
            "AppMeta { command_aliases: {}, demographics: Demographics { groups: GroupMapWrapper({}) }, repository: Repository { data_store_enabled: false, recent: [], time: Time { days: 1, hours: 8, minutes: 0, seconds: 0 } } }",
            format!("{:?}", app_meta),
        );
    }
}
