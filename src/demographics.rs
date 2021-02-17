use super::RandomTable;
use initiative_macros::RandomTable;

pub struct Demographics {
    wealth: Wealth,
    size: u8,
}

impl Default for Demographics {
    fn default() -> Self {
        Self {
            wealth: Wealth::Modest,
            size: 127,
        }
    }
}

#[derive(RandomTable)]
enum Wealth {
    Wretched,
    Squalid,
    Poor,
    Modest,
    Comfortable,
    Wealthy,
    Aristocratic,
}
