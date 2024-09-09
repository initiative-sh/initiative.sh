use crate::common::sync_app;

#[test]
fn it_works() {
    let output = sync_app().command("changelog").unwrap();
    assert_eq!(
        10,
        output.lines().filter(|s| s.starts_with('*')).count(),
        "{}",
        output,
    );
    assert!(output.lines().count() > 10, "{}", output);
}
