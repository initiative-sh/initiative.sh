use crate::app::{Context, StorageCommand};

pub fn command(command: &StorageCommand, context: &mut Context) -> String {
    match command {
        StorageCommand::Load { query } => {
            let lowercase_query = query.to_lowercase();
            if let Some(result) = context.recent().iter().find(|t| {
                t.name()
                    .value()
                    .map_or(false, |s| s.to_lowercase() == lowercase_query)
            }) {
                format!("{}", result.display_details())
            } else {
                format!("No matches for \"{}\"", query)
            }
        }
    }
}
