use super::{Npc, Place, Thing};
use crate::app::{AppMeta, Autocomplete, ContextAwareParse};
use crate::utils::CaseInsensitiveStr;
use async_trait::async_trait;
use std::borrow::Cow;
use std::convert::Infallible;
use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct ThingName<ThingType = Thing, Source = FromAny>
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
        (
            Source::get_by_name(input, app_meta)
                .await
                .filter(|t| ThingType::filter(t))
                .and_then(|t| match t {
                    Thing::Npc(Npc { mut name, .. }) => name.value_mut().map(mem::take),
                    Thing::Place(Place { mut name, .. }) => name.value_mut().map(mem::take),
                })
                .map(Self::new),
            Vec::new(),
        )
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
    ) -> Vec<(Cow<'static, str>, Cow<'static, str>)> {
        Source::get_by_name_start(input, app_meta)
            .await
            .drain(..)
            .filter(|t| ThingType::filter(t))
            .map(|thing| {
                (
                    thing.name().to_string().into(),
                    thing.display_description().to_string().into(),
                )
            })
            .collect()
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

impl From<&str> for ThingName {
    fn from(input: &str) -> Self {
        Self::new(input.to_string())
    }
}

impl From<&String> for ThingName {
    fn from(input: &String) -> Self {
        Self::new(input.to_owned())
    }
}

impl From<String> for ThingName {
    fn from(input: String) -> Self {
        Self::new(input)
    }
}

impl From<ThingName> for String {
    fn from(input: ThingName) -> String {
        input.name
    }
}

impl AsRef<str> for ThingName {
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
    fn filter(thing: &Thing) -> bool;
}

#[async_trait(?Send)]
pub trait GetThings {
    async fn get_by_name(name: &str, app_meta: &AppMeta) -> Option<Thing>;

    async fn get_by_name_start(name_start: &str, app_meta: &AppMeta) -> Vec<Thing>;
}

impl FilterThingsByType for Thing {
    fn filter(_thing: &Thing) -> bool {
        true
    }
}

impl FilterThingsByType for Npc {
    fn filter(thing: &Thing) -> bool {
        thing.npc().is_some()
    }
}

impl FilterThingsByType for Place {
    fn filter(thing: &Thing) -> bool {
        thing.place().is_some()
    }
}

#[async_trait(?Send)]
impl GetThings for FromAny {
    async fn get_by_name(name: &str, app_meta: &AppMeta) -> Option<Thing> {
        app_meta.repository.get_by_name(name).await.ok()
    }

    async fn get_by_name_start(name_start: &str, app_meta: &AppMeta) -> Vec<Thing> {
        app_meta
            .repository
            .get_by_name_start(name_start, Some(10))
            .await
            .unwrap_or_default()
    }
}

#[async_trait(?Send)]
impl GetThings for FromRecent {
    async fn get_by_name(name: &str, app_meta: &AppMeta) -> Option<Thing> {
        app_meta
            .repository
            .recent()
            .find(|t| t.name().value().map_or(false, |s| s.eq_ci(name)))
            .cloned()
    }

    async fn get_by_name_start(name_start: &str, app_meta: &AppMeta) -> Vec<Thing> {
        app_meta
            .repository
            .recent()
            .filter(|t| {
                t.name()
                    .value()
                    .map_or(false, |s| s.starts_with_ci(name_start))
            })
            .take(20)
            .cloned()
            .collect()
    }
}

#[async_trait(?Send)]
impl GetThings for FromJournal {
    async fn get_by_name(name: &str, app_meta: &AppMeta) -> Option<Thing> {
        app_meta
            .repository
            .get_by_name(name)
            .await
            .ok()
            .filter(|t| t.uuid().is_some())
    }

    async fn get_by_name_start(name_start: &str, app_meta: &AppMeta) -> Vec<Thing> {
        app_meta
            .repository
            .get_by_name_start(name_start, Some(20))
            .await
            .unwrap_or_default()
            .drain(..)
            .filter(|t| t.uuid().is_some())
            .collect()
    }
}
