pub trait Autocomplete {
    fn autocomplete(input: &str) -> Vec<String>;
}
