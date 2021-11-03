use initiative_macros::WordList;

#[derive(WordList)]
pub enum SettlementType {
    Camp,
    Campsite,
    Capital,
    #[alias = "metropolis"]
    City,
    #[alias = "ward"]
    #[alias = "quarter"]
    #[alias = "neighborhood"]
    District,
    Outpost,
    #[alias = "hamlet"]
    #[alias = "village"]
    #[alias = "parish"]
    Town,
}
