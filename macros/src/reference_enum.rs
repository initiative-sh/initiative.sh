use initiative_reference::srd_5e;
use proc_macro::TokenStream;
use quote::quote;

pub fn run(input: TokenStream) -> Result<TokenStream, String> {
    let ident = parse_args(input)?;

    let data: Vec<(syn::Ident, String, Option<String>, String)> =
        match format!("{}", ident).as_str() {
            "Spell" => srd_5e::spells()?
                .iter()
                .map(|spell| {
                    (
                        syn::parse_str(spell.token().as_str()).unwrap(),
                        spell.name(),
                        None,
                        format!("{}", spell.display_details()),
                    )
                })
                .collect(),
            "Item" => srd_5e::equipment()?
                .iter()
                .map(|equipment| {
                    (
                        syn::parse_str(equipment.token().as_str()).unwrap(),
                        equipment.name(),
                        equipment.alt_name(),
                        format!("{}", equipment.display_details()),
                    )
                })
                .collect(),
            "ItemCategory" => {
                let items = srd_5e::equipment()?;

                srd_5e::equipment_categories()?
                    .iter()
                    .map(|category| {
                        (
                            syn::parse_str(category.token().as_str()).unwrap(),
                            category.name().to_lowercase(),
                            category.alt_name(),
                            format!("{}", category.display_table(&items)),
                        )
                    })
                    .collect()
            }
            _ => unimplemented!(),
        };

    let variants = data.iter().map(|(variant, _, _, _)| quote! { #variant, });

    let inputs_to_ok_variants = data.iter().map(|(variant, name, alt_name, _)| {
        if let Some(alt_name) = alt_name {
            quote! { #name => Ok(#ident::#variant), #alt_name => Ok(#ident::#variant), }
        } else {
            quote! { #name => Ok(#ident::#variant), }
        }
    });

    let variants_to_names = data
        .iter()
        .map(|(variant, name, _, _)| quote! { #ident::#variant => #name, });

    let variants_to_outputs = data
        .iter()
        .map(|(variant, _, _, output)| quote! { #ident::#variant => #output, });

    let mut list_output = format!("# {}s", ident);
    srd_5e::spells()?.iter().for_each(|spell| {
        list_output.push_str(format!("\n* {}", spell.display_summary()).as_str())
    });

    let words = data.iter().map(|(_, name, alt_name, _)| {
        if let Some(alt_name) = alt_name {
            quote! { #name, #alt_name, }
        } else {
            quote! { #name, }
        }
    });

    let result = quote! {
        #[derive(Clone, Debug, PartialEq)]
        pub enum #ident {
            #(#variants)*
        }

        impl #ident {
            pub fn get_words() -> &'static [&'static str] {
                &[#(#words)*]
            }

            pub fn get_list() -> &'static str {
                #list_output
            }

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
        }

        impl std::str::FromStr for #ident {
            type Err = ();

            fn from_str(raw: &str) -> Result<#ident, ()> {
                match raw {
                    #(#inputs_to_ok_variants)*
                    _ => Err(()),
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
