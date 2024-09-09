use crate::common::{sync_app, sync_app_with_invalid_data_store};

const TUTORIAL_STEPS: usize = 16;

#[test]
fn happy_path() {
    let mut app = sync_app();

    let output = app.command("tutorial").unwrap();
    println!("{}", output);

    let mut command = "next".to_string();

    for i in 0..100 {
        println!("> {}\n", command);
        let output = app.command(&command).unwrap();
        println!("{}", output);

        let tutorial_pos = output.find("# Tutorial").unwrap();

        if output.contains("## Tutorial: Conclusion") {
            assert_eq!(TUTORIAL_STEPS - 2, i);
            return;
        }

        command = output[tutorial_pos..]
            .split(&['`', '~'][..])
            .nth(1)
            .unwrap()
            .to_string();
    }

    panic!("Broke out of infinite loop!");
}

#[test]
fn shouting_happy_path() {
    let mut app = sync_app();

    let output = app.command("TUTORIAL").unwrap();
    println!("{}", output);

    let mut command = "NEXT".to_string();

    for i in 0..100 {
        println!("> {}\n", command);
        let output = app.command(&command).unwrap();
        println!("{}", output);

        let tutorial_pos = output.find("# Tutorial").unwrap();

        if output.contains("## Tutorial: Conclusion") {
            assert_eq!(TUTORIAL_STEPS - 2, i);
            return;
        }

        command = output[tutorial_pos..]
            .split(&['`', '~'][..])
            .nth(1)
            .unwrap()
            .to_uppercase();
    }

    panic!("Broke out of infinite loop!");
}

#[test]
fn works_without_local_storage() {
    let mut app = sync_app_with_invalid_data_store();

    let output = app.command("tutorial").unwrap();
    println!("{}", output);

    let mut command = "next".to_string();

    for i in 0..100 {
        println!("> {}\n", command);
        let output = app.command(&command).unwrap_or_else(|e| e);
        println!("{}", output);

        let tutorial_pos = output.find("# Tutorial").unwrap();

        if output.contains("## Tutorial: Conclusion") {
            assert_eq!(TUTORIAL_STEPS - 2, i);
            return;
        }

        command = output[tutorial_pos..]
            .split(&['`', '~'][..])
            .nth(1)
            .unwrap()
            .to_string();
    }

    panic!("Broke out of infinite loop!");
}

#[test]
fn cancel() {
    for i in 0..TUTORIAL_STEPS - 1 {
        println!("Cancelling after {} steps...", i);
        cancel_after_steps(i);
    }
}

fn cancel_after_steps(steps: usize) {
    let mut app = sync_app();

    app.command("tutorial").unwrap();

    let mut command = "next".to_string();

    for _ in 0..steps {
        println!("> {}\n", command);
        let output = app.command(&command).unwrap();
        println!("{}", output);

        command = output[output.find("# Tutorial").unwrap()..]
            .split(&['`', '~'][..])
            .nth(1)
            .unwrap()
            .to_string();
    }

    println!("> cancel\n");
    let cancel_result = app.command("cancel").unwrap();
    println!("{}", cancel_result);

    assert_eq!("The tutorial has been cancelled.", cancel_result.trim());

    let journal_output = app.command("journal").unwrap();
    assert!(
        journal_output.contains("Your journal is currently empty."),
        "The journal should be empty.\n\n{}",
        journal_output,
    );
}

#[test]
fn resume() {
    for i in 0..TUTORIAL_STEPS - 1 {
        println!("Resuming at step {}...", i);
        resume_at_step(i);
    }
}

fn resume_at_step(step: usize) {
    let mut app = sync_app();

    app.command("tutorial").unwrap();

    let mut command = "next".to_string();

    for _ in 0..step {
        println!("> {}\n", command);
        let output = app.command(&command).unwrap();
        println!("{}", output);

        command = output[output.find("# Tutorial").unwrap()..]
            .split(&['`', '~'][..])
            .nth(1)
            .unwrap()
            .to_string();
    }

    let output = app.command("about").unwrap();
    assert!(output.contains("## Tutorial"), "{}", output);
    command = "resume".to_string();

    for i in step..TUTORIAL_STEPS {
        println!("> {}\n", command);
        let output = app.command(&command).unwrap();
        println!("{}", output);

        command = if i == 0 {
            "next".to_string()
        } else {
            output[output.find("# Tutorial").unwrap()..]
                .split(&['`', '~'][..])
                .nth(1)
                .unwrap()
                .to_string()
        };
    }

    app.command("cancel").unwrap_err();

    let journal_output = app.command("journal").unwrap();
    assert!(
        journal_output.contains("Your journal is currently empty."),
        "{}",
        journal_output,
    );
}

#[test]
fn restart() {
    for i in 0..TUTORIAL_STEPS - 1 {
        println!("\n\nRestarting from step {}...\n\n", i);
        restart_from_step(i);
    }
}

fn restart_from_step(step: usize) {
    let mut app = sync_app();

    println!("> tutorial\n");
    println!("{}", app.command("tutorial").unwrap());

    let mut command = "next".to_string();

    for _ in 0..step {
        println!("> {}\n", command);
        let output = app.command(&command).unwrap();
        println!("{}", output);

        command = output[output.find("# Tutorial").unwrap()..]
            .split(&['`', '~'][..])
            .nth(1)
            .unwrap()
            .to_string();
    }

    println!("> about\n");
    let output = app.command("about").unwrap();
    println!("{}", output);
    assert!(output.contains("## Tutorial"), "{}", output);

    println!("> restart\n");
    let output = app.command("restart").unwrap();
    println!("{}", output);
    assert!(output.contains("# Tutorial: Introduction"), "{}", output);

    command = "next".to_string();

    for _ in 1..TUTORIAL_STEPS {
        println!("> {}\n", command);
        let output = app.command(&command).unwrap();
        println!("{}", output);

        command = output[output.find("# Tutorial").unwrap()..]
            .split(&['`', '~'][..])
            .nth(1)
            .unwrap()
            .to_string();
    }

    println!("> cancel\n");
    println!("{}\n", app.command("cancel").unwrap_err());

    println!("> journal\n");
    let journal_output = app.command("journal").unwrap();
    println!("{}", journal_output);
    assert!(
        journal_output.contains("Your journal is currently empty."),
        "{}",
        journal_output,
    );
}
