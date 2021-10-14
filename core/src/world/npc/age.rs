use super::Species;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Age {
    Infant,
    Child,
    Adolescent,
    YoungAdult,
    Adult,
    MiddleAged,
    Elderly,
    Geriatric,
}

impl Age {
    pub fn fmt_with_species(
        &self,
        species: Option<&Species>,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        if let Some(species) = species {
            match self {
                Age::Infant | Age::Child => write!(f, "{} {}", species, self),
                _ => write!(f, "{} {}", self, species),
            }
        } else {
            write!(f, "{}", self)
        }
    }
}

impl fmt::Display for Age {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Infant => write!(f, "infant"),
            Self::Child => write!(f, "child"),
            Self::Adolescent => write!(f, "adolescent"),
            Self::YoungAdult => write!(f, "young adult"),
            Self::Adult => write!(f, "adult"),
            Self::MiddleAged => write!(f, "middle-aged"),
            Self::Elderly => write!(f, "elderly"),
            Self::Geriatric => write!(f, "geriatric"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display_test() {
        assert_eq!("infant", format!("{}", Age::Infant));
        assert_eq!("child", format!("{}", Age::Child));
        assert_eq!("adolescent", format!("{}", Age::Adolescent));
        assert_eq!("young adult", format!("{}", Age::YoungAdult));
        assert_eq!("adult", format!("{}", Age::Adult));
        assert_eq!("middle-aged", format!("{}", Age::MiddleAged));
        assert_eq!("elderly", format!("{}", Age::Elderly));
        assert_eq!("geriatric", format!("{}", Age::Geriatric));
    }

    #[test]
    fn fmt_with_species_test_some_species() {
        let r = Species::Human;

        assert_eq!(
            "human infant",
            format!("{}", TestWrapper(&Age::Infant, Some(&r))),
        );
        assert_eq!(
            "human child",
            format!("{}", TestWrapper(&Age::Child, Some(&r))),
        );
        assert_eq!(
            "adolescent human",
            format!("{}", TestWrapper(&Age::Adolescent, Some(&r))),
        );
        assert_eq!(
            "young adult human",
            format!("{}", TestWrapper(&Age::YoungAdult, Some(&r))),
        );
        assert_eq!(
            "adult human",
            format!("{}", TestWrapper(&Age::Adult, Some(&r))),
        );
        assert_eq!(
            "middle-aged human",
            format!("{}", TestWrapper(&Age::MiddleAged, Some(&r))),
        );
        assert_eq!(
            "elderly human",
            format!("{}", TestWrapper(&Age::Elderly, Some(&r))),
        );
        assert_eq!(
            "geriatric human",
            format!("{}", TestWrapper(&Age::Geriatric, Some(&r))),
        );
    }

    #[test]
    fn fmt_with_species_test_none() {
        assert_eq!("infant", format!("{}", TestWrapper(&Age::Infant, None)));
        assert_eq!("adult", format!("{}", TestWrapper(&Age::Adult, None)));
    }

    #[test]
    fn serialize_deserialize_test() {
        assert_eq!(r#""Adult""#, serde_json::to_string(&Age::Adult).unwrap(),);

        let value: Age = serde_json::from_str(r#""Adult""#).unwrap();
        assert_eq!(Age::Adult, value);
    }

    struct TestWrapper<'a>(&'a Age, Option<&'a Species>);

    impl<'a> fmt::Display for TestWrapper<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.0.fmt_with_species(self.1, f)
        }
    }
}
