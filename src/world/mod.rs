use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use rand::Rng;
use uuid::Uuid;

pub use demographics::Demographics;
pub use location::Location;
pub use npc::Npc;
pub use region::Region;

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
    pub regions: HashMap<Rc<region::Uuid>, Region>,
    pub locations: HashMap<Rc<location::Uuid>, Location>,
    pub npcs: HashMap<Rc<npc::Uuid>, Npc>,
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
        regions.insert(Rc::new(Self::ROOT_UUID.into()), Region::default());
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

    pub fn replace_with<F: FnOnce(Option<T>) -> T>(&mut self, f: F) {
        if self.is_unlocked() {
            let value = self.value.take();
            self.replace(f(value));
        }
    }

    pub fn clear(&mut self) {
        if self.is_unlocked() {
            self.value = None;
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

        field.replace_with(|_| 1);
        assert_eq!(Field::new_generated(1), field);

        field.replace_with(|i| i.unwrap() + 1);
        assert_eq!(Field::new_generated(2), field);

        field.lock();

        field.replace_with(|_| 3);
        assert_eq!(Field::new(2), field);
    }

    #[test]
    fn test_clear() {
        let mut field: Field<_> = Field::new_generated(123);
        field.clear();
        assert!(field.is_none());

        let mut field: Field<_> = Field::new(123);
        field.clear();
        assert!(field.is_some());
    }
}
