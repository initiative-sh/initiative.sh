use std::convert::TryInto;
use std::fmt;

use rand::prelude::*;

use super::parser::RawCommand;
use super::syntax;
use crate::world;
use crate::world::Generate;

pub struct Context;

impl Context {
    pub fn run(&mut self, command: &RawCommand) -> Box<impl fmt::Display> {
        let demographics = world::Demographics::default();

        if let Some(verb) = command.get_verb() {
            Box::new(format!("{:?}", verb))
        } else if let Some(&noun) = command.get_noun() {
            if let Ok(location_subtype) = noun.try_into() {
                let mut output = String::new();
                let location = world::Location::generate_subtype(
                    location_subtype,
                    &mut thread_rng(),
                    &demographics,
                );

                output.push_str(&format!("\n{}\n", location.display_details()));

                (0..10).for_each(|i| {
                    output.push_str(&format!(
                        "{} {}\n",
                        i,
                        world::Location::generate_subtype(
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
                let npc = world::Npc::generate(&mut thread_rng(), &demographics);

                output.push_str(&format!("\n{}\n", npc.display_details()));

                (0..10).for_each(|i| {
                    output.push_str(&format!(
                        "{} {}\n",
                        i,
                        world::Npc::generate(&mut thread_rng(), &demographics).display_summary()
                    ))
                });

                Box::new(output)
            } else {
                match noun {
                    syntax::Noun::Npc => {
                        let mut output = String::new();
                        let npc = world::Npc::generate(&mut thread_rng(), &demographics);

                        output.push_str(&format!("\n{}\n", npc.display_details()));

                        (0..10).for_each(|i| {
                            output.push_str(&format!(
                                "{} {}\n",
                                i,
                                world::Npc::generate(&mut thread_rng(), &demographics)
                                    .display_summary()
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
}

impl Default for Context {
    fn default() -> Self {
        Self
    }
}
