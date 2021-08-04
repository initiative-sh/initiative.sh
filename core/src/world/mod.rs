pub mod demographics;
pub mod location;
pub mod npc;
pub mod region;

pub use command::WorldCommand;
pub use demographics::Demographics;
pub use field::Field;
pub use location::Location;
pub use npc::Npc;
pub use region::Region;
pub use thing::Thing;

mod command;
mod field;
mod thing;

use rand::Rng;

pub trait Generate: Default {
    fn generate(rng: &mut impl Rng, demographics: &Demographics) -> Self {
        let mut result = Self::default();
        result.regenerate(rng, demographics);
        result
    }

    fn regenerate(&mut self, rng: &mut impl Rng, demographics: &Demographics);
}

trait PopulateFields {
    fn populate_fields(&mut self, rng: &mut impl Rng, demographics: &Demographics);
}
