use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use rand::prelude::*;
use uuid::Uuid;

use world::{Generate, World};

mod command;
mod world;

pub struct Context {
    worlds: HashMap<Rc<Uuid>, World>,
    active_world: Rc<Uuid>,
}

impl Context {
    pub fn run(&mut self, command: &str) -> Box<impl fmt::Display> {
        Box::new(format!(
            "{:?}\n\n{:?}",
            world::Location::generate(&mut StdRng::from_entropy(), &world::Demographics {}),
            command.parse::<command::Command>()
        ))
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
