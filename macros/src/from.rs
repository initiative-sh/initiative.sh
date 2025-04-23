use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned as _;

pub fn run(input: TokenStream) -> TokenStream {
    syn::parse2(input)
        .and_then(|ast| impl_from(&ast))
        .unwrap_or_else(syn::Error::into_compile_error)
}

fn lifetimes_to_path_arguments<I>(input: I) -> syn::PathArguments
where
    I: IntoIterator<Item = syn::Lifetime>,
{
    syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
        colon2_token: None,
        lt_token: syn::Token![<](proc_macro2::Span::call_site()),
        args: input
            .into_iter()
            .map(syn::GenericArgument::Lifetime)
            .collect(),
        gt_token: syn::Token![>](proc_macro2::Span::call_site()),
    })
}

fn impl_from(ast: &syn::DeriveInput) -> Result<TokenStream, syn::Error> {
    let mut structure_name = syn::PathSegment {
        ident: ast.ident.clone(),
        arguments: lifetimes_to_path_arguments(
            ast.generics
                .params
                .iter()
                .map(|generic_param| {
                    match generic_param {
                        syn::GenericParam::Lifetime(lifetime_param) => {
                            Ok(lifetime_param.lifetime.clone())
                        }
                        syn::GenericParam::Type(_) => {
                            Err("Generic type parameters are not supported")
                        }
                        syn::GenericParam::Const(_) => {
                            Err("Generic const parameters are not supported")
                        }
                    }
                    .map_err(|e| syn::Error::new(generic_param.span(), e))
                })
                .collect::<Result<Vec<_>, _>>()?,
        ),
    };

    if structure_name.arguments.is_empty() {
        structure_name.arguments = syn::PathArguments::None;
    }

    match &ast.data {
        syn::Data::Enum(data) => impl_from_enum(&structure_name, data),
        syn::Data::Struct(data) => impl_from_struct(&structure_name, data),
        syn::Data::Union(_) => Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "From cannot be derived on unions",
        )),
    }
}

fn impl_from_enum(
    structure_name: &syn::PathSegment,
    data_enum: &syn::DataEnum,
) -> Result<TokenStream, syn::Error> {
    if data_enum
        .variants
        .iter()
        .any(|variant| !variant.attrs.is_empty())
    {
        data_enum
            .variants
            .iter()
            .filter(|variant| !variant.attrs.is_empty())
            .map(|variant| impl_from_enum_variant(structure_name, variant))
            .collect()
    } else {
        let token_stream: TokenStream = data_enum
            .variants
            .iter()
            .filter_map(|variant| impl_from_enum_variant(structure_name, variant).ok())
            .collect();

        if !token_stream.is_empty() {
            Ok(token_stream)
        } else {
            Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "This enum has no supported variants",
            ))
        }
    }
}

fn impl_from_enum_variant(
    structure_name: &syn::PathSegment,
    variant: &syn::Variant,
) -> Result<TokenStream, syn::Error> {
    let mut result_path: syn::Path = structure_name.ident.clone().into();
    result_path.segments.push(variant.ident.clone().into());
    impl_from_case(structure_name, result_path, &variant.fields)
}

fn impl_from_struct(
    structure_name: &syn::PathSegment,
    data_struct: &syn::DataStruct,
) -> Result<TokenStream, syn::Error> {
    let result_path = structure_name.ident.clone();
    impl_from_case(structure_name, result_path, &data_struct.fields)
}

fn impl_from_case<ResultPath: Into<syn::Path>>(
    structure_name: &syn::PathSegment,
    result_path: ResultPath,
    fields: &syn::Fields,
) -> Result<TokenStream, syn::Error> {
    let result_path = result_path.into();
    let structure_arguments = &structure_name.arguments;

    match &fields {
        syn::Fields::Named(fields_named) => {
            if fields_named.named.len() == 1 {
                let field = fields_named.named.first().unwrap();
                let field_ident = field.ident.as_ref().unwrap();
                let field_ty = &field.ty;

                Ok(quote! {
                    impl #structure_arguments From<#field_ty> for #structure_name {
                        fn from(#field_ident: #field_ty) -> #structure_name {
                            #result_path { #field_ident }
                        }
                    }
                })
            } else {
                Err("From can only be derived on variants or structs with one field")
            }
        }
        syn::Fields::Unnamed(fields_unnamed) => {
            if fields_unnamed.unnamed.len() == 1 {
                let field = fields_unnamed.unnamed.first().unwrap();
                let field_ty = &field.ty;

                Ok(quote! {
                    impl #structure_arguments From<#field_ty> for #structure_name {
                        fn from(input: #field_ty) -> #structure_name {
                            #result_path(input)
                        }
                    }
                })
            } else {
                Err("From can only be derived on variants or structs with one field")
            }
        }
        syn::Fields::Unit => Err("From cannot be derived for unit variants or structs"),
    }
    .map_err(|e| syn::Error::new(fields.span(), e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enum_test_success() {
        let input = quote! {
            enum Test {
                EmptyStruct {},
                OneStruct {
                    field_one: u8,
                },
                TwoStruct {
                    field_one: u16,
                    field_two: u32,
                },
                EmptyTuple(),
                OneTuple(i8),
                TwoTuple(i16, i32),
                Unit,
            }
        };

        let output = quote! {
            impl From<u8> for Test {
                fn from(field_one: u8) -> Test {
                    Test::OneStruct { field_one }
                }
            }

            impl From<i8> for Test {
                fn from(input: i8) -> Test {
                    Test::OneTuple(input)
                }
            }
        };

        assert_eq!(output.to_string(), run(input).to_string());
    }

    #[test]
    fn enum_test_no_supported() {
        let input = quote! {
            enum Test {
                EmptyStruct {},
                TwoStruct {
                    field_one: u16,
                    field_two: u32,
                },
                EmptyTuple(),
                TwoTuple(i16, i32),
                Unit,
            }
        };

        let output = quote! {
            ::core::compile_error!{"This enum has no supported variants"}
        };

        assert_eq!(output.to_string(), run(input).to_string());
    }

    #[test]
    fn enum_test_marked_explicit() {
        let input = quote! {
            enum Test {
                One(u8),
                #[from]
                Two(u16),
            }
        };

        let output = quote! {
            impl From<u16> for Test {
                fn from(input: u16) -> Test {
                    Test::Two(input)
                }
            }
        };

        assert_eq!(output.to_string(), run(input).to_string());
    }

    #[test]
    fn struct_unnamed_test_success() {
        let input = quote! {
            struct Test(u8);
        };

        let output = quote! {
            impl From<u8> for Test {
                fn from(input: u8) -> Test {
                    Test(input)
                }
            }
        };

        assert_eq!(output.to_string(), run(input).to_string());
    }

    #[test]
    fn struct_named_test_success() {
        let input = quote! {
            struct Test {
                field: u8,
            }
        };

        let output = quote! {
            impl From<u8> for Test {
                fn from(field: u8) -> Test {
                    Test { field }
                }
            }
        };

        assert_eq!(output.to_string(), run(input).to_string());
    }

    #[test]
    fn enum_test_lifetime() {
        let input = quote! {
            enum Test<'a> {
                Yes(&'a str),
            }
        };

        let output = quote! {
            impl<'a> From<&'a str> for Test<'a> {
                fn from(input: &'a str) -> Test<'a> {
                    Test::Yes(input)
                }
            }
        };

        assert_eq!(output.to_string(), run(input).to_string());
    }
}
