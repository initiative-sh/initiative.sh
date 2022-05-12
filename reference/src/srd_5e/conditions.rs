use super::write_text_block;
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct Condition {
    index: String,
    pub name: String,

    desc: Vec<String>,
}

pub struct SummaryView<'a>(&'a Condition);

pub struct DetailsView<'a>(&'a Condition);

impl Condition {
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

impl<'a> fmt::Display for SummaryView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let condition = self.0;
        write!(f, "`{}`", condition.name)
    }
}

impl<'a> fmt::Display for DetailsView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let condition = self.0;

        write!(f, "# {}\n\n", condition.name)?;
        write_text_block(f, &condition.desc[..])?;

        Ok(())
    }
}
