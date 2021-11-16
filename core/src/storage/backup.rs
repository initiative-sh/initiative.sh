use super::repository::{KeyValue, Repository};
use crate::world::Thing;
use futures::join;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BackupData {
    #[serde(rename = "_")]
    pub comment: &'static str,

    pub things: Vec<Thing>,

    #[serde(rename = "keyValue")]
    pub key_value: KeyValueBackup,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KeyValueBackup {
    pub time: Option<String>,
}

pub async fn export(repo: &Repository) -> BackupData {
    let (things, time) = join!(repo.journal(), repo.get_key_value(&KeyValue::Time(None)));

    BackupData {
        comment: "This document is exported from initiative.sh. Please note that this format is currently undocumented and no guarantees of forward compatibility are provided, although a reasonable effort will be made to ensure that older backups can be safely imported.",
        things: things.unwrap_or_default(),
        key_value: KeyValueBackup {
            time: time.ok().and_then(|t| t.time()).map(|t| t.display_short().to_string()),
        },
    }
}
