use super::{CommandEnum, CommandVariant, CommandVariantSyntax, Trait, UnitStructCommandVariant};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::iter;

pub fn run(input: TokenStream) -> Result<TokenStream, String> {
    let command_enum = CommandEnum::try_from(input)?;

    let mod_ident = format_ident!("impl_autocomplete_for_{}", command_enum.ident_with_sep("_"));
    let enum_ident = &command_enum.ident;

    let init_suggestions_with_unit_cases = init_suggestions_with_unit_cases(&command_enum)?;

    let tuple_cases = get_tuple_cases(&command_enum)?;

    let struct_cases = get_struct_cases(&command_enum)?;

    let result = quote! {
        mod #mod_ident {
            use super::*;
            use crate::app::{AppMeta, Autocomplete, ContextAwareParse};
            use crate::utils::CaseInsensitiveStr;
            use async_trait::async_trait;
            use std::borrow::Cow;
            use std::str::FromStr;

            #[async_trait(?Send)]
            impl Autocomplete for #enum_ident {
                async fn autocomplete(
                    input: &str,
                    app_meta: &AppMeta,
                ) -> Vec<(Cow<'static, str>, Cow<'static, str>)> {
                    #init_suggestions_with_unit_cases

                    #tuple_cases

                    #struct_cases

                    suggestions
                }
            }
        }
    };

    //panic!("{}", result);

    Ok(result)
}

fn init_suggestions_with_unit_cases(command_enum: &CommandEnum) -> Result<TokenStream, String> {
    let mut tokens_simple = Vec::new();
    let mut tokens_with_callback = Vec::new();

    for variant in command_enum.variants.iter().filter_map(|variant| {
        if let CommandVariant::Unit(unit_variant) = variant {
            if unit_variant.is_ignored {
                None
            } else {
                Some(unit_variant)
            }
        } else {
            None
        }
    }) {
        for syntax in iter::once(&variant.syntax)
            .chain(variant.aliases.iter())
            .filter(|syntax| !syntax.no_autocomplete)
        {
            let term = syntax.to_string();

            if let Some(desc_fn) = variant.autocomplete_desc_fn.as_ref() {
                tokens_with_callback.push(quote! {
                    if #term.starts_with_ci(input) {
                        suggestions.push((
                            #term.into(),
                            #desc_fn(input, app_meta),
                        ));
                    }
                });
            } else {
                let desc = variant
                    .autocomplete_desc
                    .as_ref()
                    .map_or_else(|| variant.syntax.to_string(), |s| s.to_string());
                tokens_simple.push(quote! { ( #term, #desc ) })
            };
        }
    }

    if tokens_simple.len() + tokens_with_callback.len() > 10 {
        for token in tokens_with_callback.iter_mut() {
            *token = quote! {
                if suggestions.len() >= 10 {
                    return suggestions;
                }
                #token
            };
        }
    }

    let init = if tokens_simple.is_empty() {
        quote! {
            let mut suggestions: Vec<(Cow<'static, str>, Cow<'static, str>)> = Vec::new();
        }
    } else {
        quote! {
            let mut suggestions: Vec<(Cow<'static, str>, Cow<'static, str>)> = [ #(#tokens_simple),* ]
                .iter()
                .filter(|(s, _)| s.starts_with_ci(input))
                .map(|&(a, b)| (a.into(), b.into()))
                .take(10)
                .collect();
        }
    };

    Ok(quote! {
        #init
        #(#tokens_with_callback)*
    })
}

fn get_tuple_cases(command_enum: &CommandEnum) -> Result<Option<TokenStream>, String> {
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
        .filter_map(|variant| {
            let ty = &variant.ty;

            match variant.implements {
                Trait::Runnable => Some(quote! {
                    suggestions.append(&mut #ty::autocomplete(input, app_meta).await);
                }),
                Trait::FromStr => None,
                Trait::WordList => todo!("WordLists in tuples are not supported by Autocomplete."),
            }
        })
        .collect();

    if tokens.is_empty() {
        Ok(None)
    } else {
        Ok(Some(quote! { #(#tokens)* }))
    }
}

fn get_struct_cases(command_enum: &CommandEnum) -> Result<Option<TokenStream>, String> {
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
        .map(|variant| {
            iter::once(parse_struct_syntax(variant, &variant.syntax))
                .chain(
                    variant
                        .aliases
                        .iter()
                        .map(|alias| parse_struct_syntax(variant, alias)),
                )
                .filter_map(|tokens| tokens.transpose())
                .collect::<Result<Vec<_>, _>>()
                .map(|clauses| {
                    quote! {
                        #(#clauses)*
                    }
                })
        })
        .collect::<Result<Vec<_>, _>>()?;

    if tokens.is_empty() {
        Ok(None)
    } else {
        Ok(Some(quote! { #(#tokens)* }))
    }
}

fn parse_struct_syntax(
    variant: &UnitStructCommandVariant,
    syntax: &CommandVariantSyntax,
) -> Result<Option<TokenStream>, String> {
    if !syntax.middle.is_empty() {
        todo!("Syntaxes with separators are not supported by Autocomplete.");
    }

    let syntax_end = if let Some(syntax_end) = &syntax.end {
        syntax_end
    } else {
        todo!("Syntaxes without trailing idents are not supported by Autocomplete.");
    };
    let field = variant
        .fields
        .iter()
        .find(|field| &field.ident == syntax_end)
        .expect("Type must be defined!");
    let ty = &field.ty;

    if syntax.no_autocomplete {
        return Ok(None);
    }

    let (desc0, desc1) = if let Some(desc_fn) = variant.autocomplete_desc_fn.as_ref() {
        (
            quote! { #desc_fn(input, app_meta, None) },
            quote! { #desc_fn(input, app_meta, Some((suggestion.into(), description.into()))) },
        )
    } else if let Some(desc) = variant.autocomplete_desc.as_ref() {
        (quote! { #desc.into() }, quote! { #desc.into() })
    } else {
        let desc = syntax.to_string();
        (quote! { #desc.into() }, quote! { #desc.into() })
    };

    match &syntax.start {
        Some(syntax_start) => {
            let syntax_str = syntax.to_string();

            Ok(Some(match field.implements {
                Trait::FromStr => quote! {
                    if suggestions.len() >= 10 {
                        return suggestions;
                    }

                    if let Some(suggestion) = input.strip_prefix_ci(#syntax_start) {
                        let suggestion: Cow<'static, str> = suggestion.into();
                        let description = suggestion.clone();
                        suggestions.push((input.to_string().into(), #desc1));
                    } else if #syntax_start.starts_with_ci(input) {
                        suggestions.push((#syntax_start.into(), #desc0));
                    }
                },
                Trait::Runnable => {
                    let format_str = format!("{}{{}}", syntax_start);
                    quote! {
                        if let Some(remainder) = input.strip_prefix_ci(#syntax_start) {
                            for (suggestion, description) in #ty::autocomplete(remainder, app_meta).await.drain(..) {
                                if suggestions.len() >= 10 {
                                    return suggestions;
                                }

                                suggestions.push((
                                    format!(#format_str, suggestion).into(),
                                    #desc1,
                                ));
                            }
                        } else if #syntax_start.starts_with_ci(input) {
                            if suggestions.len() >= 10 {
                                return suggestions;
                            }

                            suggestions.push((#syntax_str.into(), #desc0));
                        }
                    }
                }
                Trait::WordList => todo!("WordLists in structs are not supported by Autocomplete."),
            }))
        }
        None => {
            let field = variant
                .fields
                .iter()
                .find(|field| &field.ident == syntax_end)
                .expect("Type must be defined!");
            let ty = &field.ty;

            Ok(match field.implements {
                Trait::FromStr => None,
                Trait::Runnable => {
                    let suggestions_append = quote! {
                        #ty::autocomplete(input, app_meta)
                            .await
                            .drain(..)
                            .map(|(suggestion, description)| {
                                (suggestion.clone(), #desc1)
                            })
                            .take(10 - suggestions.len())
                            .for_each(|v| suggestions.push(v));
                    };

                    Some(quote! {
                        if suggestions.len() >= 10 {
                            return suggestions;
                        }

                        #suggestions_append
                        suggestions.truncate(10);
                    })
                }
                Trait::WordList => Some(quote! {
                    for suggestion in #ty::get_words().filter(|s| s.starts_with_ci(input)).take(10) {
                        if suggestions.len() >= 10 {
                            return suggestions;
                        }

                        let suggestion: Cow<'static, str> = suggestion.into();
                        let description = suggestion.clone();
                        suggestions.push((suggestion.clone(), #desc1));
                    }
                }),
            })
        }
    }
}
