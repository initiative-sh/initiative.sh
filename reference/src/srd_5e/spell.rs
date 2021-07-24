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

impl Spell {
    pub fn name(&self) -> String {
        crate::capitalize(self.name.as_str())
    }

    pub fn token(&self) -> String {
        crate::to_camel_case(self.index.as_str())
    }
}

impl fmt::Display for Spell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())?;

        match (self.level, self.school.get("name").unwrap()) {
            (0, s) => write!(f, "\n{} cantrip", s)?,
            (1, s) => write!(f, "\n1st-level {}", s.to_lowercase())?,
            (2, s) => write!(f, "\n2nd-level {}", s.to_lowercase())?,
            (3, s) => write!(f, "\n3rd-level {}", s.to_lowercase())?,
            (l, s) => write!(f, "\n{}th-level {}", l, s.to_lowercase())?,
        }

        if self.ritual {
            write!(f, " (ritual)")?;
        }

        write!(f, "\n\nCasting Time: {}", self.casting_time)?;

        {
            write!(f, "\nRange: {}", self.range)?;
            if let Some(aoe) = &self.area_of_effect {
                if let (Some(aoe_type), Some(aoe_size)) = (
                    aoe.get("type").map(|v| v.as_str()).flatten(),
                    aoe.get("size").map(|v| v.as_u64()).flatten(),
                ) {
                    write!(f, " ({}' {})", aoe_size, aoe_type)?;
                }
            }
        }

        {
            let mut component_iter = self.components.iter();
            if let Some(c) = component_iter.next() {
                write!(f, "\nComponents: {}", c)?;
                component_iter
                    .map(|c| write!(f, ", {}", c))
                    .collect::<Result<_, _>>()?;

                if let Some(m) = &self.material {
                    write!(f, " ({})", m.trim_end_matches('.').to_lowercase())?;
                }
            }
        }

        if self.concentration {
            write!(
                f,
                "\nDuration: Concentration, {}",
                self.duration.to_lowercase(),
            )?;
        } else {
            write!(f, "\nDuration: {}", self.duration)?;
        }

        self.desc
            .iter()
            .map(|line| write!(f, "\n\n{}", line))
            .collect::<Result<_, _>>()?;

        self.higher_level
            .iter()
            .enumerate()
            .map(|(i, line)| {
                if i == 0 {
                    write!(f, "\n\nAt higher levels: {}", line)
                } else {
                    write!(f, "\n\n{}", line)
                }
            })
            .collect::<Result<_, _>>()?;

        Ok(())
    }
}
