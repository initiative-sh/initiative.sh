use initiative_reference::srd_5e::conditions;

#[test]
fn prone() {
    let conditions = conditions().unwrap();
    let condition = conditions.iter().find(|i| i.name == "Prone").unwrap();

    assert_eq!("`Prone`", condition.display_summary().to_string());

    assert_eq!(
        "\
# Prone

- A prone creature's only movement option is to crawl, unless it stands up and thereby ends the condition.
- The creature has disadvantage on attack rolls.
- An attack roll against the creature has advantage if the attacker is within 5 feet of the creature. Otherwise, the attack roll has disadvantage.",
        condition.display_details().to_string(),
    );
}
