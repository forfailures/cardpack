mod standard52_rank;

use crate::fluent::Named;
use std::str::FromStr;

pub trait Rank: From<char> + FromStr + for<'a> Named<'a> {
    fn new(name_str: &str) -> Self;

    #[must_use]
    fn new_with_weight(name_str: &str, weight: u32) -> Self {
        Self::new(name_str).update_weight(weight)
    }

    fn names() -> Vec<&'static str>;

    fn get_prime(&self) -> u32;

    fn get_weight(&self) -> u32;

    #[must_use]
    fn update_weight(&self, weight: u32) -> Self;
}
