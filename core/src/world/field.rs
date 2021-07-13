use std::fmt;
use std::mem;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Field<T: fmt::Display> {
    Locked(T),
    Unlocked(T),
    Empty,
}

impl<T: fmt::Display> Field<T> {
    pub fn new(value: T) -> Self {
        Self::Locked(value)
    }

    #[cfg(test)]
    pub fn new_generated(value: T) -> Self {
        Self::Unlocked(value)
    }

    pub fn is_locked(&self) -> bool {
        matches!(self, Self::Locked(_))
    }

    pub fn is_unlocked(&self) -> bool {
        !self.is_locked()
    }

    #[cfg(test)]
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

    #[cfg(test)]
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
            Self::Unlocked(value) => Self::Unlocked(f(Some(value))),
            Self::Empty => Self::Unlocked(f(None)),
            field => field,
        }
    }

    pub fn clear(&mut self) {
        if let Self::Unlocked(_) = self {
            *self = Self::Empty;
        }
    }

    pub fn value(&self) -> Option<&T> {
        match self {
            Self::Locked(value) => Some(value),
            Self::Unlocked(value) => Some(value),
            Self::Empty => None,
        }
    }

    pub fn value_mut(&mut self) -> Option<&mut T> {
        match self {
            Self::Locked(value) => Some(value),
            Self::Unlocked(value) => Some(value),
            Self::Empty => None,
        }
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Self::Empty)
    }
}

impl<T: fmt::Display> Default for Field<T> {
    fn default() -> Self {
        Self::Empty
    }
}

impl<T: fmt::Display> From<T> for Field<T> {
    fn from(value: T) -> Field<T> {
        Self::new(value)
    }
}

impl<T: fmt::Display> From<Field<T>> for Option<T> {
    fn from(field: Field<T>) -> Option<T> {
        match field {
            Field::Locked(value) => Some(value),
            Field::Unlocked(value) => Some(value),
            Field::Empty => None,
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

#[cfg(test)]
mod test_field {
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
        let mut field = Field::Unlocked(false);

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
}
