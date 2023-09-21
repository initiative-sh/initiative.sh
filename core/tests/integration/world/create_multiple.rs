use crate::common::{sync_app, sync_app_with_data_store};
use initiative_core::NullDataStore;

#[test]
fn more_alias() {
    let mut app = sync_app();
    let mut output = app.command("npc").unwrap();

    for i in 1..2 {
        assert!(output.contains("~more~"), "Iteration {}\n\n{}", i, output);
        // # Alternative suggestions for "person":
        //
        // ~1~ `Jaya` (middle-aged human, she/her)\
        // ~2~ `Harsha` (half-elf infant, he/him)\
        // ~3~ `Lucan Amakiir` (elderly half-elf, he/him)\
        // ~4~ `Germana` (middle-aged human, she/her)\
        // ~5~ `Akachi` (geriatric human, she/her)\
        // ~6~ `Callie Bigheart` (middle-aged halfling, she/her)\
        // ~7~ `Pratima` (young adult human, she/her)\
        // ~8~ `Laelia` (human infant, she/her)\
        // ~9~ `Pierre` (adult human, he/him)\
        // ~0~ `Mokosh` (middle-aged half-elf, she/her)
        //
        // _For even more suggestions, type ~more~.
        output = app.command("more").unwrap();

        // Ensure that secondary suggestions have also been persisted.
        assert_eq!(
            10,
            output
                .lines()
                .filter(|line| line.starts_with('~'))
                .map(|s| {
                    if let Some(pos) = s.find('(') {
                        let name = &s[10..(pos - 2)];
                        assert_eq!(
                            format!("# {}", name),
                            app.command(&format!("load {}", name))
                                .unwrap()
                                .lines()
                                .nth(2)
                                .unwrap(),
                            "Iteration {}",
                            i,
                        );
                    } else {
                        panic!("Missing ( in \"{}\"", s);
                    }
                })
                .count(),
            "Iteration {}\n\n{}",
            i,
            output,
        );
    }
}

#[test]
fn more_alias_exists_with_invalid_data_store() {
    let mut app = sync_app_with_data_store(NullDataStore::default());

    let output = app.command("npc").unwrap();
    assert!(output.contains("~more~"), "{}", output);
    app.command("more").unwrap();
}

#[test]
fn more_alias_does_not_exist_with_name() {
    {
        let mut app = sync_app();
        let output = app.command("place called Home").unwrap();
        assert!(!output.contains("~more~"), "{}", output);
        app.command("more").unwrap_err();
    }

    {
        let mut app = sync_app_with_data_store(NullDataStore::default());
        let output = app.command("place called Home").unwrap();
        assert!(!output.contains("~more~"), "{}", output);
        app.command("more").unwrap_err();
    }
}

#[test]
fn numeric_aliases() {
    let mut app = sync_app();

    // Generate a data set to potentially interfere with the one being tested.
    app.command("npc").unwrap();
    app.command("npc").unwrap();

    let generated_output = app.command("more").unwrap();

    // Doing this in two steps due to borrowing issues.
    let outputs = generated_output
        .lines()
        .filter(|line| line.starts_with('~'))
        .map(|s| {
            if let Some(pos) = s.find('(') {
                let digit = &s[1..2];
                let digit_output = app.command(digit).unwrap();

                let name = &s[10..(pos - 2)];

                assert_eq!(format!("# {}", name), digit_output.lines().nth(2).unwrap());

                (digit_output, name.to_string())
            } else {
                panic!("Missing ( in \"{}\"", s);
            }
        })
        .collect::<Vec<_>>();

    assert_eq!(
        10,
        outputs
            .into_iter()
            .map(|(digit_output, name)| {
                let name_output = app.command(&format!("load {}", name)).unwrap();
                assert_eq!(digit_output, name_output);
            })
            .count(),
        "{}",
        generated_output,
    );
}
