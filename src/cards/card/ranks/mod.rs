mod standard52_rank;
mod euchre_rank;

use crate::fluent::Named;
use std::str::FromStr;

pub trait Rank: From<char> + FromStr + for<'a> Named<'a> {
    fn new(name_str: &str) -> Self;

    #[must_use]
    fn new_with_weight(name_str: &str, weight: u32) -> Self;

    fn names() -> Vec<&'static str>;

    #[must_use]
    fn ranks() -> Vec<Self> {
        Self::names().iter().map(|name| Self::new(name)).collect()
    }

    #[must_use]
    fn weighted_vector(names: Vec<&'static str>) -> Vec<Self> {
        let mut weight = 0;
        names
            .iter()
            .map(|name| {
                let rank = Self::new_with_weight(name, weight);
                weight += 1;
                rank
            })
            .collect()
    }

    fn get_prime(&self) -> u32;

    fn get_weight(&self) -> u32;

    #[must_use]
    fn update_weight(&self, weight: u32) -> Self {
        Self::new_with_weight(self.fluent_name_string().as_str(), weight)
    }
}
