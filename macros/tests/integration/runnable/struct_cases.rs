use initiative_macros::{ContextAwareParse, WordList};

#[derive(ContextAwareParse, Debug, PartialEq)]
#[allow(dead_code)]
enum Command {
    #[command(alias = "colour [color]")]
    Color {
        #[command(implements(WordList))]
        color: Colors,
    },
}

#[derive(Debug, PartialEq, WordList)]
#[allow(dead_code)]
enum Colors {
    Black,
    Blue,
    Green,
    Orange,
    Purple,
    Red,
    White,
    Yellow,
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
    }
}

mod autocomplete {
    // TODO
}
