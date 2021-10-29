use serde::{Deserialize, Serialize, Serializer};
use std::fmt;
use std::mem;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(from = "Option<T>")]
pub enum Field<T> {
    Locked(Option<T>),
    Unlocked(Option<T>),
}

impl<T> Field<T> {
    pub fn new(value: T) -> Self {
        Self::Locked(Some(value))
    }

    #[cfg(test)]
    pub fn new_generated(value: T) -> Self {
        Self::Unlocked(Some(value))
    }

    pub fn is_locked(&self) -> bool {
        matches!(self, Self::Locked(_))
    }

    pub fn is_unlocked(&self) -> bool {
        !self.is_locked()
    }

    pub fn lock(&mut self) {
        *self = match mem::take(self) {
            Self::Unlocked(value) => Self::Locked(value),
            field => field,
        }
    }

    #[cfg(test)]
    pub fn locked(mut self) -> Self {
        self.lock();
        self
    }

    pub fn unlock(&mut self) {
        *self = match mem::take(self) {
            Self::Locked(value) => Self::Unlocked(value),
            field => field,
        }
    }

    #[cfg(test)]
    pub fn unlocked(mut self) -> Self {
        self.unlock();
        self
    }

    pub fn replace(&mut self, value: T) {
        self.replace_with(|_| value);
    }

    pub fn replace_with<F: FnOnce(Option<T>) -> T>(&mut self, f: F) {
        *self = match mem::take(self) {
            Self::Unlocked(value) => Self::Unlocked(Some(f(value))),
            field => field,
        }
    }

    pub fn clear(&mut self) {
        if let Self::Unlocked(_) = self {
            *self = Self::Unlocked(None)
        }
    }

    pub fn value(&self) -> Option<&T> {
        match self {
            Self::Locked(value) => value,
            Self::Unlocked(value) => value,
        }
        .as_ref()
    }

    pub fn value_mut(&mut self) -> Option<&mut T> {
        match self {
            Self::Locked(value) => value,
            Self::Unlocked(value) => value,
        }
        .as_mut()
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }

    pub fn is_none(&self) -> bool {
        self.value().is_none()
    }

    pub fn apply_diff(&mut self, other: &mut Self) {
        if other.is_locked() {
            mem::swap(self, other);
            other.lock();
        }
    }
}

impl<T> Default for Field<T> {
    fn default() -> Self {
        Self::Unlocked(None)
    }
}

impl<T> From<T> for Field<T> {
    fn from(value: T) -> Field<T> {
        Self::new(value)
    }
}

impl<T> From<Option<T>> for Field<T> {
    fn from(value: Option<T>) -> Field<T> {
        Field::Locked(value)
    }
}

impl<T> From<Field<T>> for Option<T> {
    fn from(field: Field<T>) -> Option<T> {
        match field {
            Field::Locked(value) => value,
            Field::Unlocked(value) => value,
        }
    }
}

impl From<&str> for Field<String> {
    fn from(value: &str) -> Field<String> {
        Self::new(value.to_string())
    }
}

impl<T: fmt::Display> fmt::Display for Field<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(value) = &self.value() {
            write!(f, "{}", value)?;
        }
        Ok(())
    }
}

impl<T: Serialize> Serialize for Field<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.value() {
            Some(v) => serializer.serialize_some(v),
            None => serializer.serialize_none(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Field;

    #[test]
    fn default_test() {
        let field: Field<bool> = Field::default();
        assert!(!field.is_locked());
        assert!(field.is_none());
    }

    #[test]
    fn new_test() {
        {
            let field: Field<_> = Field::new("hello");
            assert!(field.is_locked());
            assert!(field.is_some());
            assert_eq!(Some(&"hello"), field.value());
        }

        {
            let field: Field<_> = Field::new_generated("goodbye");
            assert!(!field.is_locked());
            assert!(field.is_some());
            assert_eq!(Some(&"goodbye"), field.value());
        }
    }

    #[test]
    fn lock_unlock_test() {
        let mut field = Field::Unlocked(Some(false));

        assert!(field.is_unlocked());
        assert!(!field.is_locked());

        field.lock();

        assert!(!field.is_unlocked());
        assert!(field.is_locked());

        field.unlock();
        assert!(field.is_unlocked());

        field = field.locked();
        assert!(field.is_locked());

        field = field.unlocked();
        assert!(field.is_unlocked());
    }

    #[test]
    fn replace_with_test() {
        let mut field: Field<_> = Field::default();

        field.replace(1);
        assert_eq!(Field::new_generated(1), field);

        field.replace_with(|i| i.unwrap() + 1);
        assert_eq!(Field::new_generated(2), field);

        field.lock();

        field.replace_with(|_| 3);
        assert_eq!(Field::new(2), field);
    }

    #[test]
    fn clear_test() {
        let mut field: Field<_> = Field::new_generated(123);
        field.clear();
        assert!(field.is_none());

        let mut field: Field<_> = Field::new(123);
        field.clear();
        assert!(field.is_some());
    }

    #[test]
    fn serialize_test() {
        let field: Field<_> = Field::new(123);
        assert_eq!("123", serde_json::to_string(&field).unwrap());

        let field: Field<bool> = Field::default();
        assert_eq!("null", serde_json::to_string(&field).unwrap());
    }

    #[test]
    fn deserialize_test() {
        let field: Field<u8> = serde_json::from_str("123").unwrap();
        assert_eq!(Field::Locked(Some(123)), field);

        let field: Field<u8> = serde_json::from_str("null").unwrap();
        assert_eq!(Field::Locked(None), field);
    }

    #[test]
    fn apply_diff_test() {
        {
            let mut field = Field::Locked(Some(false));
            let mut diff = Field::Unlocked(None);
            field.apply_diff(&mut diff);
            assert_eq!(Field::Locked(Some(false)), field);
            assert_eq!(Field::Unlocked(None), diff);
        }

        {
            let mut field = Field::Locked(Some(false));
            let mut diff = Field::Locked(Some(true));
            field.apply_diff(&mut diff);
            assert_eq!(Field::Locked(Some(true)), field);
            assert_eq!(Field::Locked(Some(false)), diff);
        }

        {
            let mut field = Field::Locked(Some(false));
            let mut diff = Field::Locked(None);
            field.apply_diff(&mut diff);
            assert_eq!(Field::Locked(None), field);
            assert_eq!(Field::Locked(Some(false)), diff);
        }

        {
            let mut field = Field::Unlocked(Some(false));
            let mut diff = Field::Locked(Some(true));
            field.apply_diff(&mut diff);
            assert_eq!(Field::Locked(Some(true)), field);
            assert_eq!(Field::Locked(Some(false)), diff);
        }
    }
}
