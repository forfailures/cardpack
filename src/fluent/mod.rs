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

/// Trait used to enable localized names for card entities such as suits and ranks.
///
/// The types of `Named` attributes are
///
/// * `index` - the default letter representation of a card identifier, such as `A` for Ace, or `S` for Spades.
/// * `long` - the long name of a card identifier, such as `Ace` or `Spades`.
/// * `symbol` - the symbol representation of a card identifier, such as `♠` for Spades.
/// * `weight` - the default weight of a card identifier. Used for sorting cards.
/// * `prime` - the prime number representation of a card identifier. Used for generating binary signatures.
///
/// **REREADME:** <https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html/>
pub trait Named<'a> {
    const US_ENGLISH: LanguageIdentifier = langid!("en-US");
    const DEUTSCH: LanguageIdentifier = langid!("de");

    const FLUENT_INDEX_SECTION: &'a str = "index";
    const FLUENT_LONG_SECTION: &'a str = "long";
    const FLUENT_SYMBOL_SECTION: &'a str = "symbol";
    const FLUENT_WEIGHT_SECTION: &'a str = "weight";
    const FLUENT_PRIME_SECTION: &'a str = "prime";

    fn fluent_name(&self) -> &FluentName;
    fn fluent_name_string(&self) -> &String;
    fn is_blank(&self) -> bool;

    /// This is the core method for getting fluent values. the index, long, and default weight
    /// methods are all just methods simplifying the call to this method.
    ///
    /// ## Usage
    /// ```
    /// use cardpack::fluent::*;
    ///
    /// assert_eq!(
    ///   "♠",
    ///   FluentName::new("spades").fluent_value("symbol", &FluentName::US_ENGLISH)
    /// );
    /// ```
    fn fluent_value(&self, key_section: &str, lid: &LanguageIdentifier) -> String {
        let id = format!("{}-{}", self.fluent_name_string(), key_section);
        LOCALES.lookup(lid, id.as_str())
    }

    /// Returns the value of the `FluentName` index in the fluent templates. An index
    /// is defined as the default letter representation of a card identifier, such as
    /// `A` for Ace, or `S` for Spades.
    ///
    /// The index is defined as the identity indicator in the corner of a playing card.
    ///
    /// ## Usage
    /// ```
    /// use cardpack::fluent::*;
    ///
    /// let jack = FluentName::new("jack");
    /// assert_eq!("B", jack.index(&FluentName::DEUTSCH));
    /// ```
    fn index(&self, lid: &LanguageIdentifier) -> String {
        self.fluent_value(Self::FLUENT_INDEX_SECTION, lid)
    }

    /// Returns the default, `US_ENGLISH` index value in the fluent templates.
    ///
    /// ## Usage
    /// ```
    /// use cardpack::fluent::*;
    ///
    /// let ten = FluentName::new("ten");
    /// assert_eq!("T", ten.index_default());
    /// ```
    fn index_default(&self) -> String {
        self.index(&Self::US_ENGLISH)
    }

    /// Returns the value of the `Named` long value in the fluent templates.
    ///
    /// ## Usage
    /// ```
    /// use cardpack::fluent::*;
    ///
    /// let big_joker = FluentName::new("big-joker");
    /// assert_eq!("Großer Joker", big_joker.long(&FluentName::DEUTSCH));
    /// ```
    fn long(&self, lid: &LanguageIdentifier) -> String {
        self.fluent_value(Self::FLUENT_LONG_SECTION, lid)
    }

    /// Returns the default, `US_ENGLISH` value of the `Named` long value in the fluent templates.
    ///
    /// ## Usage
    /// ```
    /// use cardpack::fluent::*;
    ///
    /// let big_joker = FluentName::new("big-joker");
    /// assert_eq!("Full-Color Joker", big_joker.long_default());
    /// ```
    fn long_default(&self) -> String {
        self.long(&Self::US_ENGLISH)
    }

    /// Returns the weight for `Named`, used to sort cards. There is no need for an alternative
    /// `LanguageIdentifier` to `US_ENGLISH`. Weights are stored in the `core.ftl` file.
    ///
    /// # Usage
    /// ```
    /// use cardpack::fluent::*;
    ///
    /// let queen = FluentName::new("queen");
    /// assert_eq!(10, queen.weight());
    /// ```
    fn weight(&self) -> u32 {
        let weight = self.fluent_value(Self::FLUENT_WEIGHT_SECTION, &Self::US_ENGLISH);
        weight.parse().unwrap_or(0)
    }

    /// Returns the prime number for `Named`, used to generate binary signatures. There is no need
    /// for an alternative `LanguageIdentifier` to `US_ENGLISH`. Primes are stored in the `core.ftl`
    /// file.
    ///
    /// **ASIDE:** I'm not sure I like storing these as `FluentName`s.
    ///
    /// # Usage
    /// ```
    /// use cardpack::fluent::*;
    ///
    /// let queen = FluentName::new("queen");
    /// assert_eq!(31, queen.prime());
    /// ```
    fn prime(&self) -> u32 {
        let prime = self.fluent_value(Self::FLUENT_PRIME_SECTION, &Self::US_ENGLISH);
        prime.parse().unwrap_or(0)
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
    ///
    /// ## Usage
    /// ```
    /// use cardpack::fluent::*;
    ///
    /// assert_eq!("spades", FluentName::new("spades").fluent_name_string());
    ///
    /// // Defaults to `BLANK` when an invalid name is passed in.
    /// assert_eq!(
    ///   FluentName::BLANK,
    ///   FluentName::new("+++").fluent_name_string()
    /// );
    /// ```
    #[must_use]
    pub fn new(name_str: &str) -> Self {
        if Self::is_alphanumeric_hyphen_dash(name_str) {
            FluentName(name_str.to_string())
        } else {
            log::warn!("Invalid name: {} - Defaulting to 'blank'.", name_str);
            FluentName(Self::BLANK.to_string())
        }
    }

    fn is_alphanumeric_hyphen_dash(s: &str) -> bool {
        s.chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '–' || c == '—')
    }
}

/// USAGE:
/// ```
/// use std::str::FromStr;
/// use cardpack::fluent::*;
///
/// assert_eq!(
///   "hierophant",
///    FluentName::from_str("hierophant").unwrap().fluent_name_string()
/// );
/// ```
///
/// Unlike `Fluent::new()`, `Fluent::from_str()` will return a `CardError` if the
/// passed in value is invalid.
///
/// ```
/// use std::str::FromStr;
/// use cardpack::card_error::CardError;
/// use cardpack::fluent::*;
///
/// let sut = FluentName::from_str("Only alphanumeric and hyphens please.");
///
/// assert_eq!(
///   CardError::InvalidFluentName("Only alphanumeric and hyphens please.".to_string()),
///   sut.unwrap_err()
/// );
/// ```
impl FromStr for FluentName {
    type Err = CardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_alphanumeric_hyphen_dash(s) {
            Ok(FluentName(s.to_string()))
        } else {
            Err(CardError::InvalidFluentName(s.to_string()))
        }
    }
}

impl<'a> Named<'a> for FluentName {
    fn fluent_name(&self) -> &FluentName {
        self
    }

    fn fluent_name_string(&self) -> &String {
        &self.0
    }

    fn is_blank(&self) -> bool {
        self.fluent_name_string() == Self::BLANK
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod fluent_tests {
    use super::*;

    #[test]
    fn new() {
        assert_eq!(FluentName("queen".to_string()), FluentName::new("queen"));
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
            "hierophant",
            FluentName::from_str("hierophant")
                .unwrap()
                .fluent_name_string()
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

    #[test]
    fn named__fluent_value() {
        assert_eq!(
            "_",
            FluentName::new("+++").fluent_value("symbol", &FluentName::US_ENGLISH)
        );
        assert_eq!(
            "Daus",
            FluentName::new("daus").fluent_value("long", &FluentName::DEUTSCH)
        );
    }

    #[test]
    fn named__is_blank() {
        assert!(FluentName::new("blank").is_blank());
        assert!(!FluentName::new("long").is_blank());
    }

    #[test]
    fn named__index() {
        assert_eq!(
            "S",
            FluentName::new("spades").index(&FluentName::US_ENGLISH)
        );
        assert_eq!(
            "P",
            FluentName::new("pentacles").index(&FluentName::US_ENGLISH)
        );
        assert_eq!("K", FluentName::new("clubs").index(&FluentName::DEUTSCH));
    }

    #[test]
    fn named__index_default() {
        assert_eq!("S", FluentName::new("spades").index_default());
        assert_eq!("P", FluentName::new("pentacles").index_default());
    }
}