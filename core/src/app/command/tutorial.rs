use super::Runnable;
use crate::app::AppMeta;
use async_trait::async_trait;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum TutorialCommand {
    Introduction,
}

#[async_trait(?Send)]
impl Runnable for TutorialCommand {
    async fn run(&self, _input: &str, _app_meta: &mut AppMeta) -> Result<String, String> {
        if matches!(self, TutorialCommand::Introduction) {
            Ok(include_str!("../../../../data/tutorial/00-intro.md").to_string())
        } else {
            unreachable!();
        }
    }

    fn parse_input(input: &str, _app_meta: &AppMeta) -> (Option<Self>, Vec<Self>) {
        (
            if input == "tutorial" {
                Some(TutorialCommand::Introduction)
            } else {
                None
            },
            Vec::new(),
        )
    }

    fn autocomplete(input: &str, _app_meta: &AppMeta) -> Vec<(String, String)> {
        if "tutorial".starts_with(input) {
            vec![("tutorial".to_string(), "feature walkthrough".to_string())]
        } else {
            Vec::new()
        }
    }
}

impl fmt::Display for TutorialCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Introduction => write!(f, "tutorial"),
        }
    }
}
