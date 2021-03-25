use initiative_macros::WordList;

#[derive(Clone, Copy, Debug, PartialEq, WordList)]
pub enum Noun {
    Building,
    Dragonborn,
    Dwarf,
    Elf,
    HalfElf,
    Halfling,
    Human,
    Inn,
    Npc,
    Residence,
    Shop,
    Temple,
    Warehouse,
    Warforged,
}

#[cfg(test)]
mod test_noun {
    use super::Noun;

    #[test]
    fn from_str_test() {
        assert_eq!(Ok(Noun::Inn), "inn".parse::<Noun>());
        assert_eq!(Err(()), "potato".parse::<Noun>());
    }

    #[test]
    fn into_string_test() {
        assert_eq!("inn", String::from(Noun::Inn).as_str());
    }
}
