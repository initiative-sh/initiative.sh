use proc_macro::TokenStream;
use quote::quote;

pub fn run(input: TokenStream) -> Result<TokenStream, String> {
    if !input.is_empty() {
        return Err(format!("Expected no arguments, got `{}`.", input));
    }

    let changelog = include_str!("../../data/changelog.md");
    let base_motd = include_str!("../../data/motd.md");

    let motd = format!(
        "\
{}

Latest `changelog` entry:

{}",
        base_motd.trim_end(),
        changelog
            .lines()
            .enumerate()
            .take_while(|&(i, s)| i == 0 || !s.starts_with('*'))
            .map(|(_, s)| s)
            .collect::<String>()
    );

    Ok(quote! { #motd }.into())
}
