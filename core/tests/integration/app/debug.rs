use crate::common::{get_name, sync_app};

#[test]
fn it_works() {
    let mut app = sync_app();

    {
        let output = app.command("debug").unwrap();
        assert!(output.starts_with("AppMeta { "), "{}", output);
    }

    {
        let name = get_name(&app.command("npc").unwrap());
        let output = app.command("debug").unwrap();
        assert!(output.contains(&name), "{}", output);
    }
}
