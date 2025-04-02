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
macro_rules! assert_eq_unordered {
    ($left:expr, $right:expr $(,)?) => {{
        let left: Vec<_> = $left.into();
        let mut right: Vec<_> = $right.into();

        for left_item in &left {
            let Some(index) = right.iter().position(|right_item| right_item == left_item) else {
                panic!(
                    "Not found in right collection: {:?}\n\nLeft:  {:?}\nRight: {:?}",
                    left_item, left, $right,
                );
            };
            right.swap_remove(index);
        }

        if let Some(right_item) = right.first() {
            panic!(
                "Not found in left collection: {:?}\n\nLeft:  {:?}\nRight: {:?}",
                right_item, left, $right,
            );
        }
    }};
}

#[macro_export]
macro_rules! assert_empty {
    ($expr:expr $(,)?) => {
        assert!($expr.is_empty(), "Expected empty value, got:\n{:?}", $expr);
    };
}
