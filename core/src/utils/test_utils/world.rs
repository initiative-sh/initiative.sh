macro_rules! builder {
    ($thing: path | $thing_data: path, ($($f:ident: $t:path),*)) => {
        use crate::world::{Generate, Demographics};

        #[derive(Default)]
        pub struct Builder($thing_data);

        impl Builder {
            #[allow(dead_code)]
            pub fn build(self) -> $thing_data {
                self.0
            }

            #[allow(dead_code)]
            pub fn build_thing_data(self) -> crate::world::thing::ThingData {
                self.build().into()
            }

            #[allow(dead_code)]
            pub fn build_with_uuid(self, uuid: ::uuid::Uuid) -> $thing {
                $thing {
                    uuid,
                    data: self.0,
                }
            }

            #[allow(dead_code)]
            pub fn build_thing(self, uuid: ::uuid::Uuid) -> crate::world::thing::Thing {
                self.build_with_uuid(uuid).into()
            }

            #[allow(dead_code)]
            pub fn generate(
                mut self,
                rng: &mut impl ::rand::Rng,
            ) -> $thing_data {
                self.0.regenerate(rng, &Demographics::default());
                self.0
            }

            #[allow(dead_code)]
            pub fn name<S: std::fmt::Display>(mut self, name: S) -> Self {
                self.0.name = name.to_string().into();
                self
            }

            $(
                #[allow(dead_code)]
                pub fn $f<T: Into<crate::world::Field<$t>>>(mut self, $f: T) -> Self {
                    self.0.$f = $f.into();
                    self
                }
            )*
        }
    };
}

pub mod npc {
    use crate::world::npc::{Age, Ethnicity, Gender, Npc, NpcData, Size, Species};
    use uuid::Uuid;

    #[allow(unused_imports)]
    pub use super::odyssey::{odysseus, penelope, polyphemus};

    #[allow(unused_imports)]
    pub use super::odyssey::{ODYSSEUS, PENELOPE, POLYPHEMUS};

    #[expect(dead_code)]
    pub fn empty() -> NpcData {
        NpcData::default()
    }

    pub fn build() -> builder::Builder {
        builder::Builder::default()
    }

    mod builder {
        use super::*;

        builder!(
            Npc | NpcData,
            (
                age: Age,
                ethnicity: Ethnicity,
                gender: Gender,
                location_uuid: Uuid,
                size: Size,
                species: Species
            )
        );
    }
}

pub mod place {
    use crate::world::place::{Place, PlaceData, PlaceType};
    use uuid::Uuid;

    #[allow(unused_imports)]
    pub use super::odyssey::{greece, ithaca};

    #[allow(unused_imports)]
    pub use super::odyssey::{GREECE, ITHACA};

    #[expect(dead_code)]
    pub fn empty() -> PlaceData {
        PlaceData::default()
    }

    pub fn build() -> builder::Builder {
        builder::Builder::default()
    }

    mod builder {
        use super::*;

        builder!(
            Place | PlaceData,
            (
                location_uuid: Uuid,
                subtype: PlaceType,
                description: String
            )
        );
    }
}

pub mod thing {
    use crate::world::thing::Thing;

    #[expect(unused_imports)]
    pub use super::odyssey::{GREECE, ITHACA, ODYSSEUS, PENELOPE, POLYPHEMUS};

    macro_rules! thing {
        ($thing: ident) => {
            pub fn $thing() -> Thing {
                super::odyssey::$thing().into()
            }
        };
    }

    thing!(greece);
    thing!(ithaca);
    thing!(odysseus);
    thing!(penelope);
    thing!(polyphemus);
}

mod odyssey {
    use super::*;

    use crate::world::npc::{Age, Ethnicity, Gender, Npc, Size, Species};
    use crate::world::place::Place;
    use uuid::Uuid;

    pub const ITHACA: Uuid = Uuid::from_u128(0x01);
    pub const GREECE: Uuid = Uuid::from_u128(0x02);
    pub const ODYSSEUS: Uuid = Uuid::from_u128(0x11);
    pub const PENELOPE: Uuid = Uuid::from_u128(0x12);
    pub const POLYPHEMUS: Uuid = Uuid::from_u128(0x13);

    pub fn ithaca() -> Place {
        place::build()
            .name("Greece")
            .location_uuid(GREECE)
            .build_with_uuid(ITHACA)
    }

    pub fn greece() -> Place {
        place::build().name("Greece").build_with_uuid(GREECE)
    }

    pub fn odysseus() -> Npc {
        npc::build()
            .name("Odysseus")
            .age(Age::MiddleAged)
            .ethnicity(Ethnicity::Human)
            .gender(Gender::Masculine)
            .size(Size::Medium {
                height: 72,
                weight: 180,
            })
            .species(Species::Human)
            .build_with_uuid(ODYSSEUS)
    }

    pub fn penelope() -> Npc {
        npc::build()
            .name("Penelope")
            .age(Age::MiddleAged)
            .ethnicity(Ethnicity::Human)
            .gender(Gender::Feminine)
            .location_uuid(ITHACA)
            .size(Size::Medium {
                height: 66,
                weight: 120,
            })
            .species(Species::Human)
            .build_with_uuid(PENELOPE)
    }

    pub fn polyphemus() -> Npc {
        npc::build()
            .name("Polyphemus")
            .age(Age::Adult)
            .ethnicity(Ethnicity::Orcish)
            .gender(Gender::Masculine)
            .size(Size::Medium {
                height: 144,
                weight: 1000,
            })
            .species(Species::HalfOrc)
            .build_with_uuid(POLYPHEMUS)
    }
}
