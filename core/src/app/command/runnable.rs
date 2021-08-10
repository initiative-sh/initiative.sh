use crate::app::AppMeta;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait Runnable: Sized {
    async fn run(&self, app_meta: &mut AppMeta) -> String;

    fn summarize(&self) -> &str;

    fn parse_input(input: &str, app_meta: &AppMeta) -> Vec<Self>;

    fn autocomplete(input: &str, app_meta: &AppMeta) -> Vec<(String, String)>;
}

pub fn autocomplete_phrase(
    input: &str,
    vocabulary: &mut dyn Iterator<Item = &&str>,
) -> Vec<String> {
    if input.is_empty() {
        Vec::new()
    } else {
        let mut suggestions: Vec<String> = vocabulary
            .filter(|word| word.starts_with(input))
            .map(|&s| s.to_string())
            .collect();

        suggestions.sort();
        suggestions.truncate(10);

        suggestions
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn autocomplete_phrase_test() {
        let words = [
            "wolf apple",
            "wild tomato",
            "tomato",
            "potato",
            "potato bush",
            "pepino melon",
            "naranjilla",
            "kangaroo apple",
            "gilo",
            "garden huckleberry",
            "eggplant",
            "desert raisin",
            "bush tomato",
            "Turkey berry",
            "Tamarillo",
            "Solanum tuberosum",
            "Solanum torvum",
            "Solanum scabrum",
            "Solanum quitoense",
            "Solanum pimpinellifolium",
            "Solanum peruvianum",
            "Solanum muricatum",
            "Solanum melongena",
            "Solanum lycopersicum",
            "Solanum lycocarpum",
            "Solanum galapagense",
            "Solanum chilense",
            "Solanum cheesmanii",
            "Solanum betaceum",
            "Solanum aethiopicum",
        ];
        let empty_vec: Vec<String> = Vec::new();

        assert_eq!(
            vec!["potato", "potato bush"],
            autocomplete_phrase("pot", &mut words.iter()),
        );

        assert_eq!(
            vec!["potato bush"],
            autocomplete_phrase("potato ", &mut words.iter()),
        );

        assert_eq!(
            empty_vec,
            autocomplete_phrase("my tasty potato", &mut words.iter()),
        );

        assert_eq!(
            vec!["Tamarillo", "Turkey berry"],
            autocomplete_phrase("T", &mut words.iter()),
        );

        assert_eq!(
            vec![
                "Solanum aethiopicum",
                "Solanum betaceum",
                "Solanum cheesmanii",
                "Solanum chilense",
                "Solanum galapagense",
                "Solanum lycocarpum",
                "Solanum lycopersicum",
                "Solanum melongena",
                "Solanum muricatum",
                "Solanum peruvianum",
            ],
            autocomplete_phrase("Solanum", &mut words.iter()),
        );

        assert_eq!(empty_vec, autocomplete_phrase("", &mut words.iter()));
        assert_eq!(empty_vec, autocomplete_phrase("carrot", &mut words.iter()));
        assert_eq!(
            empty_vec,
            autocomplete_phrase("\u{1f954}\u{2003}\u{1f954}", &mut words.iter()),
        );
    }
}
