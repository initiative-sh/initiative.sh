pub use command::ReferenceCommand;

mod command;

use initiative_macros::reference_enum;

reference_enum!(Spell);
