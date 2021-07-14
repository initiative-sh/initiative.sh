use std::convert::TryFrom;

use super::RawCommand;

#[derive(Debug)]
pub enum StorageCommand {
    Load(RawCommand),
}

impl StorageCommand {
    pub fn raw(&self) -> &RawCommand {
        match self {
            StorageCommand::Load(c) => c,
        }
    }
}

impl TryFrom<RawCommand> for StorageCommand {
    type Error = RawCommand;

    fn try_from(raw: RawCommand) -> Result<StorageCommand, RawCommand> {
        if raw.get_proper_noun().is_some() {
            Ok(StorageCommand::Load(raw))
        } else {
            Err(raw)
        }
    }
}
