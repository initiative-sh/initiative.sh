use crate::common::{sync_app, sync_app_with_invalid_data_store};

#[test]
fn it_shows_a_message_when_empty() {
    assert_eq!(
        "\
# Journal

*Your journal is currently empty.*",
        sync_app().command("journal").unwrap(),
    );
}

#[test]
fn it_shows_an_error_without_a_data_store() {
    let mut app = sync_app_with_invalid_data_store();
    assert_eq!(
        "# Journal\n\n*Your journal is currently empty.*\n\n! Your browser does not support local storage. Any changes will not persist beyond this session.",
        app.command("journal").unwrap(),
    );
}

#[test]
fn it_shows_alphabetized_results() {
    let mut app = sync_app();

    app.command("npc").unwrap();

    let npc_list = app.command("more").unwrap();
    println!("{}", npc_list);
    let mut npcs: Vec<&str> = npc_list
        .lines()
        .filter(|s| s.starts_with('~'))
        .map(|s| s[4..].trim_end_matches('\\'))
        .inspect(|line| {
            println!(
                "{}",
                app.command(&format!(
                    "save {}",
                    line.find('(').map(|pos| &line[6..(pos - 2)]).unwrap(),
                ))
                .unwrap(),
            )
        })
        .collect();

    assert_eq!(10, npcs.len());
    npcs.sort_by(|a, b| a[6..].cmp(&b[6..]));

    app.command("inn").unwrap();

    let inn_list = app.command("more").unwrap();
    println!("{}", inn_list);
    let mut inns: Vec<&str> = inn_list
        .lines()
        .filter(|s| s.starts_with('~'))
        .map(|s| s[4..].trim_end_matches('\\'))
        .inspect(|line| {
            println!(
                "{}",
                app.command(&format!(
                    "save {}",
                    line.find('(').map(|pos| &line[6..(pos - 2)]).unwrap(),
                ))
                .unwrap(),
            )
        })
        .collect();

    assert_eq!(10, inns.len());
    inns.sort();

    let output = app.command("journal").unwrap();
    println!("{}", output);
    let mut output_iter = output.lines();

    assert_eq!(Some("# Journal"), output_iter.next());
    assert_eq!(Some(""), output_iter.next());
    assert_eq!(Some("## NPCs"), output_iter.next());

    npcs.into_iter()
        .zip(output_iter.by_ref())
        .enumerate()
        .for_each(|(i, (a, b))| {
            if i == 9 {
                assert_eq!(a, b, "{}", output)
            } else {
                assert_eq!(format!("{}\\", a), b, "{}", output)
            }
        });

    assert_eq!(Some(""), output_iter.next());
    assert_eq!(Some("## Places"), output_iter.next());

    inns.into_iter()
        .zip(output_iter.by_ref())
        .enumerate()
        .for_each(|(i, (a, b))| {
            if i == 9 {
                assert_eq!(a, b)
            } else {
                assert_eq!(format!("{}\\", a), b)
            }
        });

    assert_eq!(
        Some("*To export the contents of your journal, use `export`.*"),
        output_iter.by_ref().nth(1),
    );

    assert!(output_iter.next().is_none());
}
