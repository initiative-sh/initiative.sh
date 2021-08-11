use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Gender {
    Masculine,
    Feminine,
    Trans,
    Neuter,
}

impl Gender {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Feminine => "feminine",
            Self::Masculine => "masculine",
            Self::Neuter => "neuter",
            Self::Trans => "trans",
        }
    }

    pub fn pronouns(&self) -> &'static str {
        match self {
            Self::Feminine => "she/her",
            Self::Masculine => "he/him",
            Self::Neuter => "it",
            Self::Trans => "they/them",
        }
    }

    pub fn they(&self) -> &'static str {
        match self {
            Self::Feminine => "she",
            Self::Masculine => "he",
            Self::Neuter => "it",
            Self::Trans => "they",
        }
    }

    pub fn them(&self) -> &'static str {
        match self {
            Self::Feminine => "her",
            Self::Masculine => "him",
            Self::Neuter => "it",
            Self::Trans => "them",
        }
    }

    pub fn their(&self) -> &'static str {
        match self {
            Self::Feminine => "her",
            Self::Masculine => "his",
            Self::Neuter => "its",
            Self::Trans => "their",
        }
    }

    pub fn theirs(&self) -> &'static str {
        match self {
            Self::Feminine => "hers",
            Self::Masculine => "his",
            Self::Neuter => "its",
            Self::Trans => "theirs",
        }
    }

    pub fn themself(&self) -> &'static str {
        match self {
            Self::Feminine => "herself",
            Self::Masculine => "himself",
            Self::Neuter => "itself",
            Self::Trans => "themself",
        }
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
mod test {
    use super::*;

    #[test]
    fn pronouns_test() {
        assert_eq!("she/her", Gender::Feminine.pronouns());
        assert_eq!("he/him", Gender::Masculine.pronouns());
        assert_eq!("it", Gender::Neuter.pronouns());
        assert_eq!("they/them", Gender::Trans.pronouns());

        assert_eq!("she", Gender::Feminine.they());
        assert_eq!("he", Gender::Masculine.they());
        assert_eq!("it", Gender::Neuter.they());
        assert_eq!("they", Gender::Trans.they());

        assert_eq!("her", Gender::Feminine.them());
        assert_eq!("him", Gender::Masculine.them());
        assert_eq!("it", Gender::Neuter.them());
        assert_eq!("them", Gender::Trans.them());

        assert_eq!("her", Gender::Feminine.their());
        assert_eq!("his", Gender::Masculine.their());
        assert_eq!("its", Gender::Neuter.their());
        assert_eq!("their", Gender::Trans.their());

        assert_eq!("hers", Gender::Feminine.theirs());
        assert_eq!("his", Gender::Masculine.theirs());
        assert_eq!("its", Gender::Neuter.theirs());
        assert_eq!("theirs", Gender::Trans.theirs());

        assert_eq!("herself", Gender::Feminine.themself());
        assert_eq!("himself", Gender::Masculine.themself());
        assert_eq!("itself", Gender::Neuter.themself());
        assert_eq!("themself", Gender::Trans.themself());
    }

    #[test]
    fn fmt_test() {
        assert_eq!("masculine (he/him)", format!("{}", Gender::Masculine));
        assert_eq!("feminine (she/her)", format!("{}", Gender::Feminine));
        assert_eq!("trans (they/them)", format!("{}", Gender::Trans));
        assert_eq!("neuter (it)", format!("{}", Gender::Neuter));
    }

    #[test]
    fn serialize_deserialize_test() {
        assert_eq!("\"Trans\"", serde_json::to_string(&Gender::Trans).unwrap());

        let value: Gender = serde_json::from_str("\"Trans\"").unwrap();
        assert_eq!(Gender::Trans, value);
    }
}
