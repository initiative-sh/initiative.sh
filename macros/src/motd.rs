use super::changelog::linkify;
use proc_macro::TokenStream;
use quote::quote;
use syn::LitStr;

pub fn run(input: TokenStream) -> Result<TokenStream, String> {
    let message = parse_args(input)?;

    let changelog = include_str!("../../data/changelog.md");
    let base_motd = include_str!("../../data/motd.md");

    let mut motd = format!(
        "\
{}

Latest `changelog` entry:

{}",
        base_motd.trim_end(),
        linkify(
            &changelog
                .lines()
                .enumerate()
                .take_while(|&(i, s)| i == 0 || !s.starts_with('*'))
                .map(|(_, s)| s)
                .collect::<String>()
        )
    );

    let motd_len = motd.len();

    if let Some(message) = message {
        motd.push_str(&format!("\n\n{}", message));
    }

    Ok(quote! { ( #motd, #motd_len ) }.into())
}

fn parse_args(input: TokenStream) -> Result<Option<String>, String> {
    let mut input_iter = input.into_iter();

    match (input_iter.next(), input_iter.next()) {
        (None, _) => Ok(None),
        (Some(tree), None) => {
            let lit_str: syn::Result<LitStr> = syn::parse(tree.into());
            lit_str
                .map(|s| Some(s.value()))
                .map_err(|e| format!("{}", e))
        }
        (Some(_), Some(thing)) => Err(format!("Found unexpected token: {}", thing)),
    }
}
