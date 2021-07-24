use initiative_core::app;

#[test]
fn open_game_license() {
    assert_eq!(35, app().command("Open Game License").lines().count());
}
