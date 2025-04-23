use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned as _;

pub fn run(input: TokenStream) -> TokenStream {
    syn::parse2(input)
        .and_then(|ast| impl_command_list(&ast))
        .unwrap_or_else(syn::Error::into_compile_error)
}

fn impl_command_list(ast: &syn::DeriveInput) -> Result<TokenStream, syn::Error> {
    let (ident, variants) = parse(ast)?;

    let mut token_stream = TokenStream::default();
    token_stream.extend(impl_get_all(ident, variants)?);
    token_stream.extend(impl_command_trait(ident, variants)?);

    Ok(token_stream)
}

fn parse(
    ast: &syn::DeriveInput,
) -> Result<(&syn::Ident, impl IntoIterator<Item = &syn::Variant> + Copy), syn::Error> {
    let syn::Data::Enum(syn::DataEnum { variants, .. }) = &ast.data else {
        return Err(syn::Error::new(
            ast.span(),
            "CommandList can only be derived on enums",
        ));
    };

    let ident = &ast.ident;

    if let Some(variant) = variants.iter().find(|variant| {
        if let syn::Fields::Unnamed(field) = &variant.fields {
            field.unnamed.len() != 1
        } else {
            true
        }
    }) {
        return Err(syn::Error::new(
            variant.span(),
            "CommandList enums must have only tuple variants with a single field",
        ));
    }

    Ok((ident, variants))
}

fn impl_get_all<'a, I>(ident: &syn::Ident, variants: I) -> Result<TokenStream, syn::Error>
where
    I: IntoIterator<Item = &'a syn::Variant>,
{
    let get_all_items = variants
        .into_iter()
        .filter_map(|variant| {
            if let Some(attribute) = variant
                .attrs
                .iter()
                .find(|attribute| attribute.meta.path().is_ident("command_list"))
            {
                attribute
                    .meta
                    .require_list()
                    .and_then(|meta_list| meta_list.parse_args::<syn::Ident>())
                    .and_then(|ident| {
                        (ident == "ignore").then_some(()).ok_or_else(|| {
                            syn::Error::new(
                                ident.span(),
                                r#"Unexpected directive; supported directives are "ignore""#,
                            )
                        })
                    })
                    .err()
                    .map(Err)
            } else {
                Some(Ok(variant))
            }
        })
        .map(|result_variant| {
            result_variant.map(|variant| {
                let variant_ident = &variant.ident;
                let fields = &variant.fields;

                quote! { #ident::#variant_ident #fields }
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(quote! {impl #ident {
        const fn get_all() -> &'static [#ident] {
            &[
                #( #get_all_items, )*
            ]
        }
    }
    })
}

fn impl_command_trait<'a, I>(ident: &syn::Ident, variants: I) -> Result<TokenStream, syn::Error>
where
    I: IntoIterator<Item = &'a syn::Variant>,
{
    let match_items: Vec<_> = variants
        .into_iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            quote! { #ident::#variant_ident(c) }
        })
        .collect();

    Ok(quote! {
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
        let input = quote! {
            enum CommandList {
                About(about::About),
                Create(create::Create),
                Save(save::Save),
            }
        };

        let output = quote! {
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
        };

        assert_eq!(output.to_string(), run(input).to_string());
    }

    #[test]
    fn failure_test_not_enum() {
        let input = quote! {
            struct MarkerSchmarker;
        };

        let output = quote! {
            ::core::compile_error!{
                "CommandList can only be derived on enums"
            }
        };

        assert_eq!(output.to_string(), run(input).to_string());
    }

    #[test]
    fn failure_test_unit_variant() {
        let input = quote! {
            enum CommandList {
                Good(good::Good),
                Bad,
                GoodAgain(good_again::GoodAgain),
            }
        };

        let output = quote! {
            ::core::compile_error!{
                "CommandList enums must have only tuple variants with a single field"
            }
        };

        assert_eq!(output.to_string(), run(input).to_string());
    }

    #[test]
    fn failure_test_struct_variant() {
        let input = quote! {
            enum CommandList {
                Good(good::Good),
                Bad {
                    worse: worse::Worse,
                },
                GoodAgain(good_again::GoodAgain),
            }
        };

        let output = quote! {
            ::core::compile_error!{
                "CommandList enums must have only tuple variants with a single field"
            }
        };

        assert_eq!(output.to_string(), run(input).to_string());
    }

    #[test]
    fn failure_test_tuple_variant_too_big() {
        let input = quote! {
            enum CommandList {
                Good(good::Good),
                Bad(okay::Okay, bad::Bad),
                GoodAgain(good_again::GoodAgain),
            }
        };

        let output = quote! {
            ::core::compile_error!{
                "CommandList enums must have only tuple variants with a single field"
            }
        };

        assert_eq!(output.to_string(), run(input).to_string());
    }

    #[test]
    fn success_test_with_ignore() {
        let input = quote! {
            enum CommandList {
                One(one::One),
                #[command_list(ignore)]
                Two(two::Two),
                Three(three::Three),
            }
        };

        let output = quote! {
            impl CommandList {
                const fn get_all() -> &'static [CommandList] {
                    &[
                        CommandList::One(one::One),
                        CommandList::Three(three::Three),
                    ]
                }
            }
        };

        let ast: syn::DeriveInput = syn::parse2(input).unwrap();
        let (ident, variants) = parse(&ast).unwrap();

        assert_eq!(
            output.to_string(),
            impl_get_all(ident, variants).unwrap().to_string(),
        );
    }
}
