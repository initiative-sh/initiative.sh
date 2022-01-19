use super::{
    Command, CommandEnum, CommandVariant, CommandVariantSyntax, Trait, UnitStructCommandVariant,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::iter;

pub fn run(input: TokenStream) -> Result<TokenStream, String> {
    let command_enum = if let Command::Enum(command_enum) = Command::try_from(input)? {
        command_enum
    } else {
        todo!("Autocomplete cannot yet be derived for newtypes.");
    };

    let mod_ident = format_ident!("impl_autocomplete_for_{}", command_enum.ident_with_sep("_"));
    let enum_ident = &command_enum.ident;

    let unit_cases = unit_cases(&command_enum)?;

    let tuple_cases = get_tuple_cases(&command_enum)?;

    let struct_cases = get_struct_cases(&command_enum)?;

    let variant_names = command_enum
        .unit_variants()
        .map(|variant| {
            let name = variant.ident.to_string();
            let ident = &variant.ident;
            quote! { Self::#ident => #name, }
        })
        .chain(command_enum.tuple_variants().map(|variant| {
            let name = variant.ident.to_string();
            let ident = &variant.ident;
            quote! { Self::#ident(..) => #name, }
        }))
        .chain(command_enum.struct_variants().map(|variant| {
            let name = variant.ident.to_string();
            let ident = &variant.ident;
            quote! { Self::#ident { .. } => #name, }
        }));

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
                    include_aliases: bool,
                ) -> Vec<(Cow<'static, str>, Cow<'static, str>)> {
                    let mut suggestions: Vec<(Cow<'static, str>, Cow<'static, str>)> = Vec::new();

                    #unit_cases

                    #tuple_cases

                    #struct_cases

                    suggestions
                }

                fn get_variant_name(&self) -> &'static str {
                    match self {
                        #(#variant_names)*
                    }
                }
            }
        }
    };

    //panic!("{}", result);

    Ok(result)
}

fn unit_cases(command_enum: &CommandEnum) -> Result<TokenStream, String> {
    let mut tokens_simple = Vec::new();
    let mut alias_tokens_simple = Vec::new();
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
        for (syntax, is_canonical) in iter::once((&variant.syntax, true))
            .chain(variant.aliases.iter().map(|alias| (alias, false)))
            .filter(|(syntax, _)| !syntax.no_autocomplete)
        {
            let term = syntax.to_string();

            if let Some(desc_fn) = variant.autocomplete_desc_fn.as_ref() {
                let condition = if is_canonical {
                    quote! { #term.starts_with_ci(input) }
                } else {
                    quote! { include_aliases && #term.starts_with_ci(input) }
                };

                tokens_with_callback.push(quote! {
                    if #condition {
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

                if is_canonical {
                    tokens_simple.push(quote! { ( #term, #desc ) });
                } else {
                    alias_tokens_simple.push(quote! { ( #term, #desc ) });
                }
            };
        }
    }

    let canonical_part = if tokens_simple.is_empty() {
        quote! {}
    } else {
        quote! {
            suggestions.extend(
                [ #(#tokens_simple),* ]
                    .iter()
                    .filter(|(s, _)| s.starts_with_ci(input))
                    .map(|&(a, b)| (a.into(), b.into()))
            );
        }
    };

    let alias_part = if alias_tokens_simple.is_empty() {
        quote! {}
    } else {
        quote! {
            if include_aliases {
                suggestions.extend(
                    [ #(#alias_tokens_simple),* ]
                        .iter()
                        .filter(|(s, _)| s.starts_with_ci(input))
                        .map(|&(a, b)| (a.into(), b.into()))
                );
            }
        }
    };

    Ok(quote! {
        #canonical_part
        #alias_part
        #(#tokens_with_callback)*
    })
}

fn get_tuple_cases(command_enum: &CommandEnum) -> Result<TokenStream, String> {
    let tokens = command_enum.tuple_variants().filter_map(|variant| {
        let ty = &variant.ty;

        match variant.implements {
            Trait::Runnable => Some(quote! {
                suggestions.append(&mut #ty::autocomplete(input, app_meta, include_aliases).await);
            }),
            Trait::FromStr => None,
            Trait::WordList => todo!("WordLists in tuples are not supported by Autocomplete."),
        }
    });

    Ok(quote! { #(#tokens)* })
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
                .filter_map(|tokens| tokens.transpose())
                .collect::<Result<Vec<_>, _>>()
                .map(|clauses| {
                    quote! {
                        #(#clauses)*
                    }
                })
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(quote! { #(#tokens)* })
}

fn parse_struct_syntax(
    variant: &UnitStructCommandVariant,
    syntax: &CommandVariantSyntax,
    is_canonical: bool,
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
                    if let Some(suggestion) = input.strip_prefix_ci(#syntax_start) {
                        let suggestion: Cow<'static, str> = suggestion.to_string().into();
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
                            for (suggestion, description) in #ty::autocomplete(remainder, app_meta, include_aliases && #is_canonical).await.drain(..) {
                                suggestions.push((
                                    format!(#format_str, suggestion).into(),
                                    #desc1,
                                ));
                            }
                        } else if #syntax_start.starts_with_ci(input) {
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
                Trait::Runnable => Some(quote! {
                    #ty::autocomplete(input, app_meta, false)
                        .await
                        .drain(..)
                        .map(|(suggestion, description)| {
                            (suggestion.clone(), #desc1)
                        })
                        .for_each(|v| suggestions.push(v));
                }),
                Trait::WordList => Some(quote! {
                    for suggestion in #ty::get_words().filter(|s| s.starts_with_ci(input)) {
                        let suggestion: Cow<'static, str> = suggestion.into();
                        let description = suggestion.clone();
                        suggestions.push((suggestion.clone(), #desc1));
                    }
                }),
            })
        }
    }
}
