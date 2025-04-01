use super::{Ethnicity, Species};
use initiative_macros::WordList;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, WordList)]
#[serde(into = "&'static str", try_from = "&str")]
pub enum Age {
    #[alias = "baby"]
    Infant,

    #[alias = "boy"]
    #[alias = "girl"]
    Child,

    #[alias = "teenage"]
    #[alias = "teenager"]
    Adolescent,

    #[alias = "young"]
    #[alias = "young adult"]
    YoungAdult,

    #[alias = "man"]
    #[alias = "woman"]
    Adult,

    #[alias = "middle aged"]
    MiddleAged,

    #[alias = "old"]
    Elderly,

    #[alias = "feeble"]
    #[alias = "ancient"]
    #[alias = "wizened"]
    Geriatric,
}

impl Age {
    pub fn fmt_with_species_ethnicity(
        &self,
        species: Option<&Species>,
        ethnicity: Option<&Ethnicity>,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        if let Some(species) = species {
            match self {
                Age::Infant | Age::Child => write!(f, "{} {}", species, self),
                _ => write!(f, "{} {}", self, species),
            }
        } else if let Some(ethnicity) = ethnicity {
            match self {
                Age::MiddleAged | Age::Elderly | Age::Geriatric => {
                    write!(f, "{} {} person", self, ethnicity)
                }
                _ => write!(f, "{} {}", ethnicity, self),
            }
        } else {
            match self {
                Age::MiddleAged | Age::Elderly | Age::Geriatric => write!(f, "{} person", self),
                _ => write!(f, "{}", self),
            }
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
        let cases = [
            ("infant", Age::Infant),
            ("child", Age::Child),
            ("adolescent", Age::Adolescent),
            ("young adult", Age::YoungAdult),
            ("adult", Age::Adult),
            ("middle-aged", Age::MiddleAged),
            ("elderly", Age::Elderly),
            ("geriatric", Age::Geriatric),
        ];

        for (age_str, age) in cases {
            assert_eq!(age_str, format!("{}", age));
            assert_eq!(Ok(age), format!("{}", age).parse::<Age>());
        }
    }

    #[test]
    fn from_str_test() {
        assert_eq!(Ok(Age::Infant), "infant".parse::<Age>());
        assert_eq!(Ok(Age::Infant), "baby".parse::<Age>());

        assert_eq!(Ok(Age::Child), "child".parse::<Age>());
        assert_eq!(Ok(Age::Child), "boy".parse::<Age>());
        assert_eq!(Ok(Age::Child), "girl".parse::<Age>());

        assert_eq!(Ok(Age::Adolescent), "adolescent".parse::<Age>());
        assert_eq!(Ok(Age::Adolescent), "teenage".parse::<Age>());
        assert_eq!(Ok(Age::Adolescent), "teenager".parse::<Age>());

        assert_eq!(Ok(Age::YoungAdult), "young".parse::<Age>());
        assert_eq!(Ok(Age::YoungAdult), "young adult".parse::<Age>());
        assert_eq!(Ok(Age::YoungAdult), "young-adult".parse::<Age>());

        assert_eq!(Ok(Age::Adult), "adult".parse::<Age>());

        assert_eq!(Ok(Age::MiddleAged), "middle aged".parse::<Age>());
        assert_eq!(Ok(Age::MiddleAged), "middle-aged".parse::<Age>());

        assert_eq!(Ok(Age::Elderly), "elderly".parse::<Age>());
        assert_eq!(Ok(Age::Elderly), "old".parse::<Age>());

        assert_eq!(Ok(Age::Geriatric), "geriatric".parse::<Age>());
        assert_eq!(Ok(Age::Geriatric), "feeble".parse::<Age>());
        assert_eq!(Ok(Age::Geriatric), "ancient".parse::<Age>());
        assert_eq!(Ok(Age::Geriatric), "wizened".parse::<Age>());

        assert_eq!(Err(()), "potato".parse::<Age>());
    }

    #[test]
    fn fmt_with_species_test_some_species() {
        let s = Species::Elf;

        assert_eq!(
            "elf infant",
            format!("{}", TestWrapper(&Age::Infant, Some(&s), None)),
        );
        assert_eq!(
            "elf child",
            format!("{}", TestWrapper(&Age::Child, Some(&s), None)),
        );
        assert_eq!(
            "adolescent elf",
            format!("{}", TestWrapper(&Age::Adolescent, Some(&s), None)),
        );
        assert_eq!(
            "young adult elf",
            format!("{}", TestWrapper(&Age::YoungAdult, Some(&s), None)),
        );
        assert_eq!(
            "adult elf",
            format!("{}", TestWrapper(&Age::Adult, Some(&s), None)),
        );
        assert_eq!(
            "middle-aged elf",
            format!("{}", TestWrapper(&Age::MiddleAged, Some(&s), None)),
        );
        assert_eq!(
            "elderly elf",
            format!("{}", TestWrapper(&Age::Elderly, Some(&s), None)),
        );
        assert_eq!(
            "geriatric elf",
            format!("{}", TestWrapper(&Age::Geriatric, Some(&s), None)),
        );
    }

    #[test]
    fn fmt_with_species_test_some_ethnicity() {
        let e = Ethnicity::Elvish;

        assert_eq!(
            "elvish infant",
            format!("{}", TestWrapper(&Age::Infant, None, Some(&e))),
        );
        assert_eq!(
            "elvish child",
            format!("{}", TestWrapper(&Age::Child, None, Some(&e))),
        );
        assert_eq!(
            "elvish adolescent",
            format!("{}", TestWrapper(&Age::Adolescent, None, Some(&e))),
        );
        assert_eq!(
            "elvish young adult",
            format!("{}", TestWrapper(&Age::YoungAdult, None, Some(&e))),
        );
        assert_eq!(
            "elvish adult",
            format!("{}", TestWrapper(&Age::Adult, None, Some(&e))),
        );
        assert_eq!(
            "middle-aged elvish person",
            format!("{}", TestWrapper(&Age::MiddleAged, None, Some(&e))),
        );
        assert_eq!(
            "elderly elvish person",
            format!("{}", TestWrapper(&Age::Elderly, None, Some(&e))),
        );
        assert_eq!(
            "geriatric elvish person",
            format!("{}", TestWrapper(&Age::Geriatric, None, Some(&e))),
        );
    }

    #[test]
    fn fmt_with_species_test_none() {
        assert_eq!(
            "elf infant",
            format!(
                "{}",
                TestWrapper(&Age::Infant, Some(&Species::Elf), Some(&Ethnicity::Human)),
            ),
        );

        assert_eq!(
            "infant",
            format!("{}", TestWrapper(&Age::Infant, None, None)),
        );
        assert_eq!("adult", format!("{}", TestWrapper(&Age::Adult, None, None)));
    }

    #[test]
    fn serialize_deserialize_test() {
        assert_eq!(r#""adult""#, serde_json::to_string(&Age::Adult).unwrap());

        let value: Age = serde_json::from_str(r#""adult""#).unwrap();
        assert_eq!(Age::Adult, value);
    }

    struct TestWrapper<'a>(&'a Age, Option<&'a Species>, Option<&'a Ethnicity>);

    impl fmt::Display for TestWrapper<'_> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.0.fmt_with_species_ethnicity(self.1, self.2, f)
        }
    }
}
