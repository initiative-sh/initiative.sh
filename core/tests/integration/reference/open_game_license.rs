use crate::common::sync_app;
use initiative_core::app::AutocompleteSuggestion;

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
        vec![AutocompleteSuggestion::new(
            "Open Game License",
            "SRD license",
        )],
        sync_app().autocomplete("open game license"),
    );
}
