use crate::app::{autocomplete_phrase, AppMeta, Runnable};
use async_trait::async_trait;
use initiative_macros::WordList;

#[derive(Clone, Debug, PartialEq, WordList)]
pub enum AccountCommand {
    Login,
    Logout,
    Signup,
}

impl AccountCommand {
    fn summarize(&self) -> &'static str {
        match self {
            Self::Login => "log in to your account",
            Self::Logout => "end your session",
            Self::Signup => "create a new account",
        }
    }
}

#[async_trait(?Send)]
impl Runnable for AccountCommand {
    async fn run(&self, _app_meta: &mut AppMeta) -> Result<String, String> {
        todo!();
    }

    fn parse_input(input: &str, _app_meta: &AppMeta) -> Vec<Self> {
        input.parse().map(|c| vec![c]).unwrap_or_default()
    }

    fn autocomplete(input: &str, _app_meta: &AppMeta) -> Vec<(String, String)> {
        autocomplete_phrase(input, &mut Self::get_words().iter())
            .drain(..)
            .filter_map(|s| {
                s.parse::<Self>()
                    .ok()
                    .map(|c| (s, c.summarize().to_string()))
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::account::NullAccountManager;
    use crate::storage::NullDataStore;

    #[test]
    fn summarize_test() {
        assert_eq!("log in to your account", AccountCommand::Login.summarize());
        assert_eq!("end your session", AccountCommand::Logout.summarize());
        assert_eq!("create a new account", AccountCommand::Signup.summarize());
    }

    #[test]
    fn parse_input_test() {
        let app_meta = AppMeta::new(NullDataStore::default(), NullAccountManager::default());

        assert_eq!(
            vec![AccountCommand::Signup],
            AccountCommand::parse_input("signup", &app_meta),
        );

        assert_eq!(
            Vec::<AccountCommand>::new(),
            AccountCommand::parse_input("potato", &app_meta),
        );
    }

    #[test]
    fn autocomplete_test() {
        let app_meta = AppMeta::new(NullDataStore::default(), NullAccountManager::default());

        [
            ("login", "log in to your account"),
            ("logout", "end your session"),
            ("signup", "create a new account"),
        ]
        .iter()
        .for_each(|(word, summary)| {
            assert_eq!(
                vec![(word.to_string(), summary.to_string())],
                AccountCommand::autocomplete(word, &app_meta),
            )
        });
    }
}
