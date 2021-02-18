use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use uuid::Uuid;

use world::World;

mod command;
mod demographics;
mod world;

pub struct Context {
    worlds: HashMap<Rc<Uuid>, World>,
    active_world: Rc<Uuid>,
}

impl Context {
    pub fn run(&mut self, command: &str) -> Box<impl fmt::Display> {
        Box::new(format!("{:?}", command.parse::<command::Command>()))
    }

    fn get_world(&self) -> &World {
        self.worlds.get(&self.active_world).unwrap()
    }
}

impl Default for Context {
    fn default() -> Self {
        let world = World::default();
        let active_world = world.uuid.clone();

        let mut worlds = HashMap::new();
        worlds.insert(world.uuid.clone(), world);

        Self {
            worlds,
            active_world,
        }
    }
}

pub trait RandomTable {
    fn get_random(rng: &mut impl rand::Rng, demographics: &demographics::Demographics) -> Self;
}