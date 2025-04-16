use proc_macro2::TokenStream;
use quote::quote;

pub fn run(input: TokenStream) -> Result<TokenStream, String> {
    let ast = syn::parse2(input).map_err(|e| e.to_string())?;
    impl_command_list(&ast)
}

fn impl_command_list(ast: &syn::DeriveInput) -> Result<TokenStream, String> {
    let syn::Data::Enum(syn::DataEnum { variants, .. }) = &ast.data else {
        return Err("CommandList can only be derived on enums.".to_string());
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
            let syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed: field, .. }) = &variant.fields
            else {
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
                    #( #get_all_items )*
                ]
            }
        }

        impl Command for #ident {
            fn token(&self) -> Token {
                match self {
                    #( #match_items => c.token(), )*
                }
            }

            fn autocomplete(&self, fuzzy_match: FuzzyMatch, input: &str) -> Option<AutocompleteSuggestion> {
                match self {
                    #( #match_items => c.autocomplete(fuzzy_match, input), )*
                }
            }

            fn get_priority(&self, token_match: &TokenMatch) -> Option<CommandPriority> {
                match self {
                    #( #match_items => c.get_priority(token_match), )*
                }
            }

            fn get_canonical_form_of(&self, token_match: &TokenMatch) -> Option<String> {
                match self {
                    #( #match_items => c.get_canonical_form_of(token_match), )*
                }
            }

            async fn run<'a>(
                &self,
                token_match: TokenMatch<'a>,
                app_meta: &mut AppMeta,
            ) -> Result<impl ::std::fmt::Display, impl ::std::fmt::Display> {
                match self {
                    #(
                        #match_items => c.run(token_match, app_meta)
                            .await
                            .map(|s| s.to_string())
                            .map_err(|e| e.to_string()),
                    )*
                }
            }
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn success_test() {
        assert_eq!(
            quote! {
                impl CommandList {
                    const fn get_all() -> &'static [CommandList] {
                        &[
                            CommandList::About(about::About),
                            CommandList::Create(create::Create),
                            CommandList::Save(save::Save),
                        ]
                    }
                }

                impl Command for CommandList {
                    fn token(&self) -> Token {
                        match self {
                            CommandList::About(c) => c.token(),
                            CommandList::Create(c) => c.token(),
                            CommandList::Save(c) => c.token(),
                        }
                    }

                    fn autocomplete(&self, fuzzy_match: FuzzyMatch, input: &str) -> Option<AutocompleteSuggestion> {
                        match self {
                            CommandList::About(c) => c.autocomplete(fuzzy_match, input),
                            CommandList::Create(c) => c.autocomplete(fuzzy_match, input),
                            CommandList::Save(c) => c.autocomplete(fuzzy_match, input),
                        }
                    }

                    fn get_priority(&self, token_match: &TokenMatch) -> Option<CommandPriority> {
                        match self {
                            CommandList::About(c) => c.get_priority(token_match),
                            CommandList::Create(c) => c.get_priority(token_match),
                            CommandList::Save(c) => c.get_priority(token_match),
                        }
                    }

                    fn get_canonical_form_of(&self, token_match: &TokenMatch) -> Option<String> {
                        match self {
                            CommandList::About(c) => c.get_canonical_form_of(token_match),
                            CommandList::Create(c) => c.get_canonical_form_of(token_match),
                            CommandList::Save(c) => c.get_canonical_form_of(token_match),
                        }
                    }

                    async fn run<'a>(
                        &self,
                        token_match: TokenMatch<'a>,
                        app_meta: &mut AppMeta,
                    ) -> Result<impl ::std::fmt::Display, impl ::std::fmt::Display> {
                        match self {
                            CommandList::About(c) => c.run(token_match, app_meta)
                                .await
                                .map(|s| s.to_string())
                                .map_err(|e| e.to_string()),
                            CommandList::Create(c) => c.run(token_match, app_meta)
                                .await
                                .map(|s| s.to_string())
                                .map_err(|e| e.to_string()),
                            CommandList::Save(c) => c.run(token_match, app_meta)
                                .await
                                .map(|s| s.to_string())
                                .map_err(|e| e.to_string()),
                        }
                    }
                }
            }
            .to_string(),
            run(quote! {
                enum CommandList {
                    About(about::About),
                    Create(create::Create),
                    Save(save::Save),
                }
            })
            .unwrap()
            .to_string()
        );
    }

    #[test]
    fn failure_test_not_enum() {
        assert_eq!(
            "CommandList can only be derived on enums.",
            run(quote! {
                struct MarkerSchmarker;
            })
            .unwrap_err(),
        );
    }

    #[test]
    fn failure_test_unit_variant() {
        assert_eq!(
            "CommandList::Bad: CommandList enums must have only tuple variants with a single field",
            run(quote! {
                enum CommandList {
                    Good(good::Good),
                    Bad,
                    GoodAgain(good_again::GoodAgain),
                }
            })
            .unwrap_err(),
        );
    }

    #[test]
    fn failure_test_struct_variant() {
        assert_eq!(
            "CommandList::Bad: CommandList enums must have only tuple variants with a single field",
            run(quote! {
                enum CommandList {
                    Good(good::Good),
                    Bad {
                        worse: worse::Worse,
                    },
                    GoodAgain(good_again::GoodAgain),
                }
            })
            .unwrap_err(),
        );
    }

    #[test]
    fn failure_test_tuple_variant_too_big() {
        assert_eq!(
            "CommandList::Bad: CommandList enums must have only tuple variants with a single field",
            run(quote! {
                enum CommandList {
                    Good(good::Good),
                    Bad(okay::Okay, bad::Bad),
                    GoodAgain(good_again::GoodAgain),
                }
            })
            .unwrap_err(),
        );
    }
}
