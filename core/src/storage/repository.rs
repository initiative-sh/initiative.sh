use crate::storage::{DataStore, MemoryDataStore};
use crate::time::Time;
use crate::utils::CaseInsensitiveStr;
use crate::world::npc::{NpcData, NpcRelations};
use crate::world::place::{Place, PlaceData, PlaceRelations};
use crate::world::thing::{Thing, ThingData, ThingRelations};
use crate::Uuid;
use futures::join;
use std::collections::VecDeque;
use std::fmt;

type Name = String;

const RECENT_MAX_LEN: usize = 100;
const UNDO_HISTORY_LEN: usize = 10;

pub struct Repository {
    data_store: Box<dyn DataStore>,
    data_store_enabled: bool,
    recent: VecDeque<Thing>,
    redo_change: Option<Change>,
    undo_history: VecDeque<Change>,
}

/// Represents a modification to be applied to the Repository. This is passed to
/// Repository::modify() to be applied. An object is used to represent the change because every
/// operation has an opposite; for instance, Unsave is the opposite of Save, and Edit is the
/// opposite of Edit. This opposite is inserted into the undo history and can be applied using
/// Repository::undo().
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Change {
    /// Create a new thing and store it in recent entries.
    ///
    /// Reverse: Delete
    Create {
        thing_data: ThingData,
        uuid: Option<Uuid>,
    },

    /// Create a new thing and store it in the journal.
    ///
    /// Reverse: Delete
    CreateAndSave {
        thing_data: ThingData,
        uuid: Option<Uuid>,
    },

    /// Delete a thing from recent or journal.
    ///
    /// Reverse: Create (recent) or CreateAndSave (journal)
    Delete { uuid: Uuid, name: Name },

    /// Edit fields on a Thing.
    ///
    /// Reverse: Edit (already in journal) or EditAndUnsave (in recent)
    Edit {
        name: Name,
        uuid: Option<Uuid>,
        diff: ThingData,
    },

    /// Edit a Thing and move it from journal to recent. The reverse of edit with autosave.
    ///
    /// Reverse: Edit
    EditAndUnsave {
        uuid: Uuid,
        name: Name,
        diff: ThingData,
    },

    /// Transfer a thing from recent to journal.
    ///
    /// Reverse: Unsave
    Save { name: Name, uuid: Option<Uuid> },

    /// Transfer a thing from journal to recent. Only triggerable as the reverse to Save.
    ///
    /// Reverse: Save
    Unsave { uuid: Uuid, name: Name },

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
    UuidAlreadyExists(Thing),
    NameAlreadyExists(Thing),
    NotFound,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum KeyValue {
    Time(Option<Time>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Record {
    pub status: RecordStatus,
    pub thing: Thing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecordStatus {
    Unsaved,
    Saved,
    Deleted,
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

    /// The data store will not necessarily be available at construct, so we need to check if it's
    /// healthy or discard it and fall back on a memory data store instead.
    pub async fn init(&mut self) {
        if self.data_store.health_check().await.is_ok() {
            self.data_store_enabled = true;
        } else {
            self.data_store = Box::<MemoryDataStore>::default();
        }
    }

    /// Get the record associated with a given change, if available.
    pub async fn get_by_change(&self, change: &Change) -> Result<Record, Error> {
        let (name, uuid) = match change {
            Change::Create {
                uuid: Some(uuid), ..
            }
            | Change::CreateAndSave {
                uuid: Some(uuid), ..
            }
            | Change::EditAndUnsave { uuid, .. }
            | Change::Save {
                uuid: Some(uuid), ..
            }
            | Change::Unsave { uuid, .. }
            | Change::Delete { uuid, .. }
            | Change::Edit {
                uuid: Some(uuid), ..
            } => (None, Some(uuid)),
            Change::Create { thing_data, .. } | Change::CreateAndSave { thing_data, .. } => {
                (thing_data.name().value(), None)
            }
            Change::Save { name, .. } | Change::Edit { name, .. } => (Some(name), None),
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

    /// Load child and grandchild relations associated with a Thing (eg. location).
    pub async fn load_relations(&self, thing: &Thing) -> Result<ThingRelations, Error> {
        let locations = {
            let parent_uuid = match &thing.data {
                ThingData::Npc(NpcData { location_uuid, .. }) => location_uuid,
                ThingData::Place(PlaceData { location_uuid, .. }) => location_uuid,
            };

            let parent = {
                let parent_result = if let Some(uuid) = parent_uuid.value() {
                    self.get_by_uuid(uuid).await.and_then(|record| {
                        Place::try_from(record.thing).map_err(|_| Error::NotFound)
                    })
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
                    let grandparent_result = if let Some(uuid) = parent.data.location_uuid.value() {
                        self.get_by_uuid(uuid).await.and_then(|record| {
                            Place::try_from(record.thing).map_err(|_| Error::NotFound)
                        })
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

        match thing.data {
            ThingData::Npc(..) => Ok(NpcRelations {
                location: locations,
            }
            .into()),
            ThingData::Place(..) => Ok(PlaceRelations {
                location: locations,
            }
            .into()),
        }
    }

    /// Get all saved and recent Things beginning with a given (case-insensitive) string, up to an
    /// optional limit.
    pub async fn get_by_name_start(
        &self,
        name: &str,
        limit: Option<usize>,
    ) -> Result<Vec<Record>, Error> {
        Ok(self
            .data_store
            .get_things_by_name_start(name, limit)
            .await
            .map_err(|_| Error::DataStoreFailed)?
            .into_iter()
            .map(|thing| Record {
                status: RecordStatus::Saved,
                thing,
            })
            .chain(
                self.recent()
                    .filter(|t| t.name().value().map_or(false, |s| s.starts_with_ci(name)))
                    .map(|thing| Record {
                        status: RecordStatus::Unsaved,
                        thing: thing.clone(),
                    }),
            )
            .take(limit.unwrap_or(usize::MAX))
            .collect())
    }

    /// Get an iterator over all recent Things.
    pub fn recent(&self) -> impl Iterator<Item = &Thing> {
        let (a, b) = self.recent.as_slices();
        a.iter().chain(b.iter())
    }

    /// Get all Things contained in the journal. This could get heavy, so should not be used
    /// lightly.
    pub async fn journal(&self) -> Result<Vec<Thing>, Error> {
        self.data_store
            .get_all_the_things()
            .await
            .map_err(|_| Error::DataStoreFailed)
    }

    /// Get the Thing from saved or recent with a given name. (There should be only one.)
    pub async fn get_by_name(&self, name: &str) -> Result<Record, Error> {
        let (recent_thing, saved_thing) = join!(
            async {
                self.recent()
                    .find(|t| t.name().value().map_or(false, |s| s.eq_ci(name)))
            },
            self.data_store.get_thing_by_name(name)
        );

        match (recent_thing, saved_thing) {
            (Some(thing), _) => Ok(Record {
                status: RecordStatus::Unsaved,
                thing: thing.clone(),
            }),
            (None, Ok(Some(thing))) => Ok(Record {
                status: RecordStatus::Saved,
                thing,
            }),
            (None, Ok(None)) => Err(Error::NotFound),
            (None, Err(())) => Err(Error::DataStoreFailed),
        }
    }

    /// Get the Thing from saved or recent with a given UUID. (There should be only one.)
    pub async fn get_by_uuid(&self, uuid: &Uuid) -> Result<Record, Error> {
        let (recent_thing, saved_thing) = join!(
            async { self.recent().find(|t| &t.uuid == uuid) },
            self.data_store.get_thing_by_uuid(uuid)
        );

        match (recent_thing, saved_thing) {
            (Some(thing), _) => Ok(Record {
                status: RecordStatus::Unsaved,
                thing: thing.clone(),
            }),
            (None, Ok(Some(thing))) => Ok(Record {
                status: RecordStatus::Saved,
                thing,
            }),
            (None, Ok(None)) => Err(Error::NotFound),
            (None, Err(())) => Err(Error::DataStoreFailed),
        }
    }

    /// Apply a given Change, returning the affected Thing (with modifications applied) on success,
    /// or a tuple of the Change and Error message on failure.
    pub async fn modify(&mut self, change: Change) -> Result<Option<Record>, (Change, Error)> {
        // If we're going to delete, we should load the record being deleted first because
        // otherwise it'll be gone!
        let mut option_record = if matches!(change, Change::Delete { .. }) {
            self.get_by_change(&change).await.ok().map(|mut record| {
                record.status = RecordStatus::Deleted;
                record
            })
        } else {
            None
        };

        let undo_change = self.modify_without_undo(change).await?;

        if option_record.is_none() {
            option_record = self.get_by_change(&undo_change).await.ok();
        }

        while self.undo_history.len() >= UNDO_HISTORY_LEN {
            self.undo_history.pop_front();
        }
        self.undo_history.push_back(undo_change);

        Ok(option_record)
    }

    /// Undo the most recent Change. Returns None if the undo history is empty; otherwise returns
    /// the Result of the modify() operation.
    pub async fn undo(&mut self) -> Option<Result<Option<Record>, Error>> {
        if let Some(change) = self.undo_history.pop_back() {
            match self.modify_without_undo(change).await {
                Ok(redo_change) => {
                    let record = self.get_by_change(&redo_change).await.ok();
                    self.redo_change = Some(redo_change);
                    Some(Ok(record))
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

    /// Get an iterator over the Changes currently queued up in the undo history, from newest to
    /// oldest.
    pub fn undo_history(&self) -> impl Iterator<Item = &Change> {
        self.undo_history.iter().rev()
    }

    /// Redo the most recently undid Change. Returns None if no such change exists; otherwise
    /// returns the result of the modify() operation. This differs from undo() in that only one
    /// Change is stored in history at a time.
    pub async fn redo(&mut self) -> Option<Result<Option<Record>, Error>> {
        if let Some(change) = self.redo_change.take() {
            match self.modify(change).await {
                Ok(option_record) => Some(Ok(option_record)),
                Err((redo_change, e)) => {
                    self.redo_change = Some(redo_change);
                    Some(Err(e))
                }
            }
        } else {
            None
        }
    }

    /// Get the Change currently queued up for redo(), if any.
    pub fn get_redo(&self) -> Option<&Change> {
        self.redo_change.as_ref()
    }

    /// Apply a Change to the Repository without adding the Change to the undo history. Returns
    /// the reverse operation on success (what would be otherwise inserted into the undo history),
    /// or a tuple of the failed Change and error message on failure.
    pub async fn modify_without_undo(&mut self, change: Change) -> Result<Change, (Change, Error)> {
        match change {
            Change::Create { thing_data, uuid } => {
                let name = thing_data.name().to_string();
                self.create_thing(thing_data, uuid)
                    .await
                    .map(|uuid| Change::Delete { uuid, name })
                    .map_err(|(thing_data, e)| (Change::Create { thing_data, uuid }, e))
            }
            Change::CreateAndSave { thing_data, uuid } => {
                let name = thing_data.name().to_string();
                self.create_and_save_thing(thing_data, uuid)
                    .await
                    .map(|thing| Change::Delete {
                        uuid: thing.uuid,
                        name,
                    })
                    .map_err(|(thing_data, e)| (Change::CreateAndSave { thing_data, uuid }, e))
            }
            Change::Delete { uuid, name } => self
                .delete_thing_by_uuid(&uuid)
                .await
                .map(|Record { thing, status }| {
                    if status == RecordStatus::Saved {
                        Change::CreateAndSave {
                            thing_data: thing.data,
                            uuid: Some(thing.uuid),
                        }
                    } else {
                        Change::Create {
                            thing_data: thing.data,
                            uuid: Some(thing.uuid),
                        }
                    }
                })
                .map_err(|(_, e)| (Change::Delete { uuid, name }, e)),
            Change::Edit {
                name,
                uuid: None,
                diff,
            } => match self.edit_thing_by_name(&name, diff).await {
                Ok((Record { thing, status }, name)) => {
                    if status == RecordStatus::Saved {
                        Ok(Change::Edit {
                            uuid: Some(thing.uuid),
                            name,
                            diff: thing.data,
                        })
                    } else {
                        Ok(Change::EditAndUnsave {
                            uuid: thing.uuid,
                            name,
                            diff: thing.data,
                        })
                    }
                }
                Err((option_record, diff, e)) => Err((
                    Change::Edit {
                        name: option_record
                            .map(|record| record.thing.name().value().map(String::from))
                            .unwrap_or(None)
                            .unwrap_or(name),
                        uuid: None,
                        diff,
                    },
                    e,
                )),
            },
            Change::Edit {
                name,
                uuid: Some(uuid),
                diff,
            } => match self.edit_thing_by_uuid(&uuid, diff).await {
                Ok((Record { thing, status }, name)) => {
                    let diff = thing.data;

                    if status == RecordStatus::Saved {
                        let uuid = Some(uuid);
                        Ok(Change::Edit { uuid, name, diff })
                    } else {
                        Ok(Change::EditAndUnsave { uuid, name, diff })
                    }
                }
                Err((option_record, diff, e)) => Err((
                    Change::Edit {
                        name: option_record
                            .map(|record| record.thing.name().value().map(String::from))
                            .unwrap_or(None)
                            .unwrap_or(name),
                        uuid: Some(uuid),
                        diff,
                    },
                    e,
                )),
            },
            Change::EditAndUnsave { uuid, name, diff } => {
                match self.edit_thing_by_uuid(&uuid, diff).await {
                    Ok((Record { thing, .. }, name)) => self
                        .unsave_thing_by_uuid(&uuid)
                        .await
                        .map(|name| Change::Edit {
                            name,
                            uuid: Some(uuid),
                            diff: thing.data,
                        })
                        .map_err(|(s, e)| {
                            (
                                Change::Unsave {
                                    uuid,
                                    name: s.unwrap_or(name),
                                },
                                e,
                            )
                        }),
                    Err((_, diff, e)) => Err((Change::EditAndUnsave { uuid, name, diff }, e)),
                }
            }
            Change::Save {
                name,
                uuid: Some(uuid),
            } => match self.save_thing_by_uuid(&uuid).await {
                Ok(thing) => Ok(Change::Unsave {
                    uuid,
                    name: thing.name().value().map(String::from).unwrap_or(name),
                }),
                Err(e) => Err((
                    Change::Save {
                        name,
                        uuid: Some(uuid),
                    },
                    e,
                )),
            },
            Change::Save { name, uuid: None } => match self.save_thing_by_name(&name).await {
                Ok(thing) => Ok(Change::Unsave {
                    uuid: thing.uuid,
                    name: thing.name().value().map(String::from).unwrap_or(name),
                }),
                Err(e) => Err((Change::Save { name, uuid: None }, e)),
            },
            Change::Unsave { uuid, name } => self
                .unsave_thing_by_uuid(&uuid)
                .await
                .map(|name| Change::Save {
                    name,
                    uuid: Some(uuid),
                })
                .map_err(|(_, e)| (Change::Unsave { uuid, name }, e)),
            Change::SetKeyValue { key_value } => self
                .set_key_value(&key_value)
                .await
                .map(|old_kv| Change::SetKeyValue { key_value: old_kv })
                .map_err(|e| (Change::SetKeyValue { key_value }, e)),
        }
    }

    /// Get a value from the key-value store.
    pub async fn get_key_value(&self, key: &KeyValue) -> Result<KeyValue, Error> {
        let value_str = self.data_store.get_value(key.key_raw()).await;

        match key {
            KeyValue::Time(_) => value_str
                .and_then(|o| o.map(|s| s.parse()).transpose())
                .map(KeyValue::Time),
        }
        .map_err(|_| Error::DataStoreFailed)
    }

    /// Is the data store currently enabled? Returns false if init() has not yet been called.
    pub fn data_store_enabled(&self) -> bool {
        self.data_store_enabled
    }

    /// Set a value in the key-value store.
    ///
    /// Publicly this is done using modify() with Change::SetKeyValue.
    async fn set_key_value(&mut self, key_value: &KeyValue) -> Result<KeyValue, Error> {
        let old_key_value = self.get_key_value(key_value).await?;

        match key_value.key_value_raw() {
            (key, Some(value)) => self.data_store.set_value(key, &value).await,
            (key, None) => self.data_store.delete_value(key).await,
        }
        .map(|_| old_key_value)
        .map_err(|_| Error::DataStoreFailed)
    }

    /// Add a Thing to the recent list.
    fn push_recent(&mut self, thing: Thing) {
        while self.recent.len() >= RECENT_MAX_LEN {
            self.recent.pop_front();
        }

        self.recent.push_back(thing);
    }

    /// Remove the latest Thing in the recent list, returning it if one is present.
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

    /// Create a Thing, pushing it onto the recent list.
    ///
    /// Publicly this is invoked using modify() with Change::Create.
    async fn create_thing(
        &mut self,
        thing_data: ThingData,
        uuid: Option<Uuid>,
    ) -> Result<Uuid, (ThingData, Error)> {
        let thing = self.thing_data_into_thing(thing_data, uuid).await?;
        let uuid = thing.uuid;
        self.push_recent(thing);
        Ok(uuid)
    }

    /// Create a Thing and save it directly to the journal.
    ///
    /// Publicly this is invoked using modify() with Change::CreateAndSave.
    async fn create_and_save_thing(
        &mut self,
        thing_data: ThingData,
        uuid: Option<Uuid>,
    ) -> Result<Thing, (ThingData, Error)> {
        let thing = self.thing_data_into_thing(thing_data, uuid).await?;

        match self.save_thing(&thing).await {
            Ok(()) => Ok(thing),
            Err(e) => Err((thing.data, e)),
        }
    }

    /// Delete a Thing from recent or journal by its UUID.
    ///
    /// Publicly this is invoked using modify() with Change::Delete.
    async fn delete_thing_by_uuid(
        &mut self,
        uuid: &Uuid,
    ) -> Result<Record, (Option<Record>, Error)> {
        if let Some(thing) = self.take_recent(|t| &t.uuid == uuid) {
            Ok(Record {
                status: RecordStatus::Unsaved,
                thing,
            })
        } else {
            let record = self.get_by_uuid(uuid).await.map_err(|e| (None, e))?;

            if self.data_store.delete_thing_by_uuid(uuid).await.is_ok() {
                Ok(record)
            } else {
                Err((Some(record), Error::DataStoreFailed))
            }
        }
    }

    /// Transfer a Thing from recent to the journal, referenced by its name. Returns the Thing
    /// transferred, or an error on failure.
    ///
    /// Publicly this is invoked using modify() with Change::Save { uuid: None, .. }
    async fn save_thing_by_name(&mut self, name: &Name) -> Result<Thing, Error> {
        if let Some(thing) = self.take_recent(|t| t.name().value().map_or(false, |s| s.eq_ci(name)))
        {
            match self.save_thing(&thing).await {
                Ok(()) => Ok(thing),
                Err(e) => {
                    self.push_recent(thing);
                    Err(e)
                }
            }
        } else {
            Err(Error::NotFound)
        }
    }

    /// Transfer a Thing from recent to the journal, referenced by its UUID. Returns the Thing
    /// transferred, or an error on failure.
    ///
    /// Publicly this is invoked using modify() with Change::Save { uuid: Some(_), .. }
    async fn save_thing_by_uuid(&mut self, uuid: &Uuid) -> Result<Thing, Error> {
        if let Some(thing) = self.take_recent(|t| &t.uuid == uuid) {
            match self.save_thing(&thing).await {
                Ok(()) => Ok(thing),
                Err(e) => {
                    self.push_recent(thing);
                    Err(e)
                }
            }
        } else {
            Err(Error::NotFound)
        }
    }

    /// Write a Thing to the data store.
    async fn save_thing(&mut self, thing: &Thing) -> Result<(), Error> {
        match self.data_store.save_thing(thing).await {
            Ok(()) => Ok(()),
            Err(()) => Err(Error::DataStoreFailed),
        }
    }

    /// Remove a Thing from the data store and add it to the recent list instead. Returns the name
    /// of the Thing transferred on success, or a tuple of the optional name and an error on
    /// failure. This is asymmetric with save_thing_by_* because writing to the recent list takes
    /// ownership while writing to the data store accepts a reference, so returning a Thing here
    /// would require an unnecessary clone() call when all we really want is the name of the Thing
    /// we just unsaved.
    ///
    /// Publicly this is invoked using modify() with Change::Unsave.
    async fn unsave_thing_by_uuid(&mut self, uuid: &Uuid) -> Result<Name, (Option<Name>, Error)> {
        let thing = match self.data_store.get_thing_by_uuid(uuid).await {
            Ok(Some(thing)) => Ok(thing),
            Ok(None) => Err((None, Error::NotFound)),
            Err(()) => Err((None, Error::DataStoreFailed)),
        }?;

        let name = thing.name().to_string();

        match self.data_store.delete_thing_by_uuid(uuid).await {
            Ok(()) => {
                self.push_recent(thing);
                Ok(name)
            }
            Err(()) => Err((Some(name), Error::DataStoreFailed)),
        }
    }

    /// Apply a diff to a Thing matched by name. See edit_thing() for details.
    ///
    /// Publicly this is invoked using modify() with Change::Edit { uuid: None, .. }.
    async fn edit_thing_by_name(
        &mut self,
        name: &Name,
        diff: ThingData,
    ) -> Result<(Record, Name), (Option<Record>, ThingData, Error)> {
        match self.get_by_name(name).await {
            Ok(record) => self
                .edit_thing(record, diff)
                .await
                .map_err(|(record, data, e)| (Some(record), data, e)),
            Err(e) => Err((None, diff, e)),
        }
    }

    /// Apply a diff to a Thing matched by UUID. See edit_thing() for details.
    ///
    /// Publicly this is invoked using modify() with Change::Edit { uuid: Some(_), .. }.
    async fn edit_thing_by_uuid(
        &mut self,
        uuid: &Uuid,
        diff: ThingData,
    ) -> Result<(Record, Name), (Option<Record>, ThingData, Error)> {
        match self.get_by_uuid(uuid).await {
            Ok(record) => self
                .edit_thing(record, diff)
                .await
                .map_err(|(record, data, e)| (Some(record), data, e)),
            Err(e) => Err((None, diff, e)),
        }
    }

    /// Apply a diff to a given Record. Returns a tuple consisting of a Record containing *the
    /// modified fields* and the matched Thing's actual name on success, or a tuple consisting of
    /// an optional Record of the matched Thing, the attempted diff, and an error message on
    /// failure. Note that the successful response includes only the old values of any modified
    /// fields, so re-applying the diff will revert the Thing back to its original state.
    ///
    /// Supports the edit_thing_by_* functions.
    async fn edit_thing(
        &mut self,
        mut record: Record,
        mut diff: ThingData,
    ) -> Result<(Record, Name), (Record, ThingData, Error)> {
        if record.thing.try_apply_diff(&mut diff).is_err() {
            // This fails when the thing types don't match, eg. applying an Npc diff to a
            // Place.
            return Err((record, diff, Error::NotFound));
        }

        let name = record.thing.name().to_string();
        let diff_thing = Thing {
            uuid: record.thing.uuid,
            data: diff,
        };

        if record.is_saved() {
            match self.data_store.edit_thing(&record.thing).await {
                Ok(()) => Ok((
                    Record {
                        status: RecordStatus::Saved,
                        thing: diff_thing,
                    },
                    name,
                )),
                Err(()) => Err((record, diff_thing.data, Error::DataStoreFailed)),
            }
        } else {
            let uuid = record.thing.uuid;
            self.take_recent(|t| t.uuid == uuid);

            if let Ok(()) = self.save_thing(&record.thing).await {
                Ok((
                    Record {
                        status: RecordStatus::Unsaved,
                        thing: diff_thing,
                    },
                    name,
                ))
            } else {
                // Fail forward when implicit save was unsuccessful so that we can at least edit
                // records in memory when the data store is unavailable.
                self.push_recent(record.thing);
                Ok((
                    Record {
                        status: RecordStatus::Saved,
                        thing: diff_thing,
                    },
                    name,
                ))
            }
        }
    }

    /// Creates a new Thing from its data, generating a UUID if necessary and checking for
    /// name/UUID conflicts.
    async fn thing_data_into_thing(
        &self,
        thing_data: ThingData,
        uuid: Option<Uuid>,
    ) -> Result<Thing, (ThingData, Error)> {
        let uuid = uuid.unwrap_or_else(Uuid::new_v4);

        if let Ok(record) = self.get_by_uuid(&uuid).await {
            Err((thing_data, Error::UuidAlreadyExists(record.thing)))
        } else if let Some(name) = thing_data.name().value() {
            if let Ok(record) = self.get_by_name(name).await {
                Err((thing_data, Error::NameAlreadyExists(record.thing)))
            } else {
                Ok(Thing {
                    uuid,
                    data: thing_data,
                })
            }
        } else {
            Err((thing_data, Error::MissingName))
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
    /// Describe how applying this change from the undo queue will affect the application state
    /// ("undo change xyz").
    pub fn display_undo(&self) -> DisplayUndo {
        DisplayUndo(self)
    }

    /// Describe how applyifrom the redo queue will affect the application state ("redo change
    /// xyz").
    pub fn display_redo(&self) -> DisplayRedo {
        DisplayRedo(self)
    }
}

impl Record {
    pub fn is_saved(&self) -> bool {
        self.status == RecordStatus::Saved
    }

    pub fn is_unsaved(&self) -> bool {
        self.status == RecordStatus::Unsaved
    }

    pub fn is_deleted(&self) -> bool {
        self.status == RecordStatus::Deleted
    }
}

impl<'a> fmt::Display for DisplayUndo<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let change = self.0;

        // Note: these descriptions are _backward_ since they describe the reverse, ie. the action
        // that this Change will undo. Eg. Change::Create => "undo deleting x"
        match change {
            Change::Create { thing_data, .. } | Change::CreateAndSave { thing_data, .. } => {
                write!(f, "deleting {}", thing_data.name())
            }
            Change::Delete { name, .. } => write!(f, "creating {}", name),
            Change::Save { name, .. } => write!(f, "removing {} from journal", name),
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
            Change::Create { thing_data, .. } | Change::CreateAndSave { thing_data, .. } => {
                write!(f, "creating {}", thing_data.name())
            }
            Change::Delete { name, .. } => write!(f, "deleting {}", name),
            Change::Edit { name, .. } | Change::EditAndUnsave { name, .. } => {
                write!(f, "editing {}", name)
            }
            Change::Save { name, .. } => write!(f, "saving {} to journal", name),
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
    use crate::world::npc::Npc;
    use crate::world::place::Place;
    use async_trait::async_trait;
    use std::cell::RefCell;
    use std::rc::Rc;
    use tokio_test::block_on;
    use uuid::Uuid;

    const OLYMPUS_UUID: Uuid = Uuid::from_u128(1);
    const THESSALY_UUID: Uuid = Uuid::from_u128(2);
    const GREECE_UUID: Uuid = Uuid::from_u128(3);
    const STYX_UUID: Uuid = Uuid::from_u128(4);
    const ODYSSEUS_UUID: Uuid = Uuid::from_u128(5);

    macro_rules! assert_change_success {
        ($change: expr, $is_changed:expr, $redo_message:expr, $undo_message:expr) => {
            let change: Change = $change;
            let is_changed: &dyn Fn(&Repository, &dyn DataStore) -> bool = &$is_changed;
            let undo_message: &str = $undo_message;
            let redo_message: &str = $redo_message;

            let (mut repo, data_store) = repo_data_store();
            assert_eq!(redo_message, change.display_redo().to_string(), "change.display_redo()");

            let (original_recent, original_data_store) = (repo.recent.clone(), data_store.snapshot());

            let (modified_recent, modified_data_store) = {
                // repo.modify()
                block_on(repo.modify(change)).unwrap();
                assert!(
                    is_changed(&repo, &data_store),
                    "`is_changed()` should return true after `repo.modify()`

repo.recent = {:?}

data_store.snapshot() = {:?}",
                    repo.recent,
                    data_store.snapshot(),
                );
                assert!(
                    original_recent != repo.recent || original_data_store != data_store.snapshot(),
                    "`repo.recent` AND/OR `data_store` should have changed after `repo.modify()`

repo.recent = {:?}

data_store.snapshot() = {:?}",
                    repo.recent,
                    data_store.snapshot(),
                );

                assert_eq!(
                    undo_message,
                    repo.undo_history()
                        .next()
                        .unwrap()
                        .display_undo()
                        .to_string(),
                    "`undo_history().display_undo()`",
                );

                (repo.recent.clone(), data_store.snapshot())
            };

            {
                let undo_change = repo.undo_history().next().cloned();

                // repo.undo()
                block_on(repo.undo()).unwrap().unwrap();
                assert!(
                    !is_changed(&repo, &data_store),
                    "is_changed() should return false after repo.undo()

change = {:?}

repo.recent = {:?}

data_store.snapshot() = {:?}",
                    undo_change,
                    repo.recent,
                    data_store.snapshot(),
                );
                assert_eq!(
                    original_recent,
                    repo.recent,
                    "`repo.recent` should reset after `repo.undo()`\n\nchange = {:?}",
                    undo_change,
                );
                assert_eq!(
                    original_data_store,
                    data_store.snapshot(),
                    "`data_store` should reset after `repo.undo()`\n\nchange = {:?}",
                    undo_change,
                );
            }

            {
                // repo.redo()
                block_on(repo.redo());
                assert!(
                    is_changed(&repo, &data_store),
                    "is_changed() should return true after repo.redo()

repo.recent = {:?}

data_store.snapshot() = {:?}",
                    repo.recent,
                    data_store.snapshot(),
                );
                assert_eq!(
                    modified_recent,
                    repo.recent,
                    "`repo.recent` should return to its changed state after `repo.redo()`",
                );
                assert_eq!(
                    modified_data_store,
                    data_store.snapshot(),
                    "`data_store` should return to its changed state after `repo.redo()`",
                );
            }
        }
    }

    macro_rules! assert_change_error {
        ($repo_data_store: expr, $change:expr, $error:expr) => {
            let (mut repo, data_store): (Repository, MemoryDataStore) = $repo_data_store;
            let change: Change = $change;
            let error: Error = $error;

            let (original_recent, original_data_store) =
                (repo.recent.clone(), data_store.snapshot());

            let result = block_on(repo.modify(change.clone()));

            assert_eq!(Err((change, error)), result);
            assert_eq!(original_recent, repo.recent);
            assert_eq!(original_data_store, data_store.snapshot());
        };
    }

    macro_rules! assert_change_data_store_failed {
        ($change:expr) => {
            let change: Change = $change;

            let mut repo = null_repo();
            let original_recent = repo.recent.clone();

            let result = block_on(repo.modify(change.clone()));

            assert_eq!(Err((change, Error::DataStoreFailed)), result);
            assert_eq!(original_recent, repo.recent);
        };
    }

    #[test]
    fn recent_test() {
        let mut repository = empty_repo();

        (0..RECENT_MAX_LEN).for_each(|i| {
            repository.push_recent(thing(
                Uuid::from_u128(i.try_into().unwrap()),
                NpcData {
                    name: format!("Thing {}", i).into(),
                    ..Default::default()
                },
            ));
            assert_eq!(i + 1, repository.recent.len());
        });

        assert_eq!(
            Some(&"Thing 0".to_string()),
            repository
                .recent()
                .next()
                .and_then(|thing| thing.name().value()),
        );

        repository.push_recent(thing(
            Uuid::from_u128(u128::MAX),
            NpcData {
                name: "The Cat in the Hat".into(),
                ..Default::default()
            },
        ));
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
        let result = block_on(repo().get_by_name("ODYSSEUS")).unwrap();

        assert_eq!(RecordStatus::Unsaved, result.status);
        assert_eq!("Odysseus", result.thing.name().to_string());
    }

    #[test]
    fn get_by_name_test_from_journal() {
        let result = block_on(repo().get_by_name("OLYMPUS")).unwrap();

        assert_eq!(RecordStatus::Saved, result.status);
        assert_eq!("Olympus", result.thing.name().to_string());
    }

    #[test]
    fn get_by_name_test_not_found() {
        assert_eq!(Err(Error::NotFound), block_on(repo().get_by_name("NOBODY")));
    }

    #[test]
    fn get_by_uuid_test_from_recent() {
        let result = block_on(repo().get_by_uuid(&ODYSSEUS_UUID)).unwrap();

        assert_eq!(RecordStatus::Unsaved, result.status);
        assert_eq!("Odysseus", result.thing.name().to_string());
    }

    #[test]
    fn get_by_uuid_test_from_journal() {
        let result = block_on(repo().get_by_uuid(&OLYMPUS_UUID)).unwrap();

        assert_eq!(RecordStatus::Saved, result.status);
        assert_eq!("Olympus", result.thing.name().to_string());
    }

    #[test]
    fn change_test_delete_from_journal_success() {
        assert_change_success!(
            Change::Delete {
                uuid: OLYMPUS_UUID,
                name: "blah".to_string(),
            },
            |repo, _| block_on(repo.get_by_name("Olympus")) == Err(Error::NotFound),
            "deleting blah",
            "deleting Olympus"
        );
    }

    #[test]
    fn change_test_delete_from_recent_success() {
        assert_change_success!(
            Change::Delete {
                uuid: ODYSSEUS_UUID,
                name: "blah".to_string(),
            },
            |repo, _| block_on(repo.get_by_name("Odysseus")) == Err(Error::NotFound),
            "deleting blah",
            "deleting Odysseus"
        );
    }

    #[test]
    fn change_test_delete_not_found() {
        assert_change_error!(
            repo_data_store(),
            Change::Delete {
                uuid: Uuid::nil(),
                name: "Nobody".to_string(),
            },
            Error::NotFound
        );
    }

    #[test]
    fn change_test_delete_data_store_failed() {
        assert_change_data_store_failed!(Change::Delete {
            uuid: OLYMPUS_UUID,
            name: "Olympus".to_string(),
        });
    }

    #[test]
    fn change_test_edit_by_name_from_recent_success() {
        assert_change_success!(
            Change::Edit {
                name: "ODYSSEUS".into(),
                uuid: None,
                diff: NpcData {
                    name: "Nobody".into(),
                    ..Default::default()
                }
                .into(),
            },
            |_, ds| {
                block_on(ds.get_thing_by_uuid(&ODYSSEUS_UUID))
                    .map(|opt_t| opt_t.map(|t| t.name().to_string()))
                    == Ok(Some("Nobody".to_string()))
            },
            "editing ODYSSEUS",
            "editing Nobody"
        );
    }

    #[test]
    fn change_test_edit_by_name_from_recent_wrong_type() {
        assert_change_error!(
            repo_data_store(),
            Change::Edit {
                name: "Odysseus".into(),
                uuid: None,
                diff: PlaceData::default().into(),
            },
            Error::NotFound
        );
    }

    #[test]
    fn change_test_edit_by_name_from_recent_data_store_failed() {
        let mut repo = repo();
        repo.data_store = Box::new(NullDataStore);
        let change = Change::Edit {
            name: "Odysseus".into(),
            uuid: None,
            diff: NpcData {
                name: "Nobody".into(),
                ..Default::default()
            }
            .into(),
        };

        {
            let result = block_on(repo.modify(change));

            assert_eq!(
                Ok(Some("Nobody".to_string())),
                result.map(|opt_r| opt_r.map(|r| r.thing.name().to_string())),
            );
            assert!(repo.recent().any(|t| t.name().to_string() == "Nobody"));
        }

        {
            let undo_change = repo.undo_history().next().cloned();
            let undo_result = block_on(repo.undo());

            assert_eq!(
                Ok(Some("Odysseus".to_string())),
                undo_result
                    .unwrap()
                    .map(|opt_r| opt_r.map(|r| r.thing.name().to_string())),
                "{:?}",
                undo_change,
            );
            assert!(repo.recent().any(|t| t.name().to_string() == "Odysseus"));
        }

        {
            let redo_result = block_on(repo.redo());

            assert_eq!(
                Ok(Some("Nobody".to_string())),
                redo_result
                    .unwrap()
                    .map(|opt_r| opt_r.map(|r| r.thing.name().to_string())),
            );
            assert!(repo.recent().any(|t| t.name().to_string() == "Nobody"));
        }
    }

    #[test]
    fn change_test_edit_by_name_from_journal_success() {
        assert_change_success!(
            Change::Edit {
                name: "OLYMPUS".into(),
                uuid: None,
                diff: PlaceData {
                    name: "Hades".into(),
                    description: "This really is hell!".into(),
                    ..Default::default()
                }
                .into(),
            },
            |_, ds| {
                block_on(ds.get_thing_by_uuid(&OLYMPUS_UUID))
                    .map(|opt_t| opt_t.map(|t| t.name().to_string()))
                    == Ok(Some("Hades".to_string()))
            },
            "editing OLYMPUS",
            "editing Hades"
        );
    }

    #[test]
    fn change_test_edit_by_name_from_journal_wrong_type() {
        assert_change_error!(
            repo_data_store(),
            Change::Edit {
                name: "Olympus".into(),
                uuid: None,
                diff: NpcData::default().into(),
            },
            Error::NotFound
        );
    }

    #[test]
    fn change_test_edit_by_name_from_journal_data_store_failed() {
        assert_change_data_store_failed!(Change::Edit {
            name: "Olympus".into(),
            uuid: None,
            diff: PlaceData {
                name: "Hades".into(),
                ..Default::default()
            }
            .into(),
        });
    }

    #[test]
    fn change_test_edit_by_name_not_found() {
        assert_change_error!(
            repo_data_store(),
            Change::Edit {
                name: "Nobody".into(),
                uuid: None,
                diff: NpcData::default().into(),
            },
            Error::NotFound
        );
    }

    #[test]
    fn change_test_edit_by_uuid_from_recent_success() {
        assert_change_success!(
            Change::Edit {
                name: "blah".into(),
                uuid: Some(ODYSSEUS_UUID),
                diff: NpcData {
                    name: "Nobody".into(),
                    ..Default::default()
                }
                .into(),
            },
            |repo, ds| {
                block_on(ds.get_thing_by_uuid(&ODYSSEUS_UUID))
                    .map(|opt_t| opt_t.map(|t| t.name().to_string()))
                    == Ok(Some("Nobody".to_string()))
                    && !repo.recent().any(|t| t.uuid == ODYSSEUS_UUID)
            },
            "editing blah",
            "editing Nobody"
        );
    }

    #[test]
    fn change_test_edit_by_uuid_from_journal_success() {
        assert_change_success!(
            Change::Edit {
                name: "blah".into(),
                uuid: Some(OLYMPUS_UUID),
                diff: PlaceData {
                    name: "Hades".into(),
                    description: "This really is hell!".into(),
                    ..Default::default()
                }
                .into(),
            },
            |_, ds| {
                block_on(ds.get_thing_by_uuid(&OLYMPUS_UUID))
                    .map(|opt_t| opt_t.map(|t| t.name().to_string()))
                    == Ok(Some("Hades".to_string()))
            },
            "editing blah",
            "editing Hades"
        );
    }

    #[test]
    fn change_test_edit_by_uuid_wrong_type() {
        assert_change_error!(
            repo_data_store(),
            Change::Edit {
                name: "Olympus".into(),
                uuid: Some(OLYMPUS_UUID),
                diff: NpcData::default().into(),
            },
            Error::NotFound
        );
    }

    #[test]
    fn change_test_edit_by_uuid_not_found() {
        assert_change_error!(
            repo_data_store(),
            Change::Edit {
                name: "Nobody".into(),
                uuid: Some(Uuid::nil()),
                diff: NpcData::default().into(),
            },
            Error::NotFound
        );
    }

    #[test]
    fn change_test_edit_by_uuid_from_journal_data_store_failed() {
        assert_change_data_store_failed!(Change::Edit {
            name: "Olympus".into(),
            uuid: Some(OLYMPUS_UUID),
            diff: PlaceData {
                name: "Hades".into(),
                description: "This really is hell!".into(),
                ..Default::default()
            }
            .into(),
        });
    }

    #[test]
    fn change_test_edit_and_unsave_success() {
        assert_change_success!(
            Change::EditAndUnsave {
                uuid: OLYMPUS_UUID,
                name: "blah".into(),
                diff: PlaceData {
                    name: "Hades".into(),
                    description: "This really is hell!".into(),
                    ..Default::default()
                }
                .into(),
            },
            |repo, ds| {
                repo.recent().any(|t| t.name().to_string() == "Hades")
                    && block_on(ds.get_thing_by_uuid(&OLYMPUS_UUID)) == Ok(None)
            },
            "editing blah",
            "editing Hades"
        );
    }

    #[test]
    fn change_test_edit_and_unsave_not_found() {
        assert_change_error!(
            repo_data_store(),
            Change::EditAndUnsave {
                name: "Nobody".into(),
                uuid: Uuid::nil(),
                diff: NpcData::default().into(),
            },
            Error::NotFound
        );
    }

    #[test]
    fn change_test_edit_and_unsave_data_store_failed() {
        let mut repo = Repository::new(TimeBombDataStore::new(7));
        populate_repo(&mut repo);

        let change = Change::EditAndUnsave {
            name: "Olympus".into(),
            uuid: OLYMPUS_UUID,
            diff: PlaceData {
                name: "Hades".into(),
                description: "This really is hell!".into(),
                ..Default::default()
            }
            .into(),
        };

        assert_eq!(
            Err((
                Change::Unsave {
                    name: "Hades".into(),
                    uuid: OLYMPUS_UUID,
                },
                Error::DataStoreFailed,
            )),
            block_on(repo.modify(change)),
        );
    }

    #[test]
    fn change_test_create_success() {
        assert_change_success!(
            Change::Create {
                thing_data: NpcData {
                    name: "Penelope".into(),
                    ..Default::default()
                }
                .into(),
                uuid: None,
            },
            |repo, _| repo.recent().any(|t| t.name().to_string() == "Penelope"),
            "creating Penelope",
            "creating Penelope"
        );
    }

    #[test]
    fn change_test_create_name_already_exists_in_journal() {
        let (repo, data_store) = repo_data_store();
        let existing_thing = block_on(data_store.get_thing_by_uuid(&OLYMPUS_UUID))
            .unwrap()
            .unwrap()
            .clone();

        assert_change_error!(
            (repo, data_store),
            Change::Create {
                thing_data: NpcData {
                    name: "OLYMPUS".into(),
                    ..Default::default()
                }
                .into(),
                uuid: None,
            },
            Error::NameAlreadyExists(existing_thing)
        );
    }

    #[test]
    fn change_test_create_name_already_exists_in_recent() {
        let (repo, data_store) = repo_data_store();
        let existing_thing = repo
            .recent()
            .find(|t| t.uuid == ODYSSEUS_UUID)
            .unwrap()
            .clone();

        assert_change_error!(
            (repo, data_store),
            Change::Create {
                thing_data: NpcData {
                    name: "ODYSSEUS".into(),
                    ..Default::default()
                }
                .into(),
                uuid: None,
            },
            Error::NameAlreadyExists(existing_thing)
        );
    }

    #[test]
    fn change_test_save_by_name_success() {
        assert_change_success!(
            Change::Save {
                name: "ODYSSEUS".to_string(),
                uuid: None,
            },
            |repo, ds| {
                block_on(ds.get_thing_by_uuid(&ODYSSEUS_UUID))
                    .map(|opt_t| opt_t.map(|t| t.name().to_string()))
                    == Ok(Some("Odysseus".to_string()))
                    && !repo.recent().any(|t| t.uuid == ODYSSEUS_UUID)
            },
            "saving ODYSSEUS to journal",
            "saving Odysseus to journal"
        );
    }

    #[test]
    fn change_test_save_data_store_failed() {
        let mut repo = null_repo();

        block_on(
            repo.modify(Change::Create {
                thing_data: PlaceData {
                    name: "Odysseus".into(),
                    ..Default::default()
                }
                .into(),
                uuid: None,
            }),
        )
        .unwrap();

        let original_recent = repo.recent.clone();

        let change = Change::Save {
            name: "ODYSSEUS".to_string(),
            uuid: None,
        };
        assert_eq!(
            block_on(repo.modify(change.clone())),
            Err((change, Error::DataStoreFailed)),
        );

        assert_eq!(original_recent, repo.recent);
    }

    #[test]
    fn change_test_save_already_saved() {
        assert_change_error!(
            repo_data_store(),
            Change::Save {
                name: "OLYMPUS".to_string(),
                uuid: None,
            },
            Error::NotFound
        );
    }

    #[test]
    fn change_test_save_not_found() {
        assert_change_error!(
            repo_data_store(),
            Change::Save {
                name: "NOBODY".to_string(),
                uuid: None,
            },
            Error::NotFound
        );
    }

    #[test]
    fn change_test_unsave_success() {
        assert_change_success!(
            Change::Unsave {
                uuid: OLYMPUS_UUID,
                name: "blah".to_string(),
            },
            |repo, ds| {
                block_on(ds.get_thing_by_uuid(&OLYMPUS_UUID)) == Ok(None)
                    && repo.recent().any(|t| t.uuid == OLYMPUS_UUID)
            },
            "removing blah from journal",
            "removing Olympus from journal"
        );
    }

    #[test]
    fn change_test_create_and_save_success() {
        assert_change_success!(
            Change::CreateAndSave {
                thing_data: NpcData {
                    name: "Penelope".into(),
                    ..Default::default()
                }
                .into(),
                uuid: None,
            },
            |_, ds| block_on(ds.get_thing_by_name("Penelope"))
                .unwrap()
                .is_some(),
            "creating Penelope",
            "creating Penelope"
        );
    }

    #[test]
    fn change_test_create_and_save_name_already_exists_in_journal() {
        let (repo, data_store) = repo_data_store();
        let existing_thing = block_on(data_store.get_thing_by_uuid(&OLYMPUS_UUID))
            .unwrap()
            .unwrap()
            .clone();

        assert_change_error!(
            (repo, data_store),
            Change::CreateAndSave {
                thing_data: NpcData {
                    name: "OLYMPUS".into(),
                    ..Default::default()
                }
                .into(),
                uuid: None,
            },
            Error::NameAlreadyExists(existing_thing)
        );
    }

    #[test]
    fn change_test_create_and_save_name_already_exists_in_recent() {
        let (repo, data_store) = repo_data_store();
        let existing_thing = repo
            .recent()
            .find(|t| t.uuid == ODYSSEUS_UUID)
            .unwrap()
            .clone();

        assert_change_error!(
            (repo, data_store),
            Change::CreateAndSave {
                thing_data: NpcData {
                    name: "ODYSSEUS".into(),
                    ..Default::default()
                }
                .into(),
                uuid: None,
            },
            Error::NameAlreadyExists(existing_thing)
        );
    }

    #[test]
    fn change_test_create_and_save_data_store_failed() {
        let mut repo = null_repo();

        let change = Change::CreateAndSave {
            thing_data: NpcData {
                name: "Odysseus".into(),
                ..Default::default()
            }
            .into(),
            uuid: None,
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
        let odysseus = block_on(repo.get_by_name("Odysseus")).unwrap().thing;

        match block_on(repo.load_relations(&odysseus)) {
            Ok(ThingRelations::Npc(NpcRelations {
                location: Some((parent, None)),
            })) => {
                assert_eq!("River Styx", parent.data.name.value().unwrap());
            }
            r => panic!("{:?}", r),
        }
    }

    #[test]
    fn load_relations_test_with_grandparent_success() {
        let repo = repo();
        let olympus = block_on(repo.get_by_uuid(&OLYMPUS_UUID)).unwrap().thing;

        match block_on(repo.load_relations(&olympus)) {
            Ok(ThingRelations::Place(PlaceRelations {
                location: Some((parent, Some(grandparent))),
            })) => {
                assert_eq!("Thessaly", parent.data.name.value().unwrap());
                assert_eq!("Greece", grandparent.data.name.value().unwrap());
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
        assert!(repo.data_store_enabled());
    }

    #[test]
    fn data_store_enabled_test_failure() {
        let mut repo = null_repo();
        block_on(repo.init());
        assert!(!repo.data_store_enabled());
    }

    fn thing(uuid: Uuid, data: impl Into<ThingData>) -> Thing {
        Thing {
            uuid,
            data: data.into(),
        }
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

    fn null_repo() -> Repository {
        Repository::new(NullDataStore)
    }

    fn populate_repo(repo: &mut Repository) {
        block_on(
            repo.data_store.save_thing(
                &Place {
                    uuid: OLYMPUS_UUID,
                    data: PlaceData {
                        location_uuid: THESSALY_UUID.into(),
                        name: "Olympus".into(),
                        ..Default::default()
                    },
                }
                .into(),
            ),
        )
        .unwrap();
        block_on(
            repo.data_store.save_thing(
                &Place {
                    uuid: THESSALY_UUID,
                    data: PlaceData {
                        location_uuid: GREECE_UUID.into(),
                        name: "Thessaly".into(),
                        ..Default::default()
                    },
                }
                .into(),
            ),
        )
        .unwrap();
        block_on(
            repo.data_store.save_thing(
                &Place {
                    uuid: GREECE_UUID,
                    data: PlaceData {
                        name: "Greece".into(),
                        ..Default::default()
                    },
                }
                .into(),
            ),
        )
        .unwrap();
        block_on(
            repo.data_store.save_thing(
                &Place {
                    uuid: STYX_UUID,
                    data: PlaceData {
                        location_uuid: Uuid::nil().into(),
                        name: "River Styx".into(),
                        ..Default::default()
                    },
                }
                .into(),
            ),
        )
        .unwrap();

        repo.recent.push_back(
            Npc {
                uuid: ODYSSEUS_UUID,
                data: NpcData {
                    name: "Odysseus".into(),
                    location_uuid: STYX_UUID.into(),
                    ..Default::default()
                },
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
