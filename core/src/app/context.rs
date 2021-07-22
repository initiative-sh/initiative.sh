use super::Command;
use crate::world;
use std::collections::HashMap;

const RECENT_MAX_LEN: usize = 100;

#[derive(Debug, Default)]
pub struct Context {
    pub demographics: world::Demographics,
    pub command_aliases: HashMap<String, Command>,

    recent: Vec<world::Thing>,
}

impl Context {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn push_recent_test() {
        let mut context = Context::default();

        (0..RECENT_MAX_LEN).for_each(|i| {
            let mut npc = world::Npc::default();
            npc.name.replace(format!("Thing {}", i));
            context.push_recent(world::Thing::Npc(npc));
            assert_eq!(i + 1, context.recent.len());
        });

        assert_eq!(
            Some(&"Thing 0".to_string()),
            context
                .recent
                .first()
                .map(|thing| thing.name().value())
                .flatten()
        );

        let mut npc = world::Npc::default();
        npc.name.replace("The Cat in the Hat".to_string());
        context.push_recent(world::Thing::Npc(npc));
        assert_eq!(RECENT_MAX_LEN, context.recent.len());

        assert_eq!(
            Some(&"Thing 1".to_string()),
            context
                .recent
                .first()
                .map(|thing| thing.name().value())
                .flatten()
        );

        assert_eq!(
            Some(&"The Cat in the Hat".to_string()),
            context
                .recent
                .last()
                .map(|thing| thing.name().value())
                .flatten()
        );
    }

    #[test]
    fn batch_push_recent_test() {
        let mut context = Context::default();

        context.batch_push_recent(Vec::new());
        assert_eq!(0, context.recent.len());

        context.batch_push_recent(
            (0..50)
                .map(|i| {
                    let mut npc = world::Npc::default();
                    npc.name.replace(format!("Thing {}", i));
                    world::Thing::Npc(npc)
                })
                .collect(),
        );
        assert_eq!(50, context.recent.len());

        context.batch_push_recent(
            (50..RECENT_MAX_LEN)
                .map(|i| {
                    let mut npc = world::Npc::default();
                    npc.name.replace(format!("Thing {}", i));
                    world::Thing::Npc(npc)
                })
                .collect(),
        );
        assert_eq!(RECENT_MAX_LEN, context.recent.len());

        assert_eq!(
            Some(&"Thing 0".to_string()),
            context
                .recent
                .first()
                .map(|thing| thing.name().value())
                .flatten()
        );

        assert_eq!(
            Some(&format!("Thing {}", RECENT_MAX_LEN - 1)),
            context
                .recent
                .last()
                .map(|thing| thing.name().value())
                .flatten()
        );

        context.batch_push_recent(
            (0..50)
                .map(|i| {
                    let mut npc = world::Npc::default();
                    npc.name.replace(format!("Thang {}", i));
                    world::Thing::Npc(npc)
                })
                .collect(),
        );
        assert_eq!(RECENT_MAX_LEN, context.recent.len());

        assert_eq!(
            Some(&"Thing 50".to_string()),
            context
                .recent
                .first()
                .map(|thing| thing.name().value())
                .flatten()
        );

        context.batch_push_recent(
            (0..(RECENT_MAX_LEN * 2))
                .map(|i| {
                    let mut npc = world::Npc::default();
                    npc.name.replace(format!("Oobleck {}", i));
                    world::Thing::Npc(npc)
                })
                .collect(),
        );
        assert_eq!(RECENT_MAX_LEN, context.recent.len());

        assert_eq!(
            Some(&format!("Oobleck {}", RECENT_MAX_LEN)),
            context
                .recent
                .first()
                .map(|thing| thing.name().value())
                .flatten()
        );
    }
}
