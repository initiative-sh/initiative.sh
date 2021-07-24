use crate::app::{autocomplete_phrase, Context, Runnable};
use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
pub enum ReferenceCommand {
    OpenGameLicense,
}

impl Runnable for ReferenceCommand {
    fn run(&self, _context: &mut Context, _rng: &mut impl Rng) -> String {
        match self {
            Self::OpenGameLicense => include_str!("../../../reference/data/ogl-1.0a.txt")
                .trim_end()
                .to_string(),
        }
    }

    fn summarize(&self) -> &str {
        match self {
            Self::OpenGameLicense => "SRD license",
        }
    }

    fn parse_input(input: &str, _context: &Context) -> Vec<Self> {
        if "Open Game License" == input {
            vec![Self::OpenGameLicense]
        } else {
            Vec::new()
        }
    }

    fn autocomplete(input: &str, context: &Context) -> Vec<(String, Self)> {
        let mut suggestions = autocomplete_phrase(input, &mut ["Open Game License"].iter());

        suggestions.sort();
        suggestions.truncate(10);

        suggestions
            .iter()
            .flat_map(|s| std::iter::repeat(s).zip(Self::parse_input(s.as_str(), context)))
            .map(|(s, c)| (s.clone(), c))
            .collect()
    }
}
