use super::{Command, CommandEnum, CommandStruct, Trait};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn run(input: TokenStream) -> Result<TokenStream, String> {
    let result = match Command::try_from(input)? {
        Command::Enum(command_enum) => derive_enum(command_enum),
        Command::Struct(command_struct) => derive_struct(command_struct),
    };

    Ok(result)
}

fn derive_enum(command_enum: CommandEnum) -> TokenStream {
    let mod_ident = format_ident!("impl_display_for_{}", command_enum.ident_with_sep("_"));
    let enum_ident = &command_enum.ident;

    let unit_cases = get_unit_cases(&command_enum);
    let tuple_cases = get_tuple_cases(&command_enum);
    let struct_cases = get_struct_cases(&command_enum);

    quote! {
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
    }
}

fn derive_struct(command_struct: CommandStruct) -> TokenStream {
    let mod_ident = format_ident!("impl_display_for_{}", command_struct.ident_with_sep("_"));
    let struct_ident = &command_struct.ident;

    quote! {
        mod #mod_ident {
            use super::*;
            use std::fmt;

            impl fmt::Display for #struct_ident {
                fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                    self.0.iter().enumerate().try_for_each(|(i, v)| {
                        if i == 0 {
                            write!(f, "{}", v)
                        } else {
                            write!(f, " {}", v)
                        }
                    })
                }
            }
        }
    }
}

fn get_unit_cases(command_enum: &CommandEnum) -> TokenStream {
    let tokens = command_enum.unit_variants().map(|variant| {
        let ident = &variant.ident;
        if variant.is_ignored {
            quote! { Self::#ident => Ok(()), }
        } else {
            let syntax = variant.syntax.to_string();
            quote! { Self::#ident => write!(f, #syntax), }
        }
    });

    quote! { #(#tokens)* }
}

fn get_tuple_cases(command_enum: &CommandEnum) -> TokenStream {
    let tokens = command_enum.tuple_variants().map(|variant| {
        let ident = &variant.ident;
        if variant.implements == Trait::WordList {
            quote! { Self::#ident(v) => write!(f, "{}", v.as_str()), }
        } else {
            quote! { Self::#ident(v) => write!(f, "{}", v), }
        }
    });

    quote! { #(#tokens)* }
}

fn get_struct_cases(command_enum: &CommandEnum) -> TokenStream {
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

    quote! { #(#tokens)* }
}
