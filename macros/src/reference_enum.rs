use initiative_reference::srd_5e;
use proc_macro::TokenStream;
use quote::quote;

/*
struct Entry {
    ident: syn::Ident,
    name: String,
    aliases: Vec<String>,
    details: String,
}
*/

#[derive(Default)]
struct EntryBuilder {
    ident: Option<syn::Ident>,
    name: Option<String>,
    aliases: Vec<String>,
    details: Option<String>,
}

impl EntryBuilder {
    fn with_ident(mut self, ident: &str) -> Self {
        self.ident = syn::parse_str(ident).ok();
        self
    }

    fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    fn with_aliases(mut self, aliases: Vec<String>) -> Self {
        self.aliases = aliases;
        self
    }

    fn with_details(mut self, details: &impl ToString) -> Self {
        self.details = Some(details.to_string());
        self
    }

    fn into_tuple(self) -> Result<(syn::Ident, String, Vec<String>, String), ()> {
        Ok((
            self.ident.ok_or(())?,
            self.name.ok_or(())?,
            self.aliases,
            self.details.ok_or(())?,
        ))
    }
}

pub fn run(input: TokenStream) -> Result<TokenStream, String> {
    let ident = parse_args(input)?;

    let entries: Vec<(syn::Ident, String, Vec<String>, String)> = match format!("{}", ident)
        .as_str()
    {
        "Spell" => srd_5e::spells()?
            .iter()
            .map(|spell| {
                EntryBuilder::default()
                    .with_ident(&spell.token())
                    .with_name(spell.name())
                    .with_details(&spell.display_details())
                    .into_tuple()
                    .unwrap()
            })
            .collect(),
        "Item" => srd_5e::items()?
            .iter()
            .map(|item| {
                EntryBuilder::default()
                    .with_ident(&item.token())
                    .with_name(item.name())
                    .with_aliases(item.alt_name().into_iter().collect())
                    .with_details(&item.display_details())
                    .into_tuple()
                    .unwrap()
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
                    result.push(
                        EntryBuilder::default()
                            .with_ident(&category.token())
                            .with_name(category.name().to_lowercase())
                            .with_aliases(category.alt_names())
                            .with_details(&category.display_item_table(&items))
                            .into_tuple()
                            .unwrap(),
                    );
                }

                if has_magic_items {
                    let category_name = category.name();

                    if has_items {
                        result.push(
                            EntryBuilder::default()
                                .with_ident(&format!("Magic{}", category.token()))
                                .with_name(format!("magic {}", category_name.to_lowercase()))
                                .with_aliases(vec![format!(
                                    "{}, magic",
                                    category_name.to_lowercase()
                                )])
                                .with_details(&category.display_magic_item_list(
                                    &magic_items,
                                    &format!("Magic {}", category_name),
                                ))
                                .into_tuple()
                                .unwrap(),
                        );
                    } else {
                        result.push(
                            EntryBuilder::default()
                                .with_ident(&category.token())
                                .with_name(category_name.to_lowercase())
                                .with_aliases(category.alt_names())
                                .with_details(
                                    &category.display_magic_item_list(&magic_items, &category_name),
                                )
                                .into_tuple()
                                .unwrap(),
                        );
                    }
                }
            }

            result
        }
        "MagicItem" => srd_5e::magic_items()?
            .iter()
            .map(|item| {
                EntryBuilder::default()
                    .with_ident(&item.token())
                    .with_name(item.name())
                    .with_details(&item.display_details())
                    .into_tuple()
                    .unwrap()
            })
            .collect(),
        "Trait" => srd_5e::traits()?
            .iter()
            .filter(|t| !t.has_parent())
            .map(|t| {
                EntryBuilder::default()
                    .with_ident(&t.token())
                    .with_name(t.name.to_owned())
                    .with_details(&t.display_details())
                    .into_tuple()
                    .unwrap()
            })
            .collect(),
        _ => unimplemented!(),
    };

    let variants = entries
        .iter()
        .map(|(variant, _, _, _)| quote! { #variant, });

    let inputs_to_ok_variants = entries.iter().flat_map(|(variant, name, alt_names, _)| {
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

    let variants_to_names = entries
        .iter()
        .map(|(variant, name, _, _)| quote! { #ident::#variant => #name, });

    let variants_to_outputs = entries
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

    let words = entries.iter().flat_map(|(_, name, alt_names, _)| {
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
