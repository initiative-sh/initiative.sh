use proc_macro2::TokenStream;
use quote::quote;

pub fn run(input: TokenStream) -> Result<TokenStream, String> {
    let ast = syn::parse2(input).map_err(|e| e.to_string())?;
    impl_command_list(&ast)
}

fn impl_command_list(ast: &syn::DeriveInput) -> Result<TokenStream, String> {
    let syn::Data::Enum(syn::DataEnum { variants, .. }) = &ast.data else {
        return Err("TokenMarker can only be derived on enums.".to_string());
    };

    let ident = &ast.ident;

    if let Some(variant) = variants.iter().find(|variant| {
        if let syn::Fields::Unnamed(field) = &variant.fields {
            field.unnamed.len() != 1
        } else {
            true
        }
    }) {
        return Err(format!(
            "{}::{}: CommandList enums must have only tuple variants with a single field",
            ident, variant.ident
        ));
    }

    let (get_all_items, match_items): (Vec<_>, Vec<_>) = variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            let syn::Fields::Unnamed(field) = &variant.fields else {
                unreachable!();
            };

            (
                quote! { #ident::#variant_ident(#field), },
                quote! { #ident::#variant_ident(c) },
            )
        })
        .unzip();

    Ok(quote! {
        impl #ident {
            const fn get_all() -> &'static [#ident] {
                &[
                    #(#get_all_items)*
                ]
            }
        }

        impl Command for #ident {
            fn token(&self) -> Token {
                match self {
                    #(#match_items => c.token(),)*
                }
            }

            fn autocomplete(&self, fuzzy_match: FuzzyMatch, input: &str) -> Option<AutocompleteSuggestion> {
                match self {
                    #(#match_items => c.autocomplete(fuzzy_match, input),)*
                }
            }

            fn get_priority(&self, token_match: &TokenMatch) -> Option<CommandPriority> {
                match self {
                    #(#match_items => c.get_priority(token_match),)*
                }
            }

            fn get_canonical_form_of(&self, token_match: &TokenMatch) -> Option<String> {
                match self {
                    #(#match_items => c.get_canonical_form_of(token_match),)*
                }
            }

            async fn run<'a>(
                &self,
                token_match: TokenMatch<'a>,
                app_meta: &mut AppMeta,
            ) -> Result<String, String> {
                match self {
                    #(#match_items => c.run(token_match, app_meta).await,)*
                }
            }
        }
    })
}
