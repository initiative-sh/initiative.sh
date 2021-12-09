//! The `#[derive(Autocomplete)]` and `#[derive(ContextAwareParse)]` macros generate parsing and
//! autocomplete code based on an enum of command variants. All variant types are supported:
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
//! - `#[doc = "blah"]` or `/// blah`: User-facing documentation for the command.
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

pub mod context_aware_parse;

use proc_macro2::TokenStream;
use quote::ToTokens;
use std::collections::HashMap;
use std::fmt;
use std::iter;

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
    doc: Option<String>,
    fields: Vec<Field>,
    is_ignored: bool,
    syntax: CommandVariantSyntax,
}

#[derive(Debug)]
struct CommandVariantSyntax {
    pub syntax_parts: Vec<CommandVariantSyntaxPart>,
    pub no_autocomplete: bool,
}

#[derive(Debug)]
enum CommandVariantSyntaxPart {
    Str(String),
    Ident(syn::Ident),
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
}

impl TryFrom<TokenStream> for CommandEnum {
    type Error = String;

    fn try_from(input_raw: TokenStream) -> Result<Self, Self::Error> {
        let input: syn::DeriveInput = syn::parse2(input_raw).map_err(|e| format!("{}", e))?;
        let ident = input.ident;

        if let syn::Data::Enum(data_enum) = input.data {
            let variants = data_enum
                .variants
                .iter()
                .map(|v| v.try_into())
                .collect::<Result<_, _>>()
                .map_err(|e| format!("Error parsing {}::{}", ident, e))?;
            Ok(Self { ident, variants })
        } else {
            Err(format!("Error parsing {}: Type must be enum.", ident))
        }
    }
}

fn parse_syntax(syntax: &str, fields: &[Field]) -> Result<Vec<CommandVariantSyntaxPart>, String> {
    let mut is_ident = false;
    let mut start = 0;
    let mut parts = Vec::new();
    let mut unmatched_fields: HashMap<String, syn::Ident> = fields
        .iter()
        .map(|f| (f.ident.to_string(), f.ident.clone()))
        .collect();

    for (i, c) in syntax
        .char_indices()
        .filter(|(_, c)| ['[', ']'].contains(c))
    {
        match (is_ident, c) {
            (false, '[') => {
                if !syntax[start..i].trim().is_empty() {
                    parts.push(CommandVariantSyntaxPart::Str(
                        syntax[start..i].trim().to_string(),
                    ));
                }

                is_ident = true;
            }
            (true, ']') => {
                if let Some(ident) = unmatched_fields.remove(&syntax[start..i]) {
                    parts.push(CommandVariantSyntaxPart::Ident(ident));
                    is_ident = false;
                } else {
                    return Err(format!(
                        r#"Unknown or duplicated field in "{}": "{}"."#,
                        syntax,
                        &syntax[start..i],
                    ));
                }
            }
            _ => return Err(format!(r#"Unbalanced brackets in "{}"."#, syntax)),
        }

        start = i + 1;
    }

    if is_ident {
        return Err(format!(r#"Unclosed bracket in "{}"."#, syntax));
    }

    if !syntax[start..].trim().is_empty() {
        parts.push(CommandVariantSyntaxPart::Str(
            syntax[start..].trim().to_string(),
        ));
    }

    if let Some(missing_field) = unmatched_fields.into_keys().next() {
        Err(format!(
            r#"Field "{}" is not accounted for in syntax "{}"."#,
            missing_field, syntax
        ))
    } else {
        Ok(parts)
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
                let (fields, syntax_parts) = match &input.fields {
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

                        (
                            fields,
                            iter::once(CommandVariantSyntaxPart::Str(from_camel_case_with_sep(
                                &input.ident,
                                "-",
                            )))
                            .chain(
                                input_fields.named.iter().map(|f| {
                                    CommandVariantSyntaxPart::Ident(f.ident.clone().unwrap())
                                }),
                            )
                            .collect(),
                        )
                    }
                    syn::Fields::Unit => (
                        Vec::new(),
                        vec![CommandVariantSyntaxPart::Str(from_camel_case_with_sep(
                            &input.ident,
                            "-",
                        ))],
                    ),
                    syn::Fields::Unnamed(_) => unreachable!(),
                };

                walk_attrs(
                    &input.attrs,
                    UnitStructCommandVariant {
                        ident: input.ident.to_owned(),

                        aliases: Vec::new(),
                        doc: None,
                        fields,
                        is_ignored: false,
                        syntax: CommandVariantSyntax {
                            syntax_parts,
                            no_autocomplete: false,
                        },
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
                                variant.aliases.push(CommandVariantSyntax {
                                    syntax_parts: parse_syntax(&lit_str.value(), &variant.fields)?,
                                    no_autocomplete: false,
                                });
                            }
                            (
                                &["command", "alias_no_autocomplete"],
                                MetaValue::Lit(syn::Lit::Str(lit_str)),
                            ) => {
                                variant.aliases.push(CommandVariantSyntax {
                                    syntax_parts: parse_syntax(&lit_str.value(), &variant.fields)?,
                                    no_autocomplete: true,
                                });
                            }
                            (&["command"], MetaValue::Path(path)) if path.is_ident("ignore") => {
                                variant.is_ignored = true;
                            }
                            (&["command", "syntax"], MetaValue::Lit(syn::Lit::Str(lit_str))) => {
                                variant.syntax = CommandVariantSyntax {
                                    syntax_parts: parse_syntax(&lit_str.value(), &variant.fields)?,
                                    no_autocomplete: variant.syntax.no_autocomplete,
                                }
                            }
                            (&["command"], MetaValue::Path(path))
                                if path.is_ident("no_default_autocomplete") =>
                            {
                                variant.syntax.no_autocomplete = true;
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

impl fmt::Display for CommandVariantSyntax {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for (i, part) in self.syntax_parts.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }

            match part {
                CommandVariantSyntaxPart::Str(s) => write!(f, "{}", s)?,
                CommandVariantSyntaxPart::Ident(id) => write!(f, "[{}]", id)?,
            }
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
        let command_enum: CommandEnum = quote! {
            enum Foo {
                Bar,
                BazQuux,
            }
        }
        .try_into()
        .unwrap();

        assert_eq!("Foo", command_enum.ident.to_string());
    }

    #[test]
    fn alias_test() {
        let command_enum: CommandEnum = quote! {
            enum Foo {
                #[command(alias = "alias1")]
                #[command(alias_no_autocomplete = "alias2")]
                #[command(alias = "alias3")]
                Bar,
            }
        }
        .try_into()
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
        let command_enum: CommandEnum = quote! {
            enum Foo {
                /// This is a doc
                /// comment.
                Bar,
            }
        }
        .try_into()
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
        let command_enum: CommandEnum = quote! {
            enum Foo {
                IHaveFields {
                    field_1: bool,
                    field_2: u8,
                },
                IHaveNoFields,
            }
        }
        .try_into()
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
        let command_enum: CommandEnum = quote! {
            enum Foo {
                IsVisible,
                #[command(ignore)]
                IsIgnored,
            }
        }
        .try_into()
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
        let command_enum: CommandEnum = quote! {
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
        }
        .try_into()
        .unwrap();

        let mut variants = command_enum.variants.iter();

        for _ in 0..2 {
            match variants.next() {
                Some(CommandVariant::Unit(variant)) => {
                    assert_eq!(false, variant.syntax.no_autocomplete);
                }
                v => panic!("{:?}", v),
            }
        }

        for _ in 2..5 {
            match variants.next() {
                Some(CommandVariant::Unit(variant)) => {
                    assert_eq!(true, variant.syntax.no_autocomplete);
                }
                v => panic!("{:?}", v),
            }
        }

        assert!(variants.next().is_none());
    }

    #[test]
    fn tuple_variant_test() {
        let command_enum = CommandEnum::try_from(quote! {
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
        let command_enum: CommandEnum = quote! {
            enum Foo {
                DefaultSyntax,
                #[command(syntax = "custom syntaxxx")]
                CustomSyntax,
            }
        }
        .try_into()
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
        let command_enum: CommandEnum = quote! {
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
        }
        .try_into()
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
        let command_enum = CommandEnum::try_from(quote! {
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
        let command_enum = CommandEnum::try_from(quote! {
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
    fn syntax_test_unknown_field() {
        assert_eq!(
            r#"Error parsing Foo::NoFields: Unknown or duplicated field in "foo [bar]": "bar"."#,
            CommandEnum::try_from(quote! {
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
            CommandEnum::try_from(quote! {
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
            CommandEnum::try_from(quote! {
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
            CommandEnum::try_from(quote! {
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
            CommandEnum::try_from(quote! {
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
            CommandEnum::try_from(quote! {
                enum Foo {
                    #[command(syntax = "foo [bar[")]
                    Unbalanced,
                }
            })
            .unwrap_err(),
        );

        assert_eq!(
            r#"Error parsing Foo::Unbalanced: Unbalanced brackets in "foo ] bar"."#,
            CommandEnum::try_from(quote! {
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
            CommandEnum::try_from(quote! {
                enum Foo {
                    JustRight(bool),
                    TooFew(),
                }
            })
            .unwrap_err(),
        );

        assert_eq!(
            r#"Error parsing Foo::TooMany: Only one field is supported for tuple variants, found unexpected type "u16"."#,
            CommandEnum::try_from(quote! {
                enum Foo {
                    JustRight(bool),
                    TooMany(u8, u16),
                }
            })
            .unwrap_err(),
        );
    }
}
