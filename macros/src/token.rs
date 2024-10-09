use proc_macro::TokenStream;
use quote::quote;

pub fn run(input: TokenStream) -> Result<TokenStream, String> {
    panic!("{:?}", input);
    let ast = syn::parse(input).map_err(|e| format!("{}", e))?;
    token(&ast)
}

fn token(ast: &syn::DeriveInput) -> Result<TokenStream, String> {
    panic!("{:?}", ast);
}
