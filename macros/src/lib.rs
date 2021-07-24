use proc_macro::TokenStream;

mod word_list;

#[proc_macro_derive(WordList)]
pub fn word_list(input: TokenStream) -> TokenStream {
    word_list::run(input).unwrap()
}
