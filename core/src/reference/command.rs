use super::Spell;
use crate::app::{autocomplete_phrase, Context, Runnable};
use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
pub enum ReferenceCommand {
    Spell(Spell),
    OpenGameLicense,
}

impl Runnable for ReferenceCommand {
    fn run(&self, _context: &mut Context, _rng: &mut impl Rng) -> String {
        match self {
            Self::Spell(spell) => format!(
                "{}\n\n{} is Open Game Content subject to the `Open Game License`.",
                spell,
                spell.get_name(),
            ),
            Self::OpenGameLicense => include_str!("../../../data/ogl-1.0a.txt")
                .trim_end()
                .to_string(),
        }
    }

    fn summarize(&self) -> &str {
        match self {
            Self::Spell(_) => "SRD spell",
            Self::OpenGameLicense => "SRD license",
        }
    }

    fn parse_input(input: &str, _context: &Context) -> Vec<Self> {
        if "Open Game License" == input {
            vec![Self::OpenGameLicense]
        } else if let Ok(spell) = input.parse() {
            vec![Self::Spell(spell)]
        } else {
            Vec::new()
        }
    }

    fn autocomplete(input: &str, context: &Context) -> Vec<(String, Self)> {
        let mut suggestions = autocomplete_phrase(
            input,
            &mut ["Open Game License"]
                .iter()
                .chain(Spell::get_words().iter()),
        );

        suggestions.sort();
        suggestions.truncate(10);

        suggestions
            .iter()
            .flat_map(|s| std::iter::repeat(s).zip(Self::parse_input(s.as_str(), context)))
            .map(|(s, c)| (s.clone(), c))
            .collect()
    }
}
