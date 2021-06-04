use initiative_macros::WordList;

#[derive(Debug, PartialEq, WordList)]
pub enum Verb {
    Tutorial,
    Help,
    Quit,
}

#[cfg(test)]
mod test_verb {
    use super::Verb;

    #[test]
    fn from_str_test() {
        assert_eq!(Ok(Verb::Help), "help".parse::<Verb>());
        assert_eq!(Err(()), "potato".parse::<Verb>());
    }

    #[test]
    fn into_string_test() {
        assert_eq!("help", String::from(Verb::Help).as_str());
    }
}

#[derive(Clone, Copy, Debug, PartialEq, WordList)]
pub enum Noun {
    Building,
    Dragonborn,
    Dwarf,
    Elf,
    Gnome,
    HalfElf,
    HalfOrc,
    Halfling,
    Human,
    Inn,
    Npc,
    Residence,
    Shop,
    Temple,
    Tiefling,
    Warehouse,
    Warforged,
}

#[cfg(test)]
mod test_noun {
    use super::Noun;

    #[test]
    fn from_str_test() {
        assert_eq!(Ok(Noun::Inn), "inn".parse::<Noun>());
        assert_eq!(Ok(Noun::HalfElf), "half-elf".parse::<Noun>());
        assert_eq!(Ok(Noun::HalfElf), "half elf".parse::<Noun>());
        assert_eq!(Err(()), "potato".parse::<Noun>());
    }

    #[test]
    fn into_string_test() {
        assert_eq!("inn", String::from(Noun::Inn).as_str());
        assert_eq!("half-elf", String::from(Noun::HalfElf).as_str());
    }
}
