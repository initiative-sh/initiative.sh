#[macro_export]
macro_rules! assert_autocomplete_eq {
    ([$(($expected_suggestion: expr, $expected_description: expr)),* $(,)?], $actual: expr $(,)?) => {
        let mut expected: Vec<$crate::app::AutocompleteSuggestion> =
            [$(($expected_suggestion, $expected_description),)*]
            .into_iter()
            .map($crate::app::AutocompleteSuggestion::from)
            .collect();
        expected.sort();

        let mut actual: Vec<$crate::app::AutocompleteSuggestion> = $actual;
        actual.sort();

        assert_eq!(expected, actual);
    };
}

#[macro_export]
macro_rules! assert_autocomplete_empty {
    ($expression: expr $(,)?) => {
        let expected: Vec<$crate::app::AutocompleteSuggestion> = Vec::new();

        let mut actual: Vec<$crate::app::AutocompleteSuggestion> = $expression;
        actual.sort();

        assert_eq!(expected, actual);
    };
}
