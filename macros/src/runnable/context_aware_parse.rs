use super::{
    CommandEnum, CommandVariant, CommandVariantSyntax, CommandVariantSyntaxPart,
    UnitStructCommandVariant,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::iter;

pub fn run(input: TokenStream) -> Result<TokenStream, String> {
    let command_enum = CommandEnum::try_from(input)?;

    let mod_ident = format_ident!(
        "impl_context_aware_parse_for_{}",
        command_enum.ident_with_sep("_"),
    );
    let enum_ident = &command_enum.ident;

    let unit_cases = get_unit_cases(&command_enum);

    let tuple_cases = get_tuple_cases(&command_enum);

    let struct_cases = get_struct_cases(&command_enum);

    let result = quote! {
        mod #mod_ident {
            use super::*;
            use crate::app::{AppMeta, ContextAwareParse};
            use crate::utils::CaseInsensitiveStr;
            use async_trait::async_trait;

            #[async_trait(?Send)]
            impl ContextAwareParse for #enum_ident {
                async fn parse_input(input: &str, app_meta: &AppMeta) -> (Option<Self>, Vec<Self>) {
                    let mut exact_match = None;
                    let mut fuzzy_matches = Vec::new();

                    #unit_cases

                    #tuple_cases

                    #struct_cases

                    (exact_match, fuzzy_matches)
                }
            }
        }
    };

    //panic!("{}", result);

    Ok(result)
}

fn get_unit_cases(command_enum: &CommandEnum) -> Option<TokenStream> {
    let tokens: Vec<_> = command_enum
        .variants
        .iter()
        .filter_map(|variant| {
            if let CommandVariant::Unit(unit_variant) = variant {
                if unit_variant.is_ignored {
                    None
                } else {
                    Some(unit_variant)
                }
            } else {
                None
            }
        })
        .map(|variant| {
            let ident = &variant.ident;

            let alias_clauses: Vec<_> = variant
                .aliases
                .iter()
                .map(|alias| {
                    let syntax = alias.to_string();
                    quote! { if input.eq_ci(#syntax) { fuzzy_matches.push(Self::#ident) } else }
                })
                .collect();

            let syntax = variant.syntax.to_string();

            quote! { if input.eq_ci(#syntax) { exact_match = Some(Self::#ident) } else #(#alias_clauses)* }
        })
        .collect();

    if tokens.is_empty() {
        None
    } else {
        Some(quote! {
            #(#tokens)* {}
        })
    }
}

fn get_tuple_cases(command_enum: &CommandEnum) -> Option<TokenStream> {
    let tokens: Vec<_> = command_enum
        .variants
        .iter()
        .filter_map(|variant| {
            if let CommandVariant::Tuple(tuple_variant) = variant {
                Some(tuple_variant)
            } else {
                None
            }
        })
        .map(|variant| {
            let ident = &variant.ident;
            let ty = &variant.ty;

            quote! {
                {
                    let (subcommand_exact_match, subcommand_fuzzy_matches) = #ty::parse_input(input, app_meta).await;

                    if let Some(command) = subcommand_exact_match {
                        exact_match = Some(Self::#ident(command));
                    }

                    for command in subcommand_fuzzy_matches {
                        fuzzy_matches.push(Self::#ident(command));
                    }
                }
            }
        })
        .collect();

    if tokens.is_empty() {
        None
    } else {
        Some(quote! {
            #(#tokens)* {}
        })
    }
}

fn get_struct_cases(command_enum: &CommandEnum) -> Option<TokenStream> {
    let tokens: Vec<_> = command_enum
        .variants
        .iter()
        .filter_map(|variant| {
            if let CommandVariant::Struct(struct_variant) = variant {
                if struct_variant.is_ignored {
                    None
                } else {
                    Some(struct_variant)
                }
            } else {
                None
            }
        })
        .flat_map(|variant| {
            let clauses: Vec<_> = iter::once(parse_struct_syntax(variant, &variant.syntax, true))
                .chain(
                    variant
                        .aliases
                        .iter()
                        .map(|alias| parse_struct_syntax(variant, alias, false)),
                )
                .collect();

            quote! {
                #(#clauses)*
            }
        })
        .collect();

    if tokens.is_empty() {
        None
    } else {
        Some(quote! {
            #(#tokens)* {}
        })
    }
}

fn parse_struct_syntax(
    variant: &UnitStructCommandVariant,
    syntax: &CommandVariantSyntax,
    is_canonical: bool,
) -> TokenStream {
    let variant_ident = &variant.ident;
    let mut syntax_parts = syntax.syntax_parts.iter();

    match (
        syntax_parts.next(),
        syntax_parts.next(),
        syntax_parts.next(),
    ) {
        (
            Some(CommandVariantSyntaxPart::Str(start)),
            Some(CommandVariantSyntaxPart::Ident(field_ident)),
            None,
        ) => {
            let field = variant
                .fields
                .iter()
                .find(|field| &field.ident == field_ident)
                .expect("Type must be defined!");
            let ty = &field.ty;

            if is_canonical {
                quote! {
                    if let Some(remainder) = input.strip_prefix(#start) {
                        let (subcommand_exact_match, subcommand_fuzzy_matches) = #ty::parse_input(remainder.trim_start(), app_meta).await;

                        if let Some(command) = subcommand_exact_match {
                            exact_match = Some(Self::#variant_ident { #field_ident: command });
                        }

                        for command in subcommand_fuzzy_matches {
                            fuzzy_matches.push(Self::#variant_ident { #field_ident: command });
                        }
                    }
                }
            } else {
                quote! {
                    if let Some(remainder) = input.strip_prefix(#start) {
                        let (mut subcommand_exact_match, mut subcommand_fuzzy_matches) = #ty::parse_input(remainder.trim_start(), app_meta).await;

                        for command in subcommand_exact_match
                            .into_iter()
                            .chain(subcommand_fuzzy_matches.drain(..))
                        {
                            fuzzy_matches.push(Self::#variant_ident { #field_ident: command });
                        }
                    }
                }
            }
        }
        _ => panic!(),
    }

    /*
    if let = syntax.syntax_parts.first() {
        quote! {
            if let Some(
        }
    } else {
        panic!(
            "Non-prefixed syntaxes are not yet supported for {}: {:?}",
            ident, syntax,
        );
    }
    */
}

/*
fn parse_struct_syntax_alias(ident: &syn::Ident, syntax: &CommandVariantSyntax) -> TokenStream {
    //
}
*/

/*
#[async_trait(?Send)]
pub trait ContextAwareParse: Sized {
    async fn parse_input(input: &str, app_meta: &AppMeta) -> (Option<Self>, Vec<Self>);
}
*/
