use std::cmp::Ordering;

pub trait CaseInsensitiveStr<'a> {
    fn eq_ci<S: AsRef<str>>(&self, other: S) -> bool;

    fn ne_ci<S: AsRef<str>>(&self, other: S) -> bool {
        !self.eq_ci(other)
    }

    fn cmp_ci<S: AsRef<str>>(&self, other: S) -> Ordering;

    fn in_ci<S: AsRef<str>>(&self, haystack: &[S]) -> bool;

    fn starts_with_ci<S: AsRef<str>>(&self, prefix: S) -> bool;

    fn ends_with_ci<S: AsRef<str>>(&self, suffix: S) -> bool;

    fn strip_prefix_ci<S: AsRef<str>>(&'a self, prefix: S) -> Option<&'a str>;

    fn strip_suffix_ci<S: AsRef<str>>(&'a self, prefix: S) -> Option<&'a str>;
}

impl<'a, T: AsRef<str>> CaseInsensitiveStr<'a> for T {
    fn eq_ci<S: AsRef<str>>(&self, other: S) -> bool {
        let (a, b) = (self.as_ref(), other.as_ref());

        a == b
            || (a.len() == b.len())
                && a.chars().zip(b.chars()).all(|(a, b)| {
                    a == b
                        || !(!a.is_alphabetic()
                            || !b.is_alphabetic()
                            || a.is_lowercase() == b.is_lowercase()
                            || !a.to_lowercase().eq(b.to_lowercase()))
                })
    }

    fn cmp_ci<S: AsRef<str>>(&self, other: S) -> Ordering {
        let (a, b) = (self.as_ref(), other.as_ref());

        if a == b {
            Ordering::Equal
        } else {
            a.chars()
                .zip(b.chars())
                .find_map(|(a, b)| {
                    match if a == b {
                        Ordering::Equal
                    } else if a.is_uppercase() || b.is_uppercase() {
                        a.to_lowercase().cmp(b.to_lowercase())
                    } else {
                        a.cmp(&b)
                    } {
                        Ordering::Equal => None,
                        o => Some(o),
                    }
                })
                .unwrap_or_else(|| a.len().cmp(&b.len()))
        }
    }

    fn in_ci<S: AsRef<str>>(&self, haystack: &[S]) -> bool {
        let needle = self.as_ref();
        haystack.iter().any(|s| s.eq_ci(needle))
    }

    fn starts_with_ci<S: AsRef<str>>(&self, prefix: S) -> bool {
        let (subject, prefix) = (self.as_ref(), prefix.as_ref());

        if let Some(start) = subject.get(..prefix.len()) {
            start.eq_ci(prefix)
        } else {
            false
        }
    }

    fn ends_with_ci<S: AsRef<str>>(&self, suffix: S) -> bool {
        let (subject, suffix) = (self.as_ref(), suffix.as_ref());

        if let Some(end) = subject
            .len()
            .checked_sub(suffix.len())
            .and_then(|i| subject.get(i..))
        {
            end.eq_ci(suffix)
        } else {
            false
        }
    }

    fn strip_prefix_ci<S: AsRef<str>>(&'a self, prefix: S) -> Option<&'a str> {
        let prefix = prefix.as_ref();

        if self.starts_with_ci(prefix) {
            self.as_ref().get(prefix.len()..)
        } else {
            None
        }
    }

    fn strip_suffix_ci<S: AsRef<str>>(&'a self, suffix: S) -> Option<&'a str> {
        let suffix = suffix.as_ref();

        if self.ends_with_ci(suffix) {
            let subject = self.as_ref();

            subject
                .len()
                .checked_sub(suffix.len())
                .and_then(|i| subject.get(..i))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn eq_ci_test() {
        assert!("".eq_ci(""));
        assert!("abc".eq_ci("abc"));
        assert!("abc".eq_ci("abC"));
        assert!("!@#".eq_ci("!@#"));
        assert!("p🥔tat🥔".eq_ci("P🥔TAT🥔"));

        assert!("abcd".ne_ci("abc"));
        assert!("abc".ne_ci("abcd"));
        assert!("".ne_ci("🥔"));
        assert!("🥔".ne_ci(""));
        assert!("🥔".ne_ci("potato"));
        assert!("potato".ne_ci("🥔"));
        assert!("SS".ne_ci("ß"));
        assert!("ß".ne_ci("S"));
        assert!("S".ne_ci("ß"));
    }

    #[test]
    #[ignore]
    fn eq_ci_test_failing() {
        // This is a known limitation. Documenting here for posteritity.
        assert!("ß".eq_ci("SS"));
    }

    #[test]
    fn starts_ends_with_ci_test() {
        assert!("AbC".starts_with_ci("aB"));
        assert!("AbC".ends_with_ci("Bc"));
        assert!("AbC".starts_with_ci(""));
        assert!("AbC".ends_with_ci(""));

        assert!(!"🥔".starts_with_ci("a"));
        assert!(!"🥔".ends_with_ci("a"));
        assert!(!"abc".starts_with_ci("abcd"));
        assert!(!"abc".ends_with_ci("abcd"));
    }

    #[test]
    fn strip_prefix_suffix_ci_test() {
        assert_eq!(Some("aBC"), "aBCXYz".strip_suffix_ci("xYz"));
        assert_eq!(Some("XYz"), "aBCXYz".strip_prefix_ci("aBc"));
        assert_eq!(Some("p🥔tat"), "p🥔tat🥔".strip_suffix_ci("🥔"));

        assert_eq!(Some(""), "".strip_prefix_ci(""));
    }

    #[test]
    fn cmp_ci_test() {
        let mut data = vec![
            "ddd", "aaa", "!", "aaaa", "aAA", "", "aaa", "CCC", "🥔", "Bbb",
        ];

        data.sort_by(|a, b| a.cmp_ci(b));

        assert_eq!(
            vec!["", "!", "aaa", "aAA", "aaa", "aaaa", "Bbb", "CCC", "ddd", "🥔"],
            data,
        );
    }

    #[test]
    fn in_ci_test() {
        assert!("B".in_ci(&["a", "b", "c"]));
        assert!(!"d".in_ci(&["a", "b", "c"]));
    }
}
