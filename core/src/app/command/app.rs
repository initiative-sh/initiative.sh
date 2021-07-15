use initiative_macros::WordList;

#[derive(Debug, WordList)]
pub enum AppCommand {
    Debug,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_str_test() {
        let parsed_command = "debug".parse();
        assert!(
            matches!(parsed_command, Ok(AppCommand::Debug)),
            "{:?}",
            parsed_command,
        );

        let parsed_command = "potato".parse::<AppCommand>();
        assert!(matches!(parsed_command, Err(())), "{:?}", parsed_command);
    }
}
