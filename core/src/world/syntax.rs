use super::Thing;
use crate::app::{AppMeta, Autocomplete, ContextAwareParse};
use crate::storage::CacheEntry;
use crate::world::npc::{Age, Ethnicity, Gender, Npc, Species};
use crate::world::place::{Place, PlaceType};
use async_trait::async_trait;
use futures::join;
use initiative_macros::{Autocomplete, ContextAwareParse, Display};
use std::borrow::Cow;
use std::cmp::Ordering;
use std::convert::Infallible;
use std::fmt;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::str::FromStr;

#[derive(Autocomplete, Clone, ContextAwareParse, Debug, Display, PartialEq)]
pub enum NpcTerm {
    #[command(autocomplete_desc = "specify an age (eg. \"elderly\")")]
    #[command(alias = "[age]", no_default_autocomplete)]
    Age {
        #[command(implements(WordList))]
        age: Age,
    },

    #[command(autocomplete_desc = "specify an ethnicity (eg. \"elvish\")")]
    #[command(alias = "[ethnicity]", no_default_autocomplete)]
    Ethnicity {
        #[command(implements(WordList))]
        ethnicity: Ethnicity,
    },

    #[command(autocomplete_desc = "specify a gender")]
    #[command(alias = "[gender]", no_default_autocomplete)]
    Gender {
        #[command(implements(WordList))]
        gender: Gender,
    },

    #[command(autocomplete_desc = "specify a name")]
    #[command(syntax = "named [name]", alias = "called [name]")]
    Name {
        #[command(implements(FromStr))]
        name: String,
    },

    #[command(autocomplete_desc = "specify a species (eg. \"dwarf\")")]
    #[command(alias = "[species]", no_default_autocomplete)]
    Species {
        #[command(implements(WordList))]
        species: Species,
    },
    //#[command(catchall)]
    //UnknownWord(String),
}

#[derive(Autocomplete, Clone, ContextAwareParse, Default, Debug, Display, PartialEq)]
pub struct NpcDescription(Vec<NpcTerm>);

#[derive(Autocomplete, Clone, ContextAwareParse, Debug, Display, PartialEq)]
pub enum PlaceTerm {
    #[command(autocomplete_desc = "specify a name")]
    #[command(syntax = "named [name]", alias = "called [name]")]
    Name {
        #[command(implements(FromStr))]
        name: String,
    },

    #[command(autocomplete_desc = "specify a place type (eg. \"inn\")")]
    #[command(alias = "[subtype]", no_default_autocomplete)]
    Subtype {
        #[command(implements(WordList))]
        subtype: PlaceType,
    },
    //#[command(catchall)]
    //UnknownWord(String),
}

#[derive(Autocomplete, Clone, ContextAwareParse, Default, Debug, Display, PartialEq)]
pub struct PlaceDescription(Vec<PlaceTerm>);

#[derive(Clone, Debug, PartialEq)]
pub enum ThingDescription {
    Place(PlaceDescription),
    Npc(NpcDescription),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ThingName<ThingType, Source>
where
    ThingType: FilterThingsByType,
    Source: GetThings,
{
    pub name: String,
    thing: PhantomData<ThingType>,
    source: PhantomData<Source>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FromJournal;

#[derive(Clone, Debug, PartialEq)]
pub struct FromRecent;

#[derive(Clone, Debug, PartialEq)]
pub struct FromAny;

impl PlaceDescription {
    pub fn into_place(self) -> Place {
        self.into_place_with_unknown_words().0
    }

    pub fn into_place_with_unknown_words(mut self) -> (Place, Vec<String>) {
        let mut place = Place::default();
        let mut unknown_words = Vec::new();

        for term in self.0.drain(..) {
            match term {
                PlaceTerm::Name { name } => place.name = name.into(),
                PlaceTerm::Subtype { subtype } => place.subtype = subtype.into(),
                //PlaceTerm::UnknownWord(word) => unknown_words.push(word),
            }
        }

        (place, unknown_words)
    }

    pub fn unknown_word_count(&self) -> usize {
        /*
        self.0
            .iter()
            .filter(|v| matches!(v, PlaceTerm::UnknownWord(_)))
            .count()
        */
        0
    }
}

impl FromIterator<PlaceTerm> for PlaceDescription {
    fn from_iter<I: IntoIterator<Item = PlaceTerm>>(iter: I) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl NpcDescription {
    pub fn into_npc(self) -> Npc {
        self.into_npc_with_unknown_words().0
    }

    pub fn into_npc_with_unknown_words(mut self) -> (Npc, Vec<String>) {
        let mut npc = Npc::default();
        let mut unknown_words = Vec::new();

        for term in self.0.drain(..) {
            match term {
                NpcTerm::Age { age } => npc.age = age.into(),
                NpcTerm::Ethnicity { ethnicity } => npc.ethnicity = ethnicity.into(),
                NpcTerm::Gender { gender } => npc.gender = gender.into(),
                NpcTerm::Name { name } => npc.name = name.into(),
                NpcTerm::Species { species } => npc.species = species.into(),
                //NpcTerm::UnknownWord(_) => {}
            }
        }

        (npc, unknown_words)
    }

    pub fn unknown_word_count(&self) -> usize {
        /*
        self.0
            .iter()
            .filter(|v| matches!(v, NpcTerm::UnknownWord(_)))
            .count()
        */
        0
    }
}

impl FromIterator<NpcTerm> for NpcDescription {
    fn from_iter<I: IntoIterator<Item = NpcTerm>>(iter: I) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl ThingDescription {
    pub fn into_thing(self) -> Thing {
        match self {
            Self::Place(place_description) => Thing::Place(place_description.into_place()),
            Self::Npc(npc_description) => Thing::Npc(npc_description.into_npc()),
        }
    }

    pub fn into_thing_with_unknown_words(mut self) -> (Thing, Vec<String>) {
        match self {
            Self::Place(place_description) => {
                let (place, unknown_words) = place_description.into_place_with_unknown_words();
                (Thing::Place(place), unknown_words)
            }
            Self::Npc(npc_description) => {
                let (npc, unknown_words) = npc_description.into_npc_with_unknown_words();
                (Thing::Npc(npc), unknown_words)
            }
        }
    }

    pub fn place() -> Self {
        Self::Place(PlaceDescription::default())
    }

    pub fn npc() -> Self {
        Self::Npc(NpcDescription::default())
    }
}

impl From<PlaceDescription> for ThingDescription {
    fn from(input: PlaceDescription) -> Self {
        ThingDescription::Place(input)
    }
}

impl From<NpcDescription> for ThingDescription {
    fn from(input: NpcDescription) -> Self {
        ThingDescription::Npc(input)
    }
}

#[async_trait(?Send)]
impl Autocomplete for ThingDescription {
    async fn autocomplete(
        input: &str,
        app_meta: &AppMeta,
    ) -> Vec<(Cow<'static, str>, Cow<'static, str>)> {
        let (mut suggestions, mut more_suggestions) = join!(
            NpcDescription::autocomplete(input, app_meta),
            PlaceDescription::autocomplete(input, app_meta),
        );

        suggestions.append(&mut more_suggestions);
        suggestions.sort();
        suggestions.dedup();
        suggestions.truncate(10);
        suggestions
    }
}

#[async_trait(?Send)]
impl ContextAwareParse for ThingDescription {
    async fn parse_input(input: &str, app_meta: &AppMeta) -> (Option<Self>, Vec<Self>) {
        let (npc, place) = join!(
            NpcDescription::parse_input(input, app_meta),
            PlaceDescription::parse_input(input, app_meta),
        );

        (
            match (npc, place) {
                ((Some(npc), _), (Some(place), _)) => {
                    match npc.unknown_word_count().cmp(&place.unknown_word_count()) {
                        Ordering::Less => Some(ThingDescription::Npc(npc)),
                        Ordering::Equal => None,
                        Ordering::Greater => Some(ThingDescription::Place(place)),
                    }
                }
                ((Some(npc), _), (None, _)) => Some(ThingDescription::Npc(npc)),
                ((None, _), (Some(place), _)) => Some(ThingDescription::Place(place)),
                _ => None,
            },
            Vec::new(),
        )
    }
}

impl fmt::Display for ThingDescription {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Place(place_description) => write!(f, "{}", place_description),
            Self::Npc(npc_description) => write!(f, "{}", npc_description),
        }
    }
}

#[async_trait(?Send)]
impl<ThingType, Source> ContextAwareParse for ThingName<ThingType, Source>
where
    ThingType: FilterThingsByType,
    Source: GetThings,
{
    async fn parse_input(input: &str, app_meta: &AppMeta) -> (Option<Self>, Vec<Self>) {
        if let Some(name) = Source::get_by_name(input, app_meta)
            .filter(|entry| ThingType::filter(entry))
            .map(|entry| Self::new(entry.name.to_string()))
        {
            (Some(name), Vec::new())
        } else {
            (None, vec![Self::new(input.to_string())])
        }
    }
}

#[async_trait(?Send)]
impl<ThingType, Source> Autocomplete for ThingName<ThingType, Source>
where
    ThingType: FilterThingsByType,
    Source: GetThings,
{
    async fn autocomplete(
        input: &str,
        app_meta: &AppMeta,
        _include_aliases: bool,
    ) -> Vec<(Cow<'static, str>, Cow<'static, str>)> {
        Source::get_by_name_start(input, app_meta)
            .drain(..)
            .filter(|t| ThingType::filter(t))
            .map(|t| {
                (
                    t.name.to_string().into(),
                    if t.in_journal {
                        t.description.to_string()
                    } else {
                        format!("{} (unsaved)", t.description)
                    }
                    .into(),
                )
            })
            .collect()
    }

    fn get_variant_name(&self) -> &'static str {
        ""
    }
}

impl<ThingType, Source> ThingName<ThingType, Source>
where
    ThingType: FilterThingsByType,
    Source: GetThings,
{
    fn new(name: String) -> Self {
        Self {
            name,
            thing: PhantomData,
            source: PhantomData,
        }
    }
}

impl<ThingType, Source> From<&str> for ThingName<ThingType, Source>
where
    ThingType: FilterThingsByType,
    Source: GetThings,
{
    fn from(input: &str) -> Self {
        Self::new(input.to_string())
    }
}

impl<ThingType, Source> From<&String> for ThingName<ThingType, Source>
where
    ThingType: FilterThingsByType,
    Source: GetThings,
{
    fn from(input: &String) -> Self {
        Self::new(input.to_owned())
    }
}

impl<ThingType, Source> From<String> for ThingName<ThingType, Source>
where
    ThingType: FilterThingsByType,
    Source: GetThings,
{
    fn from(input: String) -> Self {
        Self::new(input)
    }
}

impl<ThingType, Source> From<ThingName<ThingType, Source>> for String
where
    ThingType: FilterThingsByType,
    Source: GetThings,
{
    fn from(input: ThingName<ThingType, Source>) -> String {
        input.name
    }
}

impl<ThingType, Source> AsRef<str> for ThingName<ThingType, Source>
where
    ThingType: FilterThingsByType,
    Source: GetThings,
{
    fn as_ref(&self) -> &str {
        &self.name
    }
}

impl<ThingType, Source> FromStr for ThingName<ThingType, Source>
where
    ThingType: FilterThingsByType,
    Source: GetThings,
{
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(ThingName {
            name: input.to_string(),
            thing: PhantomData,
            source: PhantomData,
        })
    }
}

impl<ThingType, Source> fmt::Display for ThingName<ThingType, Source>
where
    ThingType: FilterThingsByType,
    Source: GetThings,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.name)
    }
}

pub trait FilterThingsByType {
    fn filter(entry: &CacheEntry) -> bool;
}

pub trait GetThings {
    fn get_by_name<'a>(name: &str, app_meta: &'a AppMeta) -> Option<&'a CacheEntry>;

    fn get_by_name_start<'a>(name_start: &str, app_meta: &'a AppMeta) -> Vec<&'a CacheEntry>;
}

impl FilterThingsByType for Thing {
    fn filter(_entry: &CacheEntry) -> bool {
        true
    }
}

impl FilterThingsByType for Npc {
    fn filter(entry: &CacheEntry) -> bool {
        entry.subtype == "character"
    }
}

impl FilterThingsByType for Place {
    fn filter(entry: &CacheEntry) -> bool {
        entry.subtype == "place"
    }
}

impl GetThings for FromAny {
    fn get_by_name<'a>(name: &str, app_meta: &'a AppMeta) -> Option<&'a CacheEntry> {
        app_meta.repository.get_cached_by_name(name)
    }

    fn get_by_name_start<'a>(name_start: &str, app_meta: &'a AppMeta) -> Vec<&'a CacheEntry> {
        app_meta
            .repository
            .get_cached_by_name_start(name_start, true, true)
    }
}

impl GetThings for FromRecent {
    fn get_by_name<'a>(name: &str, app_meta: &'a AppMeta) -> Option<&'a CacheEntry> {
        app_meta
            .repository
            .get_cached_by_name(name)
            .filter(|entry| !entry.in_journal)
    }

    fn get_by_name_start<'a>(name_start: &str, app_meta: &'a AppMeta) -> Vec<&'a CacheEntry> {
        app_meta
            .repository
            .get_cached_by_name_start(name_start, false, true)
    }
}

impl GetThings for FromJournal {
    fn get_by_name<'a>(name: &str, app_meta: &'a AppMeta) -> Option<&'a CacheEntry> {
        app_meta
            .repository
            .get_cached_by_name(name)
            .filter(|entry| entry.in_journal)
    }

    fn get_by_name_start<'a>(name_start: &str, app_meta: &'a AppMeta) -> Vec<&'a CacheEntry> {
        app_meta
            .repository
            .get_cached_by_name_start(name_start, true, false)
    }
}
