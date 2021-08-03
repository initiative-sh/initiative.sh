use super::Command;
use crate::world;
use rand::prelude::*;
use std::collections::HashMap;

const RECENT_MAX_LEN: usize = 100;

#[derive(Debug)]
pub struct AppMeta {
    pub demographics: world::Demographics,
    pub command_aliases: HashMap<String, Command>,
    pub rng: SmallRng,

    recent: Vec<world::Thing>,
}

impl AppMeta {
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

    pub fn recent(&self) -> &[world::Thing] {
        self.recent.as_ref()
    }
}

impl Default for AppMeta {
    fn default() -> Self {
        Self {
            command_aliases: HashMap::default(),
            demographics: world::Demographics::default(),
            recent: Vec::default(),
            rng: SmallRng::from_entropy(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn push_recent_test() {
        let mut app_meta = AppMeta::default();

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
        let mut app_meta = AppMeta::default();

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
}
