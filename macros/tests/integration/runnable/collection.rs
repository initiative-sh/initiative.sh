use super::Colors;
use initiative_macros::{ContextAwareParse, Display};

#[derive(ContextAwareParse, Debug, Display, PartialEq)]
#[allow(dead_code)]
struct Words(Vec<Word>);

#[derive(ContextAwareParse, Debug, Display, PartialEq)]
#[allow(dead_code)]
struct WordsResult(Vec<Result<Word, String>>);

#[derive(ContextAwareParse, Debug, Display, PartialEq)]
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

#[derive(ContextAwareParse, Debug, Display, PartialEq)]
#[allow(dead_code)]
enum Flavors {
    Bitter,
    Salty,
    Sour,
    Sweet,
}

mod parse {
    use super::*;
    use initiative_core::app::ContextAwareParse;
    use tokio_test::block_on;

    #[test]
    fn test_word_list() {
        let app_meta = crate::get_app_meta();
        assert_eq!(
            (
                Some(Words(vec![
                    Word::Color(Colors::Red),
                    Word::Color(Colors::Orange),
                    Word::Color(Colors::Yellow),
                ])),
                Vec::new(),
            ),
            block_on(Words::parse_input("RED ORANGE YELLOW", &app_meta)),
        );
    }

    #[test]
    fn test_subcommand() {
        let app_meta = crate::get_app_meta();
        assert_eq!(
            (
                Some(Words(vec![
                    Word::Flavor {
                        flavor: Flavors::Bitter,
                    },
                    Word::Flavor {
                        flavor: Flavors::Salty,
                    },
                    Word::Flavor {
                        flavor: Flavors::Sweet,
                    },
                ])),
                Vec::new(),
            ),
            block_on(Words::parse_input(
                "TASTES BITTER TASTES SALTY TASTES SWEET",
                &app_meta,
            )),
        );
    }

    #[test]
    fn test_empty() {
        let app_meta = crate::get_app_meta();
        assert_eq!(
            (None, Vec::new()),
            block_on(Words::parse_input("", &app_meta)),
        );
    }

    #[test]
    fn test_unknown_word() {
        let app_meta = crate::get_app_meta();
        assert_eq!(
            (None, Vec::new()),
            block_on(Words::parse_input("red octarine", &app_meta)),
        );
        assert_eq!(
            (
                Some(Words(vec![
                    Word::Color(Colors::Red),
                    Word::Color(Colors::Blue),
                ])),
                Vec::new(),
            ),
            block_on(Words::parse_input("red octarine blue", &app_meta)),
        );
    }

    #[test]
    fn test_unknown_word_result() {
        let app_meta = crate::get_app_meta();
        assert_eq!(
            (
                None,
                vec![WordsResult(vec![
                    Ok(Word::Color(Colors::Red)),
                    Err("octarine".to_string()),
                ])],
            ),
            block_on(WordsResult::parse_input("red octarine", &app_meta)),
        );
        assert_eq!(
            (
                Some(WordsResult(vec![
                    Ok(Word::Color(Colors::Red)),
                    Err("octarine".to_string()),
                    Ok(Word::Color(Colors::Blue)),
                ])),
                Vec::new(),
            ),
            block_on(WordsResult::parse_input("red octarine blue", &app_meta)),
        );
    }

    #[test]
    fn test_comprehensive() {
        let app_meta = crate::get_app_meta();
        assert_eq!(
            (
                Some(Words(vec![
                    Word::Color(Colors::Red),
                    Word::Flavor {
                        flavor: Flavors::Sweet,
                    },
                    Word::And,
                    Word::Color(Colors::Green),
                    Word::Flavor {
                        flavor: Flavors::Sour,
                    },
                ])),
                Vec::new(),
            ),
            block_on(Words::parse_input(
                "red tastes sweet and green tastes sour",
                &app_meta,
            )),
        );
    }
}

mod display {
    use super::*;

    #[test]
    fn test_simple() {
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

    #[test]
    fn test_with_result() {
        assert_eq!(
            "red tastes sweet but green tastes sour",
            WordsResult(vec![
                Ok(Word::Color(Colors::Red)),
                Ok(Word::Flavor {
                    flavor: Flavors::Sweet,
                }),
                Err("but".to_string()),
                Ok(Word::Color(Colors::Green)),
                Ok(Word::Flavor {
                    flavor: Flavors::Sour,
                }),
            ])
            .to_string(),
        );
    }
}
