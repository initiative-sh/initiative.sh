use super::{CommandEnum, Trait};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn run(input: TokenStream) -> Result<TokenStream, String> {
    let command_enum = CommandEnum::try_from(input)?;

    let mod_ident = format_ident!("impl_display_for_{}", command_enum.ident_with_sep("_"));
    let enum_ident = &command_enum.ident;

    let unit_cases = get_unit_cases(&command_enum)?;

    let tuple_cases = get_tuple_cases(&command_enum)?;

    let struct_cases = get_struct_cases(&command_enum)?;

    let result = quote! {
        mod #mod_ident {
            use super::*;
            use std::fmt;

            impl fmt::Display for #enum_ident {
                fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                    match self {
                        #unit_cases
                        #tuple_cases
                        #struct_cases
                    }
                }
            }
        }
    };

    //panic!("{}", result);

    Ok(result)
}

fn get_unit_cases(command_enum: &CommandEnum) -> Result<TokenStream, String> {
    let tokens = command_enum.unit_variants().map(|variant| {
        let ident = &variant.ident;
        if variant.is_ignored {
            quote! { Self::#ident => Ok(()), }
        } else {
            let syntax = variant.syntax.to_string();
            quote! { Self::#ident => write!(f, #syntax), }
        }
    });

    Ok(quote! { #(#tokens)* })
}

fn get_tuple_cases(command_enum: &CommandEnum) -> Result<TokenStream, String> {
    let tokens = command_enum.tuple_variants().map(|variant| {
        let ident = &variant.ident;
        quote! { Self::#ident(v) => write!(f, "{}", v), }
    });

    Ok(quote! { #(#tokens)* })
}

fn get_struct_cases(command_enum: &CommandEnum) -> Result<TokenStream, String> {
    let tokens = command_enum.struct_variants().map(|variant| {
        let ident = &variant.ident;
        if variant.is_ignored {
            quote! { Self::#ident { .. } => Ok(()), }
        } else {
            let field_idents = variant.fields.iter().map(|field| &field.ident);
            let word_lists: Vec<&syn::Ident> = variant
                .fields
                .iter()
                .filter(|field| field.implements == Trait::WordList)
                .map(|field| &field.ident)
                .collect();

            let terms = variant
                .syntax
                .start
                .iter()
                .map(|s| quote! { write!(f, #s)?; })
                .chain(variant.syntax.middle.iter().map(|(id, s)| {
                    let fmt_str = format!("{}{{}}", s);
                    if word_lists.contains(&id) {
                        quote! { write!(f, #fmt_str, #id.as_str())?; }
                    } else {
                        quote! { write!(f, #fmt_str, #id)?; }
                    }
                }))
                .chain(variant.syntax.end.iter().map(|id| {
                    if word_lists.contains(&id) {
                        quote! { write!(f, "{}", #id.as_str())?; }
                    } else {
                        quote! { write!(f, "{}", #id)?; }
                    }
                }));

            quote! {
                Self::#ident { #(#field_idents),* } => {
                    #(#terms)*
                    Ok(())
                }
            }
        }
    });

    Ok(quote! { #(#tokens)* })
}
