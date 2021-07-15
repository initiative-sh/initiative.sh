use std::str::FromStr;

#[derive(Debug)]
pub enum StorageCommand {
    Load { query: String },
}

impl FromStr for StorageCommand {
    type Err = ();

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw.starts_with(char::is_uppercase) {
            Ok(StorageCommand::Load {
                query: raw.to_string(),
            })
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_str_test() {
        let parsed_command = "Gandalf the Grey".parse();
        if let Ok(StorageCommand::Load { query }) = parsed_command {
            assert_eq!("Gandalf the Grey", query.as_str());
        } else {
            panic!("{:?}", parsed_command);
        }

        let parsed_command = "potato".parse::<StorageCommand>();
        assert!(matches!(parsed_command, Err(())), "{:?}", parsed_command);
    }
}
