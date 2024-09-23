#![warn(clippy::pedantic)]
// #![allow(clippy::module_name_repetitions)]

use crate::fluent::{FluentName, Named};
use crate::traits::Ranked;
use std::marker::PhantomData;
use std::str::FromStr;
use crate::card_error::CardError;

pub mod card;
pub mod card_error;
pub mod decks;
pub mod fluent;
pub mod traits;

pub struct Rank<RankType>
where
    RankType: Ranked,
{
    pub weight: u32,
    pub prime: u32,
    pub  name: FluentName,
    phantom_data: PhantomData<RankType>,
}

impl<RankType> Rank<RankType>
where
    RankType: Ranked,
{
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
            phantom_data: PhantomData,
        }
    }

    fn new_with_weight(name_str: &str, weight: u32) -> Rank<RankType> {
        let name = FluentName::new(name_str);

        Rank::<RankType> {
            weight,
            prime: name.prime(),
            name,
            phantom_data: PhantomData,
        }
    }

    #[must_use]
    fn ranks(&self) -> Vec<Self> {
        RankType::names()
            .iter()
            .map(|name| Self::new(name))
            .collect()
    }
}

impl<RankType> Named<'_> for Rank<RankType>
where
    RankType: Ranked,
{
    fn fluent_name(&self) -> &FluentName {
       &self.name
    }

    fn fluent_name_string(&self) -> &String {
        self.name.fluent_name_string()
    }

    fn is_blank(&self) -> bool {
        self.name.is_blank()
    }
}

impl<RankType: Ranked> Ranked for Rank<RankType> {
    fn chars() -> Vec<char> {
        RankType::chars()
    }

    fn names() -> Vec<&'static str> {
        RankType::names()
    }
}

impl<RankType: Ranked> From<char> for Rank<RankType> {
    fn from(value: char) -> Self {
        if !RankType::is_valid_char(&value) {
            return Rank::<RankType> {
                weight: 0,
                prime: 0,
                name: FluentName::default(),
                phantom_data: PhantomData,
            };
        }
        match value {
            '2' => Rank::new(Rank::<RankType>::TWO),
            '3' => Rank::new(Rank::<RankType>::THREE),
            '4' => Rank::new(Rank::<RankType>::FOUR),
            '5' => Rank::new(Rank::<RankType>::FIVE),
            '6' => Rank::new(Rank::<RankType>::SIX),
            '7' => Rank::new(Rank::<RankType>::SEVEN),
            '8' => Rank::new(Rank::<RankType>::EIGHT),
            '9' => Rank::new(Rank::<RankType>::NINE),
            'T' | 't' | '0' => Rank::new(Rank::<RankType>::TEN),
            'J' | 'j' => Rank::new(Rank::<RankType>::JACK),
            'Q' | 'q' => Rank::new(Rank::<RankType>::QUEEN),
            'K' | 'k' => Rank::new(Rank::<RankType>::KING),
            'A' | 'a' => Rank::new(Rank::<RankType>::ACE),
            'B' | 'b' => Rank::new(Rank::<RankType>::BIG),
            'L' | 'l' => Rank::new(Rank::<RankType>::LITTLE),
            _ => Rank::new(FluentName::BLANK),
        }
    }
}

impl<RankType: Ranked> FromStr for Rank<RankType> {
    type Err = CardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(first_char) = s.chars().next() {
            let rank = Rank::<RankType>::from(first_char);
            if rank.name.is_blank() {
                Err(CardError::InvalidFluentRank(s.to_string()))
            } else {
                Ok(rank)
            }
        } else {
            Err(CardError::InvalidFluentRank(s.to_string()))
        }
    }
}