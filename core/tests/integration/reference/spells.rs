use crate::common::sync_app;

#[test]
fn spells() {
    let output = sync_app().command("spells").unwrap();
    assert_eq!(
        "\
# Spells
* `Acid Arrow` (2nd-level evocation)
* `Acid Splash` (conjuration cantrip)
* `Aid` (2nd-level abjuration)
* `Alarm` (1st-level abjuration)
",
        output.split_inclusive('\n').take(5).collect::<String>(),
    );

    assert_eq!(322, output.lines().count(), "{}", output);

    assert_eq!(
        vec![("spells".into(), "SRD index".into())],
        sync_app().autocomplete("Spells"),
    );
}
