use crate::storage::{DataStore, MemoryDataStore};
use crate::time::Time;
use crate::utils::CaseInsensitiveStr;
use crate::world::{Npc, NpcRelations, Place, PlaceRelations, Thing, ThingRelations};
use crate::Uuid;
use futures::join;
use std::collections::VecDeque;
use std::fmt;

const RECENT_MAX_LEN: usize = 100;
const UNDO_HISTORY_LEN: usize = 10;

pub struct Repository {
    data_store: Box<dyn DataStore>,
    data_store_enabled: bool,
    recent: VecDeque<Thing>,
    redo_change: Option<Change>,
    undo_history: VecDeque<Change>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Change {
    /// Create a new thing and store it in recent entries.
    ///
    /// Reverse: Delete { uuid: None, .. }
    Create { thing: Thing },

    /// Create a new thing and store it in the journal.
    ///
    /// Reverse: Delete { uuid: Some(_), .. }
    CreateAndSave { thing: Thing },

    /// Delete a thing from recent or journal.
    ///
    /// Reverse: Create (recent) or CreateAndSave (journal)
    Delete { name: String, uuid: Option<Uuid> },

    /// Edit fields on a Thing.
    ///
    /// Reverse: Edit (already in journal) or EditAndUnsave (in recent)
    Edit {
        name: String,
        uuid: Option<Uuid>,
        diff: Thing,
    },

    /// Edit a Thing and move it from journal to recent. The reverse of edit with autosave.
    ///
    /// Reverse: Edit
    EditAndUnsave {
        name: String,
        uuid: Uuid,
        diff: Thing,
    },

    /// Transfer a thing from recent to journal.
    ///
    /// Reverse: Unsave
    Save { name: String },

    /// Transfer a thing from journal to recent. Only triggerable as the reverse to Save.
    ///
    /// Reverse: Save
    Unsave { name: String, uuid: Uuid },

    /// Set a value in the key-value store.
    ///
    /// Reverse: SetKeyValue
    SetKeyValue { key_value: KeyValue },
}

pub struct DisplayUndo<'a>(&'a Change);

pub struct DisplayRedo<'a>(&'a Change);

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    DataStoreFailed,
    MissingName,
    NameAlreadyExists,
    NotFound,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum KeyValue {
    Time(Option<Time>),
}

impl Repository {
    pub fn new(data_store: impl DataStore + 'static) -> Self {
        Self {
            data_store: Box::new(data_store),
            data_store_enabled: false,
            recent: VecDeque::default(),
            redo_change: None,
            undo_history: VecDeque::default(),
        }
    }

    pub async fn init(&mut self) {
        if self.data_store.health_check().await.is_ok() {
            self.data_store_enabled = true;
        } else {
            self.data_store = Box::new(MemoryDataStore::default());
        }
    }

    pub async fn get_by_change(&self, change: &Change) -> Result<Thing, Error> {
        let (name, uuid) = match change {
            Change::Create { thing } | Change::CreateAndSave { thing } => {
                if let Some(uuid) = thing.uuid() {
                    (None, Some(uuid))
                } else {
                    (thing.name().value(), None)
                }
            }
            Change::EditAndUnsave { uuid, .. } | Change::Unsave { uuid, .. } => (None, Some(uuid)),
            Change::Delete {
                uuid: Some(uuid), ..
            }
            | Change::Edit {
                uuid: Some(uuid), ..
            } => (None, Some(uuid)),
            Change::Delete { name, .. } | Change::Edit { name, .. } | Change::Save { name } => {
                (Some(name), None)
            }
            Change::SetKeyValue { .. } => (None, None),
        };

        if let Some(uuid) = uuid {
            self.get_by_uuid(uuid).await
        } else if let Some(name) = name {
            self.get_by_name(name).await
        } else {
            Err(Error::NotFound)
        }
    }

    pub async fn load_relations(&self, thing: &Thing) -> Result<ThingRelations, Error> {
        let locations = {
            let parent_uuid = match thing {
                Thing::Npc(Npc { location_uuid, .. }) => location_uuid,
                Thing::Place(Place { location_uuid, .. }) => location_uuid,
            };

            let parent = {
                let parent_result = if let Some(uuid) = parent_uuid.value() {
                    self.get_by_uuid(&uuid.to_owned().into())
                        .await
                        .and_then(|thing| thing.into_place().map_err(|_| Error::NotFound))
                } else {
                    Err(Error::NotFound)
                };

                match parent_result {
                    Ok(parent) => Some(parent),
                    Err(Error::NotFound) => None,
                    Err(e) => return Err(e),
                }
            };

            if let Some(parent) = parent {
                let grandparent = {
                    let grandparent_result = if let Some(uuid) = parent.location_uuid.value() {
                        self.get_by_uuid(&uuid.to_owned().into())
                            .await
                            .and_then(|thing| thing.into_place().map_err(|_| Error::NotFound))
                    } else {
                        Err(Error::NotFound)
                    };

                    match grandparent_result {
                        Ok(grandparent) => Some(grandparent),
                        Err(Error::NotFound) => None,
                        Err(e) => return Err(e),
                    }
                };

                Some((parent, grandparent))
            } else {
                None
            }
        };

        match thing {
            Thing::Npc(Npc { .. }) => Ok(NpcRelations {
                location: locations,
            }
            .into()),
            Thing::Place(Place { .. }) => Ok(PlaceRelations {
                location: locations,
            }
            .into()),
        }
    }

    pub async fn get_by_name_start(
        &self,
        name: &str,
        limit: Option<usize>,
    ) -> Result<Vec<Thing>, Error> {
        let mut things = self
            .data_store
            .get_things_by_name_start(name, limit)
            .await
            .map_err(|_| Error::DataStoreFailed)?;

        self.recent()
            .filter(|t| t.name().value().map_or(false, |s| s.starts_with_ci(name)))
            .take(
                limit
                    .unwrap_or(usize::MAX)
                    .checked_sub(things.len())
                    .unwrap_or_default(),
            )
            .for_each(|t| things.push(t.clone()));

        Ok(things)
    }

    pub fn recent(&self) -> impl Iterator<Item = &Thing> {
        self.recent.as_slices().0.iter()
    }

    pub async fn journal(&self) -> Result<Vec<Thing>, Error> {
        self.data_store
            .get_all_the_things()
            .await
            .map_err(|_| Error::DataStoreFailed)
    }

    pub async fn get_by_name(&self, name: &str) -> Result<Thing, Error> {
        let (saved_thing, recent_thing) = join!(self.data_store.get_thing_by_name(name), async {
            self.recent()
                .find(|t| t.name().value().map_or(false, |s| s.eq_ci(name)))
        });

        if let Some(thing) = recent_thing {
            Ok(thing.clone())
        } else {
            match saved_thing {
                Ok(Some(thing)) => Ok(thing),
                Ok(None) => Err(Error::NotFound),
                Err(()) => Err(Error::DataStoreFailed),
            }
        }
    }

    pub async fn get_by_uuid(&self, uuid: &Uuid) -> Result<Thing, Error> {
        match self.data_store.get_thing_by_uuid(uuid).await {
            Ok(Some(thing)) => Ok(thing),
            Ok(None) => Err(Error::NotFound),
            Err(()) => Err(Error::DataStoreFailed),
        }
    }

    pub async fn modify(&mut self, change: Change) -> Result<Option<Thing>, (Change, Error)> {
        let undo_change = self.modify_without_undo(change).await?;
        let thing = self.get_by_change(&undo_change).await.ok();

        while self.undo_history.len() >= UNDO_HISTORY_LEN {
            self.undo_history.pop_front();
        }
        self.undo_history.push_back(undo_change);

        Ok(thing)
    }

    pub async fn undo(&mut self) -> Option<Result<Option<Thing>, Error>> {
        if let Some(change) = self.undo_history.pop_back() {
            match self.modify_without_undo(change).await {
                Ok(redo_change) => {
                    let thing = self.get_by_change(&redo_change).await.ok();
                    self.redo_change = Some(redo_change);
                    Some(Ok(thing))
                }
                Err((undo_change, e)) => {
                    self.undo_history.push_back(undo_change);
                    Some(Err(e))
                }
            }
        } else {
            None
        }
    }

    pub fn undo_history(&self) -> impl Iterator<Item = &Change> {
        self.undo_history.iter().rev()
    }

    pub async fn redo(&mut self) -> Option<Result<Option<Thing>, Error>> {
        if let Some(change) = self.redo_change.take() {
            match self.modify(change).await {
                Ok(option_thing) => Some(Ok(option_thing)),
                Err((redo_change, e)) => {
                    self.redo_change = Some(redo_change);
                    Some(Err(e))
                }
            }
        } else {
            None
        }
    }

    pub fn get_redo(&self) -> Option<&Change> {
        self.redo_change.as_ref()
    }

    pub async fn modify_without_undo(&mut self, change: Change) -> Result<Change, (Change, Error)> {
        match change {
            Change::Create { thing } => self
                .create_thing(thing)
                .await
                .map(|name| Change::Delete { name, uuid: None })
                .map_err(|(thing, e)| (Change::Create { thing }, e)),
            Change::CreateAndSave { thing } => {
                let name = thing.name().to_string();
                self.create_and_save_thing(thing)
                    .await
                    .map(|uuid| Change::Delete {
                        name,
                        uuid: Some(uuid),
                    })
                    .map_err(|(thing, e)| (Change::CreateAndSave { thing }, e))
            }
            Change::Delete {
                name,
                uuid: Some(uuid),
            } => self
                .delete_thing_by_uuid(&uuid)
                .await
                .map(|thing| Change::CreateAndSave { thing })
                .map_err(|(_, e)| {
                    (
                        Change::Delete {
                            name,
                            uuid: Some(uuid),
                        },
                        e,
                    )
                }),
            Change::Delete { name, uuid: None } => self
                .delete_thing_by_name(&name)
                .await
                .map(|thing| {
                    if thing.uuid().is_some() {
                        Change::CreateAndSave { thing }
                    } else {
                        Change::Create { thing }
                    }
                })
                .map_err(|e| (Change::Delete { name, uuid: None }, e)),
            Change::Edit {
                name,
                uuid: Some(uuid),
                diff,
            } => match self.edit_thing_by_uuid(&uuid, diff).await {
                Ok(diff) => Ok(Change::Edit {
                    name: self
                        .get_by_uuid(&uuid)
                        .await
                        .map(|thing| thing.name().value().map(String::from))
                        .unwrap_or(None)
                        .unwrap_or(name),
                    uuid: Some(uuid),
                    diff,
                }),
                Err((diff, e)) => Err((
                    Change::Edit {
                        name: self
                            .get_by_uuid(&uuid)
                            .await
                            .map(|thing| thing.name().value().map(String::from))
                            .unwrap_or(None)
                            .unwrap_or(name),
                        uuid: Some(uuid),
                        diff,
                    },
                    e,
                )),
            },
            Change::Edit {
                name,
                uuid: None,
                diff,
            } => self
                .edit_thing_by_name(&name, diff)
                .await
                .map_err(|(diff, e)| {
                    (
                        Change::Edit {
                            name,
                            uuid: None,
                            diff,
                        },
                        e,
                    )
                }),
            Change::EditAndUnsave { name, uuid, diff } => {
                match self.edit_thing_by_uuid(&uuid, diff).await {
                    Ok(diff) => self
                        .unsave_thing_by_uuid(&uuid)
                        .await
                        .map(|name| Change::Edit {
                            name,
                            uuid: None,
                            diff,
                        })
                        .map_err(|(s, e)| {
                            (
                                Change::Unsave {
                                    name: s.unwrap_or(name),
                                    uuid,
                                },
                                e,
                            )
                        }),
                    Err((diff, e)) => Err((Change::EditAndUnsave { name, uuid, diff }, e)),
                }
            }
            Change::Save { name } => match self.save_thing_by_name(&name).await {
                Ok(uuid) => Ok(Change::Unsave {
                    uuid,
                    name: self
                        .get_by_uuid(&uuid)
                        .await
                        .map(|t| t.name().value().map(String::from))
                        .unwrap_or(None)
                        .unwrap_or(name),
                }),
                Err(e) => Err((Change::Save { name }, e)),
            },
            Change::Unsave { name, uuid } => self
                .unsave_thing_by_uuid(&uuid)
                .await
                .map(|name| Change::Save { name })
                .map_err(|(_, e)| (Change::Unsave { name, uuid }, e)),
            Change::SetKeyValue { key_value } => self
                .set_key_value(&key_value)
                .await
                .map(|old_kv| Change::SetKeyValue { key_value: old_kv })
                .map_err(|e| (Change::SetKeyValue { key_value }, e)),
        }
    }

    pub async fn get_key_value(&self, key: &KeyValue) -> Result<KeyValue, Error> {
        let value_str = self.data_store.get_value(key.key_raw()).await;

        match key {
            KeyValue::Time(_) => value_str
                .and_then(|o| o.map(|s| s.parse()).transpose())
                .map(KeyValue::Time),
        }
        .map_err(|_| Error::DataStoreFailed)
    }

    pub fn data_store_enabled(&self) -> bool {
        self.data_store_enabled
    }

    async fn set_key_value(&mut self, key_value: &KeyValue) -> Result<KeyValue, Error> {
        let old_key_value = self.get_key_value(key_value).await?;

        match key_value.key_value_raw() {
            (key, Some(value)) => self.data_store.set_value(key, &value).await,
            (key, None) => self.data_store.delete_value(key).await,
        }
        .map(|_| old_key_value)
        .map_err(|_| Error::DataStoreFailed)
    }

    fn push_recent(&mut self, thing: Thing) {
        while self.recent.len() >= RECENT_MAX_LEN {
            self.recent.pop_front();
        }

        self.recent.push_back(thing);
    }

    fn take_recent<F>(&mut self, f: F) -> Option<Thing>
    where
        F: Fn(&Thing) -> bool,
    {
        if let Some(index) =
            self.recent
                .iter()
                .enumerate()
                .find_map(|(i, t)| if f(t) { Some(i) } else { None })
        {
            self.recent.remove(index)
        } else {
            None
        }
    }

    async fn create_thing(&mut self, thing: Thing) -> Result<String, (Thing, Error)> {
        if let Some(name) = thing.name().value() {
            if self.get_by_name(name).await.is_ok() {
                Err((thing, Error::NameAlreadyExists))
            } else {
                let name = name.to_string();
                self.push_recent(thing);
                Ok(name)
            }
        } else {
            Err((thing, Error::MissingName))
        }
    }

    async fn create_and_save_thing(&mut self, thing: Thing) -> Result<Uuid, (Thing, Error)> {
        if let Some(name) = thing.name().value() {
            if self.get_by_name(name).await.is_ok() {
                Err((thing, Error::NameAlreadyExists))
            } else {
                self.save_thing(thing).await
            }
        } else {
            Err((thing, Error::MissingName))
        }
    }

    async fn delete_thing_by_name(&mut self, name: &str) -> Result<Thing, Error> {
        if let Some(uuid) = self
            .get_by_name(name)
            .await
            .ok()
            .and_then(|t| t.uuid().cloned())
        {
            self.delete_thing_by_uuid(&uuid).await.map_err(|(_, e)| e)
        } else if let Some(thing) =
            self.take_recent(|t| t.name().value().map_or(false, |s| s.eq_ci(name)))
        {
            Ok(thing)
        } else {
            Err(Error::NotFound)
        }
    }

    async fn delete_thing_by_uuid(&mut self, uuid: &Uuid) -> Result<Thing, (Option<Thing>, Error)> {
        match (
            self.data_store.get_thing_by_uuid(uuid).await,
            self.data_store.delete_thing_by_uuid(uuid).await,
        ) {
            (Ok(Some(thing)), Ok(())) => Ok(thing),
            (Ok(Some(thing)), Err(())) => Err((Some(thing), Error::DataStoreFailed)),
            (Ok(None), _) => Err((None, Error::NotFound)),
            (Err(_), _) => Err((None, Error::DataStoreFailed)),
        }
    }

    async fn save_thing_by_name(&mut self, name: &str) -> Result<Uuid, Error> {
        if let Some(thing) = self.take_recent(|t| t.name().value().map_or(false, |s| s.eq_ci(name)))
        {
            self.save_thing(thing).await.map_err(|(thing, e)| {
                self.push_recent(thing);
                e
            })
        } else {
            Err(Error::NotFound)
        }
    }

    async fn save_thing(&mut self, mut thing: Thing) -> Result<Uuid, (Thing, Error)> {
        let uuid = if let Some(&uuid) = thing.uuid() {
            uuid
        } else {
            let uuid = Uuid::new_v4();
            thing.set_uuid(uuid);
            uuid
        };

        match self.data_store.save_thing(&thing).await {
            Ok(()) => Ok(uuid),
            Err(()) => {
                thing.clear_uuid();
                Err((thing, Error::DataStoreFailed))
            }
        }
    }

    async fn unsave_thing_by_uuid(
        &mut self,
        uuid: &Uuid,
    ) -> Result<String, (Option<String>, Error)> {
        let (mut thing, error) = match self.delete_thing_by_uuid(uuid).await {
            Ok(thing) => (thing, None),
            Err((Some(thing), e)) => (thing, Some(e)),
            Err((None, e)) => return Err((None, e)),
        };

        thing.clear_uuid();

        match (self.create_thing(thing).await, error) {
            (Ok(s), None) => Ok(s),
            (Ok(s), Some(e)) => Err((Some(s), e)),
            (Err((t, e)), None) | (Err((t, _)), Some(e)) => {
                Err((t.name().value().map(|s| s.to_string()), e))
            }
        }
    }

    async fn edit_thing_by_uuid(
        &mut self,
        uuid: &Uuid,
        mut diff: Thing,
    ) -> Result<Thing, (Thing, Error)> {
        match self.data_store.get_thing_by_uuid(uuid).await {
            Ok(Some(mut thing)) => {
                if thing.try_apply_diff(&mut diff).is_err() {
                    // This fails when the thing types don't match, eg. applying an Npc diff to a
                    // Place.
                    return Err((diff, Error::NotFound));
                }

                match self.data_store.edit_thing(&thing).await {
                    Ok(()) => Ok(diff),
                    Err(()) => Err((diff, Error::DataStoreFailed)),
                }
            }
            Ok(None) => Err((diff, Error::NotFound)),
            Err(_) => Err((diff, Error::DataStoreFailed)),
        }
    }

    async fn edit_thing_by_name(
        &mut self,
        name: &str,
        mut diff: Thing,
    ) -> Result<Change, (Thing, Error)> {
        let data_store_failed = match self.data_store.get_thing_by_name(name).await {
            Ok(Some(mut thing)) => {
                if thing.try_apply_diff(&mut diff).is_err() {
                    return Err((diff, Error::NotFound));
                }

                return match self.data_store.edit_thing(&thing).await {
                    Ok(()) => Ok(Change::Edit {
                        name: thing.name().to_string(),
                        uuid: thing.uuid().cloned(),
                        diff,
                    }),
                    Err(()) => Err((diff, Error::DataStoreFailed)),
                };
            }
            Ok(None) => false,
            Err(()) => true,
        };

        if let Some(mut thing) = self.take_recent(|thing| {
            thing.name().value().map_or(false, |s| s.eq_ci(name)) && thing.as_str() == diff.as_str()
        }) {
            thing.try_apply_diff(&mut diff).unwrap();

            let name = thing.name().to_string();
            let uuid = match self.save_thing(thing).await {
                Ok(uuid) => uuid,
                Err((thing, Error::DataStoreFailed)) => {
                    self.push_recent(thing);
                    return Ok(Change::Edit {
                        name,
                        uuid: None,
                        diff,
                    });
                }
                Err((thing, e)) => {
                    self.push_recent(thing);
                    return Err((diff, e));
                }
            };

            Ok(Change::EditAndUnsave { name, uuid, diff })
        } else {
            Err((
                diff,
                if data_store_failed {
                    Error::DataStoreFailed
                } else {
                    Error::NotFound
                },
            ))
        }
    }
}

impl KeyValue {
    pub const fn key_raw(&self) -> &'static str {
        match self {
            Self::Time(_) => "time",
        }
    }

    pub fn key_value_raw(&self) -> (&'static str, Option<String>) {
        (
            self.key_raw(),
            match self {
                Self::Time(time) => time.as_ref().map(|t| t.display_short().to_string()),
            },
        )
    }

    pub const fn time(self) -> Option<Time> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Time(time) = self {
            time
        } else {
            None
        }
    }
}

impl Change {
    pub fn display_undo(&self) -> DisplayUndo {
        DisplayUndo(self)
    }

    pub fn display_redo(&self) -> DisplayRedo {
        DisplayRedo(self)
    }

    pub fn name(&self) -> String {
        match self {
            Self::Create { thing } | Self::CreateAndSave { thing } => thing.name().to_string(),
            Self::Delete { name, .. }
            | Self::Edit { name, .. }
            | Self::EditAndUnsave { name, .. }
            | Self::Save { name }
            | Self::Unsave { name, .. } => name.to_owned(),
            Self::SetKeyValue { key_value } => key_value.key_raw().to_string(),
        }
    }
}

impl<'a> fmt::Display for DisplayUndo<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let change = self.0;

        // Note: these descriptions are _backward_ since they describe the reverse, ie. the action
        // that this Change will undo.
        match change {
            Change::Create { thing } | Change::CreateAndSave { thing } => {
                write!(f, "deleting {}", thing.name())
            }
            Change::Delete { name, .. } => write!(f, "creating {}", name),
            Change::Save { name } => write!(f, "removing {} from journal", name),
            Change::Unsave { name, .. } => write!(f, "saving {} to journal", name),

            // These changes are symmetric, so we can provide the same output in both cases.
            Change::Edit { .. } | Change::EditAndUnsave { .. } | Change::SetKeyValue { .. } => {
                write!(f, "{}", DisplayRedo(change))
            }
        }
    }
}

impl<'a> fmt::Display for DisplayRedo<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let change = self.0;

        match change {
            Change::Create { thing } => write!(f, "creating {}", thing.name()),
            Change::CreateAndSave { thing } => write!(f, "creating {}", thing.name()),
            Change::Delete { name, .. } => write!(f, "deleting {}", name),
            Change::Edit { name, .. } | Change::EditAndUnsave { name, .. } => {
                write!(f, "editing {}", name)
            }
            Change::Save { name } => write!(f, "saving {} to journal", name),
            Change::Unsave { name, .. } => write!(f, "removing {} from journal", name),
            Change::SetKeyValue { key_value } => match key_value {
                KeyValue::Time(_) => write!(f, "changing the time"),
            },
        }
    }
}

impl fmt::Debug for Repository {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Repository {{ data_store_enabled: {:?}, recent: {:?} }}",
            self.data_store_enabled, self.recent,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::storage::data_store::{MemoryDataStore, NullDataStore};
    use crate::world::npc::{Npc, Species};
    use crate::world::{Place, PlaceUuid};
    use async_trait::async_trait;
    use std::cell::RefCell;
    use std::rc::Rc;
    use tokio_test::block_on;

    const OLYMPUS_UUID: Uuid = Uuid::from_u128(1);
    const THESSALY_UUID: Uuid = Uuid::from_u128(2);
    const GREECE_UUID: Uuid = Uuid::from_u128(3);
    const STYX_UUID: Uuid = Uuid::from_u128(4);

    #[test]
    fn recent_test() {
        let mut repository = empty_repo();

        (0..RECENT_MAX_LEN).for_each(|i| {
            repository.push_recent(
                Npc {
                    name: format!("Thing {}", i).into(),
                    ..Default::default()
                }
                .into(),
            );
            assert_eq!(i + 1, repository.recent.len());
        });

        assert_eq!(
            Some(&"Thing 0".to_string()),
            repository
                .recent()
                .next()
                .and_then(|thing| thing.name().value()),
        );

        repository.push_recent(
            Npc {
                name: "The Cat in the Hat".into(),
                ..Default::default()
            }
            .into(),
        );
        assert_eq!(RECENT_MAX_LEN, repository.recent.len());

        assert_eq!(
            Some(&"Thing 1".to_string()),
            repository
                .recent()
                .next()
                .and_then(|thing| thing.name().value()),
        );

        assert_eq!(
            Some(&"The Cat in the Hat".to_string()),
            repository
                .recent()
                .last()
                .and_then(|thing| thing.name().value()),
        );
    }

    #[test]
    fn journal_recent_test() {
        let repo = repo();
        assert_eq!(4, block_on(repo.journal()).unwrap().len());
        assert_eq!(1, repo.recent().count());
    }

    #[test]
    fn get_by_name_test_from_recent() {
        assert_eq!(
            "Odysseus",
            block_on(repo().get_by_name("ODYSSEUS"))
                .map(|thing| thing.name().value().map(String::from))
                .unwrap()
                .unwrap(),
        );
    }

    #[test]
    fn get_by_name_test_from_journal() {
        assert_eq!(
            "Olympus",
            block_on(repo().get_by_name("OLYMPUS"))
                .map(|thing| thing.name().value().map(String::from))
                .unwrap()
                .unwrap(),
        );
    }

    #[test]
    fn get_by_name_test_not_found() {
        assert_eq!(Err(Error::NotFound), block_on(repo().get_by_name("NOBODY")));
    }

    #[test]
    fn get_by_uuid_test_from_journal() {
        assert_eq!(
            "Olympus",
            block_on(repo().get_by_uuid(&OLYMPUS_UUID))
                .map(|thing| thing.name().value().map(String::from))
                .unwrap()
                .unwrap(),
        );
    }

    #[test]
    fn change_test_delete_by_name_from_journal_success() {
        let (mut repo, data_store) = repo_data_store();
        let change = Change::Delete {
            name: "Olympus".to_string(),
            uuid: None,
        };
        assert_eq!("deleting Olympus", change.display_redo().to_string());

        {
            assert_eq!(Ok(None), block_on(repo.modify(change)));
            let result = repo.undo_history().next().unwrap();

            assert_eq!(
                &Change::CreateAndSave {
                    thing: Place {
                        uuid: Some(OLYMPUS_UUID.into()),
                        location_uuid: PlaceUuid::from(THESSALY_UUID).into(),
                        name: "Olympus".into(),
                        ..Default::default()
                    }
                    .into()
                },
                result,
            );
            assert_eq!("deleting Olympus", result.display_undo().to_string());
            assert_eq!(3, block_on(repo.journal()).unwrap().len());
            assert_eq!(3, block_on(data_store.get_all_the_things()).unwrap().len());
        }

        {
            assert_eq!(
                &OLYMPUS_UUID,
                block_on(repo.undo())
                    .unwrap()
                    .unwrap()
                    .unwrap()
                    .uuid()
                    .unwrap(),
            );
            assert!(block_on(repo.get_by_uuid(&OLYMPUS_UUID)).is_ok());
            assert!(block_on(repo.get_by_name("Olympus")).is_ok());
            assert_eq!(4, block_on(data_store.get_all_the_things()).unwrap().len());
        }

        {
            assert_eq!(Some(Ok(None)), block_on(repo.redo()));
            assert_eq!(Err(Error::NotFound), block_on(repo.get_by_name("Olympus")));
        }
    }

    #[test]
    fn change_test_delete_by_name_from_recent_success() {
        let mut repo = repo();
        let change = Change::Delete {
            name: "Odysseus".to_string(),
            uuid: None,
        };
        assert_eq!("deleting Odysseus", change.display_redo().to_string());

        {
            assert_eq!(Ok(None), block_on(repo.modify(change)));
            let result = repo.undo_history().next().unwrap();

            assert_eq!(
                &Change::Create {
                    thing: Npc {
                        name: "Odysseus".into(),
                        location_uuid: PlaceUuid::from(STYX_UUID).into(),
                        ..Default::default()
                    }
                    .into()
                },
                result,
            );
            assert_eq!("deleting Odysseus", result.display_undo().to_string());
            assert_eq!(0, repo.recent().count());
        }

        {
            match block_on(repo.undo()) {
                Some(Ok(Some(Thing::Npc(npc)))) => {
                    assert!(npc.uuid.is_none());
                    assert_eq!("Odysseus", npc.name.value().unwrap());
                }
                v => panic!("{:?}", v),
            }

            assert_eq!(
                Some(Change::Delete {
                    name: "Odysseus".to_string(),
                    uuid: None,
                }),
                repo.redo_change,
            );
            assert!(block_on(repo.get_by_name("odysseus")).is_ok());
            assert_eq!(1, repo.recent().count());
        }
    }

    #[test]
    fn change_test_delete_by_uuid_success() {
        let (mut repo, data_store) = repo_data_store();
        let change = Change::Delete {
            name: "olympus".to_string(),
            uuid: Some(OLYMPUS_UUID),
        };
        assert_eq!("deleting olympus", change.display_redo().to_string());

        {
            assert_eq!(Ok(None), block_on(repo.modify(change)));
            let result = repo.undo_history().next().unwrap();

            assert_eq!(
                &Change::CreateAndSave {
                    thing: Place {
                        uuid: Some(OLYMPUS_UUID.into()),
                        location_uuid: PlaceUuid::from(THESSALY_UUID).into(),
                        name: "Olympus".into(),
                        ..Default::default()
                    }
                    .into()
                },
                result,
            );
            assert_eq!("deleting Olympus", result.display_undo().to_string());
            assert_eq!(3, block_on(repo.journal()).unwrap().len());
            assert_eq!(3, block_on(data_store.get_all_the_things()).unwrap().len());
        }

        {
            match block_on(repo.undo()) {
                Some(Ok(Some(Thing::Place(place)))) => {
                    assert_eq!(Some(PlaceUuid::from(OLYMPUS_UUID)), place.uuid);
                    assert_eq!("Olympus", place.name.value().unwrap());
                }
                v => panic!("{:?}", v),
            }

            assert_eq!(
                Some(Change::Delete {
                    name: "Olympus".to_string(),
                    uuid: Some(OLYMPUS_UUID),
                }),
                repo.redo_change,
            );
            assert!(block_on(repo.get_by_uuid(&OLYMPUS_UUID)).is_ok());
            assert_eq!(4, block_on(repo.journal()).unwrap().len());
            assert_eq!(4, block_on(data_store.get_all_the_things()).unwrap().len());
        }
    }

    #[test]
    fn change_test_delete_by_uuid_not_found() {
        let change = Change::Delete {
            name: "Nobody".to_string(),
            uuid: Some(Uuid::nil()),
        };

        let result = block_on(repo().modify(change.clone())).unwrap_err();

        assert_eq!((change, Error::NotFound), result);
    }

    #[test]
    fn change_test_delete_by_uuid_data_store_failed() {
        let change = Change::Delete {
            name: "Olympus".to_string(),
            uuid: Some(OLYMPUS_UUID),
        };

        let result = block_on(null_repo().modify(change.clone())).unwrap_err();

        assert_eq!((change, Error::DataStoreFailed), result);
    }

    #[test]
    fn change_test_edit_by_name_from_recent_success() {
        let (mut repo, data_store) = repo_data_store();
        let change = Change::Edit {
            name: "Odysseus".into(),
            uuid: None,
            diff: Npc {
                name: "Nobody".into(),
                species: Species::Human.into(),
                ..Default::default()
            }
            .into(),
        };
        assert_eq!("editing Odysseus", change.display_redo().to_string());

        {
            let thing = block_on(repo.modify(change)).unwrap().unwrap();
            let uuid = thing.uuid().unwrap();
            let result = repo.undo_history().next().unwrap();

            assert_eq!(
                &Change::EditAndUnsave {
                    name: "Nobody".into(),
                    uuid: *uuid,
                    diff: Npc {
                        name: "Odysseus".into(),
                        species: None.into(),
                        ..Default::default()
                    }
                    .into(),
                },
                result,
            );

            assert!(block_on(repo.get_by_uuid(uuid)).is_ok());
            assert_eq!("editing Nobody", result.display_undo().to_string());
            assert!(block_on(repo.get_by_name("Nobody")).is_ok());
            assert_eq!(5, block_on(repo.journal()).unwrap().len());
            assert_eq!(0, repo.recent().count());
            assert_eq!(5, block_on(data_store.get_all_the_things()).unwrap().len());
            assert!(block_on(data_store.get_all_the_things())
                .unwrap()
                .iter()
                .any(|t| t.name().value().map_or(false, |s| s == "Nobody")));
        }

        {
            assert_eq!(
                "Odysseus",
                block_on(repo.undo())
                    .unwrap()
                    .unwrap()
                    .unwrap()
                    .name()
                    .value()
                    .unwrap(),
            );
            assert!(block_on(repo.get_by_name("Odysseus")).is_ok());
            assert_eq!(4, block_on(repo.journal()).unwrap().len());
            assert_eq!(1, repo.recent().count());
            assert_eq!(4, block_on(data_store.get_all_the_things()).unwrap().len());
        }

        {
            let thing = block_on(repo.redo()).unwrap().unwrap().unwrap();
            let uuid = thing.uuid().unwrap();
            assert!(block_on(repo.get_by_name("Nobody")).is_ok());
            assert!(block_on(repo.get_by_uuid(&uuid)).is_ok());
        }
    }

    #[test]
    fn change_test_edit_by_name_from_recent_wrong_type() {
        let mut repo = repo();
        let change = Change::Edit {
            name: "Odysseus".into(),
            uuid: None,
            diff: Place::default().into(),
        };

        assert_eq!(
            block_on(repo.modify(change.clone())),
            Err((change, Error::NotFound)),
        );
        assert_eq!(1, repo.recent().count());
        assert_eq!(4, block_on(repo.journal()).unwrap().len());
        assert!(block_on(repo.get_by_name("Odysseus")).is_ok());
    }

    #[test]
    fn change_test_edit_by_name_from_recent_data_store_failed() {
        let mut repo = repo();
        repo.data_store = Box::new(NullDataStore::default());
        let change = Change::Edit {
            name: "Odysseus".into(),
            uuid: None,
            diff: Npc {
                species: Species::Human.into(),
                ..Default::default()
            }
            .into(),
        };

        {
            assert_eq!(
                "Odysseus",
                block_on(repo.modify(change))
                    .unwrap()
                    .unwrap()
                    .name()
                    .value()
                    .unwrap()
            );
            let result = repo.undo_history().next().unwrap();

            assert_eq!(
                &Change::Edit {
                    name: "Odysseus".into(),
                    uuid: None,
                    diff: Npc {
                        species: None.into(),
                        ..Default::default()
                    }
                    .into()
                },
                result,
            );

            assert_eq!(
                Some(&Species::Human),
                repo.recent()
                    .find(|t| t.name().to_string() == "Odysseus")
                    .and_then(|t| t.npc())
                    .and_then(|n| n.species.value()),
            );
        }

        {
            assert_eq!(
                "Odysseus",
                block_on(repo.undo())
                    .unwrap()
                    .unwrap()
                    .unwrap()
                    .name()
                    .value()
                    .unwrap(),
            );
            assert_eq!(
                Some(false),
                repo.recent()
                    .find(|t| t.name().to_string() == "Odysseus")
                    .and_then(|t| t.npc())
                    .map(|n| n.species.is_some())
            );
        }

        {
            assert_eq!(
                "Odysseus",
                block_on(repo.redo())
                    .unwrap()
                    .unwrap()
                    .unwrap()
                    .name()
                    .value()
                    .unwrap(),
            );
            assert_eq!(
                Some(&Species::Human),
                repo.recent()
                    .find(|t| t.name().to_string() == "Odysseus")
                    .and_then(|t| t.npc())
                    .and_then(|n| n.species.value()),
            );
        }
    }

    #[test]
    fn change_test_edit_by_name_from_recent_rename_data_store_failed() {
        let mut repo = repo();
        repo.data_store = Box::new(NullDataStore::default());
        let change = Change::Edit {
            name: "Odysseus".into(),
            uuid: None,
            diff: Npc {
                name: "Nobody".into(),
                ..Default::default()
            }
            .into(),
        };

        {
            assert_eq!(
                "Nobody",
                block_on(repo.modify(change))
                    .unwrap()
                    .unwrap()
                    .name()
                    .value()
                    .unwrap()
            );
            let result = repo.undo_history().next().unwrap();

            assert_eq!(
                &Change::Edit {
                    name: "Nobody".into(),
                    uuid: None,
                    diff: Npc {
                        name: "Odysseus".into(),
                        ..Default::default()
                    }
                    .into()
                },
                result,
            );

            assert!(repo.recent().any(|t| t.name().to_string() == "Nobody"));
        }

        {
            assert_eq!(
                "Odysseus",
                block_on(repo.undo())
                    .unwrap()
                    .unwrap()
                    .unwrap()
                    .name()
                    .value()
                    .unwrap(),
            );

            assert!(repo.recent().any(|t| t.name().to_string() == "Odysseus"));
        }

        {
            assert_eq!(
                "Nobody",
                block_on(repo.redo())
                    .unwrap()
                    .unwrap()
                    .unwrap()
                    .name()
                    .value()
                    .unwrap(),
            );
            assert!(repo.recent().any(|t| t.name().to_string() == "Nobody"));
        }
    }

    #[test]
    fn change_test_edit_by_name_from_journal_success() {
        let (mut repo, data_store) = repo_data_store();
        let change = Change::Edit {
            name: "Olympus".into(),
            uuid: None,
            diff: Place {
                name: "Hades".into(),
                description: "This really is hell!".into(),
                ..Default::default()
            }
            .into(),
        };
        assert_eq!("editing Olympus", change.display_redo().to_string());

        {
            assert_eq!(
                &OLYMPUS_UUID,
                block_on(repo.modify(change))
                    .unwrap()
                    .unwrap()
                    .uuid()
                    .unwrap(),
            );
            let result = repo.undo_history().next().unwrap();

            assert_eq!(
                &Change::Edit {
                    name: "Hades".into(),
                    uuid: Some(OLYMPUS_UUID),
                    diff: Place {
                        name: "Olympus".into(),
                        description: None.into(),
                        ..Default::default()
                    }
                    .into()
                },
                result,
            );
            assert_eq!("editing Hades", result.display_undo().to_string());
            assert!(block_on(repo.get_by_name("Hades")).is_ok());
            assert!(block_on(data_store.get_all_the_things())
                .unwrap()
                .iter()
                .any(|t| t.name().value().map_or(false, |s| s == "Hades")));
        }

        {
            assert_eq!(
                &OLYMPUS_UUID,
                block_on(repo.undo())
                    .unwrap()
                    .unwrap()
                    .unwrap()
                    .uuid()
                    .unwrap(),
            );
            assert!(block_on(repo.get_by_name("OLYMPUS")).is_ok());
            assert!(block_on(data_store.get_all_the_things())
                .unwrap()
                .iter()
                .any(|t| t.name().value().map_or(false, |s| s == "Olympus")));
        }

        {
            assert_eq!(
                &OLYMPUS_UUID,
                block_on(repo.redo())
                    .unwrap()
                    .unwrap()
                    .unwrap()
                    .uuid()
                    .unwrap(),
            );
            assert!(block_on(repo.get_by_name("HADES")).is_ok());
        }
    }

    #[test]
    fn change_test_edit_by_name_from_journal_wrong_type() {
        let mut repo = repo();
        let change = Change::Edit {
            name: "Olympus".into(),
            uuid: None,
            diff: Npc::default().into(),
        };

        assert_eq!(
            block_on(repo.modify(change.clone())),
            Err((change, Error::NotFound)),
        );
    }

    #[test]
    fn change_test_edit_by_name_from_journal_data_store_failed() {
        let mut repo = repo();
        repo.data_store = Box::new(NullDataStore::default());
        let change = Change::Edit {
            name: "Olympus".into(),
            uuid: None,
            diff: Place {
                name: "Hades".into(),
                ..Default::default()
            }
            .into(),
        };

        assert_eq!(
            Err((
                Change::Edit {
                    name: "Olympus".into(),
                    uuid: None,
                    diff: Place {
                        name: "Hades".into(),
                        ..Default::default()
                    }
                    .into(),
                },
                Error::DataStoreFailed,
            )),
            block_on(repo.modify(change)),
        );
    }

    #[test]
    fn change_test_edit_by_name_not_found() {
        let mut repo = repo();
        let change = Change::Edit {
            name: "Nobody".into(),
            uuid: None,
            diff: Npc::default().into(),
        };

        assert_eq!(
            block_on(repo.modify(change.clone())),
            Err((change, Error::NotFound)),
        );
    }

    #[test]
    fn change_test_edit_by_uuid_success() {
        let (mut repo, data_store) = repo_data_store();
        let change = Change::Edit {
            name: "Olympus".into(),
            uuid: Some(OLYMPUS_UUID),
            diff: Place {
                name: "Hades".into(),
                description: "This really is hell!".into(),
                ..Default::default()
            }
            .into(),
        };
        assert_eq!("editing Olympus", change.display_redo().to_string());

        {
            assert_eq!(
                &OLYMPUS_UUID,
                block_on(repo.modify(change))
                    .unwrap()
                    .unwrap()
                    .uuid()
                    .unwrap(),
            );
            let result = repo.undo_history().next().unwrap();

            assert_eq!(
                &Change::Edit {
                    name: "Hades".into(),
                    uuid: Some(OLYMPUS_UUID),
                    diff: Place {
                        name: "Olympus".into(),
                        description: None.into(),
                        ..Default::default()
                    }
                    .into()
                },
                result,
            );
            assert_eq!("editing Hades", result.display_undo().to_string());
            assert!(block_on(repo.get_by_name("Hades")).is_ok());
            assert!(block_on(data_store.get_all_the_things())
                .unwrap()
                .iter()
                .any(|t| t.name().value().map_or(false, |s| s == "Hades")));
        }

        {
            assert_eq!(
                &OLYMPUS_UUID,
                block_on(repo.undo())
                    .unwrap()
                    .unwrap()
                    .unwrap()
                    .uuid()
                    .unwrap(),
            );

            assert!(block_on(repo.get_by_name("Olympus")).is_ok());
            assert!(block_on(data_store.get_all_the_things())
                .unwrap()
                .iter()
                .any(|t| t.name().value().map_or(false, |s| s == "Olympus")));
        }

        {
            assert_eq!(
                &OLYMPUS_UUID,
                block_on(repo.redo())
                    .unwrap()
                    .unwrap()
                    .unwrap()
                    .uuid()
                    .unwrap(),
            );
            assert!(block_on(repo.get_by_name("Hades")).is_ok());
        }
    }

    #[test]
    fn change_test_edit_by_uuid_wrong_type() {
        let mut repo = repo();
        let change = Change::Edit {
            name: "Olympus".into(),
            uuid: Some(OLYMPUS_UUID),
            diff: Npc::default().into(),
        };

        assert_eq!(
            block_on(repo.modify(change.clone())),
            Err((change, Error::NotFound)),
        );
    }

    #[test]
    fn change_test_edit_by_uuid_not_found() {
        let mut repo = repo();
        let change = Change::Edit {
            name: "Nobody".into(),
            uuid: Some(Uuid::nil()),
            diff: Npc::default().into(),
        };

        assert_eq!(
            block_on(repo.modify(change.clone())),
            Err((change, Error::NotFound)),
        );
    }

    #[test]
    fn change_test_edit_by_uuid_data_store_failed() {
        let mut repo = repo();
        repo.data_store = Box::new(NullDataStore::default());
        let change = Change::Edit {
            name: "Olympus".into(),
            uuid: Some(OLYMPUS_UUID),
            diff: Place {
                name: "Hades".into(),
                ..Default::default()
            }
            .into(),
        };

        assert_eq!(
            Err((
                Change::Edit {
                    name: "Olympus".into(),
                    uuid: Some(OLYMPUS_UUID),
                    diff: Place {
                        name: "Hades".into(),
                        ..Default::default()
                    }
                    .into(),
                },
                Error::DataStoreFailed,
            )),
            block_on(repo.modify(change)),
        );
    }

    #[test]
    fn change_test_edit_and_unsave_success() {
        let (mut repo, data_store) = repo_data_store();
        let change = Change::EditAndUnsave {
            name: "Olympus".into(),
            uuid: OLYMPUS_UUID.into(),
            diff: Place {
                name: "Hades".into(),
                description: "This really is hell!".into(),
                ..Default::default()
            }
            .into(),
        };
        assert_eq!("editing Olympus", change.display_redo().to_string());

        {
            assert_eq!(
                "Hades",
                block_on(repo.modify(change))
                    .unwrap()
                    .unwrap()
                    .name()
                    .value()
                    .unwrap(),
            );
            let result = repo.undo_history().next().unwrap();

            assert_eq!(
                &Change::Edit {
                    name: "Hades".into(),
                    uuid: None,
                    diff: Place {
                        name: "Olympus".into(),
                        description: None.into(),
                        ..Default::default()
                    }
                    .into()
                },
                result,
            );
            assert_eq!("editing Hades", result.display_undo().to_string());
            assert!(block_on(repo.get_by_name("Hades")).is_ok());
            assert_eq!(2, repo.recent().count());
            assert_eq!(3, block_on(repo.journal()).unwrap().len());
            assert_eq!(3, block_on(data_store.get_all_the_things()).unwrap().len());
        }

        {
            let thing = block_on(repo.undo()).unwrap().unwrap().unwrap();
            let uuid = thing.uuid().unwrap();
            assert_ne!(&OLYMPUS_UUID, uuid);
            assert!(block_on(repo.get_by_name("Olympus")).is_ok());
            assert!(block_on(repo.get_by_uuid(&uuid)).is_ok());
            assert_eq!(1, repo.recent().count());
            assert_eq!(4, block_on(repo.journal()).unwrap().len());
            assert!(block_on(data_store.get_all_the_things())
                .unwrap()
                .iter()
                .any(|t| t.name().value().map_or(false, |s| s == "Olympus")));
        }

        {
            assert_eq!(
                "Hades",
                block_on(repo.redo())
                    .unwrap()
                    .unwrap()
                    .unwrap()
                    .name()
                    .value()
                    .unwrap(),
            );
            assert!(block_on(repo.get_by_name("Hades")).is_ok());
        }
    }

    #[test]
    fn change_test_edit_and_unsave_not_found() {
        let mut repo = repo();
        let change = Change::EditAndUnsave {
            name: "Nobody".into(),
            uuid: Uuid::nil(),
            diff: Npc::default().into(),
        };

        assert_eq!(
            block_on(repo.modify(change.clone())),
            Err((change, Error::NotFound)),
        );
    }

    #[test]
    fn change_test_edit_and_unsave_data_store_failed() {
        let mut repo = Repository::new(TimeBombDataStore::new(7));
        populate_repo(&mut repo);

        let change = Change::EditAndUnsave {
            name: "Olympus".into(),
            uuid: OLYMPUS_UUID,
            diff: Place {
                name: "Hades".into(),
                description: "This really is hell!".into(),
                ..Default::default()
            }
            .into(),
        };

        assert_eq!(
            (
                Change::Unsave {
                    name: "Hades".into(),
                    uuid: OLYMPUS_UUID,
                },
                Error::DataStoreFailed,
            ),
            block_on(repo.modify(change)).unwrap_err(),
        );
        assert!(block_on(repo.get_by_name("Hades")).is_ok());
    }

    #[test]
    fn change_test_create_success() {
        let mut repo = empty_repo();
        let change = Change::Create {
            thing: Npc {
                name: "Odysseus".into(),
                ..Default::default()
            }
            .into(),
        };
        assert_eq!("creating Odysseus", change.display_redo().to_string());

        {
            assert_eq!(
                "Odysseus",
                block_on(repo.modify(change))
                    .unwrap()
                    .unwrap()
                    .name()
                    .value()
                    .unwrap(),
            );
            let result = repo.undo_history().next().unwrap();

            assert_eq!(
                &Change::Delete {
                    name: "Odysseus".to_string(),
                    uuid: None,
                },
                result,
            );
            assert_eq!("creating Odysseus", result.display_undo().to_string());
            assert_eq!(1, repo.recent().count());
        }

        {
            assert_eq!(Some(Ok(None)), block_on(repo.undo()));

            assert_eq!(
                Some(Change::Create {
                    thing: Npc {
                        name: "Odysseus".into(),
                        ..Default::default()
                    }
                    .into(),
                }),
                repo.redo_change,
            );
            assert_eq!(0, repo.recent().count());
        }
    }

    #[test]
    fn change_test_create_already_exists_in_journal() {
        let (mut repo, data_store) = repo_data_store();
        let change = Change::Create {
            thing: Npc {
                name: "Olympus".into(),
                ..Default::default()
            }
            .clone()
            .into(),
        };

        assert_eq!(
            block_on(repo.modify(change.clone())),
            Err((change, Error::NameAlreadyExists)),
        );
        assert_eq!(4, block_on(repo.journal()).unwrap().len());
        assert_eq!(4, block_on(data_store.get_all_the_things()).unwrap().len());
    }

    #[test]
    fn change_test_create_already_exists_in_recent() {
        let mut repo = repo();
        let change = Change::Create {
            thing: Place {
                name: "Odysseus".into(),
                ..Default::default()
            }
            .clone()
            .into(),
        };

        assert_eq!(
            block_on(repo.modify(change.clone())),
            Err((change, Error::NameAlreadyExists)),
        );
        assert_eq!(1, repo.recent().count());
    }

    #[test]
    fn change_test_save_success() {
        let (mut repo, data_store) = repo_data_store();
        let change = Change::Save {
            name: "ODYSSEUS".to_string(),
        };
        assert_eq!(
            "saving ODYSSEUS to journal",
            change.display_redo().to_string(),
        );

        {
            let thing = block_on(repo.modify(change)).unwrap().unwrap();
            let uuid = thing.uuid().unwrap();
            let result = repo.undo_history().next().unwrap();

            assert_eq!(
                &Change::Unsave {
                    name: "Odysseus".into(),
                    uuid: *uuid,
                },
                result,
            );
            assert_eq!(
                "saving Odysseus to journal",
                result.display_undo().to_string(),
            );
            assert_eq!(5, block_on(repo.journal()).unwrap().len());
            assert_eq!(5, block_on(data_store.get_all_the_things()).unwrap().len());
            assert_eq!(0, repo.recent().count());
        }

        {
            match block_on(repo.undo()) {
                Some(Ok(Some(Thing::Npc(npc)))) => {
                    assert!(npc.uuid.is_none());
                    assert_eq!("Odysseus", npc.name.value().unwrap());
                }
                v => panic!("{:?}", v),
            }

            assert_eq!(
                Some(Change::Save {
                    name: "Odysseus".to_string(),
                }),
                repo.redo_change,
            );
            assert_eq!(4, block_on(repo.journal()).unwrap().len());
            assert_eq!(4, block_on(data_store.get_all_the_things()).unwrap().len());
            assert_eq!(1, repo.recent().count());
        }
    }

    #[test]
    fn change_test_save_data_store_failed() {
        let mut repo = null_repo();

        block_on(
            repo.modify(Change::Create {
                thing: Place {
                    name: "Odysseus".into(),
                    ..Default::default()
                }
                .into(),
            }),
        )
        .unwrap();

        assert_eq!(1, repo.recent().count());

        let change = Change::Save {
            name: "ODYSSEUS".to_string(),
        };
        assert_eq!(
            block_on(repo.modify(change.clone())),
            Err((change, Error::DataStoreFailed)),
        );

        assert_eq!(1, repo.recent().count());
    }

    #[test]
    fn change_test_save_already_saved() {
        let mut repo = repo();

        let change = Change::Save {
            name: "OLYMPUS".to_string(),
        };
        assert_eq!(
            block_on(repo.modify(change.clone())),
            Err((change, Error::NotFound)),
        );
    }

    #[test]
    fn change_test_save_not_found() {
        let mut repo = repo();
        let change = Change::Save {
            name: "NOBODY".to_string(),
        };

        assert_eq!(
            block_on(repo.modify(change.clone())),
            Err((change, Error::NotFound)),
        );
    }

    #[test]
    fn change_test_unsave_success() {
        let (mut repo, data_store) = repo_data_store();
        let change = Change::Unsave {
            name: "Olympus".to_string(),
            uuid: OLYMPUS_UUID.clone(),
        };
        assert_eq!(
            "removing Olympus from journal",
            change.display_redo().to_string(),
        );

        {
            assert_eq!(
                "Olympus",
                block_on(repo.modify(change))
                    .unwrap()
                    .unwrap()
                    .name()
                    .value()
                    .unwrap(),
            );
            let result = repo.undo_history().next().unwrap();

            assert_eq!(
                &Change::Save {
                    name: "Olympus".to_string(),
                },
                result,
            );
            assert_eq!(
                "removing Olympus from journal",
                result.display_undo().to_string(),
            );
            assert_eq!(3, block_on(repo.journal()).unwrap().len());
            assert_eq!(3, block_on(data_store.get_all_the_things()).unwrap().len());
            assert_eq!(2, repo.recent().count());
            assert_eq!(None, block_on(repo.get_by_name("Olympus")).unwrap().uuid());
        }

        {
            match block_on(repo.undo()) {
                Some(Ok(Some(Thing::Place(place)))) => {
                    assert!(place.uuid.is_some());
                    assert_eq!("Olympus", place.name.value().unwrap());
                }
                v => panic!("{:?}", v),
            }

            if let Some(Change::Unsave { ref name, uuid }) = repo.redo_change {
                assert_eq!("Olympus", name);
                assert_ne!(OLYMPUS_UUID, uuid);
                assert!(block_on(repo.get_by_uuid(&uuid)).is_ok());
            } else {
                panic!();
            }
            assert_eq!(4, block_on(repo.journal()).unwrap().len());
            assert_eq!(4, block_on(data_store.get_all_the_things()).unwrap().len());
            assert_eq!(1, repo.recent().count());
        }
    }

    #[test]
    fn change_test_create_and_save_success() {
        let (mut repo, data_store) = empty_repo_data_store();
        let change = Change::CreateAndSave {
            thing: Npc {
                name: "Odysseus".into(),
                ..Default::default()
            }
            .into(),
        };
        assert_eq!("creating Odysseus", change.display_redo().to_string());

        {
            let thing = block_on(repo.modify(change)).unwrap().unwrap();
            let uuid = thing.uuid().unwrap();
            let result = repo.undo_history().next().unwrap();

            assert_eq!(
                &Change::Delete {
                    name: "Odysseus".into(),
                    uuid: Some(*uuid),
                },
                result,
            );
            assert!(block_on(repo.get_by_uuid(uuid)).is_ok());
            assert_eq!("creating Odysseus", result.display_undo().to_string());
            assert_eq!(1, block_on(repo.journal()).unwrap().len());
            assert_eq!(
                uuid,
                block_on(data_store.get_all_the_things())
                    .unwrap()
                    .first()
                    .unwrap()
                    .uuid()
                    .unwrap(),
            );
        }

        {
            let uuid = *block_on(repo.journal())
                .unwrap()
                .first()
                .unwrap()
                .uuid()
                .unwrap();
            assert_eq!(Some(Ok(None)), block_on(repo.undo()));

            assert_eq!(
                Some(Change::CreateAndSave {
                    thing: Npc {
                        uuid: Some(uuid.into()),
                        name: "Odysseus".into(),
                        ..Default::default()
                    }
                    .into(),
                }),
                repo.redo_change,
            );
            assert_eq!(0, block_on(repo.journal()).unwrap().len());
            assert_eq!(0, block_on(data_store.get_all_the_things()).unwrap().len());
        }
    }

    #[test]
    fn change_test_create_and_save_already_exists_in_journal() {
        let (mut repo, data_store) = repo_data_store();
        let change = Change::CreateAndSave {
            thing: Place {
                name: "Odysseus".into(),
                ..Default::default()
            }
            .clone()
            .into(),
        };

        assert_eq!(
            block_on(repo.modify(change.clone())),
            Err((change, Error::NameAlreadyExists)),
        );
        assert_eq!(4, block_on(repo.journal()).unwrap().len());
        assert_eq!(4, block_on(data_store.get_all_the_things()).unwrap().len());
    }

    #[test]
    fn change_test_create_and_save_already_exists_in_recent() {
        let (mut repo, data_store) = repo_data_store();
        let change = Change::CreateAndSave {
            thing: Npc {
                name: "Olympus".into(),
                ..Default::default()
            }
            .clone()
            .into(),
        };

        assert_eq!(
            block_on(repo.modify(change.clone())),
            Err((change, Error::NameAlreadyExists)),
        );
        assert_eq!(4, block_on(repo.journal()).unwrap().len());
        assert_eq!(4, block_on(data_store.get_all_the_things()).unwrap().len());
    }

    #[test]
    fn change_test_create_and_save_data_store_failed() {
        let mut repo = null_repo();

        let change = Change::CreateAndSave {
            thing: Place {
                name: "Odysseus".into(),
                ..Default::default()
            }
            .into(),
        };

        assert_eq!(
            block_on(repo.modify(change.clone())),
            Err((change, Error::DataStoreFailed)),
        );
    }

    #[test]
    fn change_test_set_key_value_success() {
        let mut repo = repo();

        let one = Time::try_new(1, 0, 0, 0).unwrap();
        let two = Time::try_new(2, 0, 0, 0).unwrap();

        assert_eq!(
            Ok(KeyValue::Time(None)),
            block_on(repo.get_key_value(&KeyValue::Time(None)))
        );

        assert_eq!(
            Ok(None),
            block_on(repo.modify(Change::SetKeyValue {
                key_value: KeyValue::Time(Some(one.clone())),
            })),
        );

        {
            let undo_result = repo.undo_history().next().unwrap();

            assert_eq!(
                &Change::SetKeyValue {
                    key_value: KeyValue::Time(None),
                },
                undo_result,
            );
            assert_eq!("changing the time", undo_result.display_undo().to_string());
            assert_eq!("changing the time", undo_result.display_redo().to_string());
        }

        block_on(repo.modify(Change::SetKeyValue {
            key_value: KeyValue::Time(Some(two.clone())),
        }))
        .unwrap();

        block_on(repo.modify(Change::SetKeyValue {
            key_value: KeyValue::Time(None),
        }))
        .unwrap();

        assert_eq!(
            Ok(KeyValue::Time(None)),
            block_on(repo.get_key_value(&KeyValue::Time(None)))
        );

        assert_eq!(Some(Ok(None)), block_on(repo.undo()));

        assert_eq!(
            Ok(KeyValue::Time(Some(two))),
            block_on(repo.get_key_value(&KeyValue::Time(None)))
        );

        block_on(repo.undo());

        assert_eq!(
            Ok(KeyValue::Time(Some(one))),
            block_on(repo.get_key_value(&KeyValue::Time(None)))
        );

        block_on(repo.undo());

        assert_eq!(
            Ok(KeyValue::Time(None)),
            block_on(repo.get_key_value(&KeyValue::Time(None)))
        );
    }

    #[test]
    fn change_test_set_key_value_data_store_failed() {
        let change = Change::SetKeyValue {
            key_value: KeyValue::Time(Some(Time::default())),
        };

        assert_eq!(
            block_on(null_repo().modify(change.clone())),
            Err((change, Error::DataStoreFailed)),
        );
    }

    #[test]
    fn load_relations_test_with_parent_success() {
        let repo = repo();
        let odysseus = block_on(repo.get_by_name("Odysseus")).unwrap();

        match block_on(repo.load_relations(&odysseus)) {
            Ok(ThingRelations::Npc(NpcRelations {
                location: Some((parent, None)),
            })) => {
                assert_eq!("River Styx", parent.name.value().unwrap());
            }
            r => panic!("{:?}", r),
        }
    }

    #[test]
    fn load_relations_test_with_grandparent_success() {
        let repo = repo();
        let olympus = block_on(repo.get_by_uuid(&OLYMPUS_UUID)).unwrap();

        match block_on(repo.load_relations(&olympus)) {
            Ok(ThingRelations::Place(PlaceRelations {
                location: Some((parent, Some(grandparent))),
            })) => {
                assert_eq!("Thessaly", parent.name.value().unwrap());
                assert_eq!("Greece", grandparent.name.value().unwrap());
            }
            r => panic!("{:?}", r),
        }
    }

    #[test]
    fn debug_test() {
        assert_eq!(
            "Repository { data_store_enabled: false, recent: [] }",
            format!("{:?}", empty_repo()),
        );
    }

    #[test]
    fn data_store_enabled_test_success() {
        let mut repo = repo();
        block_on(repo.init());
        assert_eq!(true, repo.data_store_enabled());
    }

    #[test]
    fn data_store_enabled_test_failure() {
        let mut repo = null_repo();
        block_on(repo.init());
        assert_eq!(false, repo.data_store_enabled());
    }

    fn repo() -> Repository {
        repo_data_store().0
    }

    fn repo_data_store() -> (Repository, MemoryDataStore) {
        let data_store = MemoryDataStore::default();
        let mut repo = Repository::new(data_store.clone());
        populate_repo(&mut repo);
        (repo, data_store)
    }

    fn empty_repo() -> Repository {
        Repository::new(MemoryDataStore::default())
    }

    fn empty_repo_data_store() -> (Repository, MemoryDataStore) {
        let data_store = MemoryDataStore::default();
        (Repository::new(data_store.clone()), data_store)
    }

    fn null_repo() -> Repository {
        Repository::new(NullDataStore::default())
    }

    fn populate_repo(repo: &mut Repository) {
        block_on(
            repo.data_store.save_thing(
                &Place {
                    uuid: Some(OLYMPUS_UUID.into()),
                    location_uuid: PlaceUuid::from(THESSALY_UUID).into(),
                    name: "Olympus".into(),
                    ..Default::default()
                }
                .into(),
            ),
        )
        .unwrap();
        block_on(
            repo.data_store.save_thing(
                &Place {
                    uuid: Some(THESSALY_UUID.into()),
                    location_uuid: PlaceUuid::from(GREECE_UUID).into(),
                    name: "Thessaly".into(),
                    ..Default::default()
                }
                .into(),
            ),
        )
        .unwrap();
        block_on(
            repo.data_store.save_thing(
                &Place {
                    uuid: Some(GREECE_UUID.into()),
                    name: "Greece".into(),
                    ..Default::default()
                }
                .into(),
            ),
        )
        .unwrap();
        block_on(
            repo.data_store.save_thing(
                &Place {
                    uuid: Some(STYX_UUID.into()),
                    location_uuid: PlaceUuid::from(Uuid::nil()).into(),
                    name: "River Styx".into(),
                    ..Default::default()
                }
                .into(),
            ),
        )
        .unwrap();

        repo.recent.push_back(
            Npc {
                name: "Odysseus".into(),
                location_uuid: PlaceUuid::from(STYX_UUID).into(),
                ..Default::default()
            }
            .into(),
        );

        block_on(repo.init());
    }

    struct TimeBombDataStore {
        t_minus: Rc<RefCell<usize>>,
        data_store: MemoryDataStore,
    }

    impl TimeBombDataStore {
        pub fn new(t_minus: usize) -> Self {
            Self {
                t_minus: Rc::new(t_minus.into()),
                data_store: MemoryDataStore::default(),
            }
        }

        fn tick(&self) -> Result<(), ()> {
            if *self.t_minus.borrow() == 0 {
                Err(())
            } else {
                self.t_minus.replace_with(|&mut i| i - 1);
                Ok(())
            }
        }
    }

    #[async_trait(?Send)]
    impl DataStore for TimeBombDataStore {
        async fn health_check(&self) -> Result<(), ()> {
            if *self.t_minus.borrow() == 0 {
                Err(())
            } else {
                Ok(())
            }
        }

        async fn delete_thing_by_uuid(&mut self, uuid: &Uuid) -> Result<(), ()> {
            self.tick()?;
            self.data_store.delete_thing_by_uuid(uuid).await
        }

        async fn edit_thing(&mut self, thing: &Thing) -> Result<(), ()> {
            self.tick()?;
            self.data_store.edit_thing(thing).await
        }

        async fn get_all_the_things(&self) -> Result<Vec<Thing>, ()> {
            self.tick()?;
            self.data_store.get_all_the_things().await
        }

        async fn get_thing_by_uuid(&self, uuid: &Uuid) -> Result<Option<Thing>, ()> {
            self.tick()?;
            self.data_store.get_thing_by_uuid(uuid).await
        }

        async fn get_thing_by_name(&self, name: &str) -> Result<Option<Thing>, ()> {
            self.tick()?;
            self.data_store.get_thing_by_name(name).await
        }

        async fn get_things_by_name_start(
            &self,
            name: &str,
            limit: Option<usize>,
        ) -> Result<Vec<Thing>, ()> {
            self.tick()?;
            self.data_store.get_things_by_name_start(name, limit).await
        }

        async fn save_thing(&mut self, thing: &Thing) -> Result<(), ()> {
            self.tick()?;
            self.data_store.save_thing(thing).await
        }

        async fn set_value(&mut self, key: &str, value: &str) -> Result<(), ()> {
            self.tick()?;
            self.data_store.set_value(key, value).await
        }

        async fn get_value(&self, key: &str) -> Result<Option<String>, ()> {
            self.tick()?;
            self.data_store.get_value(key).await
        }

        async fn delete_value(&mut self, key: &str) -> Result<(), ()> {
            self.tick()?;
            self.data_store.delete_value(key).await
        }
    }
}
