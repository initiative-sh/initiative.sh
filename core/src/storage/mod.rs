use crate::app::{Context, StorageCommand};
use std::fmt;

pub fn command(command: &StorageCommand, context: &mut Context) -> Box<dyn fmt::Display> {
    match command {
        StorageCommand::Load { query } => {
            let lowercase_query = query.to_lowercase();
            if let Some(result) = context.recent().iter().find(|t| {
                t.name()
                    .value()
                    .map_or(false, |s| s.to_lowercase() == lowercase_query)
            }) {
                Box::new(format!("{}", result.display_details()))
            } else {
                Box::new(format!("No matches for \"{}\"", query))
            }
        }
    }
}
