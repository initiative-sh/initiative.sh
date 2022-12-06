//! A Rust representation of a subset of the data provided by the D&D 5e API. In all cases, the
//! appropriate JSON file is parsed by Serde into a Vec of structs representing a particular
//! reference type, such as spells or magic items.
//!
//! This serves as a dependency of the `initiative_macros` crate, specifically the `reference_enum`
//! macro. As a result, it only runs at compile time.

pub mod srd_5e;

fn to_camel_case(input: &str) -> String {
    let mut word_break = true;
    let mut result = String::with_capacity(input.len());
    for c in input.chars() {
        if c.is_alphanumeric() {
            if word_break {
                c.to_uppercase().for_each(|c| result.push(c));
            } else {
                result.push(c);
            }
            word_break = false;
        } else {
            word_break = c != '\'';
        }
    }
    result.shrink_to_fit();
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_camel_case_test() {
        assert_eq!("Potato", to_camel_case("potato"));
        assert_eq!("PotatoSpud", to_camel_case("potato/spud"));
        assert_eq!("FooBar", to_camel_case("foo  ~~~~  bar"));
        assert_eq!("ArcanistsMagicAura", to_camel_case("arcanist's magic aura"));
        assert_eq!("", to_camel_case(""));
    }
}
