use std::iter::Iterator;

pub trait Autocomplete {
    fn autocomplete(input: &str) -> Vec<String>;
}

pub fn autocomplete_phrase(
    input: &str,
    vocabulary: &mut dyn Iterator<Item = &&str>,
) -> Vec<String> {
    if input.is_empty() {
        Vec::new()
    } else {
        vocabulary
            .filter(|word| word.starts_with(input))
            .take(10)
            .map(|&s| s.to_string())
            .collect()
    }
}

/*
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
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn autocomplete_phrase_test() {
        let words = [
            "Solanum aethiopicum",
            "Solanum betaceum",
            "Solanum cheesmanii",
            "Solanum chilense",
            "Solanum galapagense",
            "Solanum lycocarpum",
            "Solanum lycopersicum",
            "Solanum melongena",
            "Solanum muricatum",
            "Solanum peruvianum",
            "Solanum pimpinellifolium",
            "Solanum quitoense",
            "Solanum scabrum",
            "Solanum torvum",
            "Solanum tuberosum",
            "Tamarillo",
            "Turkey berry",
            "bush tomato",
            "desert raisin",
            "eggplant",
            "garden huckleberry",
            "gilo",
            "kangaroo apple",
            "naranjilla",
            "pepino melon",
            "potato bush",
            "potato",
            "tomato",
            "wild tomato",
            "wolf apple",
        ];
        let empty_vec: Vec<String> = Vec::new();

        assert_eq!(
            vec!["potato bush", "potato"],
            autocomplete_phrase("pot", &mut words.iter()),
        );

        assert_eq!(
            vec!["potato bush"],
            autocomplete_phrase("potato ", &mut words.iter()),
        );

        assert_eq!(
            empty_vec,
            autocomplete_phrase("my tasty potato", &mut words.iter()),
        );

        assert_eq!(
            vec!["Tamarillo", "Turkey berry"],
            autocomplete_phrase("T", &mut words.iter()),
        );

        assert_eq!(
            vec![
                "Solanum aethiopicum",
                "Solanum betaceum",
                "Solanum cheesmanii",
                "Solanum chilense",
                "Solanum galapagense",
                "Solanum lycocarpum",
                "Solanum lycopersicum",
                "Solanum melongena",
                "Solanum muricatum",
                "Solanum peruvianum",
            ],
            autocomplete_phrase("Solanum", &mut words.iter()),
        );

        assert_eq!(empty_vec, autocomplete_phrase("", &mut words.iter()));
        assert_eq!(empty_vec, autocomplete_phrase("carrot", &mut words.iter()));
        assert_eq!(
            empty_vec,
            autocomplete_phrase("\u{1f954}\u{2003}\u{1f954}", &mut words.iter()),
        );
    }
}
