use super::write_text_block;
use serde::Deserialize;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct Spell {
    index: String,
    name: String,

    level: u8,

    #[serde(default)]
    school: HashMap<String, String>,

    casting_time: String,
    range: String,
    area_of_effect: Option<HashMap<String, JsonValue>>,
    components: Vec<char>,
    material: Option<String>,
    duration: String,

    #[serde(default)]
    desc: Vec<String>,

    #[serde(default)]
    higher_level: Vec<String>,

    #[serde(default)]
    ritual: bool,

    #[serde(default)]
    concentration: bool,
}

pub struct SummaryView<'a>(&'a Spell);

pub struct DetailsView<'a>(&'a Spell);

impl Spell {
    pub fn name(&self) -> String {
        crate::capitalize(self.name.as_str())
    }

    pub fn token(&self) -> String {
        crate::to_camel_case(self.index.as_str())
    }

    pub fn display_summary(&self) -> SummaryView {
        SummaryView(self)
    }

    pub fn display_details(&self) -> DetailsView {
        DetailsView(self)
    }

    fn get_level_school(&self) -> String {
        match (self.level, self.school.get("name").unwrap().to_lowercase()) {
            (0, s) => format!("{} cantrip", s),
            (1, s) => format!("1st-level {}", s),
            (2, s) => format!("2nd-level {}", s),
            (3, s) => format!("3rd-level {}", s),
            (l, s) => format!("{}th-level {}", l, s),
        }
    }
}

impl<'a> fmt::Display for SummaryView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let spell = self.0;
        write!(f, "`{}` ({})", spell.name(), spell.get_level_school())
    }
}

impl<'a> fmt::Display for DetailsView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let spell = self.0;

        write!(f, "# {}\n*{}", spell.name(), {
            let level_school = spell.get_level_school();
            let mut chars_iter = level_school.chars();
            format_args!(
                "{}{}",
                chars_iter.next().unwrap().to_uppercase(),
                chars_iter.collect::<String>(),
            )
        })?;

        if spell.ritual {
            write!(f, " (ritual)")?;
        }

        write!(f, "*\n\n**Casting Time:** {}", spell.casting_time)?;

        {
            write!(f, "\\\n**Range:** {}", spell.range)?;
            if let Some(aoe) = &spell.area_of_effect {
                if let (Some(aoe_type), Some(aoe_size)) = (
                    aoe.get("type").map(|v| v.as_str()).flatten(),
                    aoe.get("size").map(|v| v.as_u64()).flatten(),
                ) {
                    write!(f, " ({}' {})", aoe_size, aoe_type)?;
                }
            }
        }

        {
            let mut component_iter = spell.components.iter();
            if let Some(c) = component_iter.next() {
                write!(f, "\\\n**Components:** {}", c)?;
                component_iter.try_for_each(|c| write!(f, ", {}", c))?;

                if let Some(m) = &spell.material {
                    write!(f, " ({})", m.trim_end_matches('.').to_lowercase())?;
                }
            }
        }

        if spell.concentration {
            write!(
                f,
                "\\\n**Duration:** Concentration, {}",
                spell.duration.to_lowercase(),
            )?;
        } else {
            write!(f, "\\\n**Duration:** {}", spell.duration)?;
        }

        if !spell.desc.is_empty() {
            write!(f, "\n\n")?;
            write_text_block(f, &spell.desc[..])?;
        }

        if !spell.higher_level.is_empty() {
            write!(f, "\n\n***At higher levels:*** ")?;
            write_text_block(f, &spell.higher_level[..])?;
        }

        Ok(())
    }
}
