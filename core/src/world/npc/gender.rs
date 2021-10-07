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

    pub fn they_cap(&self) -> &'static str {
        match self {
            Self::Feminine => "She",
            Self::Masculine => "He",
            Self::Neuter => "It",
            Self::Trans => "They",
        }
    }

    pub fn theyre(&self) -> &'static str {
        match self {
            Self::Feminine => "she's",
            Self::Masculine => "he's",
            Self::Neuter => "it's",
            Self::Trans => "they're",
        }
    }

    pub fn theyre_cap(&self) -> &'static str {
        match self {
            Self::Feminine => "She's",
            Self::Masculine => "He's",
            Self::Neuter => "It's",
            Self::Trans => "They're",
        }
    }

    pub fn theyve(&self) -> &'static str {
        match self {
            Self::Feminine => "she's",
            Self::Masculine => "he's",
            Self::Neuter => "it's",
            Self::Trans => "they've",
        }
    }

    pub fn theyve_cap(&self) -> &'static str {
        match self {
            Self::Feminine => "She's",
            Self::Masculine => "He's",
            Self::Neuter => "It's",
            Self::Trans => "They've",
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

    pub fn them_cap(&self) -> &'static str {
        match self {
            Self::Feminine => "Her",
            Self::Masculine => "Him",
            Self::Neuter => "It",
            Self::Trans => "Them",
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

    pub fn their_cap(&self) -> &'static str {
        match self {
            Self::Feminine => "Her",
            Self::Masculine => "His",
            Self::Neuter => "Its",
            Self::Trans => "Their",
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

    pub fn theirs_cap(&self) -> &'static str {
        match self {
            Self::Feminine => "Hers",
            Self::Masculine => "His",
            Self::Neuter => "Its",
            Self::Trans => "Theirs",
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

    pub fn themself_cap(&self) -> &'static str {
        match self {
            Self::Feminine => "Herself",
            Self::Masculine => "Himself",
            Self::Neuter => "Itself",
            Self::Trans => "Themself",
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
        assert_eq!("She", Gender::Feminine.they_cap());
        assert_eq!("he", Gender::Masculine.they());
        assert_eq!("He", Gender::Masculine.they_cap());
        assert_eq!("it", Gender::Neuter.they());
        assert_eq!("It", Gender::Neuter.they_cap());
        assert_eq!("they", Gender::Trans.they());
        assert_eq!("They", Gender::Trans.they_cap());

        assert_eq!("she's", Gender::Feminine.theyre());
        assert_eq!("She's", Gender::Feminine.theyre_cap());
        assert_eq!("he's", Gender::Masculine.theyre());
        assert_eq!("He's", Gender::Masculine.theyre_cap());
        assert_eq!("it's", Gender::Neuter.theyre());
        assert_eq!("It's", Gender::Neuter.theyre_cap());
        assert_eq!("they're", Gender::Trans.theyre());
        assert_eq!("They're", Gender::Trans.theyre_cap());

        assert_eq!("she's", Gender::Feminine.theyve());
        assert_eq!("She's", Gender::Feminine.theyve_cap());
        assert_eq!("he's", Gender::Masculine.theyve());
        assert_eq!("He's", Gender::Masculine.theyve_cap());
        assert_eq!("it's", Gender::Neuter.theyve());
        assert_eq!("It's", Gender::Neuter.theyve_cap());
        assert_eq!("they've", Gender::Trans.theyve());
        assert_eq!("They've", Gender::Trans.theyve_cap());

        assert_eq!("her", Gender::Feminine.them());
        assert_eq!("Her", Gender::Feminine.them_cap());
        assert_eq!("him", Gender::Masculine.them());
        assert_eq!("Him", Gender::Masculine.them_cap());
        assert_eq!("it", Gender::Neuter.them());
        assert_eq!("It", Gender::Neuter.them_cap());
        assert_eq!("them", Gender::Trans.them());
        assert_eq!("Them", Gender::Trans.them_cap());

        assert_eq!("her", Gender::Feminine.their());
        assert_eq!("Her", Gender::Feminine.their_cap());
        assert_eq!("his", Gender::Masculine.their());
        assert_eq!("His", Gender::Masculine.their_cap());
        assert_eq!("its", Gender::Neuter.their());
        assert_eq!("Its", Gender::Neuter.their_cap());
        assert_eq!("their", Gender::Trans.their());
        assert_eq!("Their", Gender::Trans.their_cap());

        assert_eq!("hers", Gender::Feminine.theirs());
        assert_eq!("Hers", Gender::Feminine.theirs_cap());
        assert_eq!("his", Gender::Masculine.theirs());
        assert_eq!("His", Gender::Masculine.theirs_cap());
        assert_eq!("its", Gender::Neuter.theirs());
        assert_eq!("Its", Gender::Neuter.theirs_cap());
        assert_eq!("theirs", Gender::Trans.theirs());
        assert_eq!("Theirs", Gender::Trans.theirs_cap());

        assert_eq!("herself", Gender::Feminine.themself());
        assert_eq!("Herself", Gender::Feminine.themself_cap());
        assert_eq!("himself", Gender::Masculine.themself());
        assert_eq!("Himself", Gender::Masculine.themself_cap());
        assert_eq!("itself", Gender::Neuter.themself());
        assert_eq!("Itself", Gender::Neuter.themself_cap());
        assert_eq!("themself", Gender::Trans.themself());
        assert_eq!("Themself", Gender::Trans.themself_cap());
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
