use std::fmt;

use super::Race;

#[derive(Debug, PartialEq)]
pub enum Age {
    Infant(u16),
    Child(u16),
    Adolescent(u16),
    YoungAdult(u16),
    Adult(u16),
    MiddleAged(u16),
    Elderly(u16),
    Geriatric(u16),
}

impl Age {
    pub fn years(&self) -> u16 {
        match self {
            Self::Infant(i) => *i,
            Self::Child(i) => *i,
            Self::Adolescent(i) => *i,
            Self::YoungAdult(i) => *i,
            Self::Adult(i) => *i,
            Self::MiddleAged(i) => *i,
            Self::Elderly(i) => *i,
            Self::Geriatric(i) => *i,
        }
    }

    pub fn category(&self) -> &'static str {
        match self {
            Self::Infant(_) => "infant",
            Self::Child(_) => "child",
            Self::Adolescent(_) => "adolescent",
            Self::YoungAdult(_) => "young adult",
            Self::Adult(_) => "adult",
            Self::MiddleAged(_) => "middle-aged",
            Self::Elderly(_) => "elderly",
            Self::Geriatric(_) => "geriatric",
        }
    }

    pub fn fmt_with_race(&self, race: Option<&Race>, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(race) = race {
            match self {
                // "human infant"
                // "human child"
                Age::Infant(_) | Age::Child(_) => {
                    write!(f, "{} {}", race, self.category())
                }

                // "adolescent human"
                // "adult human"
                // "middle-aged human"
                // "elderly human"
                // "geriatric human"
                _ => {
                    write!(f, "{} {}", self.category(), race)
                }
            }
        } else {
            write!(f, "{}", self.category())
        }
    }
}

impl fmt::Display for Age {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({} years)", self.category(), self.years())
    }
}
