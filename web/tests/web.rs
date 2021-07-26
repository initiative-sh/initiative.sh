#![cfg(target_arch = "wasm32")]

use initiative_web::app;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn motd() {
    let motd = app().motd();
    assert!(motd.lines().count() > 1, "{}", motd);
}

#[wasm_bindgen_test]
fn autocomplete() {
    let suggestions = app().autocomplete("h");
    assert_eq!(
        [
            ("half-elf", "generate"),
            ("half-orc", "generate"),
            ("halfling", "generate"),
            ("help", "how to use initiative.sh"),
            ("human", "generate"),
        ]
        .iter()
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect::<Vec<_>>(),
        suggestions,
    );
}

#[wasm_bindgen_test]
fn command() {
    let output = app().command("about");
    assert!(output.contains("initiative.sh"), "{}", output);
}

#[wasm_bindgen_test]
fn app_is_persistent() {
    let npc_output = app().command("npc");
    let npc_name = npc_output.lines().next().unwrap();
    let npc_details = app().command(npc_name);

    assert!(npc_details.lines().count() > 1, "{}", npc_details);

    assert_eq!(npc_details, app().command(npc_name));
}

#[wasm_bindgen_test]
fn memory_exhaustion() {
    let npc_output = loop {
        let output = app().command("npc");
        if output
            .lines()
            .next()
            .map_or(false, |name| name.contains(' '))
        {
            break output;
        }
    };

    let npc_name = npc_output.lines().next().unwrap();
    let npc_details = app().command(npc_name);
    assert!(npc_details.lines().count() > 1, "{}", npc_details);

    // Expect to keep at least 88 NPCs in memory before evicting.
    for i in 0..8 {
        app().command("npc");
        assert_eq!(
            npc_details,
            app().command(npc_name),
            "Failed on iteration {}",
            i,
        );
    }
}
