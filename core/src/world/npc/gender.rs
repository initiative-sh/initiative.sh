use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Gender {
    Masculine,
    Feminine,
    NonBinaryThey,
    Neuter,
}

impl Gender {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Feminine => "feminine",
            Self::Masculine => "masculine",
            Self::Neuter => "neuter",
            Self::NonBinaryThey => "non-binary",
        }
    }

    pub fn pronouns(&self) -> &'static str {
        match self {
            Self::Feminine => "she/her",
            Self::Masculine => "he/him",
            Self::Neuter => "it",
            Self::NonBinaryThey => "they/them",
        }
    }

    pub fn they(&self) -> &'static str {
        match self {
            Self::Feminine => "she",
            Self::Masculine => "he",
            Self::Neuter => "it",
            Self::NonBinaryThey => "they",
        }
    }

    pub fn they_cap(&self) -> &'static str {
        match self {
            Self::Feminine => "She",
            Self::Masculine => "He",
            Self::Neuter => "It",
            Self::NonBinaryThey => "They",
        }
    }

    pub fn theyre(&self) -> &'static str {
        match self {
            Self::Feminine => "she's",
            Self::Masculine => "he's",
            Self::Neuter => "it's",
            Self::NonBinaryThey => "they're",
        }
    }

    pub fn theyre_cap(&self) -> &'static str {
        match self {
            Self::Feminine => "She's",
            Self::Masculine => "He's",
            Self::Neuter => "It's",
            Self::NonBinaryThey => "They're",
        }
    }

    pub fn theyve(&self) -> &'static str {
        match self {
            Self::Feminine => "she's",
            Self::Masculine => "he's",
            Self::Neuter => "it's",
            Self::NonBinaryThey => "they've",
        }
    }

    pub fn theyve_cap(&self) -> &'static str {
        match self {
            Self::Feminine => "She's",
            Self::Masculine => "He's",
            Self::Neuter => "It's",
            Self::NonBinaryThey => "They've",
        }
    }

    pub fn them(&self) -> &'static str {
        match self {
            Self::Feminine => "her",
            Self::Masculine => "him",
            Self::Neuter => "it",
            Self::NonBinaryThey => "them",
        }
    }

    pub fn them_cap(&self) -> &'static str {
        match self {
            Self::Feminine => "Her",
            Self::Masculine => "Him",
            Self::Neuter => "It",
            Self::NonBinaryThey => "Them",
        }
    }

    pub fn their(&self) -> &'static str {
        match self {
            Self::Feminine => "her",
            Self::Masculine => "his",
            Self::Neuter => "its",
            Self::NonBinaryThey => "their",
        }
    }

    pub fn their_cap(&self) -> &'static str {
        match self {
            Self::Feminine => "Her",
            Self::Masculine => "His",
            Self::Neuter => "Its",
            Self::NonBinaryThey => "Their",
        }
    }

    pub fn theirs(&self) -> &'static str {
        match self {
            Self::Feminine => "hers",
            Self::Masculine => "his",
            Self::Neuter => "its",
            Self::NonBinaryThey => "theirs",
        }
    }

    pub fn theirs_cap(&self) -> &'static str {
        match self {
            Self::Feminine => "Hers",
            Self::Masculine => "His",
            Self::Neuter => "Its",
            Self::NonBinaryThey => "Theirs",
        }
    }

    pub fn themself(&self) -> &'static str {
        match self {
            Self::Feminine => "herself",
            Self::Masculine => "himself",
            Self::Neuter => "itself",
            Self::NonBinaryThey => "themself",
        }
    }

    pub fn themself_cap(&self) -> &'static str {
        match self {
            Self::Feminine => "Herself",
            Self::Masculine => "Himself",
            Self::Neuter => "Itself",
            Self::NonBinaryThey => "Themself",
        }
    }

    pub fn conjugate<'a>(&self, singular_form: &'a str, plural_form: &'a str) -> &'a str {
        if self == &Self::NonBinaryThey {
            plural_form
        } else {
            singular_form
        }
    }
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Masculine => write!(f, "masculine (he/him)"),
            Self::Feminine => write!(f, "feminine (she/her)"),
            Self::NonBinaryThey => write!(f, "non-binary (they/them)"),
            Self::Neuter => write!(f, "neuter (it)"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pronouns_test() {
        let [f, m, n, t] = variants();

        assert_eq!("she/her", f.pronouns());
        assert_eq!("he/him", m.pronouns());
        assert_eq!("it", n.pronouns());
        assert_eq!("they/them", t.pronouns());

        assert_eq!("she", f.they());
        assert_eq!("She", f.they_cap());
        assert_eq!("he", m.they());
        assert_eq!("He", m.they_cap());
        assert_eq!("it", n.they());
        assert_eq!("It", n.they_cap());
        assert_eq!("they", t.they());
        assert_eq!("They", t.they_cap());

        assert_eq!("she's", f.theyre());
        assert_eq!("She's", f.theyre_cap());
        assert_eq!("he's", m.theyre());
        assert_eq!("He's", m.theyre_cap());
        assert_eq!("it's", n.theyre());
        assert_eq!("It's", n.theyre_cap());
        assert_eq!("they're", t.theyre());
        assert_eq!("They're", t.theyre_cap());

        assert_eq!("she's", f.theyve());
        assert_eq!("She's", f.theyve_cap());
        assert_eq!("he's", m.theyve());
        assert_eq!("He's", m.theyve_cap());
        assert_eq!("it's", n.theyve());
        assert_eq!("It's", n.theyve_cap());
        assert_eq!("they've", t.theyve());
        assert_eq!("They've", t.theyve_cap());

        assert_eq!("her", f.them());
        assert_eq!("Her", f.them_cap());
        assert_eq!("him", m.them());
        assert_eq!("Him", m.them_cap());
        assert_eq!("it", n.them());
        assert_eq!("It", n.them_cap());
        assert_eq!("them", t.them());
        assert_eq!("Them", t.them_cap());

        assert_eq!("her", f.their());
        assert_eq!("Her", f.their_cap());
        assert_eq!("his", m.their());
        assert_eq!("His", m.their_cap());
        assert_eq!("its", n.their());
        assert_eq!("Its", n.their_cap());
        assert_eq!("their", t.their());
        assert_eq!("Their", t.their_cap());

        assert_eq!("hers", f.theirs());
        assert_eq!("Hers", f.theirs_cap());
        assert_eq!("his", m.theirs());
        assert_eq!("His", m.theirs_cap());
        assert_eq!("its", n.theirs());
        assert_eq!("Its", n.theirs_cap());
        assert_eq!("theirs", t.theirs());
        assert_eq!("Theirs", t.theirs_cap());

        assert_eq!("herself", f.themself());
        assert_eq!("Herself", f.themself_cap());
        assert_eq!("himself", m.themself());
        assert_eq!("Himself", m.themself_cap());
        assert_eq!("itself", n.themself());
        assert_eq!("Itself", n.themself_cap());
        assert_eq!("themself", t.themself());
        assert_eq!("Themself", t.themself_cap());
    }

    #[test]
    fn conjugate_test() {
        let [f, m, n, t] = variants();

        assert_eq!("conjugate", m.conjugate("conjugate", "conjugates"));
        assert_eq!("conjugate", f.conjugate("conjugate", "conjugates"));
        assert_eq!("conjugates", t.conjugate("conjugate", "conjugates"));
        assert_eq!("conjugate", n.conjugate("conjugate", "conjugates"));
    }

    #[test]
    fn fmt_test() {
        assert_eq!("masculine (he/him)", format!("{}", Gender::Masculine));
        assert_eq!("feminine (she/her)", format!("{}", Gender::Feminine));
        assert_eq!(
            "non-binary (they/them)",
            format!("{}", Gender::NonBinaryThey)
        );
        assert_eq!("neuter (it)", format!("{}", Gender::Neuter));
    }

    #[test]
    fn serialize_deserialize_test() {
        let [f, m, n, t] = variants();

        assert_eq!("\"Feminine\"", serde_json::to_string(&f).unwrap());
        assert_eq!("\"Masculine\"", serde_json::to_string(&m).unwrap());
        assert_eq!("\"Neuter\"", serde_json::to_string(&n).unwrap());
        assert_eq!("\"NonBinaryThey\"", serde_json::to_string(&t).unwrap());

        let value: Gender = serde_json::from_str("\"Feminine\"").unwrap();
        assert_eq!(f, value);

        let value: Gender = serde_json::from_str("\"Masculine\"").unwrap();
        assert_eq!(m, value);

        let value: Gender = serde_json::from_str("\"Neuter\"").unwrap();
        assert_eq!(n, value);

        let value: Gender = serde_json::from_str("\"NonBinaryThey\"").unwrap();
        assert_eq!(t, value);
    }

    fn variants() -> [Gender; 4] {
        [
            Gender::Feminine,
            Gender::Masculine,
            Gender::Neuter,
            Gender::NonBinaryThey,
        ]
    }
}
