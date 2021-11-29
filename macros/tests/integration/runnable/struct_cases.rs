use super::Colors;
use initiative_macros::{Autocomplete, ContextAwareParse, Display};

#[derive(ContextAwareParse, Debug, Display, PartialEq)]
#[allow(dead_code)]
enum Command {
    #[command(alias = "colour [color]")]
    Color {
        #[command(implements(WordList))]
        color: Colors,
    },

    #[command(alias = "sub [command]")]
    #[command(autocomplete_desc = "torpedoes away")]
    Subcommand { command: Subcommand },
}

#[derive(Autocomplete, ContextAwareParse, Debug, Display, PartialEq)]
#[allow(dead_code)]
enum CommandWithoutWordList {
    #[command(alias = "sub [command]")]
    #[command(autocomplete_desc = "torpedoes away")]
    Subcommand { command: Subcommand },
}

#[derive(Autocomplete, ContextAwareParse, Debug, Display, PartialEq)]
#[allow(dead_code)]
enum Subcommand {
    #[command(alias = "blah-alias")]
    Blah,
}

mod parse {
    use super::*;
    use initiative_core::app::ContextAwareParse;
    use tokio_test::block_on;

    #[test]
    fn test_canonical() {
        let app_meta = crate::get_app_meta();
        assert_eq!(
            (
                Some(Command::Color {
                    color: Colors::Blue,
                }),
                Vec::new(),
            ),
            block_on(Command::parse_input("COLOR BLUE", &app_meta)),
        );
        assert_eq!(
            (
                Some(Command::Subcommand {
                    command: Subcommand::Blah,
                }),
                Vec::new(),
            ),
            block_on(Command::parse_input("SUBCOMMAND BLAH", &app_meta)),
        );
    }

    #[test]
    fn test_alias() {
        let app_meta = crate::get_app_meta();
        assert_eq!(
            (
                None,
                vec![Command::Color {
                    color: Colors::Blue,
                }],
            ),
            block_on(Command::parse_input("COLOUR BLUE", &app_meta)),
        );
        assert_eq!(
            (
                None,
                vec![Command::Subcommand {
                    command: Subcommand::Blah,
                }],
            ),
            block_on(Command::parse_input("SUB BLAH", &app_meta)),
        );
        assert_eq!(
            (
                None,
                vec![Command::Subcommand {
                    command: Subcommand::Blah,
                }],
            ),
            block_on(Command::parse_input("SUBCOMMAND BLAH-ALIAS", &app_meta)),
        );
        assert_eq!(
            (None, Vec::new()),
            block_on(Command::parse_input("sub blah-alias", &app_meta)),
        );
    }
}

mod autocomplete {
    use super::*;
    use initiative_core::app::Autocomplete;
    use std::borrow::Cow;
    use tokio_test::block_on;

    #[test]
    fn test_subcommand() {
        let app_meta = crate::get_app_meta();
        assert_eq!(
            vec![
                (
                    Cow::from("subcommand [command]"),
                    Cow::from("torpedoes away"),
                ),
                (Cow::from("sub [command]"), Cow::from("torpedoes away")),
            ],
            block_on(CommandWithoutWordList::autocomplete("sub", &app_meta, true)),
        );
        assert_eq!(
            vec![(Cow::from("sub blah"), Cow::from("torpedoes away")),],
            block_on(CommandWithoutWordList::autocomplete(
                "sub ", &app_meta, true
            )),
        );
        assert_eq!(
            vec![
                (Cow::from("subcommand blah"), Cow::from("torpedoes away")),
                (
                    Cow::from("subcommand blah-alias"),
                    Cow::from("torpedoes away"),
                ),
            ],
            block_on(CommandWithoutWordList::autocomplete(
                "subcommand ",
                &app_meta,
                true,
            )),
        );
    }
}

mod display {
    use super::*;

    #[test]
    fn test_runnable() {
        assert_eq!(
            "subcommand blah",
            Command::Subcommand {
                command: Subcommand::Blah,
            }
            .to_string(),
        );
    }

    #[test]
    fn test_color() {
        assert_eq!(
            "color blue",
            Command::Color {
                color: Colors::Blue,
            }
            .to_string(),
        );
    }
}
