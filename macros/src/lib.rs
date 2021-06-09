use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(RandomTable)]
pub fn random_table_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_random_table(&ast)
}

fn impl_random_table(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    if let syn::Data::Enum(data_enum) = &ast.data {
        let cases = data_enum.variants.iter().enumerate().map(|(i, variant)| {
            quote! { #i => #name::#variant, }
        });
        let len = cases.len();
        let gen = quote! {
            impl RandomTable for #name {
                fn get_random(rng: &mut impl rand::Rng, _demographics: &Demographics) -> Self {
                    //println!("{}", stringify!(#ast));
                    match rng.gen_range(0..#len) {
                        #(#cases)*
                        _ => unreachable!(),
                    }
                }
            }
        };
        gen.into()
    } else {
        unimplemented!();
    }
}

#[proc_macro_derive(WordList)]
pub fn word_list_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_word_list(&ast)
}

fn impl_word_list(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    if let syn::Data::Enum(data_enum) = &ast.data {
        let (words_to_variants, variants_to_words): (Vec<_>, Vec<_>) = data_enum
            .variants
            .iter()
            .map(|variant| {
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

                (
                    if word.contains('-') {
                        let alt_word: String = word
                            .chars()
                            .map(|c| if c == '-' { ' ' } else { c })
                            .collect();

                        quote! {
                            #word => Ok(#name::#variant),
                            #alt_word => Ok(#name::#variant),
                        }
                    } else {
                        quote! { #word => Ok(#name::#variant), }
                    },
                    quote! { #name::#variant => #word, },
                )
            })
            .unzip();

        let gen = quote! {
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
        gen.into()
    } else {
        unimplemented!();
    }
}

/*
#[proc_macro_derive(Demographics)]
pub fn demographics_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_demographics(&ast)
}

fn impl_demographics(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    if let syn::Data::Struct(data_struct) = &ast.data {
    } else {
    }
}
*/
