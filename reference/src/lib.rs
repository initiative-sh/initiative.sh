pub mod srd_5e;

fn to_camel_case(input: &str) -> String {
    capitalize(input)
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect()
}

fn capitalize(input: &str) -> String {
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
            result.push(c);
            word_break = c != '\'';
        }
    }
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

    #[test]
    fn capitalize_test() {
        assert_eq!("Potato", capitalize("potato"));
        assert_eq!("Potato/Spud", capitalize("potato/spud"));
        assert_eq!("Foo  ~~~~  Bar", capitalize("foo  ~~~~  bar"));
        assert_eq!("Arcanist's Magic Aura", capitalize("arcanist's magic aura"));
        assert_eq!("", capitalize(""));
    }
}
