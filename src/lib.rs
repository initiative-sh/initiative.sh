use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use uuid::Uuid;

use entity::Entity;
use initiative_macros;

mod command;
mod entity;

pub struct Context {
    worlds: HashMap<Rc<Uuid>, World>,
    active_world: Rc<Uuid>,
}

pub struct World {
    entities: HashMap<Rc<Uuid>, Entity>,
    uuid: Rc<Uuid>,
    name: String,
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

impl World {
    pub fn new(name: String) -> Self {
        let root_entity = Entity::new_root();
        let mut entities = HashMap::new();
        entities.insert(root_entity.uuid.clone(), root_entity);

        Self {
            entities,
            name,
            uuid: Rc::new(Uuid::new_v4()),
        }
    }

    pub fn run(&mut self, command: &str) -> Box<impl fmt::Display> {
        Box::new(command.to_string())
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new("Default World".to_string())
    }
}
