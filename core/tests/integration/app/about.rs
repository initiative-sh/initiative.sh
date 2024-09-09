use crate::common::sync_app;

#[test]
fn it_works() {
    let output = sync_app().command("about").unwrap();
    assert!(output.contains("initiative.sh"), "{}", output);
}
