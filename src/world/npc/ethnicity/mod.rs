use super::Race;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Ethnicity {
    Arabic,
    Celtic,
    Chinese,
    Egyptian,
    English,
    French,
    German,
    Greek,
    Indian,
    Japanese,
    Mesoamerican,
    NigerCongo,
    Norse,
    Polynesian,
    Roman,
    Slavic,
    Spanish,
    Warforged,
}

impl Ethnicity {
    fn default_race(&self) -> Race {
        match self {
            Self::Arabic
            | Self::Celtic
            | Self::Chinese
            | Self::Egyptian
            | Self::English
            | Self::French
            | Self::German
            | Self::Greek
            | Self::Indian
            | Self::Japanese
            | Self::Mesoamerican
            | Self::NigerCongo
            | Self::Norse
            | Self::Polynesian
            | Self::Roman
            | Self::Slavic
            | Self::Spanish => Race::Human,
            Self::Warforged => Race::Warforged,
        }
    }
}

#[cfg(test)]
mod test_ethnicity {
    use super::*;

    #[test]
    fn default_race_test() {
        assert_eq!(Race::Human, Ethnicity::Arabic.default_race());
        assert_eq!(Race::Warforged, Ethnicity::Warforged.default_race());
    }
}
