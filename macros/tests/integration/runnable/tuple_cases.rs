use initiative_macros::ContextAwareParse;

#[derive(ContextAwareParse, Debug, PartialEq)]
#[allow(dead_code)]
enum Command {
    Subcommand(Subcommand),
}

#[derive(ContextAwareParse, Debug, PartialEq)]
#[allow(dead_code)]
enum Subcommand {
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
    // TODO
}
