use crate::localization::{FluentName, Named};
use crate::old::decks::french::French;
use crate::old::decks::modern::Modern;
use crate::old::decks::skat::Skat;
use crate::old::decks::tarot::Tarot;
use crate::types::traits::Suited;
use crate::types::utils::Bit;
use colored::Color;
use std::collections::HashMap;
use std::fmt::Display;
use std::marker::PhantomData;

/// A macro to simplify the creation of a `Suit`.
///
/// ```
/// use cardpack::prelude::*;
///
/// assert_eq!(suit!(S), Suit::<French>::new(French::SPADES));
/// assert_eq!(suit!(H), Suit::<French>::new(French::HEARTS));
/// assert_eq!(suit!(D), Suit::<French>::new(French::DIAMONDS));
/// assert_eq!(suit!(C), Suit::<French>::new(French::CLUBS));
/// assert_eq!(suit!(J), Suit::<Modern>::new(Modern::JOKER));
/// assert_eq!(suit!(M), Suit::<Tarot>::new(Tarot::MAJOR_ARCANA));
/// assert_eq!(suit!(WANDS), Suit::<Tarot>::new(Tarot::WANDS));
/// assert_eq!(suit!(CUPS), Suit::<Tarot>::new(Tarot::CUPS));
/// assert_eq!(suit!(SWORDS), Suit::<Tarot>::new(Tarot::SWORDS));
/// assert_eq!(suit!(PENTACLES), Suit::<Tarot>::new(Tarot::PENTACLES));
/// assert_eq!(suit!(EICHEL), Suit::<Skat>::new(Skat::EICHEL));
/// assert_eq!(suit!(LAUB), Suit::<Skat>::new(Skat::LAUB));
/// assert_eq!(suit!(HERZ), Suit::<Skat>::new(Skat::HERZ));
/// assert_eq!(suit!(SHELLEN), Suit::<Skat>::new(Skat::SHELLEN));
/// assert_eq!(suit!(_), Suit::<French>::new(FluentName::BLANK));
/// ```
#[macro_export]
macro_rules! suit {
    (S) => {
        Suit::<French>::new(French::SPADES)
    };
    (H) => {
        Suit::<French>::new(French::HEARTS)
    };
    (D) => {
        Suit::<French>::new(French::DIAMONDS)
    };
    (C) => {
        Suit::<French>::new(French::CLUBS)
    };
    (J) => {
        Suit::<Modern>::new(Modern::JOKER)
    };
    (M) => {
        Suit::<Tarot>::new(Tarot::MAJOR_ARCANA)
    };
    (WANDS) => {
        Suit::<Tarot>::new(Tarot::WANDS)
    };
    (CUPS) => {
        Suit::<Tarot>::new(Tarot::CUPS)
    };
    (SWORDS) => {
        Suit::<Tarot>::new(Tarot::SWORDS)
    };
    (PENTACLES) => {
        Suit::<Tarot>::new(Tarot::PENTACLES)
    };
    (EICHEL) => {
        Suit::<Skat>::new(Skat::EICHEL)
    };
    (LAUB) => {
        Suit::<Skat>::new(Skat::LAUB)
    };
    (HERZ) => {
        Suit::<Skat>::new(Skat::HERZ)
    };
    (SHELLEN) => {
        Suit::<Skat>::new(Skat::SHELLEN)
    };
    (_) => {
        Suit::<French>::new(FluentName::BLANK)
    };
}

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
    /// TODO: need a way to add the jokers suit. Right now this assumes standard 52
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
    pub fn index(&self) -> String {
        self.name.fluent_value(
            Suit::<SuitType>::FLUENT_INDEX_SECTION,
            &Suit::<SuitType>::US_ENGLISH,
        )
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

/// # NOTE
///
/// This trait seems to be made redundant with the 2nd refactoring where it
/// is simply folded into the `Suited` trait.
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
                'S' | 's' | '♤' | '♠' => Suit::<SuitType>::new(French::SPADES),
                'H' | 'h' | '♡' | '♥' => Suit::<SuitType>::new(French::HEARTS),
                'D' | 'd' | '♢' | '♦' => Suit::<SuitType>::new(French::DIAMONDS),
                'C' | 'c' | '♧' | '♣' => Suit::<SuitType>::new(French::CLUBS),
                '🃟' | 'J' | 'j' | 'T' | 't' => Suit::new(Modern::JOKER),
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
        let suit = Suit::<French>::from('♠');

        assert_eq!(suit.symbol(), "♠");
    }

    #[test]
    fn ckc_number() {
        assert_eq!(
            0b0000_0000_0000_0000_1000_0000_0000_0000,
            Suit::<French>::from('♠').ckc_number()
        );
        assert_eq!(
            0b0000_0000_0000_0000_1000_0000_0000_0000,
            Suit::<French>::from('S').ckc_number()
        );
        assert_eq!(
            0b0000_0000_0000_0000_0100_0000_0000_0000,
            Suit::<French>::from('H').ckc_number()
        );
        assert_eq!(
            0b0000_0000_0000_0000_0010_0000_0000_0000,
            Suit::<French>::from('D').ckc_number()
        );
        assert_eq!(
            0b0000_0000_0000_0000_0001_0000_0000_0000,
            Suit::<French>::from('C').ckc_number()
        );
        assert_eq!(0, Suit::<French>::from('_').ckc_number());
    }

    #[test]
    fn symbol() {
        let suit = Suit::<French>::new(French::SPADES);

        assert_eq!(suit.symbol(), "♠");
        assert_eq!(suit.to_string(), suit.symbol())
    }

    #[test]
    fn symbol_blank() {
        let suit = Suit::<French>::from('_');

        assert_eq!(suit.symbol(), "_");
        assert_eq!(suit.to_string(), suit.symbol())
    }

    #[rstest]
    #[case('♠', French::SPADES)]
    #[case('♤', French::SPADES)]
    #[case('S', French::SPADES)]
    #[case('s', French::SPADES)]
    #[case('♥', French::HEARTS)]
    #[case('♡', French::HEARTS)]
    #[case('H', French::HEARTS)]
    #[case('h', French::HEARTS)]
    #[case('♦', French::DIAMONDS)]
    #[case('♢', French::DIAMONDS)]
    #[case('D', French::DIAMONDS)]
    #[case('d', French::DIAMONDS)]
    #[case('♣', French::CLUBS)]
    #[case('♧', French::CLUBS)]
    #[case('C', French::CLUBS)]
    #[case('c', French::CLUBS)]
    #[case('🃟', FluentName::BLANK)]
    #[case('T', FluentName::BLANK)]
    #[case('t', FluentName::BLANK)]
    #[case(' ', FluentName::BLANK)]
    #[case('F', FluentName::BLANK)]
    fn from__char(#[case] input: char, #[case] expected: &str) {
        assert_eq!(Suit::<French>::new(expected), Suit::<French>::from(input));
    }

    #[test]
    fn named__weighted_vector() {
        let mut v = Suit::<French>::suit_names();
        v.reverse();

        let suits = Suit::<French>::weighted_vector(&v);

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
