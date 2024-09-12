mod standard52_rank;

use crate::fluent::fluent_name::FluentName;
use std::str::FromStr;

pub trait Rank: From<char> + FromStr {
    fn get_weight(&self) -> u32;
    fn get_fluent_name(&self) -> FluentName;
}
