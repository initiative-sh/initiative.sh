//! The `#[derive(Autocomplete)]`, `#[derive(Display)]`, and `#[derive(ContextAwareParse)]` macros
//! generate parsing and autocomplete code based on an enum of command variants. All variant types
//! are supported:
//!
//! enum TestCommand {
//!     UnitVariant,
//!     TupleVariant(Subcommand),
//!     StructVariant {
//!         field1: u8,
//!         field2: u8,
//!     },
//! }
//!
//! * Unit variants are assigned canonical syntaxes by converting the variant name from CamelCase
//!   to kebab-case. In the above case, this would be `unit-variant`.
//! * Tuple variants must only have one element, and are considered transparent: results from the
//!   parse and autocomplete logic of the child element are included in the parent.
//! * Struct variants are assigned a canonical syntax according to the same rules as unit variants,
//!   with the variant fields suffixed. The above case would be `struct-variant [field1] [field2]`.
//!
//! The following attributes are recognized on unit and struct variants:
//!
//! - `#[command(no_default_autocomplete)]`: Do not show the canonical syntax in autocomplete
//!   results.
//! - `#[command(syntax = "foo [bar]")]`: Override the default syntax. All variant fields must be
//!   present, wrapped in square brackets. This syntax should be unambiguous, ie. if it matches,
//!   the parser will run that command even if there are other aliases that match.
//! - `#[command(alias = "foo [bar]")]`: Define an alias for the command. If only one alias matches
//!   a user input, that alias will be run. If multiple aliases match, no command will be run and
//!   the user will be asked to provide the canonical syntax (see above) that they want to run.
//! - `#[command(alias_no_autocomplete = "foo [bar]")]`: As above, but the alias won't appear in
//!   autocomplete suggestions.
//! - `#[command(ignore)]`: There is no syntax for this command. Used for commands that are
//!   runnable by alias only.
//! - `#[doc = "This command does stuff."]` or `/// Blah.`: Description for help docs.
//! - `#[command(autocomplete_desc = "some command")]`: Description for autocomplete results.
//! - `#[command(autocomplete_desc_fn(my_func))]`: Produce autocomplete descriptions by calling
//!   `my_func(input: &str, app_meta: &AppMeta, param1: Option<Cow<'static, str>>, ..) -> Cow<'static, str>`,
//!   where each parameter contains the autocomplete result from the corresponding struct field.
//!
//! The following attribute is recognized on tuple variants:
//!
//! - `#[command(implements(WordList))]` - Indicates that the field implements a different
//!   supported trait. Rather than transparently passing through to the underlying logic, the
//!   autocomplete and parsing logic will be handled by the parent parser. Recognized values:
//!   `WordList`, `Runnable` (default).
//!
//! The following attribute is recognized on individual fields of struct variants:
//!
//! - `#[command(implements(WordList))]` - Indicates that the field implements a different
//!   supported trait. By default it is assumed to implement `ContextAwareParse` and `Autocomplete`
//!   (collectively referred to as `Runnable`). Recognized values: `WordList`, `Runnable`.

pub mod autocomplete;
pub mod context_aware_parse;
pub mod display;

use proc_macro2::TokenStream;
use quote::ToTokens;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
enum Command {
    Enum(CommandEnum),
    Struct(CommandStruct),
}

#[derive(Debug)]
struct CommandStruct {
    ident: syn::Ident,
    is_result: bool,
    subtype: syn::Path,
}

#[derive(Debug)]
struct CommandEnum {
    ident: syn::Ident,
    variants: Vec<CommandVariant>,
}

#[derive(Debug)]
enum CommandVariant {
    Unit(UnitStructCommandVariant),
    Tuple(TupleCommandVariant),
    Struct(UnitStructCommandVariant),
}

#[derive(Debug)]
struct TupleCommandVariant {
    ident: syn::Ident,
    implements: Trait,
    ty: syn::Type,
}

#[derive(Debug)]
struct UnitStructCommandVariant {
    ident: syn::Ident,

    aliases: Vec<CommandVariantSyntax>,
    autocomplete_desc: Option<String>,
    autocomplete_desc_fn: Option<syn::Ident>,
    doc: Option<String>,
    fields: Vec<Field>,
    is_ignored: bool,
    syntax: CommandVariantSyntax,
}

#[derive(Debug)]
struct CommandVariantSyntax {
    pub start: Option<String>,
    pub middle: Vec<(syn::Ident, String)>,
    pub end: Option<syn::Ident>,
    pub no_autocomplete: bool,
}

#[derive(Debug)]
struct Field {
    ident: syn::Ident,
    implements: Trait,
    ty: syn::Type,
}

#[derive(Debug, PartialEq)]
enum Trait {
    FromStr,
    Runnable,
    WordList,
}

impl CommandEnum {
    fn ident_with_sep(&self, sep: &str) -> String {
        from_camel_case_with_sep(&self.ident, sep)
    }

    fn unit_variants(&self) -> impl Iterator<Item = &UnitStructCommandVariant> {
        self.variants.iter().filter_map(|variant| {
            if let CommandVariant::Unit(unit_variant) = variant {
                Some(unit_variant)
            } else {
                None
            }
        })
    }

    fn tuple_variants(&self) -> impl Iterator<Item = &TupleCommandVariant> {
        self.variants.iter().filter_map(|variant| {
            if let CommandVariant::Tuple(tuple_variant) = variant {
                Some(tuple_variant)
            } else {
                None
            }
        })
    }

    fn struct_variants(&self) -> impl Iterator<Item = &UnitStructCommandVariant> {
        self.variants.iter().filter_map(|variant| {
            if let CommandVariant::Struct(struct_variant) = variant {
                Some(struct_variant)
            } else {
                None
            }
        })
    }
}

impl CommandStruct {
    fn ident_with_sep(&self, sep: &str) -> String {
        from_camel_case_with_sep(&self.ident, sep)
    }
}

impl TryFrom<TokenStream> for Command {
    type Error = String;

    fn try_from(input_raw: TokenStream) -> Result<Self, Self::Error> {
        let input: syn::DeriveInput = syn::parse2(input_raw).map_err(|e| format!("{}", e))?;
        let ident = input.ident;

        match input.data {
            syn::Data::Enum(data_enum) => {
                let variants = data_enum
                    .variants
                    .iter()
                    .map(|v| v.try_into())
                    .collect::<Result<_, _>>()
                    .map_err(|e| format!("Error parsing {}::{}", ident, e))?;

                Ok(Self::Enum(CommandEnum { ident, variants }))
            }
            syn::Data::Struct(data_struct) => {
                let fields = if let syn::Fields::Unnamed(fields) = data_struct.fields {
                    fields
                } else {
                    return Err(format!(
                        "Error parsing {}: Only tuple structs are supported.",
                        ident,
                    ));
                };

                let field = if fields.unnamed.len() == 1 {
                    fields.unnamed.first().unwrap()
                } else {
                    return Err(format!(
                        "Error parsing {}: Tuple structs must have exactly one field.",
                        ident,
                    ));
                };

                let field_path = if let syn::Type::Path(path) = &field.ty {
                    &path.path
                } else {
                    return Err(format!(
                        r#"Error parsing {}: Field type must be a path, specifically "Vec<T>"."#,
                        ident,
                    ));
                };

                let field_type = if field_path.segments.len() == 1 {
                    field_path.segments.first().unwrap()
                } else {
                    return Err(format!(
                        r#"Error parsing {}: Field type must have one path segment, specifically "Vec<T>")."#,
                        ident,
                    ));
                };

                if field_type.ident != "Vec" {
                    return Err(format!("Error parsing {}: Field type must be Vec.", ident));
                }

                let field_arguments =
                    if let syn::PathArguments::AngleBracketed(arguments) = &field_type.arguments {
                        arguments
                    } else {
                        return Err(format!(
                            "Error parsing {}: Missing or invalid field type for inner type.",
                            ident,
                        ));
                    };

                let field_argument = if field_arguments.args.len() == 1 {
                    field_arguments.args.first().unwrap()
                } else {
                    return Err(format!(
                        "Error parsing {}: Expected exactly one type argument for inner type.",
                        ident,
                    ));
                };

                let subtype =
                    if let syn::GenericArgument::Type(syn::Type::Path(path)) = field_argument {
                        path.path.clone()
                    } else {
                        return Err(format!(
                            "Error parsing {}: Expected inner type to be a path.",
                            ident,
                        ));
                    };

                // Handle the special case where the inner type is a Result<T, String>
                if let Some(syn::PathSegment {
                    ident: result_ident,
                    arguments,
                }) = subtype.segments.first()
                {
                    if result_ident == &syn::Ident::new("Result", proc_macro2::Span::call_site())
                        && subtype.segments.len() == 1
                    {
                        if let syn::PathArguments::AngleBracketed(field_argument) = arguments {
                            if let Some(syn::GenericArgument::Type(syn::Type::Path(path))) =
                                field_argument.args.first()
                            {
                                return Ok(Self::Struct(CommandStruct {
                                    ident,
                                    is_result: true,
                                    subtype: path.path.clone(),
                                }));
                            }
                        }
                    }
                }

                Ok(Self::Struct(CommandStruct {
                    ident,
                    is_result: false,
                    subtype,
                }))
            }
            syn::Data::Union(_) => Err(format!(
                "Error parsing {}: Why are you even using unions?",
                ident
            )),
        }
    }
}

fn parse_syntax(input: &str, fields: &[Field]) -> Result<CommandVariantSyntax, String> {
    let mut syntax = CommandVariantSyntax {
        start: None,
        middle: Vec::new(),
        end: None,
        no_autocomplete: false,
    };

    let mut start_pos = 0;
    let mut unmatched_fields: HashMap<String, syn::Ident> = fields
        .iter()
        .map(|f| (f.ident.to_string(), f.ident.clone()))
        .collect();
    let mut hold_ident = None;

    for (pos, c) in input.char_indices().filter(|(_, c)| ['[', ']'].contains(c)) {
        match (hold_ident.take(), c) {
            (Some(ident), '[') => {
                let s = &input[start_pos..pos];

                if s.is_empty() {
                    return Err(format!(
                        r#"There must be at least one character dividing syntax parts in "{}"."#,
                        input,
                    ));
                } else {
                    syntax.middle.push((ident, s.to_string()));
                }
            }
            (None, ']') if start_pos > 0 => {
                if let Some(ident) = unmatched_fields.remove(&input[start_pos..pos]) {
                    hold_ident = Some(ident);
                } else {
                    return Err(format!(
                        r#"Unknown or duplicated field in "{}": "{}"."#,
                        input,
                        &input[start_pos..pos],
                    ));
                }
            }
            (None, '[') if syntax.start.is_none() => {
                if pos > 0 {
                    syntax.start = Some(input[start_pos..pos].to_string());
                }
            }
            _ => return Err(format!(r#"Unbalanced brackets in "{}"."#, input)),
        }

        start_pos = pos + 1;
    }

    if let Some(ident) = hold_ident.take() {
        let remainder = &input[start_pos..];
        if remainder.is_empty() {
            syntax.end = Some(ident)
        } else {
            syntax.middle.push((ident, remainder.to_string()))
        }
    } else if start_pos == 0 {
        syntax.start = Some(input.to_string());
    } else {
        return Err(format!(r#"Unclosed bracket in "{}"."#, input));
    }

    if let Some(missing_field) = unmatched_fields.into_keys().next() {
        Err(format!(
            r#"Field "{}" is not accounted for in syntax "{}"."#,
            missing_field, input,
        ))
    } else {
        Ok(syntax)
    }
}

fn from_camel_case_with_sep(input: &syn::Ident, sep: &str) -> String {
    input
        .to_string()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if c.is_uppercase() {
                if i > 0 {
                    format!("{}{}", sep, c.to_lowercase())
                } else {
                    c.to_lowercase().to_string()
                }
            } else {
                c.to_string()
            }
        })
        .collect()
}

fn walk_attrs<T, F: FnMut(T, &[&str], MetaValue) -> Result<T, String>>(
    attrs: &[syn::Attribute],
    initial: T,
    callback: &mut F,
) -> Result<T, String> {
    let mut value = initial;

    for attr in attrs {
        value = walk_meta_recursive(
            &attr.parse_meta().unwrap(),
            value,
            callback,
            &mut Vec::new(),
        )?;
    }

    Ok(value)
}

fn walk_meta_recursive<T, F: FnMut(T, &[&str], MetaValue) -> Result<T, String>>(
    meta: &syn::Meta,
    initial: T,
    callback: &mut F,
    path: &mut Vec<String>,
) -> Result<T, String> {
    let mut value = initial;

    match meta {
        syn::Meta::List(meta_list) => {
            path.push(meta_list.path.to_token_stream().to_string());
            for nested in meta_list.nested.iter() {
                match nested {
                    syn::NestedMeta::Meta(meta) => {
                        value = walk_meta_recursive(meta, value, callback, path)?;
                    }
                    syn::NestedMeta::Lit(lit) => {
                        value = callback(
                            value,
                            &path.iter().map(|s| s.as_str()).collect::<Vec<_>>()[..],
                            MetaValue::Lit(lit.to_owned()),
                        )?;
                    }
                }
            }
            path.pop();
        }
        syn::Meta::Path(meta_path) => {
            value = callback(
                value,
                &path.iter().map(|s| s.as_str()).collect::<Vec<_>>()[..],
                MetaValue::Path(meta_path.to_owned()),
            )?;
        }
        syn::Meta::NameValue(name_value) => {
            path.push(name_value.path.to_token_stream().to_string());
            value = callback(
                value,
                &path.iter().map(|s| s.as_str()).collect::<Vec<_>>()[..],
                MetaValue::Lit(name_value.lit.to_owned()),
            )?;
            path.pop();
        }
    }

    Ok(value)
}

#[derive(Debug)]
enum MetaValue {
    Path(syn::Path),
    Lit(syn::Lit),
}

impl MetaValue {
    fn filter_path<F: FnOnce(&syn::Path) -> bool>(&self, callback: F) -> bool {
        if let Self::Path(path) = self {
            callback(path)
        } else {
            false
        }
    }
}

impl TryFrom<&syn::Variant> for CommandVariant {
    type Error = String;

    fn try_from(input: &syn::Variant) -> Result<Self, Self::Error> {
        if let syn::Fields::Unnamed(fields) = &input.fields {
            let mut field_iter = fields.unnamed.iter();

            let ty = field_iter
                .next()
                .map(|field| field.ty.to_owned())
                .ok_or_else(|| {
                    format!("{}: One field is required for tuple variants.", input.ident)
                })?;

            if let Some(field) = field_iter.next() {
                return Err(format!(
                    r#"{}: Only one field is supported for tuple variants, found unexpected type "{}"."#,
                    input.ident,
                    field.ty.to_token_stream(),
                ));
            }

            Ok(Self::Tuple(walk_attrs(
                &input.attrs,
                TupleCommandVariant {
                    ident: input.ident.to_owned(),
                    implements: Trait::Runnable,
                    ty,
                },
                &mut |mut variant, path, value| match path {
                    ["command", "implements"] => {
                        if value.filter_path(|path| path.is_ident("FromStr")) {
                            variant.implements = Trait::FromStr;
                            Ok(variant)
                        } else if value.filter_path(|path| path.is_ident("Runnable")) {
                            variant.implements = Trait::Runnable;
                            Ok(variant)
                        } else if value.filter_path(|path| path.is_ident("WordList")) {
                            variant.implements = Trait::WordList;
                            Ok(variant)
                        } else {
                            Err(format!("Unsupported trait: {:?}", value))
                        }
                    }
                    path if path.starts_with(&["command"]) => {
                        Err(format!("Unknown command attribute: {:?}", path))
                    }
                    _ => Ok(variant),
                },
            )?))
        } else {
            let variant = {
                let (fields, syntax) = match &input.fields {
                    syn::Fields::Named(input_fields) => {
                        let fields = input_fields
                            .named
                            .iter()
                            .map(|field| {
                                walk_attrs(
                                    &field.attrs,
                                    Field {
                                        ident: field.ident.clone().unwrap(),
                                        implements: Trait::Runnable,
                                        ty: field.ty.to_owned(),
                                    },
                                    &mut |mut field, path, value| match path {
                                        ["command", "implements"] => {
                                            if value.filter_path(|path| path.is_ident("FromStr")) {
                                                field.implements = Trait::FromStr;
                                                Ok(field)
                                            } else if value
                                                .filter_path(|path| path.is_ident("Runnable"))
                                            {
                                                field.implements = Trait::Runnable;
                                                Ok(field)
                                            } else if value
                                                .filter_path(|path| path.is_ident("WordList"))
                                            {
                                                field.implements = Trait::WordList;
                                                Ok(field)
                                            } else {
                                                Err(format!("Unsupported trait: {:?}", value))
                                            }
                                        }
                                        path if path.starts_with(&["command"]) => {
                                            Err(format!("Unknown command attribute: {:?}", path))
                                        }
                                        _ => Ok(field),
                                    },
                                )
                            })
                            .collect::<Result<_, String>>()?;

                        (fields, input.try_into().unwrap())
                    }
                    syn::Fields::Unit => (Vec::new(), input.try_into().unwrap()),
                    syn::Fields::Unnamed(_) => unreachable!(),
                };

                walk_attrs(
                    &input.attrs,
                    UnitStructCommandVariant {
                        ident: input.ident.to_owned(),

                        aliases: Vec::new(),
                        autocomplete_desc: None,
                        autocomplete_desc_fn: None,
                        doc: None,
                        fields,
                        is_ignored: false,
                        syntax,
                    },
                    &mut |mut variant, path, value| {
                        match (path, value) {
                            (&["doc"], MetaValue::Lit(syn::Lit::Str(lit_str))) => {
                                if let Some(mut doc) = variant.doc.take() {
                                    doc.push('\n');
                                    doc.push_str(lit_str.value().trim());
                                    variant.doc = Some(doc);
                                } else {
                                    variant.doc = Some(lit_str.value().trim().to_string());
                                }
                            }
                            (&["command", "alias"], MetaValue::Lit(syn::Lit::Str(lit_str))) => {
                                variant
                                    .aliases
                                    .push(parse_syntax(&lit_str.value(), &variant.fields)?);
                            }
                            (
                                &["command", "alias_no_autocomplete"],
                                MetaValue::Lit(syn::Lit::Str(lit_str)),
                            ) => {
                                let mut syntax = parse_syntax(&lit_str.value(), &variant.fields)?;
                                syntax.no_autocomplete = true;
                                variant.aliases.push(syntax);
                            }
                            (
                                &["command", "autocomplete_desc"],
                                MetaValue::Lit(syn::Lit::Str(lit_str)),
                            ) => {
                                variant.autocomplete_desc = Some(lit_str.value());
                            }
                            (&["command", "autocomplete_desc_fn"], MetaValue::Path(path)) => {
                                variant.autocomplete_desc_fn =
                                    Some(path.get_ident().unwrap().to_owned());
                            }
                            (&["command"], MetaValue::Path(path)) if path.is_ident("ignore") => {
                                variant.is_ignored = true;
                            }
                            (&["command"], MetaValue::Path(path))
                                if path.is_ident("no_default_autocomplete") =>
                            {
                                variant.syntax.no_autocomplete = true;
                            }
                            (&["command", "syntax"], MetaValue::Lit(syn::Lit::Str(lit_str))) => {
                                let no_autocomplete = variant.syntax.no_autocomplete;
                                variant.syntax = parse_syntax(&lit_str.value(), &variant.fields)?;
                                variant.syntax.no_autocomplete = no_autocomplete;
                            }
                            (path, _) if path.starts_with(&["command"]) => {
                                return Err(format!("Unsupported command attribute: {:?}.", path))
                            }
                            _ => {}
                        }
                        Ok(variant)
                    },
                )
            }
            .map_err(|e| format!("{}: {}", input.ident, e))?;

            if matches!(input.fields, syn::Fields::Unit) {
                Ok(Self::Unit(variant))
            } else if variant.fields.is_empty() {
                Err(format!(
                    "{}: Struct command types must have at least one attribute.",
                    variant.ident
                ))
            } else {
                Ok(Self::Struct(variant))
            }
        }
    }
}

impl TryFrom<&syn::Variant> for CommandVariantSyntax {
    type Error = ();

    fn try_from(input: &syn::Variant) -> Result<Self, Self::Error> {
        match &input.fields {
            syn::Fields::Unit => Ok(CommandVariantSyntax {
                start: Some(from_camel_case_with_sep(&input.ident, "-")),
                middle: Vec::new(),
                end: None,
                no_autocomplete: false,
            }),
            syn::Fields::Unnamed(_) => Err(()),
            syn::Fields::Named(named_fields) => {
                let mut end = None;
                Ok(CommandVariantSyntax {
                    start: Some(format!("{} ", from_camel_case_with_sep(&input.ident, "-"))),
                    middle: named_fields
                        .named
                        .iter()
                        .enumerate()
                        .filter_map(|(i, field)| {
                            if i + 1 == named_fields.named.len() {
                                end = Some(field.ident.clone().unwrap());
                                None
                            } else {
                                Some((field.ident.clone().unwrap(), " ".to_string()))
                            }
                        })
                        .collect(),
                    end: Some(end.unwrap()),
                    no_autocomplete: false,
                })
            }
        }
    }
}

impl fmt::Display for CommandVariantSyntax {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if let Some(s) = &self.start {
            write!(f, "{}", s)?;
        }

        self.middle
            .iter()
            .try_for_each(|(ident, s)| write!(f, "[{}]{}", ident, s))?;

        if let Some(ident) = &self.end {
            write!(f, "[{}]", ident)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use quote::quote;

    #[test]
    fn ident_test() {
        let command_enum = get_command_enum(quote! {
            enum Foo {
                Bar,
                BazQuux,
            }
        })
        .unwrap();

        assert_eq!("Foo", command_enum.ident.to_string());
    }

    #[test]
    fn alias_test() {
        let command_enum = get_command_enum(quote! {
            enum Foo {
                #[command(alias = "alias1")]
                #[command(alias_no_autocomplete = "alias2")]
                #[command(alias = "alias3")]
                Bar,
            }
        })
        .unwrap();

        match command_enum.variants.first() {
            Some(CommandVariant::Unit(variant)) => {
                let mut aliases = variant
                    .aliases
                    .iter()
                    .map(|alias| (alias.to_string(), alias.no_autocomplete));

                assert_eq!(Some(("alias1".to_string(), false)), aliases.next());
                assert_eq!(Some(("alias2".to_string(), true)), aliases.next());
                assert_eq!(Some(("alias3".to_string(), false)), aliases.next());
            }
            v => panic!("{:?}", v),
        }
    }

    #[test]
    fn doc_test() {
        let command_enum = get_command_enum(quote! {
            enum Foo {
                /// This is a doc
                /// comment.
                Bar,
            }
        })
        .unwrap();

        match command_enum.variants.first() {
            Some(CommandVariant::Unit(variant)) => {
                assert_eq!(Some("This is a doc\ncomment.".to_string()), variant.doc);
            }
            v => panic!("{:?}", v),
        }
    }

    #[test]
    fn fields_test() {
        let command_enum = get_command_enum(quote! {
            enum Foo {
                IHaveFields {
                    field_1: bool,
                    field_2: u8,
                },
                IHaveNoFields,
            }
        })
        .unwrap();

        let mut variants = command_enum.variants.iter();

        match variants.next() {
            Some(CommandVariant::Struct(variant)) => {
                let expected = vec![
                    ("field_1".to_string(), "bool".to_string()),
                    ("field_2".to_string(), "u8".to_string()),
                ];
                let mut actual = variant
                    .fields
                    .iter()
                    .map(|field| {
                        (
                            field.ident.to_string(),
                            field.ty.to_token_stream().to_string(),
                        )
                    })
                    .collect::<Vec<_>>();
                actual.sort();
                assert_eq!(expected, actual);
            }
            v => panic!("{:?}", v),
        }

        match variants.next() {
            Some(CommandVariant::Unit(variant)) => assert!(variant.fields.is_empty()),
            v => panic!("{:?}", v),
        }

        assert!(variants.next().is_none());
    }

    #[test]
    fn is_ignored_test() {
        let command_enum = get_command_enum(quote! {
            enum Foo {
                IsVisible,
                #[command(ignore)]
                IsIgnored,
            }
        })
        .unwrap();

        let mut variants = command_enum.variants.iter();

        match variants.next() {
            Some(CommandVariant::Unit(variant)) => {
                assert_eq!(false, variant.is_ignored);
            }
            v => panic!("{:?}", v),
        }

        match variants.next() {
            Some(CommandVariant::Unit(variant)) => {
                assert_eq!(true, variant.is_ignored);
            }
            v => panic!("{:?}", v),
        }

        assert!(variants.next().is_none());
    }

    #[test]
    fn no_default_autocomplete_test() {
        let command_enum = get_command_enum(quote! {
            enum Foo {
                DefaultSyntax,
                #[command(syntax = "blah")]
                CustomSyntax,

                #[command(no_default_autocomplete)]
                DefaultSyntaxHidden,
                #[command(syntax = "blah")]
                #[command(no_default_autocomplete)]
                CustomSyntaxHidden1,
                #[command(no_default_autocomplete)]
                #[command(syntax = "blah")]
                CustomSyntaxHidden2,
            }
        })
        .unwrap();

        let mut variants = command_enum.variants.iter();

        for _ in 0..2 {
            match variants.next() {
                Some(CommandVariant::Unit(variant)) => {
                    assert!(!variant.syntax.no_autocomplete);
                }
                v => panic!("{:?}", v),
            }
        }

        for _ in 2..5 {
            match variants.next() {
                Some(CommandVariant::Unit(variant)) => {
                    assert!(variant.syntax.no_autocomplete);
                }
                v => panic!("{:?}", v),
            }
        }

        assert!(variants.next().is_none());
    }

    #[test]
    fn tuple_variant_test() {
        let command_enum = get_command_enum(quote! {
            enum Foo {
                IAmATuple(String),
            }
        })
        .unwrap();

        let mut variants = command_enum.variants.iter();

        match variants.next() {
            Some(CommandVariant::Tuple(variant)) => {
                assert_eq!("IAmATuple", variant.ident.to_string());
                assert_eq!("String", variant.ty.to_token_stream().to_string());
            }
            v => panic!("{:?}", v),
        }
    }

    #[test]
    fn syntax_test_simple() {
        let command_enum = get_command_enum(quote! {
            enum Foo {
                DefaultSyntax,
                #[command(syntax = "custom syntaxxx")]
                CustomSyntax,
            }
        })
        .unwrap();

        let mut variants = command_enum.variants.iter();

        match variants.next() {
            Some(CommandVariant::Unit(variant)) => {
                assert_eq!("default-syntax", variant.syntax.to_string());
            }
            v => panic!("{:?}", v),
        }

        match variants.next() {
            Some(CommandVariant::Unit(variant)) => {
                assert_eq!("custom syntaxxx", variant.syntax.to_string());
            }
            v => panic!("{:?}", v),
        }

        assert!(variants.next().is_none());
    }

    #[test]
    fn syntax_test_with_fields() {
        let command_enum = get_command_enum(quote! {
            enum Foo {
                DefaultWithFields {
                    foo: String,
                    bar: String,
                },
                #[command(syntax = "[foo] blah [bar]")]
                #[command(alias = "blah [bar] [foo]")]
                SyntaxWithFields {
                    foo: bool,
                    bar: bool,
                },
            }
        })
        .unwrap();

        let mut variants = command_enum.variants.iter();

        match variants.next() {
            Some(CommandVariant::Struct(variant)) => {
                assert_eq!(
                    "default-with-fields [foo] [bar]",
                    variant.syntax.to_string(),
                );
            }
            v => panic!("{:?}", v),
        }

        match variants.next() {
            Some(CommandVariant::Struct(variant)) => {
                assert_eq!("[foo] blah [bar]", variant.syntax.to_string());
                assert_eq!(
                    "blah [bar] [foo]",
                    variant.aliases.first().unwrap().to_string(),
                );
            }
            v => panic!("{:?}", v),
        }

        assert!(variants.next().is_none());
    }

    #[test]
    fn syntax_test_tuple_implements() {
        let command_enum = get_command_enum(quote! {
            enum Foo {
                RunnableTuple(bool),

                #[command(implements(FromStr))]
                FromStrTuple(bool),

                #[command(implements(Runnable))]
                AnotherRunnableTuple(bool),

                #[command(implements(WordList))]
                WordListTuple(bool),
            }
        })
        .unwrap();

        let mut variants = command_enum.variants.iter();

        match variants.next() {
            Some(CommandVariant::Tuple(variant)) => {
                assert_eq!(Trait::Runnable, variant.implements);
            }
            v => panic!("{:?}", v),
        }

        match variants.next() {
            Some(CommandVariant::Tuple(variant)) => {
                assert_eq!(Trait::FromStr, variant.implements);
            }
            v => panic!("{:?}", v),
        }

        match variants.next() {
            Some(CommandVariant::Tuple(variant)) => {
                assert_eq!(Trait::Runnable, variant.implements);
            }
            v => panic!("{:?}", v),
        }

        match variants.next() {
            Some(CommandVariant::Tuple(variant)) => {
                assert_eq!(Trait::WordList, variant.implements);
            }
            v => panic!("{:?}", v),
        }

        assert!(variants.next().is_none());
    }

    #[test]
    fn syntax_test_field_implements() {
        let command_enum = get_command_enum(quote! {
            enum Foo {
                Variant {
                    runnable_field: bool,

                    #[command(implements(FromStr))]
                    from_str_field: bool,

                    #[command(implements(Runnable))]
                    another_runnable_field: bool,

                    #[command(implements(WordList))]
                    word_list_field: bool,
                }
            }
        })
        .unwrap();

        match command_enum.variants.first() {
            Some(CommandVariant::Struct(variant)) => {
                let mut traits = variant.fields.iter().map(|field| &field.implements);

                assert_eq!(Some(&Trait::Runnable), traits.next());
                assert_eq!(Some(&Trait::FromStr), traits.next());
                assert_eq!(Some(&Trait::Runnable), traits.next());
                assert_eq!(Some(&Trait::WordList), traits.next());
                assert_eq!(None, traits.next());
            }
            v => panic!("{:?}", v),
        }
    }

    #[test]
    fn syntax_test_autocomplete_desc() {
        let command_enum = get_command_enum(quote! {
            enum Foo {
                JustAVariant,

                #[command(autocomplete_desc = "Description")]
                ADescribedVariant,

                #[command(autocomplete_desc_fn(some_function))]
                ASpecialVariant,
            }
        })
        .unwrap();

        let mut variants = command_enum.variants.iter();

        match variants.next() {
            Some(CommandVariant::Unit(variant)) => {
                assert!(variant.autocomplete_desc.is_none());
                assert!(variant.autocomplete_desc_fn.is_none());
            }
            v => panic!("{:?}", v),
        }

        match variants.next() {
            Some(CommandVariant::Unit(variant)) => {
                assert_eq!("Description", variant.autocomplete_desc.as_ref().unwrap());
                assert!(variant.autocomplete_desc_fn.is_none());
            }
            v => panic!("{:?}", v),
        }

        match variants.next() {
            Some(CommandVariant::Unit(variant)) => {
                assert!(variant.autocomplete_desc.is_none());
                assert_eq!(
                    "some_function",
                    variant.autocomplete_desc_fn.as_ref().unwrap().to_string(),
                );
            }
            v => panic!("{:?}", v),
        }
    }

    #[test]
    fn syntax_test_unknown_field() {
        assert_eq!(
            r#"Error parsing Foo::NoFields: Unknown or duplicated field in "foo [bar]": "bar"."#,
            get_command_enum(quote! {
                enum Foo {
                    #[command(syntax = "foo [bar]")]
                    NoFields,
                }
            })
            .unwrap_err(),
        );
    }

    #[test]
    fn syntax_test_duplicate_field() {
        assert_eq!(
            r#"Error parsing Foo::DuplicatedField: Unknown or duplicated field in "foo [bar] [bar]": "bar"."#,
            get_command_enum(quote! {
                enum Foo {
                    #[command(syntax = "foo [bar] [bar]")]
                    DuplicatedField {
                        bar: bool,
                    },
                }
            })
            .unwrap_err(),
        );
    }

    #[test]
    fn syntax_test_missing_field() {
        assert_eq!(
            r#"Error parsing Foo::MissingField: Field "bar" is not accounted for in syntax "foo"."#,
            get_command_enum(quote! {
                enum Foo {
                    #[command(syntax = "foo")]
                    MissingField {
                        bar: bool,
                    },
                }
            })
            .unwrap_err(),
        );

        assert_eq!(
            r#"Error parsing Foo::MissingField: Field "bar" is not accounted for in syntax "baz"."#,
            get_command_enum(quote! {
                enum Foo {
                    #[command(syntax = "foo [bar]")]
                    #[command(alias = "baz")]
                    MissingField {
                        bar: bool,
                    },
                }
            })
            .unwrap_err(),
        );
    }

    #[test]
    fn syntax_text_unclosed_bracket() {
        assert_eq!(
            r#"Error parsing Foo::UnclosedBracket: Unclosed bracket in "foo [bar"."#,
            get_command_enum(quote! {
                enum Foo {
                    #[command(syntax = "foo [bar")]
                    UnclosedBracket {
                        bar: bool,
                    },
                }
            })
            .unwrap_err(),
        );
    }

    #[test]
    fn syntax_test_unbalanced_brackets() {
        assert_eq!(
            r#"Error parsing Foo::Unbalanced: Unbalanced brackets in "foo [bar["."#,
            get_command_enum(quote! {
                enum Foo {
                    #[command(syntax = "foo [bar[")]
                    Unbalanced,
                }
            })
            .unwrap_err(),
        );

        assert_eq!(
            r#"Error parsing Foo::Unbalanced: Unbalanced brackets in "foo ] bar"."#,
            get_command_enum(quote! {
                enum Foo {
                    #[command(syntax = "foo ] bar")]
                    Unbalanced,
                }
            })
            .unwrap_err(),
        );
    }

    #[test]
    fn tuple_variant_test_wrong_argument_count() {
        assert_eq!(
            r#"Error parsing Foo::TooFew: One field is required for tuple variants."#,
            get_command_enum(quote! {
                enum Foo {
                    JustRight(bool),
                    TooFew(),
                }
            })
            .unwrap_err(),
        );

        assert_eq!(
            r#"Error parsing Foo::TooMany: Only one field is supported for tuple variants, found unexpected type "u16"."#,
            get_command_enum(quote! {
                enum Foo {
                    JustRight(bool),
                    TooMany(u8, u16),
                }
            })
            .unwrap_err(),
        );
    }

    #[test]
    fn struct_variant_test_success() {
        let command_struct = get_command_struct(quote! {
            struct Foo(Vec<a::b::C>);
        })
        .unwrap();

        let ident: syn::Ident = syn::parse2(quote! { Foo }).unwrap();
        assert_eq!(ident, command_struct.ident);

        let subtype: syn::Path = syn::parse2(quote! { a::b::C }).unwrap();
        assert_eq!(subtype, command_struct.subtype);
    }

    #[test]
    fn struct_variant_test_failure_only_tuples() {
        assert_eq!(
            "Error parsing Foo: Only tuple structs are supported.",
            get_command_struct(quote! {
                struct Foo;
            })
            .unwrap_err(),
        );

        assert_eq!(
            "Error parsing Foo: Only tuple structs are supported.",
            get_command_struct(quote! {
                struct Foo {
                    blah: Vec<a::b::C>,
                }
            })
            .unwrap_err(),
        );
    }

    #[test]
    fn struct_variant_test_failure_one_field_required() {
        assert_eq!(
            "Error parsing Foo: Tuple structs must have exactly one field.",
            get_command_struct(quote! {
                struct Foo();
            })
            .unwrap_err(),
        );

        assert_eq!(
            "Error parsing Foo: Tuple structs must have exactly one field.",
            get_command_struct(quote! {
                struct Foo(Vec<A>, Vec<B>);
            })
            .unwrap_err(),
        );
    }

    #[test]
    fn struct_variant_test_failure_wrong_wrapper() {
        assert_eq!(
            "Error parsing Foo: Field type must be Vec.",
            get_command_struct(quote! {
                struct Foo(Option<a::b::C>);
            })
            .unwrap_err(),
        );
    }

    fn get_command_enum(input: TokenStream) -> Result<CommandEnum, String> {
        if let Command::Enum(command_enum) = Command::try_from(input)? {
            Ok(command_enum)
        } else {
            Err("Input type parses as a struct, not an enum.".to_string())
        }
    }

    fn get_command_struct(input: TokenStream) -> Result<CommandStruct, String> {
        if let Command::Struct(command_struct) = Command::try_from(input)? {
            Ok(command_struct)
        } else {
            Err("Input type parses as an enum, not a struct.".to_string())
        }
    }
}
