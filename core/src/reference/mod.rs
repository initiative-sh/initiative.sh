pub use command::ReferenceCommand;

mod command;

use initiative_macros::reference_enum;

reference_enum!(Item);

reference_enum!(ItemCategory);

reference_enum!(Spell);

reference_enum!(MagicItem);
