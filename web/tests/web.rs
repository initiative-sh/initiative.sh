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
async fn autocomplete() {
    let suggestions = app().autocomplete("h").await;
    assert_eq!(
        [
            ("half-elf", "generate"),
            ("half-orc", "generate"),
            ("halfling", "generate"),
            ("heavy armor", "SRD item category"),
            ("help", "how to use initiative.sh"),
            ("holy symbols", "SRD item category"),
            ("human", "generate"),
        ]
        .iter()
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect::<Vec<_>>(),
        suggestions,
    );
}

#[wasm_bindgen_test]
async fn command() {
    let output = app().command("about").await;
    assert!(output.contains("initiative.sh"), "{}", output);
}

#[wasm_bindgen_test]
async fn app_is_persistent() {
    let npc_output = app().command("npc").await;
    let npc_name = npc_output.lines().next().unwrap().trim_start_matches("# ");
    let npc_details = app().command(npc_name).await;

    assert!(npc_details.lines().count() > 1, "{}", npc_details);

    assert_eq!(npc_details, app().command(npc_name).await);
}

#[wasm_bindgen_test]
async fn memory_exhaustion() {
    let npc_output = loop {
        let output = app().command("npc").await;
        if output
            .lines()
            .next()
            .map_or(false, |name| name.contains(' '))
        {
            break output;
        }
    };

    let npc_name = npc_output.lines().next().unwrap().trim_start_matches("# ");
    let npc_details = app().command(npc_name).await;
    assert!(npc_details.lines().count() > 1, "{}", npc_details);

    // Expect to keep at least 88 NPCs in memory before evicting.
    for i in 0..8 {
        app().command("npc").await;
        assert_eq!(
            npc_details,
            app().command(npc_name).await,
            "Failed on iteration {}",
            i,
        );
    }
}
