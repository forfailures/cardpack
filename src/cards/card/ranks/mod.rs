mod standard52_rank;

use crate::fluent::Named;
use std::str::FromStr;

pub trait Rank: From<char> + FromStr + Named {
    fn get_prime(&self) -> u32;
    fn get_weight(&self) -> u32;
}
