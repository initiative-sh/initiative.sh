//! Rust requires macros to reside in a separate crate with the `proc-macro` flag set. Therefore,
//! enter the `initiative_macros` crate, which exists exclusively to support the `initiative_core`
//! crate.
//!
//! Some macros are large, others are small. There's no particular rhyme or reason to code
//! organization here; they're just all dependencies in one way or another.

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod changelog;
mod motd;
mod reference_enum;
mod token_marker;
mod word_list;

/// A microoptimization to compile only part of the lengthy `changelog.md` file into the
/// application binary.
#[proc_macro]
pub fn changelog(input: TokenStream) -> TokenStream {
    changelog::run(input).unwrap()
}

/// A microoptimization that generates the welcome message as a static string combined from several
/// sources. In retrospect, there wasn't much point to making this a macro.
#[proc_macro]
pub fn motd(input: TokenStream) -> TokenStream {
    motd::run(input).unwrap()
}

/// Generate an enum containing all subtypes of a given reference topic, eg. all spells. This
/// dosen't generate the `spell` listing, but it does automatically generate `Fireball` (and all of
/// the other SRD spells) based on the `initiative_reference` crate.
#[proc_macro]
pub fn reference_enum(input: TokenStream) -> TokenStream {
    reference_enum::run(input).unwrap()
}

#[proc_macro_derive(TokenMarker)]
pub fn token_marker(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    token_marker::run(input).unwrap().into()
}

/// There are a lot of enums containing lists of terms scattered throughout the application. In
/// order to surface those terms to the user, adding `#[derive(WordList)]` will provide the
/// following functions:
///
/// * `get_words()`
/// * `as_str()`
/// * `word_count()`
/// * `parse_cs()`
///
/// It also implements the following traits:
///
/// * `std::str::FromStr`
/// * `std::convert::TryFrom<&str>` (for use by Serde, I think)
/// * `From<T> for &'static str`
/// * `From<T> for String`
///
/// Recognized attributes on enum variants are:
///
/// * `#[term = "abc"]` - Overrides the automatically-generated term for the variant
/// * `#[alias = "abc"]` - Defines an additional string that will be parsed as this variant
#[proc_macro_derive(WordList, attributes(alias, term))]
pub fn word_list(input: TokenStream) -> TokenStream {
    word_list::run(input).unwrap()
}
