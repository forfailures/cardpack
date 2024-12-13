use crate::decks::modern::Modern;
use crate::decks::standard52::Standard52;
use crate::localization::{FluentName, Named};
use crate::types::traits::Suited;
use colored::Color;
use std::collections::HashMap;
use std::fmt::Display;
use std::marker::PhantomData;

/// TODO: Create a five suited deck to test the boundaries.
/// <https://cards.fandom.com/wiki/Suit_(cards)#Five_Suit_Decks/>
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Suit<SuitType>
where
    SuitType: Suited,
{
    pub weight: u32,
    pub name: FluentName,
    phantom_data: PhantomData<SuitType>,
}

impl<SuitType> Suit<SuitType>
where
    SuitType: Suited,
{
    #[must_use]
    pub fn new(name_str: &str) -> Suit<SuitType> {
        let name = FluentName::new(name_str);

        Suit::<SuitType> {
            weight: name.weight(),
            name,
            phantom_data: PhantomData,
        }
    }

    /// Used to generate the `Card`'s binary signature.
    ///
    /// The value that is used to generate [Cactus Kev](https://suffe.cool/poker/evaluator.html)
    /// numbers.
    #[must_use]
    pub fn binary_signature(&self) -> u32 {
        match self.weight {
            4 => 0x1000,
            3 => 0x2000,
            2 => 0x4000,
            1 => 0x8000,
            _ => 0xF000,
        }
    }

    /// Revised version of the `binary_signature()` method that inverts the weight for sorting
    /// Spades first. Has no effect on the generated card ranks, but does make sorting easier.
    #[must_use]
    pub fn binary_signature_revised(&self) -> u32 {
        match self.weight {
            1 => 0x1000,
            2 => 0x2000,
            3 => 0x4000,
            4 => 0x8000,
            _ => 0xF000,
        }
    }

    #[must_use]
    pub fn suits() -> Vec<Self> {
        SuitType::suit_names()
            .iter()
            .map(|name| Self::new(name))
            .collect()
    }

    #[must_use]
    pub fn symbol(&self) -> String {
        self.name.fluent_value(
            Suit::<SuitType>::FLUENT_SYMBOL_SECTION,
            &Suit::<SuitType>::US_ENGLISH,
        )
    }
}

impl<SuitType> Display for Suit<SuitType>
where
    SuitType: Suited,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl<SuitType> Named<'_> for Suit<SuitType>
where
    SuitType: Suited,
{
    fn new_with_weight(name_str: &str, weight: u32) -> Suit<SuitType> {
        let name = FluentName::new(name_str);

        Suit::<SuitType> {
            weight,
            name,
            phantom_data: PhantomData,
        }
    }

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

impl<SuiteType: Suited> Suited for Suit<SuiteType> {
    fn suit_chars() -> Vec<char> {
        SuiteType::suit_chars()
    }

    fn suit_names() -> Vec<&'static str> {
        SuiteType::suit_names()
    }

    fn colors() -> HashMap<char, Color> {
        todo!()
    }
}

impl<SuitType: Suited> From<char> for Suit<SuitType> {
    fn from(c: char) -> Self {
        if !SuitType::is_valid_suit_char(&c) {
            return Suit::<SuitType> {
                weight: 0,
                name: FluentName::default(),
                phantom_data: PhantomData,
            };
        }
        match c {
            'S' | 's' | '♤' | '♠' => Suit::<SuitType>::new(Standard52::SPADES),
            'H' | 'h' | '♡' | '♥' => Suit::<SuitType>::new(Standard52::HEARTS),
            'D' | 'd' | '♢' | '♦' => Suit::<SuitType>::new(Standard52::DIAMONDS),
            'C' | 'c' | '♧' | '♣' => Suit::<SuitType>::new(Standard52::CLUBS),
            '🃟' | 'T' | 't' => Suit::new(Modern::TRUMP),
            _ => Suit::new(FluentName::BLANK),
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod types__suit__tests {
    use super::*;

    #[test]
    fn from_str__symbol() {
        let suit = Suit::<Standard52>::from('♠');

        assert_eq!(suit.symbol(), "♠");
    }
}
