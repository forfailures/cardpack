pub mod decks;
pub mod traits;

use crate::localization::FluentName;
use crate::types::utils::Bit;

use crate::refact::traits::{Ranked, Suited};
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::marker::PhantomData;

pub const BLANK: char = '_';

#[derive(Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Pile<RankType, SuitType>(Vec<Card<RankType, SuitType>>)
where
    RankType: Ranked + Clone + Copy + PartialOrd + Ord + Default + Hash,
    SuitType: Suited + Clone + Copy + PartialOrd + Ord + Default + Hash;

impl<
        RankType: Ranked + Clone + Copy + PartialOrd + Ord + Default + Hash,
        SuitType: Suited + Clone + Copy + PartialOrd + Ord + Default + Hash,
    > Pile<RankType, SuitType>
{
    #[must_use]
    pub fn cards(&self) -> Vec<Card<RankType, SuitType>> {
        self.0.clone()
    }

    #[must_use]
    pub fn iter(&self) -> std::vec::IntoIter<Card<RankType, SuitType>> {
        <&Self as IntoIterator>::into_iter(self)
    }

    pub fn push(&mut self, card: Card<RankType, SuitType>) -> bool {
        if card.is_blank() {
            false
        } else {
            self.0.push(card);
            true
        }
    }

    #[must_use]
    pub fn sort(&self) -> Self {
        let mut cards: Vec<Card<RankType, SuitType>> = self.0.clone();
        cards.sort();
        cards.reverse();
        Self(cards)
    }
}

/// ```
/// use cardpack::refactored::*;
///
/// let pile: Pile<French, French> = Pile::<French, French>::default();
/// ```
impl<
        RankType: Ranked + Clone + Copy + PartialOrd + Ord + Default + Hash,
        SuitType: Suited + Clone + Copy + PartialOrd + Ord + Default + Hash,
    > Display for Pile<RankType, SuitType>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self
            .0
            .iter()
            .map(Card::to_string)
            .collect::<Vec<String>>()
            .join(" ");
        write!(f, "{s}")
    }
}

impl<
        RankType: Ranked + Clone + Copy + PartialOrd + Ord + Default + Hash,
        SuitType: Suited + Clone + Copy + PartialOrd + Ord + Default + Hash,
    > From<Vec<Card<RankType, SuitType>>> for Pile<RankType, SuitType>
{
    fn from(cards: Vec<Card<RankType, SuitType>>) -> Self {
        Pile(cards)
    }
}

impl<
        RankType: Ranked + Clone + Copy + PartialOrd + Ord + Default + Hash,
        SuitType: Suited + Clone + Copy + PartialOrd + Ord + Default + Hash,
    > IntoIterator for Pile<RankType, SuitType>
{
    type Item = Card<RankType, SuitType>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<
        RankType: Ranked + Clone + Copy + PartialOrd + Ord + Default + Hash,
        SuitType: Suited + Clone + Copy + PartialOrd + Ord + Default + Hash,
    > IntoIterator for &Pile<RankType, SuitType>
{
    type Item = Card<RankType, SuitType>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.clone().into_iter()
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Card<RankType, SuitType>
where
    RankType: Ranked + Clone + Copy + PartialOrd + Ord + Default + Hash,
    SuitType: Suited + Clone + Copy + PartialOrd + Ord + Default + Hash,
{
    pub suit: Suit<SuitType>,
    pub rank: Rank<RankType>,
}

impl<RankType, SuitType> Card<RankType, SuitType>
where
    RankType: Ranked + Clone + Copy + PartialOrd + Ord + Default + Hash,
    SuitType: Suited + Clone + Copy + PartialOrd + Ord + Default + Hash,
{
    #[must_use]
    pub fn is_blank(&self) -> bool {
        self.rank.is_blank() | self.suit.is_blank()
    }
}

impl<RankType, SuitType> Display for Card<RankType, SuitType>
where
    RankType: Ranked + Clone + Copy + PartialOrd + Ord + Default + Hash,
    SuitType: Suited + Clone + Copy + PartialOrd + Ord + Default + Hash,
{
    /// ```
    /// use cardpack::refactored::*;
    ///
    /// let card: Card<French, French> = Card {
    ///     suit: French::CLUBS,
    ///     rank: French::DEUCE,
    /// };
    ///
    /// assert_eq!(card.to_string(), "2♣");
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Suit<SuitType>
where
    SuitType: Suited,
{
    pub weight: u32,
    pub index: char,
    pub phantom_data: PhantomData<SuitType>,
}

impl<SuitType> Suit<SuitType>
where
    SuitType: Suited,
{
    /// ```
    /// use cardpack::refactored::*;
    ///
    /// assert_eq!(0b00000000_00000000_00010000_00000000, French::CLUBS.ckc_number());
    ///
    /// ```
    #[must_use]
    pub fn ckc_number(&self) -> u32 {
        match self.weight {
            0 => 0,
            _ => 1 << (Bit::SUIT_FLAG_SHIFT + self.weight),
        }
    }

    #[must_use]
    pub fn is_blank(&self) -> bool {
        self.index == BLANK
    }
}

impl<SuitType: Suited> Default for Suit<SuitType> {
    fn default() -> Self {
        Suit {
            weight: 0,
            index: '_',
            phantom_data: PhantomData,
        }
    }
}

impl<SuitType> Display for Suit<SuitType>
where
    SuitType: Suited,
{
    /// ```
    /// use cardpack::refactored::*;
    ///
    /// assert_eq!(French::DIAMONDS.to_string(), "♦");
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Suit::<SuitType>::get_suit_symbol(self.index))
    }
}

impl<SuitType: Suited> From<char> for Suit<SuitType> {
    fn from(c: char) -> Self {
        // Implement the conversion logic from char to Suit<SuitType>
        // This is a placeholder implementation
        SuitType::from(c)
    }
}

// impl<SuiteType: Suited> From<char> for Suit<SuiteType> {
//     fn from(index: char) -> Self {
//         SuiteType::from(index)
//     }
// }

impl<SuiteType: Suited> Suited for Suit<SuiteType> {
    fn get_suit_fluent_name(index: char) -> FluentName {
        SuiteType::get_suit_fluent_name(index)
    }

    fn get_suit_symbol(index: char) -> char {
        SuiteType::get_suit_symbol(index)
    }

    fn suit_indexes() -> Vec<char> {
        SuiteType::suit_indexes()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Rank<RankType>
where
    RankType: Ranked,
{
    pub weight: u32,
    pub index: char,
    pub phantom_data: PhantomData<RankType>,
}

impl<RankType> Rank<RankType>
where
    RankType: Ranked,
{
    const PRIMES: [u32; 20] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
    ];

    #[must_use]
    pub fn get_name(&self) -> FluentName {
        RankType::get_rank_fluent_name(self.index)
    }

    /// Returns the xth prime number where x is the weight of the rank.
    ///
    ///The goal of this function is to replace the earlier version of the struct that stored the
    /// prime number as a field. In refactoring this code, I am trying to take a minimalist approach
    /// to the types. The original structs come from when I was very new to Rust.
    ///
    /// ```
    /// use cardpack::refactored::*;
    ///
    /// assert_eq!(French::DEUCE.get_prime(), 2);
    /// assert_eq!(French::TREY.get_prime(), 3);
    /// assert_eq!(French::FOUR.get_prime(), 5);
    /// assert_eq!(French::FIVE.get_prime(), 7);
    /// assert_eq!(French::SIX.get_prime(), 11);
    /// assert_eq!(French::SEVEN.get_prime(), 13);
    /// assert_eq!(French::EIGHT.get_prime(), 17);
    /// assert_eq!(French::NINE.get_prime(), 19);
    /// assert_eq!(French::TEN.get_prime(), 23);
    /// assert_eq!(French::JACK.get_prime(), 29);
    /// assert_eq!(French::QUEEN.get_prime(), 31);
    /// assert_eq!(French::KING.get_prime(), 37);
    /// assert_eq!(French::ACE.get_prime(), 41);
    /// ```
    ///
    /// It only goes up to 20:
    ///
    /// ```
    /// use std::marker::PhantomData;
    /// use cardpack::refactored::*;
    ///
    /// let heavy_card = French::TREY.update_weight(21);
    ///
    /// assert_eq!(heavy_card.get_prime(), 0);
    /// ```
    /// TODO: Hack
    #[must_use]
    pub fn get_prime(&self) -> u32 {
        if self.weight as usize >= Rank::<RankType>::PRIMES.len() {
            0
        } else {
            Rank::<RankType>::PRIMES[(self.weight) as usize]
        }
    }

    #[must_use]
    pub fn is_blank(&self) -> bool {
        self.index == BLANK
    }

    #[must_use]
    pub fn update_weight(&self, weight: u32) -> Rank<RankType> {
        Rank {
            weight,
            index: self.index,
            phantom_data: PhantomData,
        }
    }

    //\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\
    // CKC Calculations
    /// Used to generate the `Rank` portion of the `Card`'s binary number,
    /// aka [Cactus Kev](https://suffe.cool/poker/evaluator.html) number.
    ///
    /// ```
    /// use cardpack::refactored::*;
    ///
    /// assert_eq!(0b00000000_00000001_00000000_00000010, French::DEUCE.ckc_number());
    /// assert_eq!(0b00000000_00000010_00000001_00000011, French::TREY.ckc_number());
    /// assert_eq!(0b00000000_00000100_00000010_00000101, French::FOUR.ckc_number());
    /// assert_eq!(0b00000000_00001000_00000011_00000111, French::FIVE.ckc_number());
    /// assert_eq!(0b00000000_00010000_00000100_00001011, French::SIX.ckc_number());
    /// assert_eq!(0b00000000_00100000_00000101_00001101, French::SEVEN.ckc_number());
    /// assert_eq!(0b00000000_01000000_00000110_00010001, French::EIGHT.ckc_number());
    /// assert_eq!(0b00000000_10000000_00000111_00010011, French::NINE.ckc_number());
    /// assert_eq!(0b00000001_00000000_00001000_00010111, French::TEN.ckc_number());
    /// assert_eq!(0b00000010_00000000_00001001_00011101, French::JACK.ckc_number());
    /// assert_eq!(0b00000100_00000000_00001010_00011111, French::QUEEN.ckc_number());
    /// assert_eq!(0b00001000_00000000_00001011_00100101, French::KING.ckc_number());
    /// assert_eq!(0b00010000_00000000_00001100_00101001, French::ACE.ckc_number());
    /// ```
    #[must_use]
    pub fn ckc_number(&self) -> u32 {
        self.get_bits() | self.get_shift8() | self.get_prime()
    }

    #[must_use]
    fn get_bits(&self) -> u32 {
        1 << (Bit::RANK_FLAG_SHIFT + self.weight)
    }

    #[must_use]
    pub fn get_shift8(&self) -> u32 {
        self.weight << 8
    }
}

impl<RankType: Ranked> Default for Rank<RankType> {
    fn default() -> Self {
        Rank {
            weight: 0,
            index: '_',
            phantom_data: PhantomData,
        }
    }
}

impl<RankType> Display for Rank<RankType>
where
    RankType: Ranked,
{
    /// ```
    /// use cardpack::refactored::*;
    ///
    /// assert_eq!(French::DEUCE_INDEX.to_string(), French::DEUCE.to_string());
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.index)
    }
}

// impl<RankType: Ranked> From<char> for Rank<RankType> {
//     fn from(index: char) -> Self {
//         RankType::from(index)
//     }
// }

#[cfg(test)]
#[allow(non_snake_case)]
mod card_tests {
    use super::*;
    use crate::refactored::French;

    #[test]
    fn card__is_blank() {
        let card = Card::<French, French>::default();

        println!("{:?}", card);

        assert!(card.is_blank());
    }

    #[test]
    fn card__sort() {
        let mut v: Vec<Card<French, French>> = Vec::new();

        for suit_char in French::suit_indexes() {
            for rank_char in French::rank_indexes() {
                let card = Card::<French, French> {
                    suit: Suit::<French>::from(suit_char),
                    rank: Rank::<French>::from(rank_char),
                };

                println!("{}", card);
                v.push(card);
            }
        }
        v.reverse();
        v.sort();
        println!("{:?}", v);
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod ranks {
    // use super::*;
}
