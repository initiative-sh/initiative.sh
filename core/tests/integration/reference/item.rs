use crate::common::sync_app;

#[test]
fn light_crossbow() {
    let output = sync_app().command("Light Crossbow").unwrap();

    assert_eq!(
        "\
# Light Crossbow
*Weapon (Simple Ranged)*

**Cost:** 25 gp\\
**Damage:** `1d8` piercing\\
**Properties:** Ammunition (range 80/320), loading, two-handed\\
**Weight:** 5 lbs

*Light Crossbow is Open Game Content subject to the `Open Game License`.*",
        output,
    );

    assert_eq!(output, sync_app().command("Crossbow, Light").unwrap());
    assert_eq!(
        output,
        sync_app().command("srd item Light Crossbow").unwrap(),
    );

    assert_eq!(
        vec![("Light Crossbow".into(), "SRD item".into())],
        sync_app().autocomplete("light crossbow"),
    );
}
