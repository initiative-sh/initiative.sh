use std::convert::TryFrom;

use super::{RawCommand, Verb};

#[derive(Debug)]
pub enum AppCommand {
    Debug(RawCommand),
    Help(RawCommand),
    Quit(RawCommand),
}

impl TryFrom<RawCommand> for AppCommand {
    type Error = RawCommand;

    fn try_from(raw: RawCommand) -> Result<AppCommand, RawCommand> {
        match raw.get_verb() {
            Some(Verb::Debug) => Ok(AppCommand::Debug(raw)),
            Some(Verb::Help) => Ok(AppCommand::Help(raw)),
            Some(Verb::Quit) => Ok(AppCommand::Quit(raw)),
            _ => Err(raw),
        }
    }
}
