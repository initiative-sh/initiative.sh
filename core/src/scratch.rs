use initiative_macros::ContextAwareParse;

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

    /// struct-variant [field]
    #[command(alias = "struct variant [field]")]
    StructVariant {
        field: TestSubcommand,
    },

    #[allow(dead_code)]
    #[command(ignore)]
    IgnoreMe,

    #[allow(dead_code)]
    #[command(ignore)]
    IgnoreMeWithFields {
        field: TestSubcommand,
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
                    field: TestSubcommand::YetAnotherUnitVariant,
                }),
                Vec::new(),
            ),
            block_on(TestCommand::parse_input(
                "struct-variant yet-another-unit-variant",
                &meta
            )),
        );

        for term in [
            "struct-variant yet another alias",
            "struct variant yet-another-alias",
            "struct variant yet another alias",
        ] {
            assert_eq!(
                (
                    None,
                    vec![TestCommand::StructVariant {
                        field: TestSubcommand::YetAnotherUnitVariant,
                    }],
                ),
                block_on(TestCommand::parse_input(term, &meta)),
                r#"Term: "{}""#,
                term,
            );
        }
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
