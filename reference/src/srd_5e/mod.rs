pub use spell::Spell;
pub use std::fmt;

mod spell;

use serde::Deserialize;

pub fn spells() -> Result<Vec<Spell>, String> {
    serde_json::from_str(include_str!("../../../data/srd_5e/src/5e-SRD-Spells.json"))
        .map_err(|e| format!("{}", e))
}

#[derive(Debug, Deserialize)]
pub struct Reference {
    //index: String,
    name: String,
    //url: String,
}

fn write_text_block(f: &mut fmt::Formatter, lines: &[String]) -> fmt::Result {
    let mut prev_line: Option<&str> = None;

    for line in lines.iter() {
        if prev_line.is_some() {
            if !prev_line.map_or(false, |l| l.starts_with(&['-', '*'][..]))
                || !line.starts_with(&['-', '*'][..])
            {
                writeln!(f)?;
            }
            writeln!(f)?;
        }
        write!(f, "{}", line)?;
        prev_line = Some(line);
    }

    Ok(())
}
