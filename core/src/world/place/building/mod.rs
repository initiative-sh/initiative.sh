mod business;
mod education;
mod government;
mod military;
mod religious;
mod travel;

use initiative_macros::WordList;

#[derive(WordList)]
pub enum BuildingType {
    #[term = "building"]
    Any,

    Business(business::BusinessType),
    Education(education::EducationType),
    Government(government::GovernmentType),
    Military(military::MilitaryType),
    Religious(religious::ReligiousType),
    #[alias = "house"]
    #[alias = "lodge"]
    #[alias = "manor"]
    #[alias = "mansion"]
    Residence,
    Travel(travel::TravelType),
}
