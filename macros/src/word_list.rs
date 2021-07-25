use proc_macro::TokenStream;
use quote::quote;

pub fn run(input: TokenStream) -> Result<TokenStream, String> {
    let ast = syn::parse(input).map_err(|e| format!("{}", e))?;
    impl_word_list(&ast)
}

fn impl_word_list(ast: &syn::DeriveInput) -> Result<TokenStream, String> {
    let name = &ast.ident;

    if let syn::Data::Enum(data_enum) = &ast.data {
        let mut words_to_variants = Vec::new();
        let mut variants_to_words = Vec::new();
        let mut words = Vec::new();

        data_enum.variants.iter().for_each(|variant| {
            let word: String = variant
                .ident
                .to_string()
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    if c.is_uppercase() {
                        if i > 0 {
                            format!("-{}", c.to_lowercase())
                        } else {
                            c.to_lowercase().to_string()
                        }
                    } else {
                        c.to_string()
                    }
                })
                .collect();

            words.push(quote! { #word, });
            variants_to_words.push(quote! { #name::#variant => #word, });
            words_to_variants.push(quote! { #word => Ok(#name::#variant),});

            if word.contains('-') {
                let alt_word: String = word
                    .chars()
                    .map(|c| if c == '-' { ' ' } else { c })
                    .collect();

                words_to_variants.push(quote! {
                    #alt_word => Ok(#name::#variant),
                });
            }
        });

        let gen = quote! {
            impl #name {
                pub fn get_words() -> &'static [&'static str] {
                    return &[
                        #(#words)*
                    ];
                }
            }

            impl std::str::FromStr for #name {
                type Err = ();

                fn from_str(raw: &str) -> Result<#name, ()> {
                    match raw {
                        #(#words_to_variants)*
                        _ => Err(()),
                    }
                }
            }

            impl From<#name> for &'static str {
                fn from(variant: #name) -> &'static str {
                    match variant {
                        #(#variants_to_words)*
                    }
                }
            }

            impl From<#name> for String {
                fn from(variant: #name) -> String {
                    let variant_str: &'static str = variant.into();
                    variant_str.to_string()
                }
            }
        };

        Ok(gen.into())
    } else {
        Err("WordLists must be enums.".to_string())
    }
}