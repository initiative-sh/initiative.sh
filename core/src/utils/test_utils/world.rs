macro_rules! builder {
    ($thing: path | $thing_data: path, ($($f:ident: $t:path),* $(,)?) $(,)?) => {
        use crate::world::Generate as _;

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
                self.0.regenerate(rng, &crate::world::Demographics::default());
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

macro_rules! relations_builder {
    ($thing_relations: path, ($($f:ident: $t:path),* $(,)?) $(,)?) => {
        #[derive(Default)]
        pub struct RelationsBuilder($thing_relations);

        impl RelationsBuilder {
            #[allow(dead_code)]
            pub fn build(self) -> $thing_relations {
                self.0
            }

            $(
                #[allow(dead_code)]
                pub fn $f(mut self, $f: $t) -> Self {
                    if let Some(value) = self.0.$f.as_mut() {
                        if value.1.is_none() {
                            value.1 = Some($f);
                        }
                    } else {
                        self.0.$f = Some(($f, None));
                    }
                    self
                }
            )*
        }
    };
}

pub use npc::builder as npc;
pub mod npc {
    use crate::world::npc::{Age, Ethnicity, Gender, Npc, NpcData, NpcRelations, Size, Species};
    use crate::world::place::Place;
    use uuid::Uuid;

    #[allow(unused_imports)]
    pub use super::odyssey::{odysseus, penelope, polyphemus};

    #[allow(unused_imports)]
    pub use super::odyssey::{ODYSSEUS, PENELOPE, POLYPHEMUS};

    pub fn builder() -> builder::Builder {
        builder::Builder::default()
    }

    pub fn relations() -> builder::RelationsBuilder {
        builder::RelationsBuilder::default()
    }

    mod builder {
        use super::*;

        builder!(
            Npc | NpcData,
            (
                age: Age,
                age_years: u16,
                ethnicity: Ethnicity,
                gender: Gender,
                location_uuid: Uuid,
                size: Size,
                species: Species,
            ),
        );

        relations_builder!(NpcRelations, (location: Place));
    }
}

pub use place::builder as place;
pub mod place {
    use crate::world::place::{Place, PlaceData, PlaceRelations, PlaceType};
    use uuid::Uuid;

    #[allow(unused_imports)]
    pub use super::odyssey::{greece, ithaca, styx};

    #[allow(unused_imports)]
    pub use super::odyssey::{GREECE, ITHACA, STYX};

    pub fn builder() -> builder::Builder {
        builder::Builder::default()
    }

    pub fn relations() -> builder::RelationsBuilder {
        builder::RelationsBuilder::default()
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

        relations_builder!(PlaceRelations, (location: Place));
    }
}

pub mod thing {
    pub use super::odyssey::{GREECE, ITHACA, ODYSSEUS, PENELOPE, POLYPHEMUS, STYX};

    use crate::world::thing::Thing;

    macro_rules! thing {
        ($thing: ident) => {
            pub fn $thing() -> Thing {
                super::odyssey::$thing().into()
            }
            pub mod $thing {
                use crate::world::thing::{ThingData, ThingRelations};

                #[allow(dead_code)]
                pub fn data() -> ThingData {
                    super::$thing().data
                }

                #[allow(dead_code)]
                pub fn relations() -> ThingRelations {
                    super::super::odyssey::$thing::relations().into()
                }
            }
        };
    }

    thing!(greece);
    thing!(ithaca);
    thing!(odysseus);
    thing!(penelope);
    thing!(polyphemus);
    thing!(styx);
}

mod odyssey {
    use super::*;

    use crate::world::npc::{Age, Ethnicity, Gender, Npc, NpcData, NpcRelations, Size, Species};
    use crate::world::place::{Place, PlaceData, PlaceRelations, PlaceType};
    use uuid::Uuid;

    pub use greece::UUID as GREECE;
    pub use ithaca::UUID as ITHACA;
    pub use odysseus::UUID as ODYSSEUS;
    pub use penelope::UUID as PENELOPE;
    pub use polyphemus::UUID as POLYPHEMUS;
    pub use styx::UUID as STYX;

    macro_rules! place {
        ($name: ident, $uuid: expr, $build: expr, $relations: expr $(,)?) => {
            pub use $name::place as $name;
            pub mod $name {
                use super::*;

                pub const UUID: Uuid = Uuid::from_u128($uuid);

                pub fn place() -> Place {
                    Place {
                        data: data(),
                        uuid: UUID,
                    }
                }

                pub fn data() -> PlaceData {
                    $build
                }

                pub fn relations() -> PlaceRelations {
                    $relations
                }
            }
        };
    }

    macro_rules! npc {
        ($name: ident, $uuid: expr, $build: expr, $relations: expr $(,)?) => {
            pub use $name::npc as $name;
            pub mod $name {
                use super::*;

                pub const UUID: Uuid = Uuid::from_u128($uuid);

                pub fn npc() -> Npc {
                    Npc {
                        data: data(),
                        uuid: UUID,
                    }
                }

                pub fn data() -> NpcData {
                    $build
                }

                pub fn relations() -> NpcRelations {
                    $relations
                }
            }
        };
    }

    place!(
        ithaca,
        0x01,
        place::builder()
            .name("Ithaca")
            .subtype("island".parse::<PlaceType>().unwrap())
            .location_uuid(GREECE)
            .build(),
        PlaceRelations {
            location: Some((greece(), None)),
        },
    );

    place!(
        greece,
        0x02,
        place::builder()
            .name("Greece")
            .description("You're cruisin' for a bruisin'.")
            .subtype("territory".parse::<PlaceType>().unwrap())
            .build(),
        PlaceRelations::default(),
    );

    place!(
        styx,
        0x03,
        place::builder()
            .name("Styx")
            .description("This really is hell!")
            .subtype("river".parse::<PlaceType>().unwrap())
            .build(),
        PlaceRelations::default(),
    );

    npc!(
        odysseus,
        0x11,
        npc::builder()
            .name("Odysseus")
            .age(Age::MiddleAged)
            .age_years(50)
            .ethnicity(Ethnicity::Human)
            .gender(Gender::Masculine)
            .location_uuid(STYX)
            .size(Size::Medium {
                height: 72,
                weight: 180,
            })
            .species(Species::Human)
            .build(),
        NpcRelations {
            location: Some((styx(), None)),
        },
    );

    npc!(
        penelope,
        0x12,
        npc::builder()
            .name("Penelope")
            .age(Age::MiddleAged)
            .age_years(40)
            .ethnicity(Ethnicity::Human)
            .gender(Gender::Feminine)
            .location_uuid(ITHACA)
            .size(Size::Medium {
                height: 66,
                weight: 120,
            })
            .species(Species::Human)
            .build(),
        NpcRelations {
            location: Some((ithaca(), Some(greece()))),
        },
    );

    npc!(
        polyphemus,
        0x13,
        npc::builder()
            .name("Polyphemus")
            .age(Age::Adult)
            .age_years(15)
            .ethnicity(Ethnicity::Orcish)
            .gender(Gender::Masculine)
            .size(Size::Medium {
                height: 144,
                weight: 1000,
            })
            .species(Species::HalfOrc)
            .build(),
        NpcRelations::default(),
    );
}
