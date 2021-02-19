use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use rand::Rng;
use uuid::Uuid;

pub use demographics::Demographics;
pub use location::{Location, LocationUuid};
pub use npc::{Npc, NpcUuid};
pub use region::{Region, RegionUuid};

mod demographics;
mod location;
mod npc;
mod region;

pub type WorldUuid = Uuid;

pub trait Generate: Default {
    fn generate(rng: &mut impl rand::Rng, demographics: &Demographics) -> Self {
        let mut result = Self::default();
        result.regenerate(rng, demographics);
        result
    }

    fn regenerate(&mut self, rng: &mut impl Rng, demographics: &Demographics);
}

trait PopulateFields {
    fn populate_fields(&mut self, rng: &mut impl Rng, demographics: &Demographics);
}

pub struct World {
    pub uuid: Rc<WorldUuid>,
    pub regions: HashMap<Rc<RegionUuid>, Region>,
    pub locations: HashMap<Rc<LocationUuid>, Location>,
    pub npcs: HashMap<Rc<NpcUuid>, Npc>,
}

#[derive(Debug, PartialEq)]
pub struct Field<T> {
    is_locked: bool,
    value: Option<T>,
}

impl World {
    const ROOT_UUID: Uuid = Uuid::from_bytes([0xFF; 16]);
}

impl Default for World {
    fn default() -> Self {
        let mut regions = HashMap::new();
        regions.insert(Rc::new(Self::ROOT_UUID), Region::default());
        World {
            uuid: Rc::new(Uuid::new_v4()),
            regions,
            locations: HashMap::default(),
            npcs: HashMap::default(),
        }
    }
}

impl<T> Field<T> {
    pub fn new(value: T) -> Self {
        Self {
            is_locked: true,
            value: Some(value),
        }
    }

    pub fn new_generated(value: T) -> Self {
        Self {
            is_locked: false,
            value: Some(value),
        }
    }

    pub const fn is_locked(&self) -> bool {
        self.is_locked
    }

    pub const fn is_unlocked(&self) -> bool {
        !self.is_locked()
    }

    pub fn lock(&mut self) {
        self.is_locked = true;
    }

    pub fn unlock(&mut self) {
        self.is_locked = false;
    }

    pub fn replace_with<F: FnOnce() -> T>(&mut self, f: F) {
        if !self.is_locked() {
            self.replace(f());
        }
    }
}

impl<T> Default for Field<T> {
    fn default() -> Self {
        Self {
            is_locked: false,
            value: None,
        }
    }
}

impl<T> From<T> for Field<T> {
    fn from(value: T) -> Field<T> {
        Self::new(value)
    }
}

impl<T> Deref for Field<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Field<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

#[cfg(test)]
mod test_field {
    use super::Field;

    #[test]
    fn test_deref_from() {
        let mut field: Field<u8> = 123.into();
        assert!(field.is_some());
        assert_eq!(Some(123), field.take());
        assert!(field.is_none());
    }

    #[test]
    fn test_default() {
        let field: Field<bool> = Field::default();
        assert!(!field.is_locked());
        assert!(field.is_none());
    }

    #[test]
    fn test_new() {
        {
            let mut field: Field<_> = Field::new("hello");
            assert!(field.is_locked());
            assert!(field.is_some());
            assert_eq!(Some("hello"), field.take());
        }

        {
            let mut field: Field<_> = Field::new_generated("goodbye");
            assert!(!field.is_locked());
            assert!(field.is_some());
            assert_eq!(Some("goodbye"), field.take());
        }
    }

    #[test]
    fn test_lock() {
        let mut field: Field<bool> = Field::default();

        assert!(field.is_unlocked());
        assert!(!field.is_locked());

        field.lock();

        assert!(!field.is_unlocked());
        assert!(field.is_locked());

        field.unlock();

        assert!(field.is_unlocked());
        assert!(!field.is_locked());
    }

    #[test]
    fn test_replace_with() {
        let mut field: Field<_> = Field::default();

        field.replace_with(|| "Hello");
        assert_eq!(Field::new_generated("Hello"), field);

        field.lock();

        field.replace_with(|| "Goodbye");
        assert_eq!(Field::new("Hello"), field);
    }
}
