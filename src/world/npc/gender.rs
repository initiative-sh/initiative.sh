use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Gender {
    Masculine,
    Feminine,
    Trans,
    Neuter,
}

impl Gender {
    pub fn pronouns(&self) -> &'static str {
        match self {
            Self::Masculine => "he/him",
            Self::Feminine => "she/her",
            Self::Trans => "they/them",
            Self::Neuter => "it",
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
