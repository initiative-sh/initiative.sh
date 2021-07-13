use std::fmt;

use crate::app::{Context, StorageCommand};

pub fn command(command: &StorageCommand, context: &mut Context) -> Box<dyn fmt::Display> {
    match command {
        StorageCommand::Load(s) => {
            if let Some(proper_noun) = s.get_proper_noun() {
                let query = proper_noun.to_lowercase();
                if let Some(result) = context.recent().iter().find(|t| {
                    t.name()
                        .value()
                        .map_or(false, |s| s.to_lowercase() == query)
                }) {
                    Box::new(format!("{}", result.display_details()))
                } else {
                    Box::new(format!("No matches for \"{}\"", proper_noun))
                }
            } else {
                Box::new("Error: invalid command")
            }
        }
    }
}
