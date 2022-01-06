use proc_macro::TokenStream;

mod changelog;
mod motd;
mod reference_enum;
mod runnable;
mod uuid;
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

#[proc_macro]
pub fn uuid(input: TokenStream) -> TokenStream {
    uuid::run(input).unwrap()
}

#[proc_macro_derive(WordList, attributes(alias, term))]
pub fn word_list(input: TokenStream) -> TokenStream {
    word_list::run(input).unwrap()
}

#[proc_macro_derive(ContextAwareParse, attributes(command, doc))]
pub fn context_aware_parse(input: TokenStream) -> TokenStream {
    runnable::context_aware_parse::run(input.into())
        .unwrap()
        .into()
}

#[proc_macro_derive(Autocomplete, attributes(command, doc))]
pub fn autocomplete(input: TokenStream) -> TokenStream {
    runnable::autocomplete::run(input.into()).unwrap().into()
}

#[proc_macro_derive(Display, attributes(command, doc))]
pub fn display(input: TokenStream) -> TokenStream {
    runnable::display::run(input.into()).unwrap().into()
}
