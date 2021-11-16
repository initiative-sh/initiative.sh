use crate::common::sync_app;

#[test]
fn it_works() {
    let output = sync_app().command("help").unwrap();
    assert!(output.contains("command"), "{}", output);
}
