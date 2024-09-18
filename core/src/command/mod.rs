mod about;

mod token;

/*
use std::iter;
use std::pin::{pin, Pin};

use crate::app::{AppMeta, AutocompleteSuggestion};

use token::Token;

use futures::future::FutureExt;
use futures::stream::{SelectAll, Stream, StreamExt};

trait Command {
    async fn autocomplete(
        self,
        input: &str,
        app_meta: &AppMeta,
    ) -> impl Stream<Item = AutocompleteSuggestion> + Unpin;
}

fn commands() -> impl Iterator<Item = impl Command> {
    [about::About].into_iter()
}

pub async fn autocomplete(input: &str, app_meta: &AppMeta) -> Vec<AutocompleteSuggestion> {
    let mut select = SelectAll::new();
    let mut command_streams = Vec::new();

    for command in commands() {
        let stream = command.autocomplete(input, app_meta).into_stream();
        pin!(stream);
        select.push(stream);
    }

    Vec::new()
}
*/
