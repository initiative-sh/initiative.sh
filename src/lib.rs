use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt;
use std::rc::Rc;

use rand::prelude::*;
use uuid::Uuid;

use command::Noun;
use world::{Generate, Location, Npc, World};

mod command;
mod world;

pub struct Context {
    worlds: HashMap<Rc<Uuid>, World>,
    active_world: Rc<Uuid>,
}

impl Context {
    pub fn run(&mut self, command: &str) -> Box<impl fmt::Display> {
        let command: command::Command = command.parse().unwrap();
        let demographics = world::Demographics::default();

        if let Some(verb) = command.get_verb() {
            Box::new(format!("{:?}", verb))
        } else if let Some(&noun) = command.get_noun() {
            if let Ok(location_subtype) = noun.try_into() {
                let mut output = String::new();
                let location =
                    Location::generate_subtype(location_subtype, &mut thread_rng(), &demographics);

                output.push_str(&format!("\n{}\n", location.display_details()));

                (0..10).for_each(|i| {
                    output.push_str(&format!(
                        "{} {}\n",
                        i,
                        Location::generate_subtype(
                            location_subtype,
                            &mut thread_rng(),
                            &demographics
                        )
                        .display_summary()
                    ))
                });

                Box::new(output)
            } else if let Ok(race) = noun.try_into() {
                let demographics = demographics.only_race(&race);

                let mut output = String::new();
                let npc = Npc::generate(&mut thread_rng(), &demographics);

                output.push_str(&format!("\n{}\n", npc.display_details()));

                (0..10).for_each(|i| {
                    output.push_str(&format!(
                        "{} {}\n",
                        i,
                        Npc::generate(&mut thread_rng(), &demographics).display_summary()
                    ))
                });

                Box::new(output)
            } else {
                match noun {
                    Noun::Npc => {
                        let mut output = String::new();
                        let npc = Npc::generate(&mut thread_rng(), &demographics);

                        output.push_str(&format!("\n{}\n", npc.display_details()));

                        (0..10).for_each(|i| {
                            output.push_str(&format!(
                                "{} {}\n",
                                i,
                                Npc::generate(&mut thread_rng(), &demographics).display_summary()
                            ))
                        });

                        Box::new(output)
                    }
                    _ => Box::new(format!("{:?}", noun)),
                }
            }
        } else {
            Box::new(format!("{:?}", command))
        }
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
