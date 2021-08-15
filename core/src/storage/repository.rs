use crate::app::AppMeta;
use crate::world::Thing;
use uuid::Uuid;

pub async fn save(app_meta: &mut AppMeta, name: &str) -> Result<String, String> {
    let lowercase_name = name.to_lowercase();
    if let Some(mut thing) = app_meta.take_recent(|t| {
        t.name()
            .value()
            .map_or(false, |s| s.to_lowercase() == lowercase_name)
    }) {
        thing.set_uuid(Uuid::new_v4());

        let result = app_meta
            .data_store
            .save(&thing)
            .await
            .map_err(|_| format!("Couldn't save `{}`", thing.name()))
            .map(|_| format!("Saving {}", thing.display_summary()));

        if result.is_ok() {
            app_meta.cache.insert(*thing.uuid().unwrap(), thing);
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

pub fn load<'a>(app_meta: &'a AppMeta, name: &str) -> Option<&'a Thing> {
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

pub fn load_all(app_meta: &AppMeta) -> impl Iterator<Item = &Thing> {
    app_meta.cache.values()
}

pub async fn init_cache(app_meta: &mut AppMeta) {
    let things = app_meta.data_store.get_all().await;

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
}
