use std::str::FromStr;
use fluent_templates::{langid, static_loader, LanguageIdentifier};

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

pub struct FluentName(String);

impl FluentName {

}

impl FromStr for FluentName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(FluentName(s.to_string()))
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod fluent_tests {
    use super::*;
}