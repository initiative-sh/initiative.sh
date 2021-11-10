use crate::storage::{DataStore, MemoryDataStore};
use crate::time::Time;
use crate::{Thing, Uuid};
use std::collections::{HashMap, VecDeque};
use std::fmt;

const RECENT_MAX_LEN: usize = 100;
const UNDO_HISTORY_LEN: usize = 10;

pub struct Repository {
    cache: HashMap<Uuid, Thing>,
    data_store: Box<dyn DataStore>,
    data_store_enabled: bool,
    recent: VecDeque<Thing>,
    redo_change: Option<Change>,
    time: Time,
    undo_history: VecDeque<Change>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Change {
    /// Create a new thing and store it in recent entries.
    ///
    /// Reverse: Delete { id: Id::Name }
    Create { thing: Thing },

    /// Create a new thing and store it in the journal.
    ///
    /// Reverse: Delete { id: Id::Uuid }
    CreateAndSave { thing: Thing },

    /// Delete a thing from recent or journal.
    ///
    /// Reverse: Create (recent) or CreateAndSave (journal)
    Delete { name: String, id: Id },

    /// Edit fields on a Thing.
    ///
    /// Reverse: Edit (already in journal) or EditAndUnsave (in recent)
    Edit { name: String, id: Id, diff: Thing },

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
}

pub struct DisplayUndo<'a>(&'a Change);

pub struct DisplayRedo<'a>(&'a Change);

#[derive(Clone, Debug, PartialEq)]
pub enum Id {
    Name(String),
    Uuid(Uuid),
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DataStoreFailed,
    MissingName,
    NameAlreadyExists,
    NotFound,
}

impl Repository {
    pub fn new(data_store: impl DataStore + 'static) -> Self {
        Self {
            cache: HashMap::default(),
            data_store: Box::new(data_store),
            data_store_enabled: false,
            recent: VecDeque::default(),
            redo_change: None,
            time: Time::try_new(1, 8, 0, 0).unwrap(),
            undo_history: VecDeque::default(),
        }
    }

    pub async fn init(&mut self) {
        let things = self.data_store.get_all_the_things().await;

        if let Ok(mut things) = things {
            self.cache = things
                .drain(..)
                .filter_map(|thing| {
                    if let Some(&uuid) = thing.uuid() {
                        Some((uuid, thing))
                    } else {
                        None
                    }
                })
                .collect();
            self.data_store_enabled = true;
        } else {
            self.data_store = Box::new(MemoryDataStore::default());
        }

        if let Ok(Some(time_str)) = self.data_store.get_value("time").await {
            if let Ok(time) = time_str.parse() {
                self.set_time(time).await;
            }
        }
    }

    pub async fn load(&self, id: &Id) -> Result<Thing, Error> {
        match id {
            Id::Name(name) => self
                .load_thing_by_name(name)
                .cloned()
                .ok_or(Error::NotFound),
            Id::Uuid(uuid) => self.load_thing_by_uuid(uuid).await,
        }
    }

    pub fn all(&self) -> impl Iterator<Item = &Thing> {
        self.journal().chain(self.recent())
    }

    pub fn recent(&self) -> impl Iterator<Item = &Thing> {
        self.recent.as_slices().0.iter()
    }

    pub fn journal(&self) -> impl Iterator<Item = &Thing> {
        self.cache.values()
    }

    pub async fn modify(&mut self, change: Change) -> Result<Id, (Change, Error)> {
        self.modify_without_undo(change).await.map(|change| {
            let id = change.id();
            while self.undo_history.len() >= UNDO_HISTORY_LEN {
                self.undo_history.pop_front();
            }
            self.undo_history.push_back(change);
            id
        })
    }

    pub async fn undo(&mut self) -> Option<Result<Id, Error>> {
        if let Some(change) = self.undo_history.pop_back() {
            Some(
                self.modify_without_undo(change)
                    .await
                    .map(|change| {
                        let id = change.id();
                        self.redo_change = Some(change);
                        id
                    })
                    .map_err(|(change, e)| {
                        self.undo_history.push_back(change);
                        e
                    }),
            )
        } else {
            None
        }
    }

    pub fn undo_history(&self) -> impl Iterator<Item = &Change> {
        self.undo_history.iter().rev()
    }

    pub async fn redo(&mut self) -> Option<Result<Id, Error>> {
        if let Some(change) = self.redo_change.take() {
            Some(self.modify(change).await.map_err(|(change, e)| {
                self.redo_change = Some(change);
                e
            }))
        } else {
            None
        }
    }

    pub fn get_redo(&self) -> Option<&Change> {
        self.redo_change.as_ref()
    }

    async fn modify_without_undo(&mut self, change: Change) -> Result<Change, (Change, Error)> {
        match change {
            Change::Create { thing } => self
                .create_thing(thing)
                .map(|name| Change::Delete {
                    id: name.as_str().into(),
                    name: name.to_owned(),
                })
                .map_err(|(thing, e)| (Change::Create { thing }, e)),
            Change::CreateAndSave { thing } => {
                let name = thing.name().to_string();
                self.create_and_save_thing(thing)
                    .await
                    .map(|uuid| Change::Delete {
                        id: uuid.into(),
                        name,
                    })
                    .map_err(|(thing, e)| (Change::CreateAndSave { thing }, e))
            }
            Change::Delete { id, name } => match id {
                Id::Name(id_name) => self
                    .delete_thing_by_name(&id_name)
                    .await
                    .map(|thing| {
                        if thing.uuid().is_some() {
                            Change::CreateAndSave { thing }
                        } else {
                            Change::Create { thing }
                        }
                    })
                    .map_err(|e| {
                        (
                            Change::Delete {
                                id: Id::Name(id_name),
                                name,
                            },
                            e,
                        )
                    }),
                Id::Uuid(uuid) => self
                    .delete_thing_by_uuid(&uuid)
                    .await
                    .map(|thing| Change::CreateAndSave { thing })
                    .map_err(|(_, e)| {
                        (
                            Change::Delete {
                                id: Id::Uuid(uuid),
                                name,
                            },
                            e,
                        )
                    }),
            },
            Change::Edit { name, id, diff } => match id {
                Id::Name(id_name) => {
                    let new_name = diff.name().value().map(|s| s.to_string());

                    self.edit_thing_by_name(&id_name, diff)
                        .await
                        .map_err(|(diff, e)| {
                            if let Some(new_name) = new_name {
                                (
                                    Change::Edit {
                                        id: new_name.as_str().into(),
                                        name: new_name,
                                        diff,
                                    },
                                    e,
                                )
                            } else {
                                (
                                    Change::Edit {
                                        id: Id::Name(id_name),
                                        name,
                                        diff,
                                    },
                                    e,
                                )
                            }
                        })
                }
                Id::Uuid(uuid) => match self.edit_thing_by_uuid(&uuid, diff).await {
                    Ok(diff) => Ok(Change::Edit {
                        name: self
                            .load(&Id::Uuid(uuid))
                            .await
                            .map(|thing| thing.name().value().map(String::from))
                            .unwrap_or(None)
                            .unwrap_or(name),
                        id: Id::Uuid(uuid),
                        diff,
                    }),
                    Err((diff, e)) => Err((
                        Change::Edit {
                            name: self
                                .load(&Id::Uuid(uuid))
                                .await
                                .map(|thing| thing.name().value().map(String::from))
                                .unwrap_or(None)
                                .unwrap_or(name),
                            id: Id::Uuid(uuid),
                            diff,
                        },
                        e,
                    )),
                },
            },
            Change::EditAndUnsave { name, uuid, diff } => {
                match self.edit_thing_by_uuid(&uuid, diff).await {
                    Ok(diff) => self
                        .unsave_thing_by_uuid(&uuid)
                        .await
                        .map(|name| Change::Edit {
                            id: name.as_str().into(),
                            name,
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
            Change::Save { name } => match self.save_thing_by_name(&name.to_lowercase()).await {
                Ok(uuid) => Ok(Change::Unsave {
                    uuid,
                    name: self
                        .load_thing_by_uuid(&uuid)
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
        }
    }

    pub async fn set_time(&mut self, time: Time) {
        self.data_store
            .set_value("time", &time.display_short().to_string())
            .await
            .ok();
        self.time = time;
    }

    pub fn get_time(&self) -> &Time {
        &self.time
    }

    pub fn data_store_enabled(&self) -> bool {
        self.data_store_enabled
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

    fn create_thing(&mut self, thing: Thing) -> Result<String, (Thing, Error)> {
        if let Some(name) = thing.name().value() {
            if self.load_thing_by_name(&name.to_lowercase()).is_some() {
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
            if self.load_thing_by_name(&name.to_lowercase()).is_some() {
                Err((thing, Error::NameAlreadyExists))
            } else {
                self.save_thing(thing).await
            }
        } else {
            Err((thing, Error::MissingName))
        }
    }

    async fn delete_thing_by_name(&mut self, name: &str) -> Result<Thing, Error> {
        let name_matches = |s: &String| s.to_lowercase() == name;

        let cached_uuid = if let Some((uuid, _)) = self
            .cache
            .iter()
            .find(|(_, t)| t.name().value().map_or(false, name_matches))
        {
            Some(*uuid)
        } else {
            None
        };

        if let Some(uuid) = cached_uuid {
            self.delete_thing_by_uuid(&uuid).await.map_err(|(_, e)| e)
        } else if let Some(thing) =
            self.take_recent(|t| t.name().value().map_or(false, name_matches))
        {
            Ok(thing)
        } else {
            Err(Error::NotFound)
        }
    }

    async fn delete_thing_by_uuid(&mut self, uuid: &Uuid) -> Result<Thing, (Option<Thing>, Error)> {
        match (
            self.cache.remove(uuid),
            self.data_store.delete_thing_by_uuid(uuid).await,
        ) {
            (Some(thing), Ok(())) => Ok(thing),
            (Some(thing), Err(())) => Err((Some(thing), Error::DataStoreFailed)),
            (None, _) => Err((None, Error::NotFound)),
        }
    }

    fn load_thing_by_name<'a>(&'a self, name: &str) -> Option<&'a Thing> {
        self.all()
            .find(|t| t.name().value().map_or(false, |s| s.to_lowercase() == name))
    }

    async fn load_thing_by_uuid(&self, uuid: &Uuid) -> Result<Thing, Error> {
        match self.data_store.get_thing_by_uuid(uuid).await {
            Ok(Some(thing)) => Ok(thing),
            Ok(None) => Err(Error::NotFound),
            Err(()) => Err(Error::DataStoreFailed),
        }
    }

    async fn save_thing_by_name(&mut self, name: &str) -> Result<Uuid, Error> {
        if let Some(thing) =
            self.take_recent(|t| t.name().value().map_or(false, |s| s.to_lowercase() == name))
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
            Ok(()) => {
                self.cache.insert(uuid, thing);
                Ok(uuid)
            }
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

        match (self.create_thing(thing), error) {
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
        if let Some(thing) = self.cache.get_mut(uuid) {
            if thing.try_apply_diff(&mut diff).is_err() {
                return Err((diff, Error::NotFound));
            }

            match self.data_store.edit_thing(thing).await {
                Ok(()) => Ok(diff),
                Err(()) => Err((diff, Error::DataStoreFailed)),
            }
        } else {
            Err((diff, Error::NotFound))
        }
    }

    async fn edit_thing_by_name(
        &mut self,
        name: &str,
        mut diff: Thing,
    ) -> Result<Change, (Thing, Error)> {
        if let Some(thing) = self.cache.values_mut().find(|thing| {
            thing
                .name()
                .value()
                .map_or(false, |s| s.to_lowercase() == name)
        }) {
            if thing.try_apply_diff(&mut diff).is_err() {
                return Err((diff, Error::NotFound));
            }

            match self.data_store.edit_thing(thing).await {
                Ok(()) => Ok(Change::Edit {
                    name: thing.name().to_string(),
                    id: thing.uuid().unwrap().to_owned().into(),
                    diff,
                }),
                Err(()) => Err((diff, Error::DataStoreFailed)),
            }
        } else if let Some(mut thing) = self.take_recent(|thing| {
            thing
                .name()
                .value()
                .map_or(false, |s| s.to_lowercase() == name)
                && thing.as_str() == diff.as_str()
        }) {
            thing.try_apply_diff(&mut diff).unwrap();

            let name = thing.name().to_string();
            let uuid = match self.save_thing(thing).await {
                Ok(uuid) => uuid,
                Err((thing, Error::DataStoreFailed)) => {
                    self.push_recent(thing);
                    return Ok(Change::Edit {
                        id: name.as_str().into(),
                        name,
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
            Err((diff, Error::NotFound))
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
        }
    }

    pub fn id(&self) -> Id {
        let uuid = match self {
            Change::Create { thing } | Change::CreateAndSave { thing } => thing.uuid(),
            Change::Delete { id, .. } | Change::Edit { id, .. } => return id.to_owned(),
            Change::Unsave { uuid, .. } | Change::EditAndUnsave { uuid, .. } => Some(uuid),
            Change::Save { .. } => None,
        };

        if let Some(uuid) = uuid {
            Id::Uuid(uuid.to_owned())
        } else {
            self.name().as_str().into()
        }
    }
}

impl<'a> fmt::Display for DisplayUndo<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let change = self.0;

        // Note: these descriptions are _backward_ since they describe the reverse, ie. the action
        // that this Change will undo.
        match change {
            Change::Create { thing } => write!(f, "deleting {}", thing.name()),
            Change::CreateAndSave { thing } => write!(f, "deleting {}", thing.name()),
            Change::Delete { name, .. } => write!(f, "creating {}", name),
            Change::Edit { name, .. } | Change::EditAndUnsave { name, .. } => {
                write!(f, "editing {}", name)
            }
            Change::Save { name } => write!(f, "removing {} from journal", name),
            Change::Unsave { name, .. } => write!(f, "saving {} to journal", name),
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
        }
    }
}

impl From<Uuid> for Id {
    fn from(input: Uuid) -> Self {
        Id::Uuid(input)
    }
}

impl From<&String> for Id {
    fn from(input: &String) -> Self {
        input.as_str().into()
    }
}

impl From<&str> for Id {
    fn from(input: &str) -> Self {
        Id::Name(input.to_lowercase())
    }
}

impl fmt::Debug for Repository {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Repository {{ cache: {:?}, data_store_enabled: {:?}, recent: {:?}, time: {:?} }}",
            self.cache, self.data_store_enabled, self.recent, self.time,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::storage::data_store::{MemoryDataStore, NullDataStore};
    use crate::world::npc::{Npc, Species};
    use crate::world::Place;
    use async_trait::async_trait;
    use std::cell::RefCell;
    use std::rc::Rc;
    use tokio_test::block_on;

    const TEST_UUID: Uuid = Uuid::from_u128(u128::MAX);

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
    fn all_journal_test() {
        let repo = repo();
        assert_eq!(1, repo.recent().count());
        assert_eq!(1, repo.journal().count());
        assert_eq!(2, repo.all().count());
    }

    #[test]
    fn load_test_from_recent_by_name() {
        assert_eq!(
            "Odysseus",
            block_on(repo().load(&"ODYSSEUS".into()))
                .map(|thing| thing.name().value().map(String::from))
                .unwrap()
                .unwrap(),
        );
    }

    #[test]
    fn load_test_from_journal_by_name() {
        assert_eq!(
            "Olympus",
            block_on(repo().load(&"OLYMPUS".into()))
                .map(|thing| thing.name().value().map(String::from))
                .unwrap()
                .unwrap(),
        );
    }

    #[test]
    fn load_test_not_found() {
        assert_eq!(
            Err(Error::NotFound),
            block_on(repo().load(&"NOBODY".into())),
        );
    }

    #[test]
    fn load_test_by_uuid() {
        assert_eq!(
            "Olympus",
            block_on(repo().load(&TEST_UUID.into()))
                .map(|thing| thing.name().value().map(String::from))
                .unwrap()
                .unwrap(),
        );
    }

    #[test]
    fn change_test_delete_by_name_from_journal_success() {
        let (mut repo, data_store) = repo_data_store();
        let change = Change::Delete {
            id: "OLYMPUS".into(),
            name: "Olympus".to_string(),
        };
        assert_eq!("deleting Olympus", change.display_redo().to_string());

        {
            assert_eq!(Id::Uuid(TEST_UUID), block_on(repo.modify(change)).unwrap());
            let result = repo.undo_history().next().unwrap();

            assert_eq!(
                &Change::CreateAndSave {
                    thing: Place {
                        uuid: Some(TEST_UUID.into()),
                        name: "Olympus".into(),
                        ..Default::default()
                    }
                    .into()
                },
                result,
            );
            assert_eq!("deleting Olympus", result.display_undo().to_string());
            assert_eq!(0, repo.journal().count());
            assert_eq!(0, block_on(data_store.get_all_the_things()).unwrap().len());
        }

        {
            assert_eq!(Id::Uuid(TEST_UUID), block_on(repo.undo()).unwrap().unwrap());
            assert!(block_on(repo.load(&TEST_UUID.into())).is_ok());
            assert!(block_on(repo.load(&"Olympus".into())).is_ok());
            assert_eq!(1, block_on(data_store.get_all_the_things()).unwrap().len());
        }

        {
            assert_eq!(Id::Uuid(TEST_UUID), block_on(repo.redo()).unwrap().unwrap());
            assert_eq!(Err(Error::NotFound), block_on(repo.load(&"Olympus".into())));
        }
    }

    #[test]
    fn change_test_delete_by_name_from_recent_success() {
        let mut repo = repo();
        let change = Change::Delete {
            id: "ODYSSEUS".into(),
            name: "Odysseus".to_string(),
        };
        assert_eq!("deleting Odysseus", change.display_redo().to_string());

        {
            assert_eq!(Id::from("Odysseus"), block_on(repo.modify(change)).unwrap());
            let result = repo.undo_history().next().unwrap();

            assert_eq!(
                &Change::Create {
                    thing: Npc {
                        name: "Odysseus".into(),
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
            let _undo_result = block_on(repo.undo()).unwrap().unwrap();

            assert_eq!(
                Some(Change::Delete {
                    id: "odysseus".into(),
                    name: "Odysseus".to_string(),
                }),
                repo.redo_change,
            );
            assert!(block_on(repo.load(&"odysseus".into())).is_ok());
            assert_eq!(1, repo.recent().count());
        }
    }

    #[test]
    fn change_test_delete_by_uuid_success() {
        let (mut repo, data_store) = repo_data_store();
        let change = Change::Delete {
            id: TEST_UUID.into(),
            name: "olympus".to_string(),
        };
        assert_eq!("deleting olympus", change.display_redo().to_string());

        {
            assert_eq!(Id::Uuid(TEST_UUID), block_on(repo.modify(change)).unwrap());
            let result = repo.undo_history().next().unwrap();

            assert_eq!(
                &Change::CreateAndSave {
                    thing: Place {
                        uuid: Some(TEST_UUID.into()),
                        name: "Olympus".into(),
                        ..Default::default()
                    }
                    .into()
                },
                result,
            );
            assert_eq!("deleting Olympus", result.display_undo().to_string());
            assert_eq!(0, repo.journal().count());
            assert_eq!(0, block_on(data_store.get_all_the_things()).unwrap().len());
        }

        {
            let _undo_result = block_on(repo.undo()).unwrap().unwrap();

            assert_eq!(
                Some(Change::Delete {
                    id: TEST_UUID.into(),
                    name: "Olympus".to_string(),
                }),
                repo.redo_change,
            );
            assert!(block_on(repo.load(&TEST_UUID.into())).is_ok());
            assert_eq!(1, repo.journal().count());
            assert_eq!(1, block_on(data_store.get_all_the_things()).unwrap().len());
        }
    }

    #[test]
    fn change_test_delete_by_uuid_not_found() {
        let change = Change::Delete {
            id: Uuid::nil().into(),
            name: "Nobody".to_string(),
        };

        let result = block_on(repo().modify(change.clone())).unwrap_err();

        assert_eq!((change, Error::NotFound), result);
    }

    #[test]
    fn change_test_delete_by_uuid_data_store_failed() {
        let (mut repo, mut data_store) = repo_data_store();
        block_on(data_store.delete_thing_by_uuid(&TEST_UUID)).unwrap();
        let change = Change::Delete {
            id: TEST_UUID.into(),
            name: "Olympus".to_string(),
        };

        let result = block_on(repo.modify(change.clone())).unwrap_err();

        assert_eq!((change, Error::DataStoreFailed), result);
    }

    #[test]
    fn change_test_edit_by_name_from_recent_success() {
        let (mut repo, data_store) = repo_data_store();
        let change = Change::Edit {
            id: "ODYSSEUS".into(),
            name: "Odysseus".into(),
            diff: Npc {
                name: "Nobody".into(),
                species: Species::Human.into(),
                ..Default::default()
            }
            .into(),
        };
        assert_eq!("editing Odysseus", change.display_redo().to_string());

        if let Id::Uuid(uuid) = block_on(repo.modify(change)).unwrap() {
            let result = repo.undo_history().next().unwrap();

            assert_eq!(
                &Change::EditAndUnsave {
                    name: "Nobody".into(),
                    uuid,
                    diff: Npc {
                        name: "Odysseus".into(),
                        species: None.into(),
                        ..Default::default()
                    }
                    .into(),
                },
                result,
            );

            assert!(block_on(repo.load(&uuid.to_owned().into())).is_ok());
            assert_eq!("editing Nobody", result.display_undo().to_string());
            assert!(block_on(repo.load(&"Nobody".into())).is_ok());
            assert_eq!(2, repo.journal().count());
            assert_eq!(0, repo.recent().count());
            assert_eq!(2, block_on(data_store.get_all_the_things()).unwrap().len());
            assert!(block_on(data_store.get_all_the_things())
                .unwrap()
                .iter()
                .any(|thing| thing.name().to_string() == "Nobody"));
        } else {
            panic!();
        }

        {
            assert_eq!(
                Id::from("ODYSSEUS"),
                block_on(repo.undo()).unwrap().unwrap(),
            );
            assert!(block_on(repo.load(&"Odysseus".into())).is_ok());
            assert_eq!(1, repo.journal().count());
            assert_eq!(1, repo.recent().count());
            assert_eq!(1, block_on(data_store.get_all_the_things()).unwrap().len());
        }

        if let Id::Uuid(uuid) = block_on(repo.redo()).unwrap().unwrap() {
            assert!(block_on(repo.load(&"Nobody".into())).is_ok());
            assert!(block_on(repo.load(&uuid.into())).is_ok());
        } else {
            panic!();
        }
    }

    #[test]
    fn change_test_edit_by_name_from_recent_wrong_type() {
        let mut repo = repo();
        let change = Change::Edit {
            id: "ODYSSEUS".into(),
            name: "Odysseus".into(),
            diff: Place::default().into(),
        };

        assert_eq!(
            block_on(repo.modify(change.clone())),
            Err((change, Error::NotFound)),
        );
        assert_eq!(1, repo.recent().count());
        assert_eq!(1, repo.journal().count());
        assert!(block_on(repo.load(&"Odysseus".into())).is_ok());
    }

    #[test]
    fn change_test_edit_by_name_from_recent_data_store_failed() {
        let mut repo = repo();
        repo.data_store = Box::new(NullDataStore::default());
        let change = Change::Edit {
            id: "ODYSSEUS".into(),
            name: "Odysseus".into(),
            diff: Npc {
                species: Species::Human.into(),
                ..Default::default()
            }
            .into(),
        };

        {
            assert_eq!(Ok("Odysseus".into()), block_on(repo.modify(change)));
            let result = repo.undo_history().next().unwrap();

            assert_eq!(
                &Change::Edit {
                    id: "ODYSSEUS".into(),
                    name: "Odysseus".into(),
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
                Id::from("ODYSSEUS"),
                block_on(repo.undo()).unwrap().unwrap(),
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
                Id::from("ODYSSEUS"),
                block_on(repo.redo()).unwrap().unwrap(),
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
            id: "ODYSSEUS".into(),
            name: "Odysseus".into(),
            diff: Npc {
                name: "Nobody".into(),
                ..Default::default()
            }
            .into(),
        };

        {
            assert_eq!(Ok("NOBODY".into()), block_on(repo.modify(change)));
            let result = repo.undo_history().next().unwrap();

            assert_eq!(
                &Change::Edit {
                    id: "NOBODY".into(),
                    name: "Nobody".into(),
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
                Id::from("ODYSSEUS"),
                block_on(repo.undo()).unwrap().unwrap(),
            );

            assert!(repo.recent().any(|t| t.name().to_string() == "Odysseus"));
        }

        {
            assert_eq!(Id::from("NOBODY"), block_on(repo.redo()).unwrap().unwrap());
            assert!(repo.recent().any(|t| t.name().to_string() == "Nobody"));
        }
    }

    #[test]
    fn change_test_edit_by_name_from_journal_success() {
        let (mut repo, data_store) = repo_data_store();
        let change = Change::Edit {
            id: "OLYMPUS".into(),
            name: "Olympus".into(),
            diff: Place {
                name: "Hades".into(),
                description: "This really is hell!".into(),
                ..Default::default()
            }
            .into(),
        };
        assert_eq!("editing Olympus", change.display_redo().to_string());

        {
            assert_eq!(Id::Uuid(TEST_UUID), block_on(repo.modify(change)).unwrap());
            let result = repo.undo_history().next().unwrap();

            assert_eq!(
                &Change::Edit {
                    id: TEST_UUID.into(),
                    name: "Hades".into(),
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
            assert!(block_on(repo.load(&"Hades".into())).is_ok());
            assert_eq!(
                "Hades",
                block_on(data_store.get_all_the_things())
                    .unwrap()
                    .first()
                    .unwrap()
                    .name()
                    .value()
                    .unwrap(),
            );
        }

        {
            assert_eq!(Id::Uuid(TEST_UUID), block_on(repo.undo()).unwrap().unwrap());
            assert!(block_on(repo.load(&"OLYMPUS".into())).is_ok());
            assert_eq!(
                "Olympus",
                block_on(data_store.get_all_the_things())
                    .unwrap()
                    .first()
                    .unwrap()
                    .name()
                    .value()
                    .unwrap(),
            );
        }

        {
            assert_eq!(Id::Uuid(TEST_UUID), block_on(repo.redo()).unwrap().unwrap());
            assert!(block_on(repo.load(&"HADES".into())).is_ok());
        }
    }

    #[test]
    fn change_test_edit_by_name_from_journal_wrong_type() {
        let mut repo = repo();
        let change = Change::Edit {
            id: "OLYMPUS".into(),
            name: "Olympus".into(),
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
            id: "OLYMPUS".into(),
            name: "Olympus".into(),
            diff: Place {
                name: "Hades".into(),
                ..Default::default()
            }
            .into(),
        };

        {
            let result = block_on(repo.modify(change));
            assert!(block_on(repo.load(&"Hades".into())).is_ok());
            assert_eq!(Err(Error::NotFound), block_on(repo.load(&"Olympus".into())));

            assert_eq!(
                Err((
                    Change::Edit {
                        id: "Hades".into(),
                        name: "Hades".into(),
                        diff: Place {
                            name: "Olympus".into(),
                            ..Default::default()
                        }
                        .into(),
                    },
                    Error::DataStoreFailed,
                )),
                result,
            );
        }
    }

    #[test]
    fn change_test_edit_by_name_not_found() {
        let mut repo = repo();
        let change = Change::Edit {
            id: "NOBODY".into(),
            name: "Nobody".into(),
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
            id: TEST_UUID.into(),
            name: "Olympus".into(),
            diff: Place {
                name: "Hades".into(),
                description: "This really is hell!".into(),
                ..Default::default()
            }
            .into(),
        };
        assert_eq!("editing Olympus", change.display_redo().to_string());

        {
            assert_eq!(Id::Uuid(TEST_UUID), block_on(repo.modify(change)).unwrap());
            let result = repo.undo_history().next().unwrap();

            assert_eq!(
                &Change::Edit {
                    id: TEST_UUID.into(),
                    name: "Hades".into(),
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
            assert!(block_on(repo.load(&"Hades".into())).is_ok());
            assert_eq!(
                "Hades",
                block_on(data_store.get_all_the_things())
                    .unwrap()
                    .first()
                    .unwrap()
                    .name()
                    .value()
                    .unwrap(),
            );
        }

        {
            assert_eq!(Id::Uuid(TEST_UUID), block_on(repo.undo()).unwrap().unwrap());

            assert!(block_on(repo.load(&"Olympus".into())).is_ok());
            assert_eq!(
                "Olympus",
                block_on(data_store.get_all_the_things())
                    .unwrap()
                    .first()
                    .unwrap()
                    .name()
                    .value()
                    .unwrap(),
            );
        }

        {
            assert_eq!(Id::Uuid(TEST_UUID), block_on(repo.redo()).unwrap().unwrap());
            assert!(block_on(repo.load(&"Hades".into())).is_ok());
        }
    }

    #[test]
    fn change_test_edit_by_uuid_wrong_type() {
        let mut repo = repo();
        let change = Change::Edit {
            id: TEST_UUID.into(),
            name: "Olympus".into(),
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
            id: Uuid::nil().into(),
            name: "Nobody".into(),
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
            id: TEST_UUID.into(),
            name: "Olympus".into(),
            diff: Place {
                name: "Hades".into(),
                ..Default::default()
            }
            .into(),
        };

        {
            let result = block_on(repo.modify(change));
            assert!(block_on(repo.load(&"Hades".into())).is_ok());
            assert_eq!(
                Err(Error::DataStoreFailed),
                block_on(repo.load(&TEST_UUID.into())),
            );
            assert_eq!(Err(Error::NotFound), block_on(repo.load(&"Olympus".into())));

            assert_eq!(
                Err((
                    Change::Edit {
                        id: TEST_UUID.into(),
                        name: "Olympus".into(),
                        diff: Place {
                            // FIXME
                            name: "Olympus".into(),
                            // name: "Hades".into(),
                            ..Default::default()
                        }
                        .into(),
                    },
                    Error::DataStoreFailed,
                )),
                result,
            );
        }
    }

    #[test]
    fn change_test_edit_and_unsave_success() {
        let (mut repo, data_store) = repo_data_store();
        let change = Change::EditAndUnsave {
            name: "Olympus".into(),
            uuid: TEST_UUID.into(),
            diff: Place {
                name: "Hades".into(),
                description: "This really is hell!".into(),
                ..Default::default()
            }
            .into(),
        };
        assert_eq!("editing Olympus", change.display_redo().to_string());

        {
            assert_eq!(Id::from("Hades"), block_on(repo.modify(change)).unwrap());
            let result = repo.undo_history().next().unwrap();

            assert_eq!(
                &Change::Edit {
                    id: "Hades".into(),
                    name: "Hades".into(),
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
            assert!(block_on(repo.load(&"Hades".into())).is_ok());
            assert_eq!(2, repo.recent().count());
            assert_eq!(0, repo.journal().count());
            assert!(block_on(data_store.get_all_the_things())
                .unwrap()
                .is_empty());
        }

        if let Id::Uuid(uuid) = block_on(repo.undo()).unwrap().unwrap() {
            assert_ne!(TEST_UUID, uuid);
            assert!(block_on(repo.load(&"Olympus".into())).is_ok());
            assert!(block_on(repo.load(&uuid.into())).is_ok());
            assert_eq!(1, repo.recent().count());
            assert_eq!(1, repo.journal().count());
            assert_eq!(
                "Olympus",
                block_on(data_store.get_all_the_things())
                    .unwrap()
                    .first()
                    .unwrap()
                    .name()
                    .value()
                    .unwrap(),
            );
        } else {
            panic!();
        }

        {
            assert_eq!(Id::from("Hades"), block_on(repo.redo()).unwrap().unwrap());
            assert!(block_on(repo.load(&"Hades".into())).is_ok());
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
        let mut repo = Repository::new(TimeBombDataStore::new(4));
        populate_repo(&mut repo);

        let change = Change::EditAndUnsave {
            name: "Olympus".into(),
            uuid: TEST_UUID,
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
                    uuid: TEST_UUID,
                },
                Error::DataStoreFailed,
            ),
            block_on(repo.modify(change)).unwrap_err(),
        );
        assert!(block_on(repo.load(&"Hades".into())).is_ok());
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
            assert_eq!(Id::from("Odysseus"), block_on(repo.modify(change)).unwrap());
            let result = repo.undo_history().next().unwrap();

            assert_eq!(
                &Change::Delete {
                    id: "Odysseus".into(),
                    name: "Odysseus".to_string(),
                },
                result,
            );
            assert_eq!("creating Odysseus", result.display_undo().to_string());
            assert_eq!(1, repo.recent().count());
        }

        {
            let _undo_result = block_on(repo.undo()).unwrap().unwrap();

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
        assert_eq!(1, repo.journal().count());
        assert_eq!(1, block_on(data_store.get_all_the_things()).unwrap().len());
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

        if let Id::Uuid(uuid) = block_on(repo.modify(change)).unwrap() {
            let result = repo.undo_history().next().unwrap();

            assert_eq!(
                &Change::Unsave {
                    name: "Odysseus".into(),
                    uuid,
                },
                result,
            );
            assert_eq!(
                "saving Odysseus to journal",
                result.display_undo().to_string(),
            );
            assert_eq!(2, repo.journal().count());
            assert_eq!(2, block_on(data_store.get_all_the_things()).unwrap().len());
            assert_eq!(0, repo.recent().count());
        } else {
            panic!();
        }

        {
            let _undo_result = block_on(repo.undo()).unwrap().unwrap();

            assert_eq!(
                Some(Change::Save {
                    name: "Odysseus".to_string(),
                }),
                repo.redo_change,
            );
            assert_eq!(1, repo.journal().count());
            assert_eq!(1, block_on(data_store.get_all_the_things()).unwrap().len());
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

        assert_eq!(0, repo.journal().count());
        assert_eq!(1, repo.recent().count());

        let change = Change::Save {
            name: "ODYSSEUS".to_string(),
        };
        assert_eq!(
            block_on(repo.modify(change.clone())),
            Err((change, Error::DataStoreFailed)),
        );

        assert_eq!(0, repo.journal().count());
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
            uuid: TEST_UUID.clone(),
        };
        assert_eq!(
            "removing Olympus from journal",
            change.display_redo().to_string(),
        );

        {
            assert_eq!(Id::from("Olympus"), block_on(repo.modify(change)).unwrap());
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
            assert_eq!(0, repo.journal().count());
            assert_eq!(0, block_on(data_store.get_all_the_things()).unwrap().len());
            assert_eq!(2, repo.recent().count());
            assert_eq!(None, block_on(repo.load(&"Olympus".into())).unwrap().uuid());
        }

        {
            let _undo_result = block_on(repo.undo()).unwrap().unwrap();

            if let Some(Change::Unsave { ref name, uuid }) = repo.redo_change {
                assert_eq!("Olympus", name);
                assert_ne!(TEST_UUID, uuid);
                assert!(block_on(repo.load(&uuid.into())).is_ok());
            } else {
                panic!();
            }
            assert_eq!(1, repo.journal().count());
            assert_eq!(1, block_on(data_store.get_all_the_things()).unwrap().len());
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

        if let Id::Uuid(uuid) = block_on(repo.modify(change)).unwrap() {
            let result = repo.undo_history().next().unwrap();

            assert_eq!(
                &Change::Delete {
                    id: Id::Uuid(uuid),
                    name: "Odysseus".into(),
                },
                result,
            );
            assert!(block_on(repo.load(&uuid.to_owned().into())).is_ok());
            assert_eq!("creating Odysseus", result.display_undo().to_string());
            assert_eq!(1, repo.journal().count());
            assert_eq!(
                &uuid,
                block_on(data_store.get_all_the_things())
                    .unwrap()
                    .first()
                    .unwrap()
                    .uuid()
                    .unwrap(),
            );
        } else {
            panic!();
        }

        {
            let uuid = *repo.journal().next().unwrap().uuid().unwrap();
            let _undo_result = block_on(repo.undo()).unwrap().unwrap();

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
            assert_eq!(0, repo.journal().count());
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
        assert_eq!(1, repo.journal().count());
        assert_eq!(1, block_on(data_store.get_all_the_things()).unwrap().len());
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
        assert_eq!(1, repo.journal().count());
        assert_eq!(1, block_on(data_store.get_all_the_things()).unwrap().len());
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
        assert_eq!(0, repo.journal().count());
    }

    #[test]
    fn debug_test() {
        assert_eq!(
            "Repository { cache: {}, data_store_enabled: false, recent: [], time: Time { days: 1, hours: 8, minutes: 0, seconds: 0 } }",
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
                    uuid: Some(TEST_UUID.into()),
                    name: "Olympus".into(),
                    ..Default::default()
                }
                .into(),
            ),
        )
        .unwrap();

        repo.recent.push_back(
            Npc {
                name: "Odysseus".into(),
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
