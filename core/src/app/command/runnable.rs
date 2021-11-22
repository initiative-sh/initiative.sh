use crate::app::AppMeta;
use async_trait::async_trait;
use std::borrow::Cow;

#[async_trait(?Send)]
pub trait Runnable: Sized {
    async fn run(self, input: &str, app_meta: &mut AppMeta) -> Result<String, String>;
}

#[async_trait(?Send)]
pub trait ContextAwareParse: Sized {
    async fn parse_input(input: &str, app_meta: &AppMeta) -> (Option<Self>, Vec<Self>);
}

#[async_trait(?Send)]
pub trait Autocomplete {
    async fn autocomplete(
        input: &str,
        app_meta: &AppMeta,
    ) -> Vec<(Cow<'static, str>, Cow<'static, str>)>;
}

#[cfg(test)]
pub fn assert_autocomplete(
    expected_suggestions: &[(&'static str, &'static str)],
    actual_suggestions: Vec<(Cow<'static, str>, Cow<'static, str>)>,
) {
    let mut expected: Vec<_> = expected_suggestions
        .into_iter()
        .map(|(a, b)| ((*a).into(), (*b).into()))
        .collect();
    expected.sort();

    let mut actual = actual_suggestions;
    actual.sort();

    assert_eq!(expected, actual);
}
