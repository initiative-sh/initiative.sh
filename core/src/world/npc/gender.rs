use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Gender {
    Masculine,
    Feminine,
    Trans,
    Neuter,
}

impl Gender {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Masculine => "masculine",
            Self::Feminine => "feminine",
            Self::Trans => "trans",
            Self::Neuter => "neuter",
        }
    }

    pub fn pronouns(&self) -> &'static str {
        match self {
            Self::Masculine => "he/him",
            Self::Feminine => "she/her",
            Self::Trans => "they/them",
            Self::Neuter => "it",
        }
    }
}

#[cfg(test)]
mod test_gender {
    use super::*;

    #[test]
    fn pronouns_test() {
        assert_eq!("he/him", Gender::Masculine.pronouns());
        assert_eq!("she/her", Gender::Feminine.pronouns());
        assert_eq!("they/them", Gender::Trans.pronouns());
        assert_eq!("it", Gender::Neuter.pronouns());
    }
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Masculine => write!(f, "masculine (he/him)"),
            Self::Feminine => write!(f, "feminine (she/her)"),
            Self::Trans => write!(f, "trans (they/them)"),
            Self::Neuter => write!(f, "neuter (it)"),
        }
    }
}

#[cfg(test)]
mod test_display_for_gender {
    use super::*;

    #[test]
    fn fmt_test() {
        assert_eq!("masculine (he/him)", format!("{}", Gender::Masculine));
        assert_eq!("feminine (she/her)", format!("{}", Gender::Feminine));
        assert_eq!("trans (they/them)", format!("{}", Gender::Trans));
        assert_eq!("neuter (it)", format!("{}", Gender::Neuter));
    }
}
