use crate::card_error::CardError;
use fluent_templates::{langid, static_loader, LanguageIdentifier, Loader};
use std::str::FromStr;
use std::string::ToString;

static_loader! {
    pub static LOCALES = {
        locales: "./src/fluent/locales",
        fallback_language: "en-US",
        // A fluent resource that is shared with every locale.
        core_locales: "./src/fluent/locales/core.ftl",
    };
}

pub const US_ENGLISH: LanguageIdentifier = langid!("en-US");
pub const GERMAN: LanguageIdentifier = langid!("de");

pub const FLUENT_INDEX_SECTION: &str = "index";
pub const FLUENT_LONG_SECTION: &str = "long";
pub const FLUENT_SYMBOL_SECTION: &str = "symbol";
pub const FLUENT_WEIGHT_SECTION: &str = "weight";
pub const FLUENT_PRIME_SECTION: &str = "prime";

pub trait Named {
    fn fluent_name(&self) -> &FluentName;
    fn fluent_name_string(&self) -> String;

    /// This is the core method for getting fluent values. the index, long, and default weight
    /// methods are all just methods simplifying the call to this method.
    fn fluent_value(&self, key_section: &str, lid: &LanguageIdentifier) -> String {
        let id = format!("{}-{}", self.fluent_name_string(), key_section);
        LOCALES.lookup(lid, id.as_str())
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FluentName(String);

impl FluentName {
    pub const BLANK: &str = "blank";

    ///  The difference between `new` and `from_str` is that `new` will default to
    /// `BLANK` if the passed in `&str` isn't  valid, whereas with `from_str` it
    ///  will return a `CardError`.
    ///
    /// One of the things that you want to consider when coding stuff like this
    /// is the user going "WHAT THE FUCK???" when things don't work as expected.
    ///
    /// Having a default value when passing shit in may be convenient at the moment,
    /// but can be a major pain in the ass when you're trying to debug things at the
    /// heat of the moment.
    ///
    /// Empathy of the users of your code is one of the traits that I have encountered
    /// in the wild as a software developer. **Remember, nine times out of ten the
    /// developer you will be cursing over they lack of empathy when their coded
    /// something will be you.**
    ///
    /// **NOTE:** there is no perfect way to do this. Empathy is an art form.
    #[must_use]
    pub fn new(name_str: &str) -> Self {
        match Self::is_alphanumeric_hyphen_dash(name_str) {
            true => FluentName(name_str.to_string()),
            false => {
                log::warn!("Invalid name: {} - Defaulting to 'blank'.", name_str);
                FluentName(Self::BLANK.to_string())
            }
        }
    }

    fn is_alphanumeric_hyphen_dash(s: &str) -> bool {
        s.chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '–' || c == '—')
    }
}

impl FromStr for FluentName {
    type Err = CardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::is_alphanumeric_hyphen_dash(s) {
            true => Ok(FluentName(s.to_string())),
            false => Err(CardError::InvalidFluentName(s.to_string())),
        }
    }
}

impl Named for FluentName {
    fn fluent_name(&self) -> &FluentName {
        self
    }

    fn fluent_name_string(&self) -> String {
        self.0.clone()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod fluent_tests {
    use super::*;

    #[test]
    fn new() {
        assert_eq!("abide", FluentName::new("abide").fluent_name_string());
        assert_eq!(
            FluentName::BLANK,
            FluentName::new("+++").fluent_name_string()
        );
    }

    #[test]
    fn is_alphanumeric_hyphen_dash() {
        assert!(FluentName::is_alphanumeric_hyphen_dash("Hello-World"));
        assert!(FluentName::is_alphanumeric_hyphen_dash("HelloWorld"));
        assert!(!FluentName::is_alphanumeric_hyphen_dash("🁚"));
        assert!(!FluentName::is_alphanumeric_hyphen_dash("  "));
    }

    #[test]
    fn from_str() {
        assert_eq!(
            "abide",
            FluentName::from_str("abide").unwrap().fluent_name_string()
        );
    }

    #[test]
    fn from_str__error() {
        let sut = FluentName::from_str("I'm a bad bad fluent string name.");

        let my_err = sut.unwrap_err();

        assert_eq!(
            CardError::InvalidFluentName("I'm a bad bad fluent string name.".to_string()),
            my_err
        );
        assert_eq!(
            "Invalid FluentName: `I'm a bad bad fluent string name.`. Must be alphanumeric with hyphens, en-dashes, or em-dashes.",
            my_err.to_string()
        );
    }
}
