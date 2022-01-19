use super::{Npc, Place, Thing};
use crate::app::{AppMeta, Autocomplete, ContextAwareParse};
use crate::storage::CacheEntry;
use async_trait::async_trait;
use std::borrow::Cow;
use std::convert::Infallible;
use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

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
