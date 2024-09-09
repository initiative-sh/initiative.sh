use crate::common::sync_app;

#[test]
fn it_works() {
    let output = sync_app().command("help").unwrap();
    assert!(output.contains("command"), "{}", output);
}

#[test]
fn all_commands_are_valid() {
    let output = sync_app().command("help").unwrap();

    for output_part in output.split('*') {
        let mut app = sync_app();
        for command in output_part
            .split('`')
            .skip(1)
            .step_by(2)
            .filter(|s| !s.contains('['))
        {
            // Basically, we just want to make sure all commands run successfully.
            app.command(command).expect(command);
        }
    }
}
