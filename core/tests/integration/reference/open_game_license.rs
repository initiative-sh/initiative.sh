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
}
