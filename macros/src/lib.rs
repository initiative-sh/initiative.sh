use proc_macro::TokenStream;

mod changelog;
mod motd;
mod reference_enum;
mod word_list;

#[proc_macro]
pub fn changelog(input: TokenStream) -> TokenStream {
    changelog::run(input).unwrap()
}

#[proc_macro]
pub fn motd(input: TokenStream) -> TokenStream {
    motd::run(input).unwrap()
}

#[proc_macro]
pub fn reference_enum(input: TokenStream) -> TokenStream {
    reference_enum::run(input).unwrap()
}

#[proc_macro_derive(WordList)]
pub fn word_list(input: TokenStream) -> TokenStream {
    word_list::run(input).unwrap()
}
