use crate::app::{AppMeta, ContextAwareParse};
use async_trait::async_trait;
use initiative_macros::ContextAwareParse;
use std::str::FromStr;

#[derive(ContextAwareParse, Debug, PartialEq)]
enum TestCommand {
    /// unit-variant
    UnitVariant,

    #[command(syntax = "otherunit-variant")]
    /// other-unit-variant
    OtherUnitVariant,

    #[command(alias = "unit-variant-with-alias")]
    /// aliased-unit-variant
    AliasedUnitVariant,

    TupleVariant(TestSubcommand),

    #[command(alias = "[capital_str]")]
    #[command(alias = "struct variant [capital_str]")]
    StructVariant {
        capital_str: LiterallyAnything,
    },

    #[allow(dead_code)]
    #[command(ignore)]
    IgnoreMe,

    #[allow(dead_code)]
    #[command(ignore)]
    IgnoreMeWithFields {
        field: LiterallyAnything,
    },
    /*
    #[command(syntax = "[subject] is [adjective]")]
    ComplexStructVariant {
        subject: String,
        adjective: String,
    },
    */
}

#[derive(ContextAwareParse, Debug, PartialEq)]
enum TestSubcommand {
    #[command(alias = "yet-another-alias")]
    #[command(alias = "yet another alias")]
    YetAnotherUnitVariant,
}

#[derive(ContextAwareParse, Debug, PartialEq)]
enum AnotherSubcommand {
    Blah,
}

#[derive(Debug, PartialEq)]
struct LiterallyAnything(String);

#[async_trait(?Send)]
impl ContextAwareParse for LiterallyAnything {
    async fn parse_input(input: &str, _app_meta: &AppMeta) -> (Option<Self>, Vec<Self>) {
        (
            if input.starts_with(char::is_uppercase) {
                Some(Self(input.to_string()))
            } else {
                None
            },
            Vec::new(),
        )
    }
}

impl FromStr for LiterallyAnything {
    type Err = std::convert::Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self(input.to_string()))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::app::AppMeta;
    use crate::app::ContextAwareParse;
    use crate::storage::NullDataStore;
    use tokio_test::block_on;

    #[test]
    fn unit_variant_test() {
        let meta = app_meta();

        assert_eq!(
            (Some(TestCommand::UnitVariant), Vec::new()),
            block_on(TestCommand::parse_input("unit-variant", &meta)),
        );

        assert_eq!(
            (Some(TestCommand::OtherUnitVariant), Vec::new()),
            block_on(TestCommand::parse_input("otherunit-variant", &meta)),
        );

        assert_eq!(
            (Some(TestCommand::OtherUnitVariant), Vec::new()),
            block_on(TestCommand::parse_input("otherunit-variant", &meta)),
        );

        assert_eq!(
            (Some(TestCommand::AliasedUnitVariant), Vec::new()),
            block_on(TestCommand::parse_input("aliased-unit-variant", &meta)),
        );

        assert_eq!(
            (None, vec![TestCommand::AliasedUnitVariant]),
            block_on(TestCommand::parse_input("unit-variant-with-alias", &meta)),
        );
    }

    #[test]
    fn tuple_variant_test() {
        let meta = app_meta();

        assert_eq!(
            (
                Some(TestCommand::TupleVariant(
                    TestSubcommand::YetAnotherUnitVariant
                )),
                Vec::new(),
            ),
            block_on(TestCommand::parse_input("yet-another-unit-variant", &meta)),
        );

        assert_eq!(
            (
                None,
                vec![TestCommand::TupleVariant(
                    TestSubcommand::YetAnotherUnitVariant
                )],
            ),
            block_on(TestCommand::parse_input("yet-another-alias", &meta)),
        );
    }

    #[test]
    fn struct_variant_test() {
        let meta = app_meta();

        assert_eq!(
            (
                Some(TestCommand::StructVariant {
                    capital_str: "blah".parse().unwrap(),
                }),
                Vec::new(),
            ),
            block_on(TestCommand::parse_input("struct-variant blah", &meta)),
        );

        assert_eq!(
            (
                Some(TestCommand::StructVariant {
                    capital_str: "Blah".parse().unwrap(),
                }),
                Vec::new(),
            ),
            block_on(TestCommand::parse_input("struct-variant Blah", &meta)),
        );

        assert_eq!(
            (
                None,
                vec![TestCommand::StructVariant {
                    capital_str: "Blah".parse().unwrap(),
                }],
            ),
            block_on(TestCommand::parse_input("struct variant Blah", &meta))
        );

        assert_eq!(
            (
                None,
                vec![TestCommand::StructVariant {
                    capital_str: "Blah".parse().unwrap(),
                }],
            ),
            block_on(TestCommand::parse_input("Blah", &meta)),
        );

        assert_eq!(
            (None, Vec::new()),
            block_on(TestCommand::parse_input("blah", &meta))
        );

        assert_eq!(
            (None, Vec::new()),
            block_on(TestCommand::parse_input("struct variant blah", &meta))
        );
    }

    #[test]
    fn ignored_variant_test() {
        let meta = app_meta();

        assert_eq!(
            (None, Vec::new()),
            block_on(TestCommand::parse_input("ignore-me", &meta)),
        );

        assert_eq!(
            (None, Vec::new()),
            block_on(TestCommand::parse_input(
                "ignore-me-with-fields yet-another-unit-variant",
                &meta
            )),
        );
    }

    fn app_meta() -> AppMeta {
        AppMeta::new(NullDataStore::default(), &null_parser)
    }

    fn null_parser(_event: crate::app::Event) {}
}