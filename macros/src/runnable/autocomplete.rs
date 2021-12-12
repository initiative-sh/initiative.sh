use super::{CommandEnum, CommandVariant, CommandVariantSyntax, Trait, UnitStructCommandVariant};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::iter;

pub fn run(input: TokenStream) -> Result<TokenStream, String> {
    let command_enum = CommandEnum::try_from(input)?;

    let mod_ident = format_ident!("impl_autocomplete_for_{}", command_enum.ident_with_sep("_"));
    let enum_ident = &command_enum.ident;

    let unit_cases_or_empty = get_unit_cases_or_empty(&command_enum)?;

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
                    if input.is_empty() {
                        return Vec::new();
                    }

                    let mut suggestions: Vec<(Cow<'static, str>, Cow<'static, str>)> = #unit_cases_or_empty;

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

fn get_unit_cases_or_empty(command_enum: &CommandEnum) -> Result<TokenStream, String> {
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
        .flat_map(|variant| {
            iter::once(&variant.syntax)
                .chain(variant.aliases.iter())
                .filter(|syntax| !syntax.no_autocomplete)
                .map(|syntax| {
                    let term = syntax.to_string();
                    let desc = variant
                        .autocomplete_desc
                        .as_ref()
                        .map_or_else(|| variant.syntax.to_string(), |s| s.to_string());

                    quote! { ( #term, #desc ) }
                })
        })
        .collect();

    if tokens.is_empty() {
        Ok(quote! { Vec::new() })
    } else {
        Ok(quote! {
            [ #(#tokens),* ]
                .iter()
                .filter(|(s, _)| s.starts_with_ci(input))
                .map(|&(a, b)| (a.into(), b.into()))
                .collect()
        })
    }
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
                Trait::WordList => todo!(),
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
        todo!();
    }

    if syntax.no_autocomplete {
        return Ok(None);
    }

    let desc = variant
        .autocomplete_desc
        .as_ref()
        .map_or_else(|| syntax.to_string(), |s| s.to_string());

    match (&syntax.start, &syntax.end) {
        (Some(syntax_start), Some(syntax_end)) => {
            let field = variant
                .fields
                .iter()
                .find(|field| &field.ident == syntax_end)
                .expect("Type must be defined!");
            let ty = &field.ty;
            let syntax_str = syntax.to_string();

            Ok(Some(match field.implements {
                Trait::FromStr => quote! {
                    if suggestions.len() >= 10 {
                        return suggestions;
                    }

                    if #syntax_start.starts_with_ci(input) {
                        suggestions.push((#syntax_start.into(), #desc.into()));
                    } else if input.starts_with_ci(#syntax_start) {
                        suggestions.push((input.to_string().into(), #desc.into()));
                    }
                },
                Trait::Runnable => quote! {
                    if #syntax_start.starts_with_ci(input) {
                        if suggestions.len() >= 10 {
                            return suggestions;
                        }

                        suggestions.push((#syntax_str.into(), #desc.into()));
                    } else if let Some(remainder) = input.strip_prefix_ci(#syntax_start) {
                        for (a, b) in #ty::autocomplete(remainder, app_meta).await.drain(..) {
                            if suggestions.len() >= 10 {
                                return suggestions;
                            }

                            suggestions.push((
                                format!("{}{}", #syntax_start, a).into(),
                                #desc.into(),
                            ));
                        }
                    }
                },
                Trait::WordList => todo!(),
            }))
        }
        (None, Some(syntax_end)) => {
            let ty = &variant
                .fields
                .iter()
                .find(|field| &field.ident == syntax_end)
                .expect("Type must be defined!")
                .ty;
            let field = variant
                .fields
                .iter()
                .find(|field| &field.ident == syntax_end)
                .expect("Type must be defined!");

            Ok(match field.implements {
                Trait::FromStr => None,
                Trait::Runnable => Some(quote! {
                    if suggestions.len() >= 10 {
                        return suggestions;
                    }

                    suggestions.append(&mut #ty::autocomplete(input, app_meta).await);
                    suggestions.truncate(10);
                }),
                Trait::WordList => Some(quote! {
                    for word in #ty::get_words().filter(|s| s.starts_with_ci(input)).take(10) {
                        if suggestions.len() >= 10 {
                            return suggestions;
                        }

                        suggestions.push((word.into(), #desc.into()));
                    }
                }),
            })
        }
        _ => todo!(),
    }
}
