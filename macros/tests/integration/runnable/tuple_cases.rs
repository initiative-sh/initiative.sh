use initiative_macros::{Autocomplete, ContextAwareParse};

#[derive(Autocomplete, ContextAwareParse, Debug, PartialEq)]
#[allow(dead_code)]
enum Command {
    Subcommand(Subcommand),
}

#[derive(Autocomplete, ContextAwareParse, Debug, PartialEq)]
#[allow(dead_code)]
enum Subcommand {
    #[command(autocomplete_desc = "describe me like one of your French girls")]
    OneThing,

    #[command(alias = "alias")]
    Another,
}

mod parse {
    use super::*;
    use initiative_core::app::ContextAwareParse;
    use tokio_test::block_on;

    #[test]
    fn test_subcommand() {
        let app_meta = crate::get_app_meta();
        assert_eq!(
            (Some(Command::Subcommand(Subcommand::OneThing)), Vec::new()),
            block_on(Command::parse_input("ONE-THING", &app_meta)),
        );
        assert_eq!(
            (Some(Command::Subcommand(Subcommand::Another)), Vec::new()),
            block_on(Command::parse_input("ANOTHER", &app_meta)),
        );
        assert_eq!(
            (None, vec![Command::Subcommand(Subcommand::Another)]),
            block_on(Command::parse_input("ALIAS", &app_meta)),
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
            vec![(
                Cow::from("one-thing"),
                Cow::from("describe me like one of your French girls"),
            )],
            block_on(Command::autocomplete("ONE", &app_meta, true)),
        );
        assert_eq!(
            vec![
                (Cow::from("another"), Cow::from("another")),
                (Cow::from("alias"), Cow::from("another")),
            ],
            block_on(Command::autocomplete("A", &app_meta, true)),
        );
    }
}
