use initiative_reference::srd_5e;
use proc_macro::TokenStream;
use quote::quote;

pub fn run(input: TokenStream) -> Result<TokenStream, String> {
    let ident = parse_args(input)?;

    let data: Vec<(syn::Ident, String, Vec<String>, String)> = match format!("{}", ident).as_str() {
        "Spell" => srd_5e::spells()?
            .iter()
            .map(|spell| {
                (
                    syn::parse_str(&spell.token()).unwrap(),
                    spell.name(),
                    Vec::new(),
                    format!("{}", spell.display_details()),
                )
            })
            .collect(),
        "Item" => srd_5e::items()?
            .iter()
            .map(|item| {
                (
                    syn::parse_str(&item.token()).unwrap(),
                    item.name(),
                    item.alt_name().into_iter().collect(),
                    format!("{}", item.display_details()),
                )
            })
            .collect(),
        "ItemCategory" => {
            let items = srd_5e::items()?;
            let magic_items = srd_5e::magic_items()?;
            let mut result = Vec::new();

            for category in srd_5e::item_categories()? {
                let (has_items, has_magic_items) =
                    (category.has_items(), category.has_magic_items());

                if has_items {
                    result.push((
                        syn::parse_str(&category.token()).unwrap(),
                        category.name().to_lowercase(),
                        category.alt_names(),
                        format!("{}", category.display_item_table(&items)),
                    ));
                }

                if has_magic_items {
                    let category_name = category.name();

                    if has_items {
                        result.push((
                            syn::parse_str(&format!("Magic{}", category.token())).unwrap(),
                            format!("magic {}", category_name.to_lowercase()),
                            vec![format!("{}, magic", category_name.to_lowercase())],
                            format!(
                                "{}",
                                category.display_magic_item_list(
                                    &magic_items,
                                    &format!("Magic {}", category_name),
                                )
                            ),
                        ));
                    } else {
                        result.push((
                            syn::parse_str(&category.token()).unwrap(),
                            category_name.to_lowercase(),
                            category.alt_names(),
                            format!(
                                "{}",
                                &category.display_magic_item_list(&magic_items, &category_name)
                            ),
                        ));
                    }
                }
            }

            result
        }
        "MagicItem" => srd_5e::magic_items()?
            .iter()
            .map(|item| {
                (
                    syn::parse_str(&item.token()).unwrap(),
                    item.name(),
                    Vec::new(),
                    format!("{}", item.display_details()),
                )
            })
            .collect(),
        _ => unimplemented!(),
    };

    let variants = data.iter().map(|(variant, _, _, _)| quote! { #variant, });

    let inputs_to_ok_variants = data.iter().flat_map(|(variant, name, alt_names, _)| {
        let name_lc = name.to_lowercase();

        std::iter::once(quote! { #name_lc => Ok(#ident::#variant), }).chain(
            alt_names
                .iter()
                .zip(std::iter::repeat(variant))
                .map(|(alt_name, variant)| {
                    let alt_name_lc = alt_name.to_lowercase();
                    quote! { #alt_name_lc => Ok(#ident::#variant), }
                }),
        )
    });

    let variants_to_names = data
        .iter()
        .map(|(variant, name, _, _)| quote! { #ident::#variant => #name, });

    let variants_to_outputs = data
        .iter()
        .map(|(variant, _, _, output)| quote! { #ident::#variant => #output, });

    let get_list = if ident == "Spell" {
        let mut list_output = format!("# {}s", ident);
        srd_5e::spells()?
            .iter()
            .for_each(|spell| list_output.push_str(&format!("\n* {}", spell.display_summary())));

        quote! {
            pub fn get_list() -> &'static str {
                #list_output
            }
        }
    } else {
        quote! {}
    };

    let words = data.iter().flat_map(|(_, name, alt_names, _)| {
        std::iter::once(quote! { #name, })
            .chain(alt_names.iter().map(|alt_name| quote! { #alt_name, }))
    });

    let result = quote! {
        #[derive(Clone, Debug, PartialEq)]
        pub enum #ident {
            #(#variants)*
        }

        impl #ident {
            pub fn get_words() -> impl Iterator<Item = &'static str> {
                [#(#words)*].into_iter()
            }

            #get_list

            pub fn get_name(&self) -> &'static str {
                match self {
                    #(#variants_to_names)*
                }
            }

            pub fn get_output(&self) -> &'static str {
                match self {
                    #(#variants_to_outputs)*
                }
            }

            pub fn parse_cs(input: &str) -> Result<Self, ()> {
                match input {
                    #(#inputs_to_ok_variants)*
                    _ => Err(()),
                }
            }
        }

        #[async_trait::async_trait(?Send)]
        impl crate::app::ContextAwareParse for #ident {
            async fn parse_input(input: &str, _app_meta: &crate::app::AppMeta) -> (Option<Self>, Vec<Self>) {
                (input.parse().ok(), Vec::new())
            }
        }

        impl std::str::FromStr for #ident {
            type Err = ();

            fn from_str(input: &str) -> Result<#ident, ()> {
                if input.chars().any(char::is_uppercase) {
                    Self::parse_cs(&input.to_lowercase())
                } else {
                    Self::parse_cs(input)
                }
            }
        }

        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.get_output())
            }
        }
    };

    Ok(result.into())
}

fn parse_args(input: TokenStream) -> Result<syn::Ident, String> {
    let mut input_iter = input.into_iter();

    let ident: syn::Ident = match input_iter.next() {
        Some(tree) => syn::parse(tree.into()).unwrap(),
        otherwise => {
            return Err(format!(
                "Expected identity (eg. `Spell`), found `{:?}`.",
                otherwise
            ));
        }
    };

    match input_iter.next() {
        None => {}
        otherwise => {
            return Err(format!(
                "Expected end of parameters, found `{:?}`.",
                otherwise
            ));
        }
    }

    Ok(ident)
}
