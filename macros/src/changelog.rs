use proc_macro::{Literal, TokenStream, TokenTree};

pub fn run(input: TokenStream) -> Result<TokenStream, String> {
    if !input.is_empty() {
        return Err(format!("Expected no arguments, got `{}`.", input));
    }

    let changelog = include_str!("../../data/changelog.md");
    let token_tree: TokenTree = Literal::string(
        changelog
            .split_inclusive('\n')
            .take(10)
            .collect::<String>()
            .trim_end(),
    )
    .into();

    Ok(token_tree.into())
}
