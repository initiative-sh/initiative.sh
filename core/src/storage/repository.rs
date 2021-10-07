use crate::app::AppMeta;
use crate::{Thing, Uuid};

pub async fn delete_thing_by_name(app_meta: &mut AppMeta, name: &str) -> Result<String, String> {
    let lowercase_name = name.to_lowercase();
    let name_matches = |s: &String| s.to_lowercase() == lowercase_name;

    let cached_thing = if let Some((uuid, thing)) = app_meta
        .cache
        .iter()
        .find(|(_, t)| t.name().value().map_or(false, name_matches))
    {
        Some((*uuid, thing.name().to_string()))
    } else {
        None
    };

    if let Some((uuid, thing_name)) = cached_thing {
        let (store_delete_success, cache_delete_success) = (
            app_meta
                .data_store
                .delete_thing_by_uuid(&uuid)
                .await
                .is_ok(),
            app_meta.cache.remove(&uuid).is_some(),
        );

        if store_delete_success || cache_delete_success {
            Ok(format!("{} was successfully deleted.", thing_name))
        } else {
            Err(format!("Could not delete {}.", thing_name))
        }
    } else if let Some(thing) =
        app_meta.take_recent(|t| t.name().value().map_or(false, name_matches))
    {
        Ok(format!(
            "{} deleted from recent entries. This isn't normally necessary as recent entries aren't automatically saved from one session to another.",
            thing.name(),
        ))
    } else {
        Err(format!("There is no entity named {}.", name))
    }
}

pub async fn init_cache(app_meta: &mut AppMeta) {
    let things = app_meta.data_store.get_all_the_things().await;

    if let Ok(mut things) = things {
        app_meta.cache = things
            .drain(..)
            .filter_map(|thing| {
                if let Some(&uuid) = thing.uuid() {
                    Some((uuid, thing))
                } else {
                    None
                }
            })
            .collect();
        app_meta.data_store_enabled = true;
    }

    if let Ok(Some(time_str)) = app_meta.data_store.get_value("time").await {
        if let Ok(time) = time_str.parse() {
            app_meta.set_time(time).await;
        }
    }
}

pub fn load_thing_by_name<'a>(app_meta: &'a AppMeta, name: &str) -> Option<&'a Thing> {
    let lowercase_name = name.to_lowercase();
    app_meta
        .cache
        .values()
        .chain(app_meta.recent().iter())
        .find(|t| {
            t.name()
                .value()
                .map_or(false, |s| s.to_lowercase() == lowercase_name)
        })
}

pub fn load_all_the_things(app_meta: &AppMeta) -> impl Iterator<Item = &Thing> {
    app_meta.cache.values()
}

pub async fn save_thing_by_name(app_meta: &mut AppMeta, name: &str) -> Result<String, String> {
    let lowercase_name = name.to_lowercase();
    if let Some(mut thing) = app_meta.take_recent(|t| {
        t.name()
            .value()
            .map_or(false, |s| s.to_lowercase() == lowercase_name)
    }) {
        thing.set_uuid(Uuid::new_v4());

        let result = app_meta
            .data_store
            .save_thing(&thing)
            .await
            .map_err(|_| format!("Couldn't save `{}`", thing.name()))
            .map(|_| format!("{} was successfully saved.", thing.display_summary()));

        if result.is_ok() {
            app_meta.cache.insert(*thing.uuid().unwrap(), thing);
        } else {
            // Oops, better put it back where we found it.
            app_meta.push_recent(thing);
        }

        result
    } else if app_meta.cache.values().any(|t| {
        t.name()
            .value()
            .map_or(false, |s| s.to_lowercase() == lowercase_name)
    }) {
        Err(format!(
            "`{}` has already been saved to your `journal`",
            name,
        ))
    } else {
        Err(format!("No matches for \"{}\"", name))
    }
}
