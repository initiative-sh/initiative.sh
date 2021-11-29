use super::Colors;
use initiative_macros::Display;

#[derive(Debug, Display, PartialEq)]
#[allow(dead_code)]
struct Words(Vec<Word>);

#[derive(Debug, Display, PartialEq)]
#[allow(dead_code)]
enum Word {
    And,

    #[command(implements(WordList))]
    Color(Colors),

    #[command(syntax = "tastes [flavor]")]
    Flavor {
        flavor: Flavors,
    },
}

#[derive(Debug, Display, PartialEq)]
#[allow(dead_code)]
enum Flavors {
    Bitter,
    Salty,
    Sour,
    Sweet,
}

mod display {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("", Words(Vec::new()).to_string());
        assert_eq!(
            "red tastes sweet and green tastes sour",
            Words(vec![
                Word::Color(Colors::Red),
                Word::Flavor {
                    flavor: Flavors::Sweet,
                },
                Word::And,
                Word::Color(Colors::Green),
                Word::Flavor {
                    flavor: Flavors::Sour,
                },
            ])
            .to_string(),
        );
    }
}
