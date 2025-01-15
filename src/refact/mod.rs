pub mod decks;
pub mod traits;

use crate::localization::FluentName;
use crate::types::utils::Bit;
use std::collections::{HashMap, HashSet};

use crate::prelude::CardError;
use crate::refact::traits::{Ranked, Suited};
use colored::Color;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::marker::PhantomData;
use std::str::FromStr;

pub const BLANK: char = '_';

//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\
// Pile

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
    /// Returns a clone of the `Pile`'s internal `Vec<Card>`.
    ///
    /// ```
    /// use cardpack::refactored::*;
    ///
    /// let pile: Pile<French, French> = Pile::<French, French>::from_str("K♣ Q♣ J♣").unwrap();
    ///
    /// let expected = vec![
    ///     Card::<French, French>::new(French::KING, French::CLUBS),
    ///     Card::<French, French>::new(French::QUEEN, French::CLUBS),
    ///     Card::<French, French>::new(French::JACK, French::CLUBS)
    /// ];
    ///
    /// assert_eq!(pile.cards(), expected);
    /// ```
    #[must_use]
    pub fn cards(&self) -> Vec<Card<RankType, SuitType>> {
        self.0.clone()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn index(&self) -> String {
        self.0
            .iter()
            .map(Card::index)
            .collect::<Vec<String>>()
            .join(" ")
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
/// let pile: Pile<French, French> = French::deck();
///
/// assert_eq!(pile.to_string(), "A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣");
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
    > From<HashSet<Card<RankType, SuitType>>> for Pile<RankType, SuitType>
{
    fn from(cards: HashSet<Card<RankType, SuitType>>) -> Self {
        Pile(cards.into_iter().collect())
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
    > FromStr for Pile<RankType, SuitType>
{
    type Err = CardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pile = Pile::<RankType, SuitType>::default();

        for card_str in s.split_whitespace() {
            let card = card_str.parse::<Card<RankType, SuitType>>()?;
            pile.push(card);
        }

        Ok(pile)
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

#[cfg(test)]
#[allow(non_snake_case)]
mod pile_tests {
    use super::*;
    use crate::refact::traits::Decked;
    use crate::refactored::French;

    #[test]
    fn pile__index() {
        let pile: Pile<French, French> = French::deck();

        assert_eq!(pile.index(), "AS KS QS JS TS 9S 8S 7S 6S 5S 4S 3S 2S AH KH QH JH TH 9H 8H 7H 6H 5H 4H 3H 2H AD KD QD JD TD 9D 8D 7D 6D 5D 4D 3D 2D AC KC QC JC TC 9C 8C 7C 6C 5C 4C 3C 2C");
    }

    #[test]
    fn pile__is_empty() {
        let pile: Pile<French, French> = Pile::<French, French>::default();

        assert!(pile.is_empty());
    }

    #[test]
    fn pile__len() {
        let pile: Pile<French, French> = Pile::<French, French>::default();

        assert_eq!(pile.len(), 0);
    }

    #[test]
    fn pile__push() {
        let mut pile: Pile<French, French> = Pile::<French, French>::default();

        let card = Card::<French, French>::new(French::DEUCE, French::CLUBS);

        assert!(pile.push(card));
        assert_eq!(pile.len(), 1);
    }

    #[test]
    fn pile__sort() {
        let mut pile: Pile<French, French> = Pile::<French, French>::default();

        let card1 = Card::<French, French>::new(French::DEUCE, French::CLUBS);
        let card2 = Card::<French, French>::new(French::TREY, French::DIAMONDS);
        let card3 = Card::<French, French>::new(French::FOUR, French::CLUBS);

        pile.push(card1);
        pile.push(card2);
        pile.push(card3);

        let sorted_pile = pile.sort();

        assert_eq!(pile.to_string(), "2♣ 3♦ 4♣");
        assert_eq!(sorted_pile.to_string(), "3♦ 4♣ 2♣");
        assert_eq!(sorted_pile.0[0], card2);
        assert_eq!(sorted_pile.0[1], card3);
        assert_eq!(sorted_pile.0[2], card1);
    }
}

//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\
// Card

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
    /// The original version of the `Card` struct included its own weight field which
    /// was creating by adding the `Suit.weight` times 1000 with the `Rank.weight`. By simply
    /// having the `Suit` be before the `Rank` in the `Card` struct, the sorting is handled
    /// automatically.
    #[must_use]
    pub fn new(rank: Rank<RankType>, suit: Suit<SuitType>) -> Card<RankType, SuitType> {
        Card { suit, rank }
    }

    #[must_use]
    pub fn index(&self) -> String {
        format!("{}{}", self.rank.index, self.suit.index)
    }

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

impl<RankType, SuitType> FromStr for Card<RankType, SuitType>
where
    RankType: Ranked + Clone + Copy + PartialOrd + Ord + Default + Hash,
    SuitType: Suited + Clone + Copy + PartialOrd + Ord + Default + Hash,
{
    type Err = CardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if s.chars().count() != 2 {
            return Err(CardError::InvalidIndex(s.to_string()));
        }

        if let Some(rank_c) = s.chars().next() {
            let rank = Rank::<RankType>::from(rank_c);
            if let Some(suit_c) = s.chars().last() {
                let suit = Suit::<SuitType>::from(suit_c);
                let card = Card::<RankType, SuitType>::new(rank, suit);
                return Ok(card);
            };
        }

        Err(CardError::Fubar)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod card_tests {
    use super::*;
    use crate::refactored::French;

    #[test]
    fn card__is_blank() {
        let card = Card::<French, French>::default();

        assert!(card.is_blank());
    }
    //
    // #[test]
    // fn card__sort() {
    //     let mut v: Vec<Card<French, French>> = Vec::new();
    //
    //     for suit_char in French::suit_indexes() {
    //         for rank_char in French::rank_indexes() {
    //             let card = Card::<French, French> {
    //                 suit: Suit::<French>::from(suit_char),
    //                 rank: Rank::<French>::from(rank_char),
    //             };
    //
    //             println!("{}", card);
    //             v.push(card);
    //         }
    //     }
    //     v.reverse();
    //     v.sort();
    //     println!("{:?}", v);
    // }

    #[test]
    fn from_str() {
        assert_eq!(
            "2♣".parse::<Card<French, French>>().unwrap(),
            Card::<French, French>::new(French::DEUCE, French::CLUBS)
        );
        assert_eq!(
            "2c".parse::<Card<French, French>>().unwrap(),
            Card::<French, French>::new(French::DEUCE, French::CLUBS)
        );
    }

    #[test]
    fn from_str__error() {
        assert!("2S♣".parse::<Card<French, French>>().is_err());
    }
}

//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\
// Rank

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
    pub const BLANK: Rank<RankType> = Rank {
        weight: 0,
        index: BLANK,
        phantom_data: PhantomData,
    };

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

/// ```
/// use std::marker::PhantomData;
/// use cardpack::refactored::*;
///
/// let expected = Rank::<French> {
///     weight: 0,
///     index: BLANK,
///     phantom_data: PhantomData,
/// };
///
/// assert_eq!(Rank::<French>::default(), expected);
/// ```
impl<RankType: Ranked> Default for Rank<RankType> {
    fn default() -> Self {
        Rank::<RankType>::BLANK
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

/// # REFACTOR WIN
///
/// This win ended up being a big W when I tried to wire it into the Card struct.
///
/// ```
/// use cardpack::refactored::*;
///
/// assert_eq!(Rank::<French>::from('a'), French::ACE);
/// assert_eq!(Rank::<French>::from('2'), French::DEUCE);
/// ```
///
/// # DOUBLE REFACTOR WIN
///
/// We've moved this down from the Deck generic instance to here, making it all-encompasing.
/// The original version was:
///
/// ```txt
/// impl From<char> for Rank<French> {
///     fn from(c: char) -> Self {
///         match c {
///             French::ACE_INDEX | 'a' => French::ACE,
///             French::KING_INDEX | 'k' => French::KING,
///             French::QUEEN_INDEX | 'q' => French::QUEEN,
///             French::JACK_INDEX | 'j' => French::JACK,
///             French::TEN_INDEX | 't' | '0' => French::TEN,
///             French::NINE_INDEX => French::NINE,
///             French::EIGHT_INDEX => French::EIGHT,
///             French::SEVEN_INDEX => French::SEVEN,
///             French::SIX_INDEX => French::SIX,
///             French::FIVE_INDEX => French::FIVE,
///             French::FOUR_INDEX => French::FOUR,
///             French::TREY_INDEX => French::TREY,
///             French::DEUCE_INDEX => French::DEUCE,
///             _ => Rank::<French>::default(),
///         }
///     }
/// }
/// ```
///
/// # TRIPLE REFACTOR WIN
///
/// We can now remove the `Rank::<RankType>::new() function and just use this instead.`
impl<RankType: Ranked> From<char> for Rank<RankType> {
    fn from(index: char) -> Self {
        // Wash the index to make sure it's the correct char.
        let index = RankType::get_rank_index(index);
        Rank {
            weight: RankType::get_rank_weight(index),
            index,
            phantom_data: PhantomData,
        }
    }
}

impl<RankType: Ranked> Ranked for Rank<RankType> {
    fn get_rank_fluent_name(c: char) -> FluentName {
        RankType::get_rank_fluent_name(c)
    }

    fn get_rank_index(c: char) -> char {
        RankType::get_rank_index(c)
    }

    fn get_rank_weight(c: char) -> u32 {
        RankType::get_rank_weight(c)
    }

    fn rank_indexes() -> Vec<char> {
        RankType::rank_indexes()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod ranks {
    use super::*;
    use crate::refactored::French;
    use rstest::rstest;

    /// TODO: Add some rows for other decks when they're added.
    #[rstest]
    #[case('A', French::ACE)]
    #[case('a', French::ACE)]
    #[case('K', French::KING)]
    #[case('k', French::KING)]
    #[case('Q', French::QUEEN)]
    #[case('q', French::QUEEN)]
    #[case('J', French::JACK)]
    #[case('j', French::JACK)]
    #[case('T', French::TEN)]
    #[case('t', French::TEN)]
    #[case('0', French::TEN)]
    #[case('9', French::NINE)]
    #[case('8', French::EIGHT)]
    #[case('7', French::SEVEN)]
    #[case('6', French::SIX)]
    #[case('5', French::FIVE)]
    #[case('4', French::FOUR)]
    #[case('3', French::TREY)]
    #[case('2', French::DEUCE)]
    #[case('1', French::BLANK_RANK)]
    #[case('F', French::BLANK_RANK)]
    fn suit__from__char(#[case] input: char, #[case] expected: Rank<French>) {
        assert_eq!(expected, Rank::<French>::from(input));
    }
}

//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\
// Suit

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
    pub const BLANK: Suit<SuitType> = Suit {
        weight: 0,
        index: BLANK,
        phantom_data: PhantomData,
    };

    /// Returns the Suit portion of the `CKC Number`.
    /// Used to generate the `Card`'s binary signature, aka [Cactus Kev](https://suffe.cool/poker/evaluator.html)
    /// numbers.
    ///
    /// Revised version that inverts the weight for sorting, making Spades be the highest. Has no
    /// effect on the generated card ranks, but does make sorting easier.
    ///
    /// TODO: need a way to add the jokers suit. Right now this assumes standard 52
    ///
    /// ```
    /// use cardpack::refactored::*;
    ///
    /// assert_eq!(0b00000000_00000000_10000000_00000000, French::SPADES.ckc_number());
    /// assert_eq!(0b00000000_00000000_01000000_00000000, French::HEARTS.ckc_number());
    /// assert_eq!(0b00000000_00000000_00100000_00000000, French::DIAMONDS.ckc_number());
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

    #[must_use]
    pub fn suits() -> Vec<Self> {
        SuitType::suit_indexes()
            .iter()
            .map(|index| Suit::<SuitType>::from(*index))
            .collect()
    }

    /// TODO: Possible REFACTOR - Add symbol char to struct to avoid need for localization
    /// call. We will save this for after REF2 is complete.
    #[must_use]
    pub fn symbol(&self) -> char {
        SuitType::get_suit_symbol(self.index)
    }
}

impl<SuitType: Suited> Default for Suit<SuitType> {
    fn default() -> Self {
        Suit::<SuitType>::BLANK
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

/// Instantiates a new Suit struct from the passed in index.
///
/// ```
/// use cardpack::refactored::*;
///
/// assert_eq!(Suit::<French>::from('♠'), French::SPADES);
/// assert_eq!(Suit::<French>::from('S'), French::SPADES);
/// assert_eq!(Suit::<French>::from('s'), French::SPADES);
/// assert_eq!(Suit::<French>::from('♤'), French::SPADES);
/// ```
impl<SuiteType: Suited> From<char> for Suit<SuiteType> {
    fn from(index: char) -> Self {
        let index = Suit::<SuiteType>::get_suit_index(index);
        Suit {
            weight: Suit::<SuiteType>::get_suit_weight(index),
            index,
            phantom_data: PhantomData,
        }
    }
}

impl<SuiteType: Suited> Suited for Suit<SuiteType> {
    fn colors() -> HashMap<char, Color> {
        SuiteType::colors()
    }

    fn get_suit_fluent_name(c: char) -> FluentName {
        SuiteType::get_suit_fluent_name(c)
    }

    fn get_suit_index(c: char) -> char {
        SuiteType::get_suit_index(c)
    }

    fn get_suit_symbol(c: char) -> char {
        SuiteType::get_suit_symbol(c)
    }

    fn get_suit_weight(c: char) -> u32 {
        SuiteType::get_suit_weight(c)
    }

    fn suit_indexes() -> Vec<char> {
        SuiteType::suit_indexes()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod suits {
    use super::*;
    use crate::refactored::French;
    use rstest::rstest;

    /// TODO: Add some rows for other decks when they're added.
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
    #[case('🃟', French::BLANK_SUIT)]
    #[case('T', French::BLANK_SUIT)]
    #[case('t', French::BLANK_SUIT)]
    #[case(' ', French::BLANK_SUIT)]
    #[case('F', French::BLANK_SUIT)]
    fn suit__from__char(#[case] input: char, #[case] expected: Suit<French>) {
        assert_eq!(expected, Suit::<French>::from(input));
    }

    #[test]
    fn suit__symbol() {
        assert_eq!(Suit::<French>::from('♠').symbol(), '♠');
        assert_eq!(Suit::<French>::from('S').symbol(), '♠');
        assert_eq!(Suit::<French>::from('s').symbol(), '♠');
        assert_eq!(Suit::<French>::from('♤').symbol(), '♠');
        assert_eq!(Suit::<French>::from(' ').symbol(), '_');
    }

    #[test]
    fn suit__suits() {
        let expected = vec![
            French::SPADES,
            French::HEARTS,
            French::DIAMONDS,
            French::CLUBS,
        ];

        assert_eq!(Suit::<French>::suits(), expected);
    }
}
