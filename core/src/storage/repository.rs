use crate::storage::DataStore;
use crate::time::Time;
use crate::{Thing, Uuid};
use std::collections::{HashMap, VecDeque};
use std::fmt;

const RECENT_MAX_LEN: usize = 100;

pub struct Repository {
    pub cache: HashMap<Uuid, Thing>,
    data_store: Box<dyn DataStore>,
    pub data_store_enabled: bool,
    recent: VecDeque<Thing>,
    time: Time,
}

#[derive(Clone, Debug)]
pub enum Change {
    Delete { id: Id },
}

#[derive(Clone, Debug)]
pub enum Id {
    Name(String),
    Uuid(Uuid),
}

impl Repository {
    pub fn new(data_store: impl DataStore + 'static) -> Self {
        Self {
            cache: HashMap::default(),
            data_store: Box::new(data_store),
            data_store_enabled: false,
            recent: VecDeque::default(),
            time: Time::try_new(1, 8, 0, 0).unwrap(),
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
        }

        if let Ok(Some(time_str)) = self.data_store.get_value("time").await {
            if let Ok(time) = time_str.parse() {
                self.set_time(time).await;
            }
        }
    }

    pub async fn modify(&mut self, change: Change) -> Result<String, String> {
        match change {
            Change::Delete { id } => match id {
                Id::Name(name) => self.delete_thing_by_name(&name).await,
                Id::Uuid(_) => unimplemented!(),
            },
        }
    }

    pub fn push_recent(&mut self, thing: Thing) {
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

    pub fn recent(&self) -> &[Thing] {
        self.recent.as_slices().0
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

    async fn delete_thing_by_name(&mut self, name: &str) -> Result<String, String> {
        let lowercase_name = name.to_lowercase();
        let name_matches = |s: &String| s.to_lowercase() == lowercase_name;

        let cached_thing = if let Some((uuid, thing)) = self
            .cache
            .iter()
            .find(|(_, t)| t.name().value().map_or(false, name_matches))
        {
            Some((*uuid, thing.name().to_string()))
        } else {
            None
        };

        if let Some((uuid, thing_name)) = cached_thing {
            let (store_delete_success, cache_delete_success) = (
                self.data_store.delete_thing_by_uuid(&uuid).await.is_ok(),
                self.cache.remove(&uuid).is_some(),
            );

            if store_delete_success || cache_delete_success {
                Ok(format!("{} was successfully deleted.", thing_name))
            } else {
                Err(format!("Could not delete {}.", thing_name))
            }
        } else if let Some(thing) =
            self.take_recent(|t| t.name().value().map_or(false, name_matches))
        {
            Ok(format!(
            "{} deleted from recent entries. This isn't normally necessary as recent entries aren't automatically saved from one session to another.",
            thing.name(),
        ))
        } else {
            Err(format!("There is no entity named \"{}\".", name))
        }
    }

    pub fn load_thing_by_name<'a>(&'a self, name: &str) -> Option<&'a Thing> {
        let lowercase_name = name.to_lowercase();
        self.cache.values().chain(self.recent().iter()).find(|t| {
            t.name()
                .value()
                .map_or(false, |s| s.to_lowercase() == lowercase_name)
        })
    }

    pub fn load_all_the_things(&self) -> impl Iterator<Item = &Thing> {
        self.cache.values()
    }

    pub async fn save_thing_by_name(&mut self, name: &str) -> Result<String, String> {
        let lowercase_name = name.to_lowercase();
        if let Some(mut thing) = self.take_recent(|t| {
            t.name()
                .value()
                .map_or(false, |s| s.to_lowercase() == lowercase_name)
        }) {
            thing.set_uuid(Uuid::new_v4());

            let result = self
                .data_store
                .save_thing(&thing)
                .await
                .map_err(|_| format!("Couldn't save `{}`", thing.name()))
                .map(|_| format!("{} was successfully saved.", thing.display_summary()));

            if result.is_ok() {
                self.cache.insert(*thing.uuid().unwrap(), thing);
            } else {
                // Oops, better put it back where we found it.
                self.push_recent(thing);
            }

            result
        } else if self.cache.values().any(|t| {
            t.name()
                .value()
                .map_or(false, |s| s.to_lowercase() == lowercase_name)
        }) {
            Err(format!(
                "`{}` has already been saved to your `journal`",
                name,
            ))
        } else {
            Err(format!("No matches for \"{}\"", name))
        }
    }
}

impl From<Uuid> for Id {
    fn from(input: Uuid) -> Self {
        Id::Uuid(input)
    }
}

impl From<String> for Id {
    fn from(input: String) -> Self {
        Id::Name(input)
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
    use crate::storage::data_store::MemoryDataStore;
    use crate::world::{Location, Npc};
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
                .first()
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
                .first()
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
    fn change_test_delete_from_journal_by_name() {
        let mut repo = repo();
        assert_eq!(
            "Olympus was successfully deleted.",
            block_on(repo.modify(Change::Delete {
                id: "olympus".to_string().into(),
            }))
            .unwrap(),
        );
        assert!(repo.cache.is_empty(), "{:?}", repo.cache);
    }

    #[test]
    fn change_test_delete_from_recent_by_name() {
        let mut repo = repo();
        assert_eq!(
                "Odysseus deleted from recent entries. This isn't normally necessary as recent entries aren't automatically saved from one session to another.",
                block_on(repo.modify(Change::Delete {
                    id: "odysseus".to_string().into(),
                }))
                .unwrap(),
            );
        assert!(repo.recent.is_empty(), "{:?}", repo.cache);
    }

    #[test]
    fn change_test_delete_by_name_not_found() {
        assert_eq!(
            "There is no entity named \"Invisible Inc.\".",
            block_on(repo().modify(Change::Delete {
                id: "Invisible Inc.".to_string().into(),
            }))
            .unwrap_err(),
        );
    }

    #[test]
    #[should_panic = "not implemented"]
    fn change_test_delete_by_uuid() {
        block_on(repo().modify(Change::Delete {
            id: TEST_UUID.into(),
        }))
        .unwrap();
    }

    #[test]
    fn debug_test() {
        assert_eq!(
            "Repository { cache: {}, data_store_enabled: false, recent: [], time: Time { days: 1, hours: 8, minutes: 0, seconds: 0 } }",
            format!("{:?}", empty_repo()),
        );
    }

    fn repo() -> Repository {
        let mut data_store = MemoryDataStore::default();
        block_on(
            data_store.save_thing(
                &Location {
                    uuid: Some(TEST_UUID.into()),
                    name: "Olympus".into(),
                    ..Default::default()
                }
                .into(),
            ),
        )
        .unwrap();

        let mut repo = Repository::new(data_store);
        block_on(repo.init());

        repo.recent.push_back(
            Npc {
                name: "Odysseus".into(),
                ..Default::default()
            }
            .into(),
        );

        repo
    }

    fn empty_repo() -> Repository {
        Repository::new(MemoryDataStore::default())
    }
}
