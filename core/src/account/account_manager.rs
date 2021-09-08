use async_trait::async_trait;

#[derive(Default)]
pub struct NullAccountManager;

#[async_trait(?Send)]
impl AccountManager for NullAccountManager {
    async fn is_enabled(&self) -> bool {
        false
    }

    async fn signup(&self) -> Option<(String, String, Option<String>)> {
        None
    }

    async fn login(&self, _username: Option<&str>) -> Option<(String, String)> {
        None
    }

    async fn logout(&self) {}
}

#[async_trait(?Send)]
pub trait AccountManager {
    async fn is_enabled(&self) -> bool;

    async fn signup(&self) -> Option<(String, String, Option<String>)>;

    async fn login(&self, username: Option<&str>) -> Option<(String, String)>;

    async fn logout(&self);
}
