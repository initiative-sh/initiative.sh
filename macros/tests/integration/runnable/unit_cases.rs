use initiative_core::app::AppMeta;
use initiative_macros::{Autocomplete, ContextAwareParse};
use std::borrow::Cow;

#[derive(Autocomplete, ContextAwareParse, Debug, PartialEq)]
#[allow(dead_code)]
enum Command {
    UnitCase,

    #[command(alias = "an-alias")]
    #[command(syntax = "blah-unit-case")]
    AnotherUnitCase,

    #[command(alias = "alias1")]
    #[command(alias = "alias2")]
    #[command(ignore)]
    AnIgnoredCase,

    #[command(no_default_autocomplete)]
    NoAutocomplete,

    #[command(alias_no_autocomplete = "some-other-autocomplete")]
    #[command(autocomplete_desc = "a description")]
    SomeAutocomplete,

    #[command(autocomplete_desc_fn(describe))]
    FunctionDescribedAutocomplete,
}

fn describe(input: &str, _app_meta: &AppMeta) -> Cow<'static, str> {
    format!(r#"input was "{}""#, input).into()
}

mod parse {
    use super::*;
    use initiative_core::app::ContextAwareParse;
    use tokio_test::block_on;

    #[test]
    fn test_canonical_default() {
        let app_meta = crate::get_app_meta();
        assert_eq!(
            (Some(Command::UnitCase), Vec::new()),
            block_on(Command::parse_input("UNIT-CASE", &app_meta)),
        );
    }

    #[test]
    fn test_canonical_custom_syntax() {
        let app_meta = crate::get_app_meta();
        assert_eq!(
            (Some(Command::AnotherUnitCase), Vec::new()),
            block_on(Command::parse_input("BLAH-UNIT-CASE", &app_meta)),
        );
        assert_eq!(
            (None, Vec::new()),
            block_on(Command::parse_input("ANOTHER-UNIT-CASE", &app_meta)),
        );
    }

    #[test]
    fn test_canonical_ignored() {
        let app_meta = crate::get_app_meta();
        assert_eq!(
            (None, Vec::new()),
            block_on(Command::parse_input("AN-IGNORED-CASE", &app_meta)),
        );
    }

    #[test]
    fn test_no_match() {
        let app_meta = crate::get_app_meta();
        assert_eq!(
            (None, Vec::new()),
            block_on(Command::parse_input("INVALID-INPUT", &app_meta)),
        );
    }

    #[test]
    fn test_alias() {
        let app_meta = crate::get_app_meta();
        assert_eq!(
            (None, vec![Command::AnotherUnitCase]),
            block_on(Command::parse_input("AN-ALIAS", &app_meta)),
        );
        assert_eq!(
            (None, vec![Command::SomeAutocomplete]),
            block_on(Command::parse_input("SOME-OTHER-AUTOCOMPLETE", &app_meta)),
        );
        assert_eq!(
            (None, vec![Command::AnIgnoredCase]),
            block_on(Command::parse_input("ALIAS1", &app_meta)),
        );
        assert_eq!(
            (None, vec![Command::AnIgnoredCase]),
            block_on(Command::parse_input("ALIAS2", &app_meta)),
        );
    }
}

mod autocomplete {
    use super::*;
    use initiative_core::app::Autocomplete;
    use tokio_test::block_on;

    #[test]
    fn test_canonical() {
        let app_meta = crate::get_app_meta();
        assert_eq!(
            vec![(Cow::from("unit-case"), Cow::from("unit-case"))],
            block_on(Command::autocomplete("UNIT", &app_meta)),
        );
        assert_eq!(
            vec![(Cow::from("some-autocomplete"), Cow::from("a description"))],
            block_on(Command::autocomplete("SOME", &app_meta)),
        );
        assert_eq!(
            vec![(
                Cow::from("function-described-autocomplete"),
                Cow::from(r#"input was "FUN""#)
            )],
            block_on(Command::autocomplete("FUN", &app_meta)),
        );
        assert_eq!(
            vec![(Cow::from("blah-unit-case"), Cow::from("blah-unit-case"))],
            block_on(Command::autocomplete("BLAH", &app_meta)),
        );
    }

    #[test]
    fn test_alias() {
        let app_meta = crate::get_app_meta();
        assert_eq!(
            vec![(Cow::from("an-alias"), Cow::from("blah-unit-case"))],
            block_on(Command::autocomplete("AN-", &app_meta)),
        );
    }

    #[test]
    fn test_disabled() {
        let app_meta = crate::get_app_meta();
        assert_eq!(
            Vec::<(Cow<'static, str>, Cow<'static, str>)>::new(),
            block_on(Command::autocomplete("SOME-OTHER-AUTOCOMPLETE", &app_meta)),
        );
        assert_eq!(
            Vec::<(Cow<'static, str>, Cow<'static, str>)>::new(),
            block_on(Command::autocomplete("NO-AUTOCOMPLETE", &app_meta)),
        );
    }
}
