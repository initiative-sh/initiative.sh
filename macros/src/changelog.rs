use proc_macro::{Literal, TokenStream, TokenTree};
use std::borrow::Cow;

pub fn run(input: TokenStream) -> Result<TokenStream, String> {
    if !input.is_empty() {
        return Err(format!("Expected no arguments, got `{}`.", input));
    }

    let changelog = include_str!("../../data/changelog.md");

    let top_items = changelog
        .split_inclusive("\n*")
        .take(10)
        .collect::<String>();

    Ok(TokenTree::from(Literal::string(
        linkify(&top_items).trim_end_matches(&['\n', '*'][..]),
    ))
    .into())
}

pub fn linkify(input: &str) -> Cow<str> {
    let re = regex::Regex::new(r"@([\w]+)").unwrap();
    re.replace_all(
        input,
        "<a href=\"https://github.com/$1\" target=\"_blank\">@$1</a>",
    )
}
