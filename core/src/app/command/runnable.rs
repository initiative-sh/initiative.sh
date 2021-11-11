use crate::app::AppMeta;
use async_trait::async_trait;

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
    async fn autocomplete(input: &str, app_meta: &AppMeta) -> Vec<(String, String)>;
}
