use super::Command;
use crate::storage::DataStore;
use crate::world;
use rand::prelude::*;
use std::collections::HashMap;
use std::fmt;

const RECENT_MAX_LEN: usize = 100;

pub struct AppMeta {
    pub demographics: world::Demographics,
    pub command_aliases: HashMap<String, Command>,
    pub rng: SmallRng,
    pub data_store: Box<dyn DataStore>,

    recent: Vec<world::Thing>,
}

impl AppMeta {
    pub fn new(data_store: impl DataStore + 'static) -> Self {
        Self {
            command_aliases: HashMap::default(),
            data_store: Box::new(data_store),
            demographics: world::Demographics::default(),
            recent: Vec::default(),
            rng: SmallRng::from_entropy(),
        }
    }

    pub fn batch_push_recent(&mut self, mut things: Vec<world::Thing>) {
        if things.len() > RECENT_MAX_LEN {
            things.drain(..(things.len() - RECENT_MAX_LEN));
        }

        if self.recent.len() + things.len() > RECENT_MAX_LEN {
            self.recent
                .drain(..(self.recent.len() + things.len() - RECENT_MAX_LEN));
        }

        self.recent.append(&mut things);
    }

    pub fn push_recent(&mut self, thing: world::Thing) {
        if self.recent.len() >= RECENT_MAX_LEN {
            self.recent.remove(0);
        }

        self.recent.push(thing);
    }

    pub fn take_recent<F>(&mut self, f: F) -> Option<world::Thing>
    where
        F: Fn(&world::Thing) -> bool,
    {
        if let Some(index) =
            self.recent
                .iter()
                .enumerate()
                .find_map(|(i, t)| if f(t) { Some(i) } else { None })
        {
            Some(self.recent.remove(index))
        } else {
            None
        }
    }

    pub fn recent(&self) -> &[world::Thing] {
        self.recent.as_ref()
    }
}

impl fmt::Debug for AppMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AppMeta {{ demographics: {:?}, command_aliases: {:?}, recent: {:?} }}",
            self.demographics, self.command_aliases, self.recent,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::storage::NullDataStore;
    use crate::world::Demographics;

    #[test]
    fn push_recent_test() {
        let mut app_meta = AppMeta::new(NullDataStore::default());

        (0..RECENT_MAX_LEN).for_each(|i| {
            let mut npc = world::Npc::default();
            npc.name.replace(format!("Thing {}", i));
            app_meta.push_recent(world::Thing::Npc(npc));
            assert_eq!(i + 1, app_meta.recent.len());
        });

        assert_eq!(
            Some(&"Thing 0".to_string()),
            app_meta
                .recent
                .first()
                .map(|thing| thing.name().value())
                .flatten()
        );

        let mut npc = world::Npc::default();
        npc.name.replace("The Cat in the Hat".to_string());
        app_meta.push_recent(world::Thing::Npc(npc));
        assert_eq!(RECENT_MAX_LEN, app_meta.recent.len());

        assert_eq!(
            Some(&"Thing 1".to_string()),
            app_meta
                .recent
                .first()
                .map(|thing| thing.name().value())
                .flatten()
        );

        assert_eq!(
            Some(&"The Cat in the Hat".to_string()),
            app_meta
                .recent
                .last()
                .map(|thing| thing.name().value())
                .flatten()
        );
    }

    #[test]
    fn batch_push_recent_test() {
        let mut app_meta = AppMeta::new(NullDataStore::default());

        app_meta.batch_push_recent(Vec::new());
        assert_eq!(0, app_meta.recent.len());

        app_meta.batch_push_recent(
            (0..50)
                .map(|i| {
                    let mut npc = world::Npc::default();
                    npc.name.replace(format!("Thing {}", i));
                    world::Thing::Npc(npc)
                })
                .collect(),
        );
        assert_eq!(50, app_meta.recent.len());

        app_meta.batch_push_recent(
            (50..RECENT_MAX_LEN)
                .map(|i| {
                    let mut npc = world::Npc::default();
                    npc.name.replace(format!("Thing {}", i));
                    world::Thing::Npc(npc)
                })
                .collect(),
        );
        assert_eq!(RECENT_MAX_LEN, app_meta.recent.len());

        assert_eq!(
            Some(&"Thing 0".to_string()),
            app_meta
                .recent
                .first()
                .map(|thing| thing.name().value())
                .flatten()
        );

        assert_eq!(
            Some(&format!("Thing {}", RECENT_MAX_LEN - 1)),
            app_meta
                .recent
                .last()
                .map(|thing| thing.name().value())
                .flatten()
        );

        app_meta.batch_push_recent(
            (0..50)
                .map(|i| {
                    let mut npc = world::Npc::default();
                    npc.name.replace(format!("Thang {}", i));
                    world::Thing::Npc(npc)
                })
                .collect(),
        );
        assert_eq!(RECENT_MAX_LEN, app_meta.recent.len());

        assert_eq!(
            Some(&"Thing 50".to_string()),
            app_meta
                .recent
                .first()
                .map(|thing| thing.name().value())
                .flatten()
        );

        app_meta.batch_push_recent(
            (0..(RECENT_MAX_LEN * 2))
                .map(|i| {
                    let mut npc = world::Npc::default();
                    npc.name.replace(format!("Oobleck {}", i));
                    world::Thing::Npc(npc)
                })
                .collect(),
        );
        assert_eq!(RECENT_MAX_LEN, app_meta.recent.len());

        assert_eq!(
            Some(&format!("Oobleck {}", RECENT_MAX_LEN)),
            app_meta
                .recent
                .first()
                .map(|thing| thing.name().value())
                .flatten()
        );
    }

    #[test]
    fn debug_test() {
        let mut app_meta = AppMeta::new(NullDataStore::default());
        app_meta.demographics = Demographics::new(HashMap::new().into());

        assert_eq!(
            "AppMeta { demographics: Demographics { groups: GroupMapWrapper({}) }, command_aliases: {}, recent: [] }",
            format!("{:?}", app_meta),
        );
    }
}
