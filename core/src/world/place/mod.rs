pub use view::{DescriptionView, DetailsView, NameView, SummaryView};

mod building;
mod location;
mod region;
mod view;

use super::{Demographics, Field, Generate};
use crate::world::thing::Thing;
use initiative_macros::WordList;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Place {
    pub uuid: Uuid,

    #[serde(flatten)]
    pub data: PlaceData,
}

#[derive(Clone, Debug, Deserialize, Default, Eq, PartialEq, Serialize)]
pub struct PlaceData {
    pub location_uuid: Field<Uuid>,
    pub subtype: Field<PlaceType>,

    pub name: Field<String>,
    pub description: Field<String>,
    // pub architecture: Option<String>,
    // pub floors: Field<u8>,
    // pub owner: Field<Vec<Uuid>>,
    // pub staff: Field<Vec<Uuid>>,
    // pub occupants: Field<Vec<Uuid>>,
    // pub services: Option<String>,
    // pub worship: Field<String>,
    // pub quality: something
    // pub price: something
}

#[derive(Debug, Default)]
pub struct PlaceRelations {
    pub location: Option<(Place, Option<Place>)>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, WordList)]
#[serde(into = "&'static str", try_from = "&str")]
pub enum PlaceType {
    #[term = "place"]
    Any,

    Building(building::BuildingType),
    Location(location::LocationType),
    Region(region::RegionType),
}

impl Place {
    pub fn display_name(&self) -> NameView {
        self.data.display_name()
    }

    pub fn display_summary(&self) -> SummaryView {
        self.data.display_summary()
    }

    pub fn display_description(&self) -> DescriptionView {
        self.data.display_description()
    }

    pub fn display_details(&self, relations: PlaceRelations) -> DetailsView {
        self.data.display_details(self.uuid, relations)
    }

    pub fn get_words() -> &'static [&'static str] {
        &["place"][..]
    }

    pub fn lock_all(&mut self) {
        self.data.lock_all()
    }

    pub fn apply_diff(&mut self, diff: &mut PlaceData) {
        self.data.apply_diff(diff)
    }
}

impl PlaceData {
    pub fn display_name(&self) -> NameView {
        NameView::new(self)
    }

    pub fn display_summary(&self) -> SummaryView {
        SummaryView::new(self)
    }

    pub fn display_description(&self) -> DescriptionView {
        DescriptionView::new(self)
    }

    pub fn display_details(&self, uuid: Uuid, relations: PlaceRelations) -> DetailsView {
        DetailsView::new(self, uuid, relations)
    }

    pub fn lock_all(&mut self) {
        let Self {
            location_uuid,
            subtype,
            name,
            description,
        } = self;

        location_uuid.lock();
        subtype.lock();
        name.lock();
        description.lock();
    }

    pub fn apply_diff(&mut self, diff: &mut Self) {
        let Self {
            location_uuid,
            subtype,
            name,
            description,
        } = self;

        location_uuid.apply_diff(&mut diff.location_uuid);
        subtype.apply_diff(&mut diff.subtype);
        name.apply_diff(&mut diff.name);
        description.apply_diff(&mut diff.description);
    }

    pub fn into_thing(self, uuid: Uuid) -> Thing {
        Thing {
            uuid: uuid,
            data: self.into(),
        }
    }
}

impl Generate for PlaceData {
    fn regenerate(&mut self, rng: &mut impl Rng, demographics: &Demographics) {
        if !self.name.is_locked() || self.subtype.is_none() {
            self.subtype
                .replace_with(|_| PlaceType::generate(rng, demographics));
        }

        if let Some(value) = self.subtype.value() {
            match value {
                PlaceType::Building(_) => building::generate(self, rng, demographics),
                PlaceType::Location(_) => location::generate(self, rng, demographics),
                _ => {}
            }
        }
    }
}

impl PlaceType {
    pub const fn get_emoji(&self) -> &'static str {
        if let Some(emoji) = match self {
            Self::Any => None,
            Self::Building(subtype) => subtype.get_emoji(),
            Self::Location(subtype) => subtype.get_emoji(),
            Self::Region(subtype) => subtype.get_emoji(),
        } {
            emoji
        } else {
            "📍"
        }
    }
}

impl Default for PlaceType {
    fn default() -> Self {
        Self::Any
    }
}

impl Generate for PlaceType {
    fn regenerate(&mut self, rng: &mut impl Rng, _demographics: &Demographics) {
        *self = Self::get_words()
            .nth(rng.gen_range(0..Self::word_count()))
            .unwrap()
            .parse()
            .unwrap();
    }
}

impl fmt::Display for PlaceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generate_test() {
        let demographics = Demographics::default();

        let mut rng = SmallRng::seed_from_u64(1);
        assert_ne!(
            PlaceData::generate(&mut rng, &demographics).subtype,
            PlaceData::generate(&mut rng, &demographics).subtype,
        );

        let mut rng1 = SmallRng::seed_from_u64(0);
        let mut rng2 = SmallRng::seed_from_u64(0);
        assert_eq!(
            PlaceData::generate(&mut rng1, &demographics).subtype,
            PlaceData::generate(&mut rng2, &demographics).subtype,
        );
    }

    #[test]
    fn place_type_default_test() {
        assert_eq!(PlaceType::Any, PlaceType::default());
    }

    #[test]
    fn place_type_serialize_deserialize_test() {
        {
            let inn: PlaceType = "inn".parse().unwrap();
            assert_eq!(r#""inn""#, serde_json::to_string(&inn).unwrap());
            assert_eq!(inn, serde_json::from_str::<PlaceType>(r#""inn""#).unwrap());
        }

        {
            let business: PlaceType = "business".parse().unwrap();
            assert_eq!(r#""business""#, serde_json::to_string(&business).unwrap());
            assert_eq!(
                business,
                serde_json::from_str::<PlaceType>(r#""business""#).unwrap()
            );
        }

        {
            let building: PlaceType = "building".parse().unwrap();
            assert_eq!(r#""building""#, serde_json::to_string(&building).unwrap());
            assert_eq!(
                building,
                serde_json::from_str::<PlaceType>(r#""building""#).unwrap(),
            );
        }

        {
            let place: PlaceType = "place".parse().unwrap();
            assert_eq!(r#""place""#, serde_json::to_string(&place).unwrap());
            assert_eq!(
                place,
                serde_json::from_str::<PlaceType>(r#""place""#).unwrap(),
            );
        }
    }

    #[test]
    fn place_serialize_deserialize_test() {
        let place = oaken_mermaid_inn();

        assert_eq!(
            r#"{"uuid":"00000000-0000-0000-0000-000000000000","location_uuid":"00000000-0000-0000-0000-000000000000","subtype":"inn","name":"Oaken Mermaid Inn","description":"I am Mordenkainen"}"#,
            serde_json::to_string(&place).unwrap(),
        );

        let value: Place = serde_json::from_str(r#"{"uuid":"00000000-0000-0000-0000-000000000000","location_uuid":"00000000-0000-0000-0000-000000000000","subtype":"inn","name":"Oaken Mermaid Inn","description":"I am Mordenkainen"}"#).unwrap();

        assert_eq!(place, value);
    }

    #[test]
    fn apply_diff_test_no_change() {
        let mut place = oaken_mermaid_inn();
        let mut diff = PlaceData::default();

        place.data.apply_diff(&mut diff);

        assert_eq!(oaken_mermaid_inn(), place);
        assert_eq!(PlaceData::default(), diff);
    }

    #[test]
    fn apply_diff_test_from_empty() {
        let oaken_mermaid_inn = oaken_mermaid_inn();

        let mut place = PlaceData::default();
        let mut diff = oaken_mermaid_inn.data.clone();

        place.apply_diff(&mut diff);

        assert_eq!(oaken_mermaid_inn.data, place);

        let mut empty_locked = PlaceData::default();
        empty_locked.lock_all();
        assert_eq!(empty_locked, diff);
    }

    #[test]
    fn lock_all_test() {
        let mut place = PlaceData::default();
        place.lock_all();

        assert_eq!(
            PlaceData {
                location_uuid: Field::Locked(None),
                subtype: Field::Locked(None),
                name: Field::Locked(None),
                description: Field::Locked(None),
            },
            place,
        );
    }

    #[test]
    fn get_emoji_test() {
        let mut words_emoji: Vec<(String, String)> = PlaceType::get_words()
            .map(|word| {
                (
                    word.to_string(),
                    PlaceType::parse_cs(word).unwrap().get_emoji().to_string(),
                )
            })
            .collect();
        words_emoji.sort();

        let expect_words_emoji: Vec<(String, String)> = [
            ("abbey", "🙏"),
            ("academy", "🎓"),
            ("archipelago", "🏝"),
            ("arena", "🏛"),
            ("armorer", "🛡"),
            ("bakery", "🍞"),
            ("bank", "🏦"),
            ("bar", "🍻"),
            ("barony", "👑"),
            ("barracks", "⚔"),
            ("barrens", "🏜"),
            ("base", "⚔"),
            ("bathhouse", "🛁"),
            ("beach", "🏖"),
            ("blacksmith", "🗡"),
            ("brewery", "🍻"),
            ("bridge", "🌉"),
            ("building", "📍"),
            ("business", "🪙"),
            ("camp", "🏕"),
            ("campsite", "🏕"),
            ("canyon", "🏞"),
            ("capital", "🏙"),
            ("caravansary", "🏨"),
            ("casino", "🃏"),
            ("castle", "🏰"),
            ("cave", "📍"),
            ("cavern", "📍"),
            ("cemetery", "🪦"),
            ("chasm", "🏞"),
            ("church", "🙏"),
            ("citadel", "🏰"),
            ("city", "🏙"),
            ("city-state", "👑"),
            ("club", ""),
            ("coastline", "🌊"),
            ("college", "🎓"),
            ("confederation", "👑"),
            ("continent", "📍"),
            ("country", "👑"),
            ("county", "👑"),
            ("court", "🏰"),
            ("crypt", "🪦"),
            ("desert", "🏜"),
            ("distillery", "🥃"),
            ("district", "🏘"),
            ("domain", "👑"),
            ("duchy", "👑"),
            ("duty-house", "🪙"),
            ("embassy", "🚩"),
            ("empire", "👑"),
            ("farm", "🌱"),
            ("ferry", "⛴"),
            ("fighting-pit", "⚔"),
            ("food-counter", "🍲"),
            ("forest", "🌳"),
            ("forge", "🔥"),
            ("fort", "🏰"),
            ("fortress", "🏰"),
            ("fountain", "⛲"),
            ("furniture-shop", "🪑"),
            ("furrier", "🦊"),
            ("gambling-hall", "🃏"),
            ("garden", "🌱"),
            ("gate", "🚪"),
            ("general-store", "🪙"),
            ("glacier", "🏔"),
            ("gorge", "🏞"),
            ("graveyard", "🪦"),
            ("grove", "🌳"),
            ("guardhouse", "🛡"),
            ("guild-hall", "🪙"),
            ("hamlet", "🏘"),
            ("harbor", "⛵"),
            ("hermitage", "🙏"),
            ("hill", "⛰"),
            ("hotel", "🏨"),
            ("house", "🏠"),
            ("imports-shop", "🪙"),
            ("inn", "🏨"),
            ("island", "🏝"),
            ("jail", "🛡"),
            ("jeweller", "💍"),
            ("jungle", "🌳"),
            ("keep", "🏰"),
            ("kingdom", "👑"),
            ("lake", "🌊"),
            ("library", "📚"),
            ("lighthouse", "⛵"),
            ("location", "📍"),
            ("lodge", "🏨"),
            ("lumberyard", "🪵"),
            ("magic-shop", "🪄"),
            ("manor", "🏠"),
            ("mansion", "🏠"),
            ("market", "🪙"),
            ("marsh", "📍"),
            ("mausoleum", "🪦"),
            ("mesa", "📍"),
            ("metropolis", "🏙"),
            ("mill", "🌾"),
            ("mine", "⚒"),
            ("monastery", "🙏"),
            ("monolith", "🗿"),
            ("monument", "🗽"),
            ("moor", "📍"),
            ("mosque", "🙏"),
            ("mountain", "⛰"),
            ("nation", "👑"),
            ("necropolis", "🪦"),
            ("neighborhood", "🏘"),
            ("nightclub", "🍻"),
            ("nunnery", "🙏"),
            ("oasis", "🌴"),
            ("ocean", "🌊"),
            ("outpost", "🚩"),
            ("palace", "🏰"),
            ("parish", "🏘"),
            ("pass", "⛰"),
            ("peninsula", "🏝"),
            ("pet-store", "🐶"),
            ("pier", "⛵"),
            ("place", "📍"),
            ("plain", "📍"),
            ("plateau", "📍"),
            ("portal", "📍"),
            ("principality", "👑"),
            ("prison", "🛡"),
            ("province", "👑"),
            ("pub", "🍻"),
            ("quarter", "🏘"),
            ("realm", "👑"),
            ("reef", "📍"),
            ("region", "👑"),
            ("region", "👑"),
            ("residence", "🏠"),
            ("restaurant", "🍽"),
            ("ridge", "⛰"),
            ("rift", "📍"),
            ("river", "🏞"),
            ("ruin", "🏚"),
            ("school", "🎓"),
            ("sea", "🌊"),
            ("shipyard", "⛵"),
            ("shop", "🪙"),
            ("shrine", "🙏"),
            ("smithy", "🗡"),
            ("specialty-shop", "🪙"),
            ("spirits-shop", "🥃"),
            ("stable", "🐎"),
            ("statue", "🗽"),
            ("store", "🪙"),
            ("street", "🏘"),
            ("stronghold", "🏰"),
            ("swamp", "📍"),
            ("synagogue", "🙏"),
            ("tavern", "🏨"),
            ("temple", "🙏"),
            ("territory", "👑"),
            ("textiles-shop", "🪙"),
            ("theater", "🎭"),
            ("tomb", "🪦"),
            ("tower", "🏰"),
            ("town", "🏘"),
            ("trading-post", "🪙"),
            ("tree", "🌳"),
            ("tundra", "❄"),
            ("university", "🎓"),
            ("vale", "🏞"),
            ("valley", "🏞"),
            ("vault", "🏦"),
            ("village", "🏘"),
            ("wainwright", "🪙"),
            ("wall", "🧱"),
            ("ward", "🏘"),
            ("warehouse", "📦"),
            ("wasteland", "🏜"),
            ("watch-house", "🛡"),
            ("weaponsmith", "🗡"),
            ("woodshop", "🪚"),
            ("world", "🌐"),
        ]
        .iter()
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect();

        /*
        expect_words_emoji
            .iter()
            .zip(words_emoji.iter())
            .for_each(|(expect, word)| assert_eq!(expect, word));
        */

        assert_eq!(expect_words_emoji, words_emoji);
    }

    fn oaken_mermaid_inn() -> Place {
        Place {
            uuid: uuid::Uuid::nil(),
            data: PlaceData {
                location_uuid: uuid::Uuid::nil().into(),
                subtype: "inn".parse::<PlaceType>().ok().into(),

                name: "Oaken Mermaid Inn".into(),
                description: "I am Mordenkainen".into(),
            },
        }
    }
}
