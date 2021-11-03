use proc_macro::TokenStream;
use quote::quote;

pub fn run(input: TokenStream) -> Result<TokenStream, String> {
    let ast = syn::parse(input).map_err(|e| format!("{}", e))?;
    impl_word_list(&ast)
}

fn impl_word_list(ast: &syn::DeriveInput) -> Result<TokenStream, String> {
    let name = &ast.ident;

    if let syn::Data::Enum(data_enum) = &ast.data {
        let mut from_str_match_cases = Vec::new();
        let mut from_str_if_cases = Vec::new();
        let mut as_str_cases = Vec::new();
        let mut words = Vec::new();

        data_enum.variants.iter().try_for_each(|variant| {
            let ident = &variant.ident;
            let mut term: String = ident
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

            for attribute in &variant.attrs {
                match attribute.parse_meta().map_err(|e| format!("{}", e))? {
                    syn::Meta::NameValue(name_value) if name_value.path.is_ident("alias") => {
                        let lit = name_value.lit;
                        words.push(quote! { #lit, });
                        from_str_match_cases.push(quote! { #lit => Ok(#name::#ident), });
                    }
                    syn::Meta::NameValue(name_value) if name_value.path.is_ident("term") => {
                        if let syn::Lit::Str(lit_str) = name_value.lit {
                            term = lit_str.value();
                        } else {
                            return Err("Unexpected value for \"term\" helper.".to_string());
                        }
                    }
                    _ => {}
                }
            }

            match &variant.fields {
                syn::Fields::Unit => {
                    words.push(quote! { #term, });
                    as_str_cases.push(quote! { #name::#ident => #term, });
                    from_str_match_cases.push(quote! { #term => Ok(#name::#ident), });
                }
                syn::Fields::Unnamed(fields) => {
                    if fields.unnamed.len() != 1 {
                        return Err("Only single-variant tuple enums are supported.".to_string());
                    }

                    as_str_cases.push(quote! { #name::#ident(value) => value.as_str(), });
                    from_str_if_cases.push(quote! {
                        if let Ok(value) = input.parse() {
                            Ok(#name::#ident(value))
                        } else
                    });
                }
                syn::Fields::Named(_) => {
                    return Err("Named enum variants are not supported.".to_string())
                }
            }

            Result::<(), String>::Ok(())
        })?;

        let gen = quote! {
            impl #name {
                pub fn get_words() -> &'static [&'static str] {
                    &[
                        #(#words)*
                    ]
                }

                pub fn as_str(&self) -> &'static str {
                    match self {
                        #(#as_str_cases)*
                    }
                }
            }

            impl std::str::FromStr for #name {
                type Err = ();

                fn from_str(input: &str) -> Result<#name, ()> {
                    #(#from_str_if_cases)*

                    {
                        match input {
                            #(#from_str_match_cases)*
                            _ => Err(()),
                        }
                    }
                }
            }

            impl From<#name> for &'static str {
                fn from(variant: #name) -> &'static str {
                    variant.as_str()
                }
            }

            impl From<#name> for String {
                fn from(variant: #name) -> String {
                    variant.as_str().to_string()
                }
            }
        };

        Ok(gen.into())
    } else {
        Err("WordLists must be enums.".to_string())
    }
}
