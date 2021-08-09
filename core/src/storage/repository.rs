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
        let result = format!("Saving {}", thing.display_summary());
        app_meta.data_store.save(&thing).await;
        app_meta.cache.insert(*thing.uuid().unwrap(), thing);
        Ok(result)
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

pub async fn init_cache(app_meta: &mut AppMeta) {
    let mut things = app_meta.data_store.get_all().await;
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
}
