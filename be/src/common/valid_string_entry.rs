use serde::{Deserialize, Serialize};
use tracing::Value;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "String")]
pub struct ValidStringEntry(String);

impl ValidStringEntry {
    pub fn parse(s: String) -> Result<ValidStringEntry, String> {
        let is_empty_or_whitespaces = s.trim().is_empty();

        let is_too_long = s.graphemes(true).count() > 256;

        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];

        let contains_forbidden_chars = s.chars().any(|g| forbidden_characters.contains(&g));

        if is_empty_or_whitespaces || is_too_long || contains_forbidden_chars {
            Err(format!("{} is not a valid book title.", s))
        } else {
            Ok(Self(s))
        }
    }

    // the caller gets a shared ref to the inner string - read only access
    // pub fn inner_ref(&self) -> &str {
    //     &self.0
    // }
}

impl AsRef<str> for ValidStringEntry {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ValidStringEntry {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::parse(s)
    }
}

#[cfg(test)]
mod tests {
    // use crate::utils::ValidStringEntry;
    // use claims::{assert_err, assert_ok};
    //
    // #[test]
    // fn valid_grapheme() {
    //     let name = "ä".repeat(255);
    //     assert_ok!(ValidStringEntry::parse(name));
    // }
    //
    // #[test]
    // fn invalid_grapheme() {
    //     let name = "ä".repeat(355);
    //     assert_err!(ValidStringEntry::parse(name));
    // }
    //
    // #[test]
    // fn reject_whitespaces() {
    //     let name = " ".to_string();
    //     assert_err!(ValidStringEntry::parse(name));
    // }
    //
    // #[test]
    // fn reject_empty_string() {
    //     let name = "".to_string();
    //     assert_err!(ValidStringEntry::parse(name));
    // }
    //
    // #[test]
    // fn reject_if_contains_illegal_chars() {
    //     for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
    //         let name = name.to_string();
    //         assert_err!(ValidStringEntry::parse(name));
    //     }
    // }
    //
    // #[test]
    // fn valid_string_passes() {
    //     let name = "Chinua Achebe".to_string();
    //     assert_ok!(ValidStringEntry::parse(name));
    // }
}
