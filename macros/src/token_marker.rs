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
        .find(|variant| !matches!(variant.fields, syn::Fields::Unit))
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
                    type Error = ();

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
                        (&input).into()
                    }
                }

                impl From<&MarkerSchmarker> for u8 {
                    fn from(input: &MarkerSchmarker) -> u8 {
                        match input {
                            MarkerSchmarker::One => 0u8,
                            MarkerSchmarker::Two => 1u8,
                            MarkerSchmarker::Three => 2u8,
                        }
                    }
                }

                impl PartialEq<u8> for MarkerSchmarker {
                    fn eq(&self, other: &u8) -> bool {
                        &u8::from(self) == other
                    }
                }

                impl PartialEq<u8> for &MarkerSchmarker {
                    fn eq(&self, other: &u8) -> bool {
                        &u8::from(*self) == other
                    }
                }

                impl PartialEq<MarkerSchmarker> for u8 {
                    fn eq(&self, other: &MarkerSchmarker) -> bool {
                        self == &u8::from(other)
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

    #[test]
    fn failure_test_not_enum() {
        assert_eq!(
            "TokenMarker can only be derived on enums.",
            run(quote! {
                struct MarkerSchmarker;
            })
            .unwrap_err(),
        );
    }

    #[test]
    fn failure_test_tuple_variant() {
        assert_eq!(
            "MarkerSchmarker::Bad: TokenMarker enums must have only unit variants",
            run(quote! {
                enum MarkerSchmarker {
                    Good,
                    Bad(Worse),
                    GoodAgain,
                }
            })
            .unwrap_err(),
        );
    }

    #[test]
    fn failure_test_struct_variant() {
        assert_eq!(
            "MarkerSchmarker::Bad: TokenMarker enums must have only unit variants",
            run(quote! {
                enum MarkerSchmarker {
                    Good,
                    Bad {
                        worse: Worse,
                    },
                    GoodAgain,
                }
            })
            .unwrap_err(),
        );
    }

    #[test]
    fn failure_test_took_big() {
        assert!(run(quote! {
            enum MarkerSchmarker {
                Good0x00, Good0x01, Good0x02, Good0x03, Good0x04, Good0x05, Good0x06, Good0x07,
                Good0x08, Good0x09, Good0x0a, Good0x0b, Good0x0c, Good0x0d, Good0x0e, Good0x0f,
                Good0x10, Good0x11, Good0x12, Good0x13, Good0x14, Good0x15, Good0x16, Good0x17,
                Good0x18, Good0x19, Good0x1a, Good0x1b, Good0x1c, Good0x1d, Good0x1e, Good0x2f,
                Good0x20, Good0x21, Good0x22, Good0x23, Good0x24, Good0x25, Good0x26, Good0x27,
                Good0x28, Good0x29, Good0x2a, Good0x2b, Good0x2c, Good0x2d, Good0x2e, Good0x3f,
                Good0x30, Good0x31, Good0x32, Good0x33, Good0x34, Good0x35, Good0x36, Good0x37,
                Good0x38, Good0x39, Good0x3a, Good0x3b, Good0x3c, Good0x3d, Good0x3e, Good0x4f,
                Good0x40, Good0x41, Good0x42, Good0x43, Good0x44, Good0x45, Good0x46, Good0x47,
                Good0x48, Good0x49, Good0x4a, Good0x4b, Good0x4c, Good0x4d, Good0x4e, Good0x5f,
                Good0x50, Good0x51, Good0x52, Good0x53, Good0x54, Good0x55, Good0x56, Good0x57,
                Good0x58, Good0x59, Good0x5a, Good0x5b, Good0x5c, Good0x5d, Good0x5e, Good0x6f,
                Good0x60, Good0x61, Good0x62, Good0x63, Good0x64, Good0x65, Good0x66, Good0x67,
                Good0x68, Good0x69, Good0x6a, Good0x6b, Good0x6c, Good0x6d, Good0x6e, Good0x7f,
                Good0x70, Good0x71, Good0x72, Good0x73, Good0x74, Good0x75, Good0x76, Good0x77,
                Good0x78, Good0x79, Good0x7a, Good0x7b, Good0x7c, Good0x7d, Good0x7e, Good0x8f,
                Good0x80, Good0x81, Good0x82, Good0x83, Good0x84, Good0x85, Good0x86, Good0x87,
                Good0x88, Good0x89, Good0x8a, Good0x8b, Good0x8c, Good0x8d, Good0x8e, Good0x9f,
                Good0x90, Good0x91, Good0x92, Good0x93, Good0x94, Good0x95, Good0x96, Good0x97,
                Good0x98, Good0x99, Good0x9a, Good0x9b, Good0x9c, Good0x9d, Good0x9e, Good0xaf,
                Good0xa0, Good0xa1, Good0xa2, Good0xa3, Good0xa4, Good0xa5, Good0xa6, Good0xa7,
                Good0xa8, Good0xa9, Good0xaa, Good0xab, Good0xac, Good0xad, Good0xae, Good0xbf,
                Good0xb0, Good0xb1, Good0xb2, Good0xb3, Good0xb4, Good0xb5, Good0xb6, Good0xb7,
                Good0xb8, Good0xb9, Good0xba, Good0xbb, Good0xbc, Good0xbd, Good0xbe, Good0xcf,
                Good0xc0, Good0xc1, Good0xc2, Good0xc3, Good0xc4, Good0xc5, Good0xc6, Good0xc7,
                Good0xc8, Good0xc9, Good0xca, Good0xcb, Good0xcc, Good0xcd, Good0xce, Good0xdf,
                Good0xd0, Good0xd1, Good0xd2, Good0xd3, Good0xd4, Good0xd5, Good0xd6, Good0xd7,
                Good0xd8, Good0xd9, Good0xda, Good0xdb, Good0xdc, Good0xdd, Good0xde, Good0xef,
                Good0xe0, Good0xe1, Good0xe2, Good0xe3, Good0xe4, Good0xe5, Good0xe6, Good0xe7,
                Good0xe8, Good0xe9, Good0xea, Good0xeb, Good0xec, Good0xed, Good0xee, Good0xff,
                Good0xf0, Good0xf1, Good0xf2, Good0xf3, Good0xf4, Good0xf5, Good0xf6, Good0xf7,
                Good0xf8, Good0xf9, Good0xfa, Good0xfb, Good0xfc, Good0xfd, Good0xfe, Good0xff,
            }
        })
        .is_ok());

        assert_eq!(
            "A maximum of 256 marker variants are permitted. What kind of enum is this?!",
            run(quote! {
                enum MarkerSchmarker {
                    Good0x00, Good0x01, Good0x02, Good0x03, Good0x04, Good0x05, Good0x06, Good0x07,
                    Good0x08, Good0x09, Good0x0a, Good0x0b, Good0x0c, Good0x0d, Good0x0e, Good0x0f,
                    Good0x10, Good0x11, Good0x12, Good0x13, Good0x14, Good0x15, Good0x16, Good0x17,
                    Good0x18, Good0x19, Good0x1a, Good0x1b, Good0x1c, Good0x1d, Good0x1e, Good0x2f,
                    Good0x20, Good0x21, Good0x22, Good0x23, Good0x24, Good0x25, Good0x26, Good0x27,
                    Good0x28, Good0x29, Good0x2a, Good0x2b, Good0x2c, Good0x2d, Good0x2e, Good0x3f,
                    Good0x30, Good0x31, Good0x32, Good0x33, Good0x34, Good0x35, Good0x36, Good0x37,
                    Good0x38, Good0x39, Good0x3a, Good0x3b, Good0x3c, Good0x3d, Good0x3e, Good0x4f,
                    Good0x40, Good0x41, Good0x42, Good0x43, Good0x44, Good0x45, Good0x46, Good0x47,
                    Good0x48, Good0x49, Good0x4a, Good0x4b, Good0x4c, Good0x4d, Good0x4e, Good0x5f,
                    Good0x50, Good0x51, Good0x52, Good0x53, Good0x54, Good0x55, Good0x56, Good0x57,
                    Good0x58, Good0x59, Good0x5a, Good0x5b, Good0x5c, Good0x5d, Good0x5e, Good0x6f,
                    Good0x60, Good0x61, Good0x62, Good0x63, Good0x64, Good0x65, Good0x66, Good0x67,
                    Good0x68, Good0x69, Good0x6a, Good0x6b, Good0x6c, Good0x6d, Good0x6e, Good0x7f,
                    Good0x70, Good0x71, Good0x72, Good0x73, Good0x74, Good0x75, Good0x76, Good0x77,
                    Good0x78, Good0x79, Good0x7a, Good0x7b, Good0x7c, Good0x7d, Good0x7e, Good0x8f,
                    Good0x80, Good0x81, Good0x82, Good0x83, Good0x84, Good0x85, Good0x86, Good0x87,
                    Good0x88, Good0x89, Good0x8a, Good0x8b, Good0x8c, Good0x8d, Good0x8e, Good0x9f,
                    Good0x90, Good0x91, Good0x92, Good0x93, Good0x94, Good0x95, Good0x96, Good0x97,
                    Good0x98, Good0x99, Good0x9a, Good0x9b, Good0x9c, Good0x9d, Good0x9e, Good0xaf,
                    Good0xa0, Good0xa1, Good0xa2, Good0xa3, Good0xa4, Good0xa5, Good0xa6, Good0xa7,
                    Good0xa8, Good0xa9, Good0xaa, Good0xab, Good0xac, Good0xad, Good0xae, Good0xbf,
                    Good0xb0, Good0xb1, Good0xb2, Good0xb3, Good0xb4, Good0xb5, Good0xb6, Good0xb7,
                    Good0xb8, Good0xb9, Good0xba, Good0xbb, Good0xbc, Good0xbd, Good0xbe, Good0xcf,
                    Good0xc0, Good0xc1, Good0xc2, Good0xc3, Good0xc4, Good0xc5, Good0xc6, Good0xc7,
                    Good0xc8, Good0xc9, Good0xca, Good0xcb, Good0xcc, Good0xcd, Good0xce, Good0xdf,
                    Good0xd0, Good0xd1, Good0xd2, Good0xd3, Good0xd4, Good0xd5, Good0xd6, Good0xd7,
                    Good0xd8, Good0xd9, Good0xda, Good0xdb, Good0xdc, Good0xdd, Good0xde, Good0xef,
                    Good0xe0, Good0xe1, Good0xe2, Good0xe3, Good0xe4, Good0xe5, Good0xe6, Good0xe7,
                    Good0xe8, Good0xe9, Good0xea, Good0xeb, Good0xec, Good0xed, Good0xee, Good0xff,
                    Good0xf0, Good0xf1, Good0xf2, Good0xf3, Good0xf4, Good0xf5, Good0xf6, Good0xf7,
                    Good0xf8, Good0xf9, Good0xfa, Good0xfb, Good0xfc, Good0xfd, Good0xfe, Good0xff,
                    Bad,
                }
            })
            .unwrap_err(),
        );
    }
}
