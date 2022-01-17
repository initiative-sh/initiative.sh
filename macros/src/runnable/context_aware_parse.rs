use super::{
    Command, CommandEnum, CommandStruct, CommandVariantSyntax, Trait, UnitStructCommandVariant,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::iter;

pub fn run(input: TokenStream) -> Result<TokenStream, String> {
    match Command::try_from(input)? {
        Command::Enum(command_enum) => derive_enum(command_enum),
        Command::Struct(command_struct) => derive_struct(command_struct),
    }
}

fn derive_enum(command_enum: CommandEnum) -> Result<TokenStream, String> {
    let mod_ident = format_ident!(
        "impl_context_aware_parse_for_{}",
        command_enum.ident_with_sep("_"),
    );
    let enum_ident = &command_enum.ident;

    let unit_cases = get_unit_cases(&command_enum)?;
    let tuple_cases = get_tuple_cases(&command_enum)?;
    let struct_cases = get_struct_cases(&command_enum)?;

    Ok(quote! {
        mod #mod_ident {
            use super::*;
            use crate::app::{AppMeta, ContextAwareParse};
            use crate::utils::CaseInsensitiveStr;
            use async_trait::async_trait;
            use std::str::FromStr;

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
    })
}

fn derive_struct(command_struct: CommandStruct) -> Result<TokenStream, String> {
    let mod_ident = format_ident!(
        "impl_context_aware_parse_for_{}",
        command_struct.ident_with_sep("_"),
    );
    let struct_ident = &command_struct.ident;
    let subtype_path = &command_struct.subtype;

    let (matches_value, reject_callback, return_clause) = if command_struct.is_result {
        (
            quote! { Ok(value) },
            quote! { |s| Some(Err(s.to_string())) },
            quote! {
                if reject_count * 2 >= output.len() {
                    (None, vec![Self(output)])
                } else {
                    (Some(Self(output)), Vec::new())
                }
            },
        )
    } else {
        (
            quote! { value },
            quote! { |_| None },
            quote! {
                (
                    if reject_count >= output.len() {
                        None
                    } else {
                        Some(Self(output))
                    },
                    Vec::new(),
                )
            },
        )
    };

    Ok(quote! {
        mod #mod_ident {
            use super::*;
            use crate::app::{AppMeta, Autocomplete, ContextAwareParse};
            use crate::utils::{CaseInsensitiveStr, QuotedWordChunk};
            use async_trait::async_trait;
            use std::str::FromStr;

            #[async_trait(?Send)]
            impl ContextAwareParse for #struct_ident {
                async fn parse_input(input: &str, app_meta: &AppMeta) -> (Option<Self>, Vec<Self>) {
                    let mut parser = input.quoted_word_chunks(#reject_callback);

                    while let Some(phrase) = parser.current() {
                        match #subtype_path::parse_input(phrase, app_meta).await {
                            (Some(value), _) => parser.matches(#matches_value),
                            (None, mut v) => {
                                if let Some(value) = v.drain(..).next() {
                                    parser.matches(#matches_value);
                                } else {
                                    parser.partially_matches();
                                }
                            }
                        }
                    }

                    let (output, reject_count) = parser.into_output_with_reject_count();

                    #return_clause
                }
            }
        }
    })
}

fn get_unit_cases(command_enum: &CommandEnum) -> Result<TokenStream, String> {
    let tokens: Vec<_> = command_enum
        .unit_variants()
        .map(|variant| {
            let ident = &variant.ident;

            let alias_clauses = variant
                .aliases
                .iter()
                .map(|alias| {
                    let syntax = alias.to_string();
                    quote! { if input.eq_ci(#syntax) { fuzzy_matches.push(Self::#ident) } else }
                });

            if !variant.is_ignored {
                let syntax = variant.syntax.to_string();
                quote! { if input.eq_ci(#syntax) { exact_match = Some(Self::#ident) } else #(#alias_clauses)* }
            } else {
                quote! { #(#alias_clauses)* }
            }
        })
        .collect();

    if tokens.is_empty() {
        Ok(quote! {})
    } else {
        Ok(quote! { #(#tokens)* {} })
    }
}

fn get_tuple_cases(command_enum: &CommandEnum) -> Result<TokenStream, String> {
    let tokens = command_enum
        .tuple_variants()
        .map(|variant| {
            let ident = &variant.ident;
            let ty = &variant.ty;

            match variant.implements {
                Trait::Runnable => quote! {
                    {
                        let (subcommand_exact_match, subcommand_fuzzy_matches) = #ty::parse_input(input, app_meta).await;

                        if let Some(command) = subcommand_exact_match {
                            exact_match = exact_match.or_else(|| Some(Self::#ident(command)));
                        }

                        for command in subcommand_fuzzy_matches {
                            fuzzy_matches.push(Self::#ident(command));
                        }
                    }
                },
                Trait::FromStr | Trait::WordList => quote! {
                    {
                        exact_match = exact_match
                            .or_else(|| #ty::from_str(input).ok().map(|word| Self::#ident(word)));
                    }
                },
            }
        });

    Ok(quote! { #(#tokens)* {} })
}

fn get_struct_cases(command_enum: &CommandEnum) -> Result<TokenStream, String> {
    let tokens: Vec<_> = command_enum
        .struct_variants()
        .filter(|variant| !variant.is_ignored)
        .map(|variant| {
            iter::once(parse_struct_syntax(variant, &variant.syntax, true))
                .chain(
                    variant
                        .aliases
                        .iter()
                        .map(|alias| parse_struct_syntax(variant, alias, false)),
                )
                .collect::<Result<Vec<_>, _>>()
                .map(|clauses| {
                    quote! {
                        #(#clauses)*
                    }
                })
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(quote! { #(#tokens)* {} })
}

fn parse_struct_syntax(
    variant: &UnitStructCommandVariant,
    syntax: &CommandVariantSyntax,
    is_canonical: bool,
) -> Result<TokenStream, String> {
    let variant_ident = &variant.ident;

    if !syntax.middle.is_empty() {
        todo!("Syntaxes with separators are not supported by ContextAwareParse.");
    }

    match (&syntax.start, &syntax.end) {
        (Some(syntax_start), Some(syntax_end)) => {
            let field = variant
                .fields
                .iter()
                .find(|field| &field.ident == syntax_end)
                .expect("Type must be defined!");
            let ty = &field.ty;

            let parse_expr = match &field.implements {
                Trait::FromStr | Trait::WordList => quote! {
                    (#ty::from_str(remainder.trim_start()).ok(), Vec::<#ty>::new())
                },
                Trait::Runnable => quote! {
                    #ty::parse_input(remainder.trim_start(), app_meta).await
                },
            };

            Ok(if is_canonical {
                quote! {
                    if let Some(remainder) = input.strip_prefix_ci(#syntax_start) {
                        let (mut subcommand_exact_match, mut subcommand_fuzzy_matches) = #parse_expr;

                        exact_match = exact_match.or_else(|| subcommand_exact_match.take().map(|command| {
                            Self::#variant_ident { #syntax_end: command }
                        }));

                        if let Some(command) = subcommand_exact_match {
                            fuzzy_matches.push(Self::#variant_ident { #syntax_end: command });
                        }

                        subcommand_fuzzy_matches
                            .drain(..)
                            .for_each(|command| fuzzy_matches.push(Self::#variant_ident { #syntax_end: command }));
                    }
                }
            } else {
                quote! {
                    if let Some(remainder) = input.strip_prefix_ci(#syntax_start) {
                        let (subcommand_exact_match, _) = #parse_expr;

                        if let Some(command) = subcommand_exact_match {
                            fuzzy_matches.push(Self::#variant_ident { #syntax_end: command });
                        }
                    }
                }
            })
        }
        (None, Some(syntax_end)) => {
            let field = variant
                .fields
                .iter()
                .find(|field| &field.ident == syntax_end)
                .expect("Type must be defined!");
            let ty = &field.ty;

            if is_canonical {
                Err(format!("Use tuple variants (eg. `Self::{}({})`) for command nesting. Struct variants must have a prefix in the canonical form.", variant_ident, quote!{ #ty }))
            } else {
                match field.implements {
                    Trait::Runnable => Ok(quote! {
                        {
                            let (subcommand_exact_match, _) = #ty::parse_input(input, app_meta).await;

                            if let Some(#syntax_end) = subcommand_exact_match {
                                fuzzy_matches.push(Self::#variant_ident { #syntax_end });
                            }
                        }
                    }),
                    Trait::WordList | Trait::FromStr => Ok(quote! {
                        if let Ok(#syntax_end) = input.parse() {
                            fuzzy_matches.push(Self::#variant_ident { #syntax_end });
                        }
                    }),
                }
            }
        }
        _ => todo!("Syntaxes without trailing idents are not supported by ContextAwareParse."),
    }
}
