pub use equipment::{Item, ItemCategory, MagicItem};
pub use spell::Spell;
pub use std::fmt;

mod equipment;
mod spell;

use serde::Deserialize;

pub fn spells() -> Result<Vec<Spell>, String> {
    serde_json::from_str(include_str!("../../../data/srd_5e/src/5e-SRD-Spells.json"))
        .map_err(|e| format!("{}", e))
}

pub fn items() -> Result<Vec<Item>, String> {
    serde_json::from_str(include_str!(
        "../../../data/srd_5e/src/5e-SRD-Equipment.json",
    ))
    .map_err(|e| format!("{}", e))
}

pub fn item_categories() -> Result<Vec<ItemCategory>, String> {
    serde_json::from_str(include_str!(
        "../../../data/srd_5e/src/5e-SRD-Equipment-Categories.json",
    ))
    .map_err(|e| format!("{}", e))
}

pub fn magic_items() -> Result<Vec<MagicItem>, String> {
    serde_json::from_str(include_str!(
        "../../../data/srd_5e/src/5e-SRD-Magic-Items.json",
    ))
    .map_err(|e| format!("{}", e))
}

#[derive(Debug, Deserialize)]
pub struct Reference {
    index: String,
    name: String,
    pub url: String,
}

impl Reference {
    pub fn token(&self) -> String {
        crate::to_camel_case(&self.index)
    }
}

fn write_text_block(f: &mut fmt::Formatter, lines: &[String]) -> fmt::Result {
    let mut prev_line: Option<&str> = None;

    let is_list = |l: &str| l.starts_with("- ") || l.starts_with("* ") || l.starts_with('|');

    for line in lines.iter() {
        if prev_line.is_some() {
            if !prev_line.map_or(false, is_list) || !is_list(line) {
                writeln!(f)?;
            }
            writeln!(f)?;
        }
        write!(f, "{}", line)?;
        prev_line = Some(line);
    }

    Ok(())
}
