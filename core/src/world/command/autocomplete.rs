use crate::app::{AppMeta, Autocomplete};
use crate::utils::quoted_words;
use crate::world::location::{BuildingType, Location};
use crate::world::npc::{Age, Ethnicity, Gender, Npc, Species};
use crate::world::Thing;
use std::collections::HashSet;
use std::str::FromStr;

struct ParsedInput<'a> {
    name_desc: &'a str,
    name: &'a str,
    desc: &'a str,
    partial: &'a str,
}

impl<'a> ParsedInput<'a> {
    fn suggestion(&self, suggestion: &str) -> String {
        format!("{}{}", self.name_desc, suggestion)
    }
}

impl<'a> From<&'a str> for ParsedInput<'a> {
    fn from(input: &'a str) -> Self {
        let name_desc_split = if let Some(comma_pos) = input.rfind(',').map(|i| i + ','.len_utf8())
        {
            if let Some(non_whitespace_pos) = input[comma_pos..].find(|c: char| !c.is_whitespace())
            {
                comma_pos + non_whitespace_pos
            } else {
                input.len()
            }
        } else {
            0
        };

        let desc_partial_split = if input.ends_with(|c: char| c == ',' || c.is_whitespace()) {
            input.len()
        } else {
            quoted_words(input)
                .last()
                .map_or_else(|| input.len(), |word| word.range().start)
        };

        Self {
            name_desc: &input[..desc_partial_split],
            name: &input[..name_desc_split],
            desc: &input[name_desc_split..desc_partial_split],
            partial: &input[desc_partial_split..],
        }
    }
}

fn autocomplete_trailing_name<T: FromStr + Into<Thing>>(input: &str) -> Option<(String, String)> {
    if !quoted_words(input)
        .skip(1)
        .any(|word| ["named", "called"].contains(&word.as_str()))
    {
        return None;
    }

    let mut input_iter = input.split_inclusive(char::is_whitespace).rev();
    let len_named = input_iter
        .find_map(|s| {
            if ["named", "called"].contains(&s.trim()) {
                Some(s.len())
            } else {
                None
            }
        })
        .unwrap();
    let before_pos: usize = input_iter.map(|s| s.len()).sum();
    let after_pos = before_pos + len_named;

    if let Ok(thing) = input[..before_pos].trim().parse::<T>().map(|t| t.into()) {
        if after_pos >= input.trim_end().len() && thing.name().is_none() {
            let mut suggestion = input.to_string();
            if !suggestion.ends_with(char::is_whitespace) {
                suggestion.push(' ');
            }
            suggestion.push_str("[name]");
            Some((suggestion, "specify a name".to_string()))
        } else {
            Some((
                input.to_string(),
                format!("create {}", thing.display_description()),
            ))
        }
    } else {
        None
    }
}

fn autocomplete_terms<T: Default + FromStr + Into<Thing>>(
    input: &str,
    basic_terms: &[&str],
    vocabulary: &[(&str, &str, &[&str])],
) -> Vec<(String, String)> {
    if let Some(result) = autocomplete_trailing_name::<T>(input) {
        return vec![result];
    }

    const ARTICLES: &[&str] = &["a", "an"];

    let parsed: ParsedInput = input.into();

    if parsed.partial.is_empty() || ARTICLES.contains(&parsed.partial) {
        // Ends with a space or ignored word - suggest new word categories
        if quoted_words(parsed.desc).all(|word| ARTICLES.contains(&word.as_str())) {
            let thing: Thing = T::default().into();

            let suggestion = format!(
                "{}{}[{} description]",
                input,
                if input.ends_with(|c: char| !c.is_whitespace()) {
                    " "
                } else {
                    ""
                },
                thing.display_description(),
            );

            vec![(
                suggestion,
                format!("create {}", thing.display_description()),
            )]
        } else if let Ok(thing) = parsed.name_desc.parse::<T>().map(|t| t.into()) {
            let mut suggestions = Vec::new();

            let words: HashSet<&str> = quoted_words(parsed.desc)
                .map(|w| w.as_own_str(parsed.desc))
                .collect();

            if thing.name().is_none() {
                suggestions.push((
                    parsed.suggestion("named [name]"),
                    "specify a name".to_string(),
                ));
            }

            for (placeholder, description, terms) in vocabulary {
                if !terms.iter().any(|term| words.contains(term)) {
                    suggestions.push((
                        parsed.suggestion(&format!("[{}]", placeholder)),
                        description.to_string(),
                    ));
                }
            }

            suggestions
        } else {
            Vec::new()
        }
    } else if !parsed.desc.is_empty() {
        // Multiple words: make suggestions if existing words made sense.
        let words: HashSet<&str> = {
            quoted_words(parsed.desc)
                .map(|w| w.as_own_str(parsed.desc))
                .filter(|s| s != &parsed.partial && !ARTICLES.contains(s))
                .collect()
        };

        if words.is_empty() || parsed.name_desc.parse::<T>().is_ok() {
            vocabulary
                .iter()
                .filter(|(_, _, terms)| !terms.iter().any(|term| words.contains(term)))
                .flat_map(|(_, _, terms)| terms.iter())
                .chain(basic_terms.iter().filter(|term| !words.contains(*term)))
                .filter(|term| term.starts_with(parsed.partial))
                .map(|term| parsed.suggestion(term))
                .filter_map(|suggestion| {
                    if let Ok(thing) = suggestion.parse::<T>().map(|t| t.into()) {
                        Some((suggestion, format!("create {}", thing.display_summary())))
                    } else {
                        None
                    }
                })
                .chain(
                    if parsed.name.is_empty() {
                        &["named [name]", "called [name]"][..]
                    } else {
                        &[][..]
                    }
                    .iter()
                    .filter(|s| s.starts_with(parsed.partial))
                    .map(|s| (parsed.suggestion(s), "specify a name".to_string())),
                )
                .collect::<HashSet<_>>()
                .drain()
                .collect()
        } else {
            Vec::new()
        }
    } else {
        // First word, autocomplete all known vocabulary
        vocabulary
            .iter()
            .flat_map(|(_, _, terms)| terms.iter())
            .chain(basic_terms.iter())
            .filter(|s| s.starts_with(parsed.partial))
            .filter_map(|term| {
                let suggestion = parsed.suggestion(term);
                suggestion.parse::<T>().ok().map(|thing| {
                    (
                        suggestion,
                        format!("create {}", thing.into().display_summary()),
                    )
                })
            })
            .collect::<HashSet<_>>()
            .drain()
            .collect()
    }
}

impl Autocomplete for Location {
    fn autocomplete(input: &str, _app_meta: &AppMeta) -> Vec<(String, String)> {
        autocomplete_terms::<Location>(
            input,
            &["location", "place"],
            &[(
                "location type",
                "specify a location type (eg. inn)",
                BuildingType::get_words(),
            )],
        )
    }
}

impl Autocomplete for Npc {
    fn autocomplete(input: &str, _app_meta: &AppMeta) -> Vec<(String, String)> {
        if let Some(word) = quoted_words(input).last().filter(|w| {
            let s = w.as_str();
            s.starts_with(|c: char| c.is_ascii_digit())
                && "-year-old".starts_with(s.trim_start_matches(|c: char| c.is_ascii_digit()))
        }) {
            let suggestion = {
                let word_str = word.as_str();
                format!(
                    "{}{}-year-old",
                    &input[..word.range().start],
                    &word_str[..word_str
                        .find(|c: char| !c.is_ascii_digit())
                        .unwrap_or(word_str.len())]
                )
            };

            if let Ok(npc) = suggestion.parse::<Npc>() {
                vec![(suggestion, format!("create {}", npc.display_summary()))]
            } else {
                Vec::new()
            }
        } else {
            autocomplete_terms::<Npc>(
                input,
                &["character", "npc", "person"],
                &[
                    ("age", "specify an age (eg. \"elderly\")", Age::get_words()),
                    (
                        "ethnicity",
                        "specify an ethnicity (eg. \"elvish\")",
                        Ethnicity::get_words(),
                    ),
                    ("gender", "specify a gender", Gender::get_words()),
                    (
                        "species",
                        "specify a species (eg. \"dwarf\")",
                        Species::get_words(),
                    ),
                ],
            )
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::storage::NullDataStore;

    #[test]
    fn parsed_input_suggestion_test() {
        assert_eq!(
            "Foo, an inn",
            ParsedInput {
                name_desc: "Foo, an ",
                name: "Foo, ",
                desc: "an ",
                partial: "i",
            }
            .suggestion("inn"),
        );
    }

    #[test]
    fn parsed_input_test_empty() {
        let parsed_input: ParsedInput = "".into();
        assert_eq!("", parsed_input.name_desc);
        assert_eq!("", parsed_input.name);
        assert_eq!("", parsed_input.desc);
        assert_eq!("", parsed_input.partial);
    }

    #[test]
    fn parsed_input_test_one_word() {
        let parsed_input: ParsedInput = "foo".into();
        assert_eq!("", parsed_input.name_desc);
        assert_eq!("", parsed_input.name);
        assert_eq!("", parsed_input.desc);
        assert_eq!("foo", parsed_input.partial);
    }

    #[test]
    fn parsed_input_test_multiple_words() {
        let parsed_input: ParsedInput = "foo bar baz".into();
        assert_eq!("foo bar ", parsed_input.name_desc);
        assert_eq!("", parsed_input.name);
        assert_eq!("foo bar ", parsed_input.desc);
        assert_eq!("baz", parsed_input.partial);
    }

    #[test]
    fn parsed_input_test_trailing_whitespace() {
        let parsed_input: ParsedInput = "foo bar baz ".into();
        assert_eq!("foo bar baz ", parsed_input.name_desc);
        assert_eq!("", parsed_input.name);
        assert_eq!("foo bar baz ", parsed_input.desc);
        assert_eq!("", parsed_input.partial);
    }

    #[test]
    fn parsed_input_test_name_only() {
        let parsed_input: ParsedInput = "Foo, ".into();
        assert_eq!("Foo, ", parsed_input.name_desc);
        assert_eq!("Foo, ", parsed_input.name);
        assert_eq!("", parsed_input.desc);
        assert_eq!("", parsed_input.partial);
    }

    #[test]
    fn parsed_input_test_name_trailing_word() {
        let parsed_input: ParsedInput = "Foo, bar".into();
        assert_eq!("Foo, ", parsed_input.name_desc);
        assert_eq!("Foo, ", parsed_input.name);
        assert_eq!("", parsed_input.desc);
        assert_eq!("bar", parsed_input.partial);
    }

    #[test]
    fn parsed_input_test_name_trailing_words() {
        let parsed_input: ParsedInput = "Foo, a bar".into();
        assert_eq!("Foo, a ", parsed_input.name_desc);
        assert_eq!("Foo, ", parsed_input.name);
        assert_eq!("a ", parsed_input.desc);
        assert_eq!("bar", parsed_input.partial);
    }

    #[test]
    fn parsed_input_test_name_trailing_whitespace() {
        let parsed_input: ParsedInput = "Foo, a bar ".into();
        assert_eq!("Foo, a bar ", parsed_input.name_desc);
        assert_eq!("Foo, ", parsed_input.name);
        assert_eq!("a bar ", parsed_input.desc);
        assert_eq!("", parsed_input.partial);
    }

    #[test]
    fn location_autocomplete_test() {
        assert_autocomplete(
            &[("inn", "create inn")][..],
            Location::autocomplete("i", &app_meta()),
        );

        assert_autocomplete(
            &[("an inn", "create inn")][..],
            Location::autocomplete("an i", &app_meta()),
        );

        assert_autocomplete(
            &[("an inn named [name]", "specify a name")][..],
            Location::autocomplete("an inn n", &app_meta()),
        );

        assert_eq!(
            Vec::<(String, String)>::new(),
            Location::autocomplete("a streetcar named desire", &app_meta()),
        );

        assert_eq!(
            Vec::<(String, String)>::new(),
            Location::autocomplete("Foo, an inn n", &app_meta()),
        );
    }

    #[test]
    fn location_autocomplete_test_typing() {
        {
            let input = "a bar called Heaven";
            let app_meta = app_meta();

            for i in 2..input.len() {
                assert_ne!(
                    Vec::<(String, String)>::new(),
                    Location::autocomplete(&input[..i], &app_meta),
                    "Input: {}",
                    &input[..i],
                );
            }
        }

        {
            let input = "Foo, inn";
            let app_meta = app_meta();

            for i in 4..input.len() {
                assert_ne!(
                    Vec::<(String, String)>::new(),
                    Location::autocomplete(&input[..i], &app_meta),
                    "Input: {}",
                    &input[..i],
                );
            }
        }
    }

    #[test]
    fn autocomplete_test_npc() {
        assert_autocomplete(
            &[
                ("elf [age]", "specify an age (eg. \"elderly\")"),
                ("elf [ethnicity]", "specify an ethnicity (eg. \"elvish\")"),
                ("elf [gender]", "specify a gender"),
                ("elf named [name]", "specify a name"),
            ][..],
            Npc::autocomplete("elf ", &app_meta()),
        );

        assert_autocomplete(
            &[
                ("human [age]", "specify an age (eg. \"elderly\")"),
                ("human [gender]", "specify a gender"),
                ("human named [name]", "specify a name"),
            ][..],
            Npc::autocomplete("human ", &app_meta()),
        );
    }

    #[test]
    fn npc_autocomplete_test_typing() {
        let input = "an elderly elvish dwarf woman named Tiramisu";
        let app_meta = app_meta();

        for i in 3..input.len() {
            assert_ne!(
                Vec::<(String, String)>::new(),
                Npc::autocomplete(&input[..i], &app_meta),
                "Input: {}",
                &input[..i],
            );
        }
    }

    fn app_meta() -> AppMeta {
        AppMeta::new(NullDataStore::default())
    }

    fn assert_autocomplete(
        expected_suggestions: &[(&str, &str)],
        actual_suggestions: Vec<(String, String)>,
    ) {
        let mut expected: Vec<_> = expected_suggestions
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect();
        expected.sort();

        let mut actual = actual_suggestions;
        actual.sort();

        assert_eq!(expected, actual);
    }
}
