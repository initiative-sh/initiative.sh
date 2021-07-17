use std::iter::Iterator;

pub trait Autocomplete {
    fn autocomplete(input: &str) -> Vec<String>;
}

pub fn autocomplete_words(input: &str, vocabulary: &mut dyn Iterator<Item = &&str>) -> Vec<String> {
    let (start, partial) = input.split_at(
        input
            .rfind(char::is_whitespace)
            .map(|i| {
                ((i + 1)..input.len())
                    .find(|&i| input.is_char_boundary(i))
                    .unwrap_or(input.len())
            })
            .unwrap_or(0),
    );

    if partial.is_empty() {
        return Vec::new();
    }

    vocabulary
        .filter_map(|word| {
            if word.starts_with(partial) {
                let mut suggestion = String::with_capacity(start.len() + partial.len());
                suggestion.push_str(start);
                suggestion.push_str(word);
                Some(suggestion)
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn autocomplete_test() {
        let words = ["potato", "sweet potato", "Potato", "potato salad"];
        let empty_vec: Vec<String> = Vec::new();

        assert_eq!(
            vec!["potato", "potato salad"],
            autocomplete_words("pot", &mut words.iter()),
        );

        assert_eq!(
            vec!["my tasty potato", "my tasty potato salad"],
            autocomplete_words("my tasty potato", &mut words.iter()),
        );

        assert_eq!(vec!["Potato"], autocomplete_words("Pot", &mut words.iter()));

        assert_eq!(empty_vec, autocomplete_words("", &mut words.iter()));
        assert_eq!(empty_vec, autocomplete_words("carrot", &mut words.iter()));
        assert_eq!(empty_vec, autocomplete_words("potato ", &mut words.iter()));
        assert_eq!(empty_vec, autocomplete_words("🥔", &mut words.iter()));

        // Non-ASCII whitespace
        assert_eq!(
            vec!["foo\u{2003}sweet potato"],
            autocomplete_words("foo\u{2003}s", &mut words.iter())
        );
    }
}
