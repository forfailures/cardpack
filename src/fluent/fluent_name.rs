use std::str::FromStr;
use crate::card_error::CardError;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FluentName(String);

impl FluentName {}

impl FromStr for FluentName {
    type Err = CardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(FluentName(s.to_string()))
    }
}

impl crate::fluent::named::Named for FluentName {
    fn fluent_name_string(&self) -> String {
        self.0.clone()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod fluent_tests {
    use super::*;
}
