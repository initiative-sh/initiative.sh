use crate::srd_5e::{write_text_block, Reference};
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct MagicItem {
    index: String,
    name: String,

    #[serde(default)]
    desc: Vec<String>,

    equipment_category: Reference,
}

impl MagicItem {
    pub fn name(&self) -> String {
        crate::capitalize(&self.name)
    }

    pub fn token(&self) -> String {
        crate::to_camel_case(&self.index)
    }

    pub fn display_summary(&self) -> SummaryView {
        SummaryView(self)
    }

    pub fn display_details(&self) -> DetailsView {
        DetailsView(self)
    }
}

pub struct SummaryView<'a>(&'a MagicItem);

pub struct DetailsView<'a>(&'a MagicItem);

impl<'a> fmt::Display for SummaryView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let magic_item = &self.0;

        write!(f, "`{}`", magic_item.name())?;

        Ok(())
    }
}

impl<'a> fmt::Display for DetailsView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let magic_item = self.0;

        writeln!(f, "# {}", magic_item.name())?;

        if let Some(line) = magic_item.desc.first() {
            writeln!(f, "\n*{}*", line)?;
        }

        if let Some(chunk) = magic_item.desc.get(1..) {
            writeln!(f)?;
            write_text_block(f, chunk)?;
        }

        Ok(())
    }
}
