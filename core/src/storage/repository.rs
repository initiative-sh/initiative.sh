use crate::app::AppMeta;
use crate::world::Thing;
use uuid::Uuid;

pub fn save(app_meta: &mut AppMeta, name: &str) -> Result<String, String> {
    let lowercase_name = name.to_lowercase();
    if let Some(mut thing) = app_meta.take_recent(|t| {
        t.name()
            .value()
            .map_or(false, |s| s.to_lowercase() == lowercase_name)
    }) {
        thing.set_uuid(Uuid::new_v4());
        let result = format!("Saving {}", thing.display_summary());
        app_meta.data_store.save(&thing);
        Ok(result)
    } else {
        Err(format!("No matches for \"{}\"", name))
    }
}

pub fn load<'a>(app_meta: &'a AppMeta, name: &str) -> Option<&'a Thing> {
    let lowercase_name = name.to_lowercase();
    app_meta.recent().iter().find(|t| {
        t.name()
            .value()
            .map_or(false, |s| s.to_lowercase() == lowercase_name)
    })
}
