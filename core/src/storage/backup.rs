use super::repository::{Change, Error as RepositoryError, KeyValue, Repository};
use crate::world::Thing;
use futures::join;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct BackupData {
    #[serde(rename(serialize = "_"), skip_deserializing)]
    pub comment: &'static str,

    pub things: Vec<Thing>,

    #[serde(rename = "keyValue")]
    pub key_value: KeyValueBackup,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KeyValueBackup {
    pub time: Option<String>,
}

#[derive(Default)]
pub struct ImportStats {
    npc_stats: ImportStat,
    place_stats: ImportStat,
    key_value_stats: ImportStat,
}

#[derive(Default)]
struct ImportStat {
    created: usize,
    updated: usize,
    failed: usize,
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

pub async fn import(
    repo: &mut Repository,
    mut data: BackupData,
) -> Result<ImportStats, RepositoryError> {
    let mut stats = ImportStats::default();

    for thing in data.things.drain(..) {
        match (
            match thing {
                Thing::Npc(_) => &mut stats.npc_stats,
                Thing::Place(_) => &mut stats.place_stats,
            },
            repo.modify_without_undo(Change::CreateAndSave { thing })
                .await,
        ) {
            (stat, Ok(_)) => stat.created += 1,
            (stat, Err((Change::CreateAndSave { thing }, RepositoryError::NameAlreadyExists))) => {
                let name = thing.name().to_string();
                match repo
                    .modify_without_undo(Change::Edit {
                        name,
                        uuid: None,
                        diff: thing,
                    })
                    .await
                {
                    Ok(_) => stat.updated += 1,
                    Err(_) => stat.failed += 1,
                }
            }
            (stat, Err(_)) => stat.failed += 1,
        }
    }

    if let Some(time) = data.key_value.time.take().and_then(|s| s.parse().ok()) {
        match repo
            .modify_without_undo(Change::SetKeyValue {
                key_value: KeyValue::Time(Some(time)),
            })
            .await
        {
            Ok(Change::SetKeyValue {
                key_value: KeyValue::Time(None),
            }) => stats.key_value_stats.created += 1,
            Ok(Change::SetKeyValue {
                key_value: KeyValue::Time(Some(_)),
            }) => stats.key_value_stats.updated += 1,
            Ok(_) => unreachable!(),
            Err(_) => stats.key_value_stats.failed += 1,
        }
    }

    Ok(stats)
}

impl fmt::Display for ImportStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut first = true;

        if !self.place_stats.is_empty() {
            write!(f, "Places: {}", self.place_stats)?;
            first = false;
        }

        if !self.npc_stats.is_empty() {
            if !first {
                writeln!(f, " \\")?;
            }
            write!(f, "Characters: {}", self.npc_stats)?;
            first = false;
        }

        if !self.key_value_stats.is_empty() {
            if !first {
                writeln!(f, " \\")?;
            }
            write!(f, "Key/values: {}", self.key_value_stats)?;
            first = false;
        }

        if first {
            write!(f, "Nothing to import.")?;
        }

        Ok(())
    }
}

impl ImportStat {
    fn is_empty(&self) -> bool {
        self.created == 0 && self.updated == 0 && self.failed == 0
    }
}

impl fmt::Display for ImportStat {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut first = true;

        if self.created != 0 {
            write!(f, "{} created", self.created)?;
            first = false;
        }

        if self.updated != 0 {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{} updated", self.updated)?;
            first = false;
        }

        if self.failed != 0 {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{} failed", self.failed)?;
        }

        Ok(())
    }
}
