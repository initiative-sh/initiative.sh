use super::ParsedThing;
use crate::app::{AppMeta, Autocomplete, AutocompleteSuggestion};
use crate::utils::{quoted_words, CaseInsensitiveStr};
use crate::world::npc::{Age, Ethnicity, Gender, Npc, Species};
use crate::world::place::{Place, PlaceType};
use crate::world::Thing;
use async_trait::async_trait;
use std::collections::HashSet;
use std::str::FromStr;

struct ParsedInput<'a> {
    name_desc: &'a str,
    name: &'a str,
    desc: &'a str,
    desc_lower: Option<String>,
    partial: &'a str,
}

impl<'a> ParsedInput<'a> {
    fn suggestion(&self, suggestion: &str) -> String {
        format!("{}{}", self.name_desc, suggestion)
    }

    fn desc_lower(&self) -> &str {
        if let Some(s) = &self.desc_lower {
            s.as_str()
        } else {
            self.desc
        }
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

        let desc = &input[name_desc_split..desc_partial_split];

        Self {
            name_desc: &input[..desc_partial_split],
            name: &input[..name_desc_split],
            desc,
            desc_lower: if desc.chars().any(char::is_uppercase) {
                Some(desc.to_lowercase())
            } else {
                None
            },
            partial: &input[desc_partial_split..],
        }
    }
}

fn autocomplete_trailing_name<T: FromStr + Into<Thing>>(
    input: &str,
) -> Option<AutocompleteSuggestion> {
    if !quoted_words(input)
        .skip(1)
        .any(|word| word.as_str().in_ci(&["named", "called"]))
    {
        return None;
    }

    let mut input_iter = input.split_inclusive(char::is_whitespace).rev();
    let len_named = input_iter
        .find_map(|s| {
            if s.trim().in_ci(&["named", "called"]) {
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
            Some(AutocompleteSuggestion::new(suggestion, "specify a name"))
        } else {
            Some(AutocompleteSuggestion::new(
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
) -> Vec<AutocompleteSuggestion> {
    if let Some(result) = autocomplete_trailing_name::<T>(input) {
        return vec![result];
    }

    const ARTICLES: &[&str] = &["a", "an"];

    let parsed: ParsedInput = input.into();

    if parsed.partial.is_empty() || parsed.partial.in_ci(ARTICLES) {
        // Ends with a space or ignored word - suggest new word categories
        if quoted_words(parsed.desc).all(|word| word.as_str().in_ci(ARTICLES)) {
            let thing: Thing = T::default().into();

            let suggestion = format!(
                "{}{}[{} description]",
                input,
                if input.ends_with(|c: char| !c.is_whitespace()) {
                    " "
                } else {
                    ""
                },
                thing.as_str(),
            );

            vec![AutocompleteSuggestion::new(
                suggestion,
                format!("create {}", thing.display_description()),
            )]
        } else if let Ok(thing) = parsed.name_desc.parse::<T>().map(|t| t.into()) {
            let mut suggestions = Vec::new();

            let words: HashSet<&str> = quoted_words(parsed.desc_lower())
                .map(|word| word.as_own_str(parsed.desc_lower()))
                .collect();

            if thing.name().is_none() {
                suggestions.push(AutocompleteSuggestion::new(
                    parsed.suggestion("named [name]"),
                    "specify a name",
                ));
            }

            for (placeholder, description, terms) in vocabulary {
                if !terms.iter().any(|term| words.contains(term)) {
                    suggestions.push(AutocompleteSuggestion::new(
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
            quoted_words(parsed.desc_lower())
                .map(|word| word.as_own_str(parsed.desc_lower()))
                .filter(|s| s != &parsed.partial && !s.in_ci(ARTICLES))
                .collect()
        };

        if words.is_empty() || parsed.name_desc.parse::<T>().is_ok() {
            vocabulary
                .iter()
                .filter(|(_, _, terms)| !terms.iter().any(|term| words.contains(term)))
                .flat_map(|(_, _, terms)| terms.iter())
                .chain(basic_terms.iter().filter(|term| !words.contains(*term)))
                .filter(|term| term.starts_with_ci(parsed.partial))
                .map(|term| parsed.suggestion(term))
                .filter_map(|term| {
                    if let Ok(thing) = term.parse::<T>().map(|t| t.into()) {
                        Some(AutocompleteSuggestion::new(
                            term,
                            format!("create {}", thing.display_description()),
                        ))
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
                    .filter(|s| s.starts_with_ci(parsed.partial))
                    .map(|s| AutocompleteSuggestion::new(parsed.suggestion(s), "specify a name")),
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
            .filter(|s| s.starts_with_ci(parsed.partial))
            .filter_map(|term| {
                let suggestion = parsed.suggestion(term);
                suggestion.parse::<T>().ok().map(|thing| {
                    AutocompleteSuggestion::new(
                        suggestion,
                        format!("create {}", thing.into().display_description()),
                    )
                })
            })
            .collect::<HashSet<_>>()
            .drain()
            .collect()
    }
}

#[async_trait(?Send)]
impl Autocomplete for Place {
    async fn autocomplete(input: &str, _app_meta: &AppMeta) -> Vec<AutocompleteSuggestion> {
        autocomplete_terms::<ParsedThing<Place>>(
            input,
            &["place"],
            &[(
                "place type",
                "specify a place type (eg. inn)",
                &PlaceType::get_words().collect::<Vec<_>>(),
            )],
        )
    }
}

#[async_trait(?Send)]
impl Autocomplete for Npc {
    async fn autocomplete(input: &str, _app_meta: &AppMeta) -> Vec<AutocompleteSuggestion> {
        if let Some(word) = quoted_words(input).last().filter(|w| {
            let s = w.as_str();
            s.starts_with(|c: char| c.is_ascii_digit())
                && "-year-old".starts_with_ci(s.trim_start_matches(|c: char| c.is_ascii_digit()))
        }) {
            let term = {
                let word_str = word.as_str();
                format!(
                    "{}{}-year-old",
                    &input[..word.range().start],
                    &word_str[..word_str
                        .find(|c: char| !c.is_ascii_digit())
                        .unwrap_or(word_str.len())]
                )
            };

            if let Some(summary) =
                term.parse::<ParsedThing<Thing>>()
                    .ok()
                    .and_then(|parsed_thing| {
                        parsed_thing
                            .thing
                            .data
                            .npc_data()
                            .map(|npc| format!("create {}", npc.display_description()))
                    })
            {
                vec![AutocompleteSuggestion::new(term, summary)]
            } else {
                Vec::new()
            }
        } else {
            autocomplete_terms::<ParsedThing<Npc>>(
                input,
                &["character", "npc", "person"],
                &[
                    (
                        "age",
                        "specify an age (eg. \"elderly\")",
                        &Age::get_words().collect::<Vec<_>>(),
                    ),
                    (
                        "ethnicity",
                        "specify an ethnicity (eg. \"elvish\")",
                        &Ethnicity::get_words().collect::<Vec<_>>(),
                    ),
                    (
                        "gender",
                        "specify a gender",
                        &Gender::get_words().collect::<Vec<_>>(),
                    ),
                    (
                        "species",
                        "specify a species (eg. \"dwarf\")",
                        &Species::get_words().collect::<Vec<_>>(),
                    ),
                ],
            )
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::app::assert_autocomplete;
    use crate::storage::NullDataStore;
    use crate::Event;
    use tokio_test::block_on;

    #[test]
    fn parsed_input_suggestion_test() {
        assert_eq!(
            "Foo, an inn",
            ParsedInput {
                name_desc: "Foo, an ",
                name: "Foo, ",
                desc: "an ",
                desc_lower: None,
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
    fn place_autocomplete_test() {
        assert_autocomplete(
            &[
                ("inn", "create inn"),
                ("imports-shop", "create imports-shop"),
                ("island", "create island"),
            ][..],
            block_on(Place::autocomplete("i", &app_meta())),
        );

        assert_autocomplete(
            &[
                ("an inn", "create inn"),
                ("an imports-shop", "create imports-shop"),
                ("an island", "create island"),
            ][..],
            block_on(Place::autocomplete("an i", &app_meta())),
        );

        assert_autocomplete(
            &[("an inn named [name]", "specify a name")][..],
            block_on(Place::autocomplete("an inn n", &app_meta())),
        );

        assert_eq!(
            Vec::<AutocompleteSuggestion>::new(),
            block_on(Place::autocomplete("a streetcar named desire", &app_meta())),
        );

        assert_eq!(
            Vec::<AutocompleteSuggestion>::new(),
            block_on(Place::autocomplete("Foo, an inn n", &app_meta())),
        );
    }

    #[test]
    fn place_autocomplete_test_typing() {
        {
            let input = "a bar called Heaven";
            let app_meta = app_meta();

            for i in 2..input.len() {
                assert_ne!(
                    Vec::<AutocompleteSuggestion>::new(),
                    block_on(Place::autocomplete(&input[..i], &app_meta)),
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
                    Vec::<AutocompleteSuggestion>::new(),
                    block_on(Place::autocomplete(&input[..i], &app_meta)),
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
            block_on(Npc::autocomplete("elf ", &app_meta())),
        );

        assert_autocomplete(
            &[
                ("human [age]", "specify an age (eg. \"elderly\")"),
                ("human [gender]", "specify a gender"),
                ("human named [name]", "specify a name"),
            ][..],
            block_on(Npc::autocomplete("human ", &app_meta())),
        );
    }

    #[test]
    fn npc_autocomplete_test_typing() {
        let input = "an elderly elvish dwarf woman named Tiramisu";
        let app_meta = app_meta();

        for i in 3..input.len() {
            assert_ne!(
                Vec::<AutocompleteSuggestion>::new(),
                block_on(Npc::autocomplete(&input[..i], &app_meta)),
                "Input: {}",
                &input[..i],
            );
        }
    }

    fn event_dispatcher(_event: Event) {}

    fn app_meta() -> AppMeta {
        AppMeta::new(NullDataStore::default(), &event_dispatcher)
    }
}
