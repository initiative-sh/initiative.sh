use proc_macro2::TokenStream;
use quote::quote;

pub fn run(input: TokenStream) -> Result<TokenStream, String> {
    let ast = syn::parse2(input).map_err(|e| e.to_string())?;
    impl_token_marker(&ast)
}

fn impl_token_marker(ast: &syn::DeriveInput) -> Result<TokenStream, String> {
    let syn::Data::Enum(syn::DataEnum { variants, .. }) = &ast.data else {
        return Err("TokenMarker can only be derived on enums.".to_string());
    };

    let ident = &ast.ident;

    if let Some(variant) = variants
        .iter()
        .find(|variant| variant.fields != syn::Fields::Unit)
    {
        return Err(format!(
            "{}::{}: TokenMarker enums must have only unit variants",
            ident, variant.ident
        ));
    }

    if variants.len() > 256 {
        return Err(
            "A maximum of 256 marker variants are permitted. What kind of enum is this?!"
                .to_string(),
        );
    }

    let (from_u8, into_u8): (Vec<_>, Vec<_>) = variants
        .iter()
        .enumerate()
        .map(|(i, variant)| {
            let i = i as u8;
            let variant_ident = &variant.ident;
            (
                quote! { #i => #ident::#variant_ident, },
                quote! { #ident::#variant_ident => #i, },
            )
        })
        .unzip();

    Ok(quote! {
        impl TryFrom<u8> for #ident {
            type Error = ();

            fn try_from(input: u8) -> Result<#ident, ()> {
                Ok(match input {
                    #(#from_u8)*
                    _ => return Err(()),
                })
            }
        }

        impl From<#ident> for u8 {
            fn from(input: #ident) -> u8 {
                (&input).into()
            }
        }

        impl From<&#ident> for u8 {
            fn from(input: &#ident) -> u8 {
                match input {
                    #(#into_u8)*
                }
            }
        }

        impl PartialEq<u8> for #ident {
            fn eq(&self, other: &u8) -> bool {
                &u8::from(self) == other
            }
        }

        impl PartialEq<u8> for &#ident {
            fn eq(&self, other: &u8) -> bool {
                &u8::from(*self) == other
            }
        }

        impl PartialEq<#ident> for u8 {
            fn eq(&self, other: &#ident) -> bool {
                self == &u8::from(other)
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
                impl TryFrom<u8> for MarkerSchmarker {
                    type Err = ();

                    fn try_from(input: u8) -> Result<MarkerSchmarker, ()> {
                        Ok(match input {
                            0u8 => MarkerSchmarker::One,
                            1u8 => MarkerSchmarker::Two,
                            2u8 => MarkerSchmarker::Three,
                            _ => return Err(()),
                        })
                    }
                }

                impl From<MarkerSchmarker> for u8 {
                    fn from(input: MarkerSchmarker) -> u8 {
                        match input {
                            MarkerSchmarker::One => 0u8,
                            MarkerSchmarker::Two => 1u8,
                            MarkerSchmarker::Three => 2u8,
                        }
                    }
                }

                impl From<MarkerSchmarker> for Option<u8> {
                    fn from(input: MarkerSchmarker) -> Option<u8> {
                        Some(input.into())
                    }
                }
            }
            .to_string(),
            run(quote! {
                enum MarkerSchmarker {
                    One,
                    Two = 3,
                    Three,
                }
            })
            .unwrap()
            .to_string()
        );
    }
}
