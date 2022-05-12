use crate::common::sync_app;

#[test]
fn open_game_license() {
    assert_eq!(
        111,
        sync_app()
            .command("Open Game License")
            .unwrap()
            .lines()
            .count()
    );

    assert_eq!(
        vec![("Open Game License".into(), "SRD license".into())],
        sync_app().autocomplete("open game license"),
    );
}
