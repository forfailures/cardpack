use crate::decks::modern::Modern;
use crate::decks::skat::Skat;
use crate::decks::standard52::Standard52;
use crate::decks::tarot::Tarot;
use crate::localization::{FluentName, Named};
use crate::types::traits::Suited;
use crate::types::utils::Bit;
use colored::Color;
use std::collections::HashMap;
use std::fmt::Display;
use std::marker::PhantomData;

/// TODO: Create a five suited deck to test the boundaries.
/// <https://cards.fandom.com/wiki/Suit_(cards)#Five_Suit_Decks/>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

    /// Used to generate the `Card`'s binary signature, aka [Cactus Kev](https://suffe.cool/poker/evaluator.html)
    /// numbers.
    ///
    /// Revised version that inverts the weight for sorting, making Spades be the highest. Has no
    /// effect on the generated card ranks, but does make sorting easier.
    ///
    /// TODO: need a way to add trumps suit. Right now this assumes standard 52
    #[must_use]
    pub fn ckc_number(&self) -> u32 {
        match self.weight {
            0 => 0,
            _ => 1 << (Bit::SUIT_FLAG_SHIFT + self.weight),
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

impl<SuitType> Default for Suit<SuitType>
where
    SuitType: Suited,
{
    fn default() -> Self {
        Suit::<SuitType> {
            weight: 0,
            name: FluentName::default(),
            phantom_data: PhantomData,
        }
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
    fn colors() -> HashMap<char, Color> {
        SuiteType::colors()
    }

    fn suit_chars() -> Vec<char> {
        SuiteType::suit_chars()
    }

    fn suit_names() -> Vec<&'static str> {
        SuiteType::suit_names()
    }

    fn type_name() -> &'static str {
        SuiteType::type_name()
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
        match SuitType::type_name() {
            Skat::DECK_NAME => match c {
                '♧' | '♣' | 'E' | 'e' => Suit::<SuitType>::new(Skat::EICHEL),
                '♤' | '♠' | 'L' | 'l' => Suit::<SuitType>::new(Skat::LAUB),
                '♡' | '♥' | 'H' | 'h' => Suit::<SuitType>::new(Skat::HERZ),
                '♢' | '♦' | 'S' | 's' => Suit::<SuitType>::new(Skat::SHELLEN),
                _ => Suit::<SuitType>::new(FluentName::BLANK),
            },
            _ => match c {
                'S' | 's' | '♤' | '♠' => Suit::<SuitType>::new(Standard52::SPADES),
                'H' | 'h' | '♡' | '♥' => Suit::<SuitType>::new(Standard52::HEARTS),
                'D' | 'd' | '♢' | '♦' => Suit::<SuitType>::new(Standard52::DIAMONDS),
                'C' | 'c' | '♧' | '♣' => Suit::<SuitType>::new(Standard52::CLUBS),
                '🃟' | 'T' | 't' => Suit::new(Modern::TRUMP),
                'M' | 'm' => Suit::new(Tarot::MAJOR_ARCANA),
                _ => Suit::new(FluentName::BLANK),
            },
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod types__suit__tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn from_str__symbol() {
        let suit = Suit::<Standard52>::from('♠');

        assert_eq!(suit.symbol(), "♠");
    }

    #[test]
    fn ckc_number() {
        assert_eq!(
            0b0000_0000_0000_0000_1000_0000_0000_0000,
            Suit::<Standard52>::from('♠').ckc_number()
        );
        assert_eq!(
            0b0000_0000_0000_0000_1000_0000_0000_0000,
            Suit::<Standard52>::from('S').ckc_number()
        );
        assert_eq!(
            0b0000_0000_0000_0000_0100_0000_0000_0000,
            Suit::<Standard52>::from('H').ckc_number()
        );
        assert_eq!(
            0b0000_0000_0000_0000_0010_0000_0000_0000,
            Suit::<Standard52>::from('D').ckc_number()
        );
        assert_eq!(
            0b0000_0000_0000_0000_0001_0000_0000_0000,
            Suit::<Standard52>::from('C').ckc_number()
        );
        assert_eq!(0, Suit::<Standard52>::from('_').ckc_number());
    }

    #[test]
    fn symbol() {
        let suit = Suit::<Standard52>::new(Standard52::SPADES);

        assert_eq!(suit.symbol(), "♠");
        assert_eq!(suit.to_string(), suit.symbol())
    }

    #[test]
    fn symbol_blank() {
        let suit = Suit::<Standard52>::from('_');

        assert_eq!(suit.symbol(), "_");
        assert_eq!(suit.to_string(), suit.symbol())
    }

    #[rstest]
    #[case('♠', Standard52::SPADES)]
    #[case('♤', Standard52::SPADES)]
    #[case('S', Standard52::SPADES)]
    #[case('s', Standard52::SPADES)]
    #[case('♥', Standard52::HEARTS)]
    #[case('♡', Standard52::HEARTS)]
    #[case('H', Standard52::HEARTS)]
    #[case('h', Standard52::HEARTS)]
    #[case('♦', Standard52::DIAMONDS)]
    #[case('♢', Standard52::DIAMONDS)]
    #[case('D', Standard52::DIAMONDS)]
    #[case('d', Standard52::DIAMONDS)]
    #[case('♣', Standard52::CLUBS)]
    #[case('♧', Standard52::CLUBS)]
    #[case('C', Standard52::CLUBS)]
    #[case('c', Standard52::CLUBS)]
    #[case('🃟', FluentName::BLANK)]
    #[case('T', FluentName::BLANK)]
    #[case('t', FluentName::BLANK)]
    #[case(' ', FluentName::BLANK)]
    #[case('F', FluentName::BLANK)]
    fn from__char(#[case] input: char, #[case] expected: &str) {
        assert_eq!(
            Suit::<Standard52>::new(expected),
            Suit::<Standard52>::from(input)
        );
    }

    #[test]
    fn named__weighted_vector() {
        let mut v = Suit::<Standard52>::suit_names();
        v.reverse();

        let suits = Suit::<Standard52>::weighted_vector(&v);

        assert_eq!(suits.len(), 4);
        assert_eq!(suits[0].fluent_name_string(), "clubs");
        assert_eq!(suits[0].weight, 3);
        assert_eq!(suits[1].fluent_name_string(), "diamonds");
        assert_eq!(suits[1].weight, 2);
        assert_eq!(suits[2].fluent_name_string(), "hearts");
        assert_eq!(suits[2].weight, 1);
        assert_eq!(suits[3].fluent_name_string(), "spades");
        assert_eq!(suits[3].weight, 0);
    }
}
