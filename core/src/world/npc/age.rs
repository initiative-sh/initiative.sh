use std::fmt;

use super::Species;

#[derive(Clone, Copy, Debug, PartialEq)]
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

    pub fn fmt_with_species(
        &self,
        species: Option<&Species>,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        if let Some(species) = species {
            match self {
                Age::Infant(_) | Age::Child(_) => write!(f, "{} {}", species, self.category()),
                _ => write!(f, "{} {}", self.category(), species),
            }
        } else {
            write!(f, "{}", self.category())
        }
    }
}

#[cfg(test)]
mod test_age {
    use super::*;

    #[test]
    fn years_test() {
        assert_eq!(1, Age::Infant(1).years());
        assert_eq!(2, Age::Child(2).years());
        assert_eq!(3, Age::Adolescent(3).years());
        assert_eq!(4, Age::YoungAdult(4).years());
        assert_eq!(5, Age::Adult(5).years());
        assert_eq!(6, Age::MiddleAged(6).years());
        assert_eq!(7, Age::Elderly(7).years());
        assert_eq!(8, Age::Geriatric(8).years());
    }

    #[test]
    fn category_test() {
        assert_eq!("infant", Age::Infant(1).category());
        assert_eq!("child", Age::Child(2).category());
        assert_eq!("adolescent", Age::Adolescent(3).category());
        assert_eq!("young adult", Age::YoungAdult(4).category());
        assert_eq!("adult", Age::Adult(5).category());
        assert_eq!("middle-aged", Age::MiddleAged(6).category());
        assert_eq!("elderly", Age::Elderly(7).category());
        assert_eq!("geriatric", Age::Geriatric(8).category());
    }

    #[test]
    fn fmt_with_species_test_some_species() {
        let r = Species::Human;

        assert_eq!(
            "human infant",
            format!("{}", TestWrapper(&Age::Infant(1), Some(&r))),
        );
        assert_eq!(
            "human child",
            format!("{}", TestWrapper(&Age::Child(2), Some(&r))),
        );
        assert_eq!(
            "adolescent human",
            format!("{}", TestWrapper(&Age::Adolescent(3), Some(&r))),
        );
        assert_eq!(
            "young adult human",
            format!("{}", TestWrapper(&Age::YoungAdult(4), Some(&r))),
        );
        assert_eq!(
            "adult human",
            format!("{}", TestWrapper(&Age::Adult(5), Some(&r))),
        );
        assert_eq!(
            "middle-aged human",
            format!("{}", TestWrapper(&Age::MiddleAged(6), Some(&r))),
        );
        assert_eq!(
            "elderly human",
            format!("{}", TestWrapper(&Age::Elderly(7), Some(&r))),
        );
        assert_eq!(
            "geriatric human",
            format!("{}", TestWrapper(&Age::Geriatric(8), Some(&r))),
        );
    }

    #[test]
    fn fmt_with_species_test_none() {
        assert_eq!("infant", format!("{}", TestWrapper(&Age::Infant(1), None)));
        assert_eq!("adult", format!("{}", TestWrapper(&Age::Adult(5), None)));
    }

    struct TestWrapper<'a>(&'a Age, Option<&'a Species>);

    impl<'a> fmt::Display for TestWrapper<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.0.fmt_with_species(self.1, f)
        }
    }
}

impl fmt::Display for Age {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({} years)", self.category(), self.years())
    }
}

#[cfg(test)]
mod test_display_for_age {
    use super::*;

    #[test]
    fn fmt_test() {
        assert_eq!("infant (1 years)", format!("{}", Age::Infant(1)));
        assert_eq!("adult (30 years)", format!("{}", Age::Adult(30)));
    }
}
