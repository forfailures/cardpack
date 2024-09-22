#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use std::marker::PhantomData;
use crate::fluent::{FluentName, Named};
use crate::traits::Ranked;

pub mod card;
pub mod card_error;
pub mod fluent;
pub mod traits;
pub mod decks;

pub struct Rank<RankType> where RankType: Ranked {
    weight: u32,
    prime: u32,
    name: FluentName,
    phantom_data: PhantomData<RankType>
}

impl<RankType: Ranked> Ranked for Rank<RankType> {
    fn chars() -> Vec<char> {
        RankType::chars()
    }

    fn names() -> Vec<&'static str> {
        RankType::names()
    }
}

impl<RankType> Rank<RankType> where RankType: Ranked {
    pub const ACE: &str = "ace";
    pub const KING: &str = "king";
    pub const QUEEN: &str = "queen";
    pub const JACK: &str = "jack";
    pub const TEN: &str = "ten";
    pub const NINE: &str = "nine";
    pub const EIGHT: &str = "eight";
    pub const SEVEN: &str = "seven";
    pub const SIX: &str = "six";
    pub const FIVE: &str = "five";
    pub const FOUR: &str = "four";
    pub const THREE: &str = "three";
    pub const TWO: &str = "two";

    // Jokers
    pub const BIG: &str = "big";
    pub const LITTLE: &str = "little";

    fn new(name_str: &str) -> Rank<RankType> {
        let name = FluentName::new(name_str);

        Rank::<RankType> {
            weight: name.weight(),
            prime: name.prime(),
            name,
            phantom_data: PhantomData
        }
    }

    #[must_use]
    fn ranks(&self) -> Vec<Self> {
        RankType::names().iter().map(|name| Self::new(name)).collect()
    }
}