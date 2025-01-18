pub mod decks;
pub mod pips;
pub mod traits;

use crate::prelude::CardError;
use crate::refact::pips::{Rank, Suit};
use crate::refact::traits::{Ranked, Suited};
use colored::Colorize;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::str::FromStr;

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
    pub fn draw(&mut self, n: usize) -> Option<Self> {
        if n > self.len() || n < 1 {
            None
        } else {
            let mut cards = Pile::<RankType, SuitType>::default();
            for _ in 0..n {
                cards.push(self.draw_first()?);
            }
            Some(cards)
        }
    }

    /// ```
    /// use cardpack::refactored::*;
    ///
    /// let mut pile = French::deck();
    /// let card = pile.draw_first().unwrap();
    ///
    /// assert_eq!(card.to_string(), "A♠");
    /// ```
    pub fn draw_first(&mut self) -> Option<Card<RankType, SuitType>> {
        match self.len() {
            0 => None,
            _ => Some(self.remove(0)),
        }
    }

    /// ```
    /// use cardpack::refactored::*;
    ///
    /// let mut pile = French::deck();
    /// let card = pile. draw_last().unwrap();
    ///
    /// assert_eq!(card. to_string(), "2♣");
    /// ```
    pub fn draw_last(&mut self) -> Option<Card<RankType, SuitType>> {
        self.0.pop()
    }

    /// ```
    /// use cardpack::refactored::*;
    ///
    /// assert!(Pile::<French, French>::default().is_empty());
    /// assert!(!French::deck().is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the number of `Card`s in the `Pile`.
    ///
    /// ```
    /// use cardpack::refactored::*;
    ///
    /// let pile: Pile<French, French> = Pile::<French, French>::from_str("5♥ 3♥ 2♣ 4♣ A♥").unwrap();
    ///
    /// assert_eq!(pile.len(), 5);
    /// assert_eq!(French::deck().len(), 52);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns a string representation of the `Pile`'s `Card`s index strings. The index is the
    /// alpha-numerical representation of the `Card`'s `Rank` and `Suit`.
    ///
    /// ```
    /// use cardpack::refactored::*;
    ///
    /// let pile: Pile<French, French> = Pile::<French, French>::from_str("5♣ 3♣ 2♣ 4♣ A♣ ").unwrap();
    ///
    /// assert_eq!(pile.index(), "5C 3C 2C 4C AC");
    /// ```
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

    /// ```
    /// use cardpack::refactored::*;
    /// let pile = French::deck();
    ///
    /// let card = Card::<French, French>::from_str("2♣").unwrap();
    ///
    /// assert_eq!(pile.position(&card).unwrap(), 51);
    /// ```
    #[must_use]
    pub fn position(&self, card: &Card<RankType, SuitType>) -> Option<usize> {
        self.0.iter().position(|c| c == card)
    }

    /// ```
    /// use cardpack::refactored::*;
    /// let mut hand = Pile::<French, French>::from_str("A♦ T♦ Q♦").unwrap();
    /// let flop = Pile::<French, French>::from_str("K♦ J♦").unwrap();
    ///
    /// hand.append(&flop);
    ///
    /// assert_eq!(hand.to_string(), "A♦ T♦ Q♦ K♦ J♦");
    /// ```
    pub fn append(&mut self, other: &Pile<RankType, SuitType>) {
        self.0.append(&mut other.0.clone());
    }

    /// ```
    /// use cardpack::refactored::*;
    /// let mut hand = Pile::<French, French>::from_str("T♦ Q♦").unwrap();
    /// let flop = Pile::<French, French>::from_str("K♦ J♦ A♦").unwrap();
    ///
    /// hand.prepend(&flop);
    ///
    /// assert_eq!(hand.to_string(), "K♦ J♦ A♦ T♦ Q♦");
    /// ```
    pub fn prepend(&mut self, other: &Pile<RankType, SuitType>) {
        let mut product = other.0.clone();
        product.append(&mut self.0);
        self.0 = product;
    }

    /// Places the Card at the bottom (end) of the Pile.
    ///
    /// ```
    /// use cardpack::refactored::*;
    ///
    /// let mut pile = Pile::<French, French>::from_str("K♠ A♠").unwrap();
    /// let card = Card::<French, French>::from_str("Q♠").unwrap();
    ///
    /// assert!(pile.push(card));
    /// assert_eq!(pile.to_string(), "K♠ A♠ Q♠");
    /// ```
    pub fn push(&mut self, card: Card<RankType, SuitType>) -> bool {
        if card.is_blank() {
            false
        } else {
            self.0.push(card);
            true
        }
    }

    /// ```
    /// use cardpack::refactored::*;
    /// let pile = Pile::<French, French>::from_str("A♠ K♠ A♣ Q♣ K♥").unwrap();
    /// assert_eq!(pile.rank_index(" "), "A K Q");
    /// assert_eq!(French::deck().rank_index(" "), "A K Q J T 9 8 7 6 5 4 3 2");
    /// ```
    #[must_use]
    pub fn rank_index(&self, joiner: &str) -> String {
        self.rank_indexed(|rank| String::from(rank.index), joiner)
    }

    /// This is a mirror of the suit version. Seems totally over the top, but it's fun to include.
    pub fn rank_indexed<F>(&self, func: F, joiner: &str) -> String
    where
        F: Fn(&Rank<RankType>) -> String,
    {
        self.ranks()
            .iter()
            .map(func)
            .collect::<Vec<String>>()
            .join(joiner)
    }

    /// ```
    /// use cardpack::refactored::*;
    /// let pile = Pile::<French, French>::from_str("A♠ K♠ A♣ Q♣ K♥").unwrap();
    ///
    /// assert_eq!(pile.rank_index_by_suit(&French::SPADES, "-").unwrap(), "A-K");
    /// assert_eq!(pile.rank_index_by_suit(&French::HEARTS, "-"), Some("K".to_string()));
    /// assert_eq!(pile.rank_index_by_suit(&French::CLUBS, "-"), Some("A-Q".to_string()));
    /// assert_eq!(pile.rank_index_by_suit(&French::DIAMONDS, "-"), None);
    /// ```
    #[must_use]
    pub fn rank_index_by_suit(&self, suit: &Suit<SuitType>, joiner: &str) -> Option<String> {
        let ranks = self.ranks_by_suit(suit)?;
        Some(
            ranks
                .iter()
                .map(|r| String::from(r.index))
                .collect::<Vec<String>>()
                .join(joiner),
        )
    }

    /// Returns a vector of the `Rank`s within the `Pile`.
    ///
    /// ```
    /// use cardpack::refactored::*;
    /// let pile = Pile::<French, French>::from_str("A♠ K♠ A♣ Q♣ K♥").unwrap();
    ///
    /// let expected = vec![
    ///     French::ACE,
    ///     French::KING,
    ///     French::QUEEN,
    /// ];
    ///
    /// assert_eq!(pile.ranks(), expected);
    /// ```
    #[must_use]
    pub fn ranks(&self) -> Vec<Rank<RankType>> {
        let mut v: Vec<Rank<RankType>> = self
            .iter()
            .map(|c| c.rank)
            .collect::<HashSet<Rank<RankType>>>()
            .into_iter()
            .collect();
        v.sort();
        v.reverse();

        v
    }

    /// ```
    /// use cardpack::refactored::*;
    /// let pile = Pile::<French, French>::from_str("A♠ K♠").unwrap();
    ///
    /// let expected = vec![
    ///     French::ACE,
    ///     French::KING,
    /// ];
    ///
    /// assert_eq!(pile.ranks_by_suit(&French::SPADES).unwrap(), expected);
    /// assert!(pile.ranks_by_suit(&French::HEARTS).is_none());
    #[must_use]
    pub fn ranks_by_suit(&self, suit: &Suit<SuitType>) -> Option<Vec<Rank<RankType>>> {
        let ranks: Vec<Rank<RankType>> = self
            .iter()
            .filter(|c| &c.suit == suit)
            .map(|c| c.rank)
            .collect();

        match ranks.len() {
            0 => None,
            _ => Some(ranks),
        }
    }

    /// TODO: Possible RF change to [`VecDeque`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)?
    pub fn remove(&mut self, x: usize) -> Card<RankType, SuitType> {
        self.0.remove(x)
    }

    /// ```
    /// use cardpack::refactored::*;
    ///
    /// let mut pile = French::deck();
    /// pile.remove_card(&Card::<French, French>::from_str("K♠").unwrap());
    ///
    /// let actual = pile.draw(2).unwrap();
    ///
    /// println!("{pile}");
    /// println!("{actual}");
    ///
    /// assert_eq!(actual, Pile::<French, French>::from_str("AS QS").unwrap());
    /// ```
    pub fn remove_card(
        &mut self,
        card: &Card<RankType, SuitType>,
    ) -> Option<Card<RankType, SuitType>> {
        let index = self.0.iter().position(|c| c == card);
        if let Some(i) = index {
            Some(self.0.remove(i))
        } else {
            None
        }
    }

    /// ```
    /// use cardpack::refactored::*;
    ///
    /// let ak = Pile::<French, French>::from_str("A♠ K♠").unwrap();
    /// let ka = Pile::<French, French>::from_str("K♠ A♠").unwrap();
    ///
    /// assert_eq!(ak.reverse(), ka);
    /// assert_eq!(ka.reverse(), ak);
    /// ```
    #[must_use]
    pub fn reverse(&self) -> Self {
        let mut pile = self.clone();
        pile.reverse_in_place();
        pile
    }

    /// ```
    /// use cardpack::refactored::*;
    ///
    /// let mut ak = Pile::<French, French>::from_str("A♠ K♠").unwrap();
    /// let ka = Pile::<French, French>::from_str("K♠ A♠").unwrap();
    ///
    /// ak.reverse_in_place();
    ///
    /// assert_eq!(ak, ka);
    /// ```
    pub fn reverse_in_place(&mut self) {
        self.0.reverse();
    }

    /// ```
    /// use cardpack::refactored::*;
    ///
    /// let pile = French::deck();
    /// let other_pile = pile.shuffle();
    ///
    /// assert!(pile.same(&pile));
    /// assert!(pile.same(&other_pile));
    /// ```
    #[must_use]
    pub fn same(&self, other: &Self) -> bool {
        let left = self.sort();
        let right = other.sort();

        left == right
    }

    #[must_use]
    pub fn shuffle(&self) -> Self {
        let mut pile = self.clone();
        pile.shuffle_in_place();
        pile
    }

    /// ```
    /// use cardpack::refactored::*;
    ///
    /// let mut pile: Pile<French, French> = French::deck();
    ///
    /// pile.shuffle_in_place();
    ///
    /// assert!(pile.same(&French::deck()));
    /// ```
    pub fn shuffle_in_place(&mut self) {
        let mut rng = thread_rng();
        self.0.shuffle(&mut rng);
    }

    /// TODO WIP
    #[allow(dead_code)]
    fn shuffle_in_place_custom<F>(&mut self, mut rng: F)
    where
        F: FnMut(usize) -> usize,
    {
        let mut cards = self.0.clone();
        let mut shuffled = Vec::new();
        while !cards.is_empty() {
            let index = rng(cards.len());
            shuffled.push(cards.remove(index));
        }
        self.0 = shuffled;
    }

    /// Returns a cloned version of the `Pile` sorted in descending order.
    ///
    /// ```
    /// use cardpack::refactored::*;
    ///
    /// let pile: Pile<French, French> = Pile::<French, French>::from_str("5♣ 3♣ 2♣ 4♣ A♣ ").unwrap();
    ///
    /// let sorted = pile.sort();
    ///
    /// assert_eq!(sorted.to_string(), "A♣ 5♣ 4♣ 3♣ 2♣");
    /// ```
    #[must_use]
    pub fn sort(&self) -> Self {
        let mut cards: Vec<Card<RankType, SuitType>> = self.0.clone();
        cards.sort();
        Self(cards)
    }

    /// ```
    /// use cardpack::refactored::*;
    ///
    /// let mut pile: Pile<French, French> = Pile::<French, French>::from_str("5♣ 3♣ 2♣ 4♣ A♣ ").unwrap();
    ///
    /// pile.sort_in_place();
    ///
    /// assert_eq!(pile.to_string(), "A♣ 5♣ 4♣ 3♣ 2♣");
    /// ```
    pub fn sort_in_place(&mut self) {
        self.0.sort();
    }

    /// Returns a `String` with all of the `Suit` index letters for the `Pile` separated by spaces.
    ///
    /// ```
    /// use cardpack::refactored::*;
    ///
    /// let pile: Pile<French, French> = French::deck();
    ///
    /// assert_eq!(pile.suit_index(), "S H D C");
    /// ```
    #[must_use]
    pub fn suit_index(&self) -> String {
        self.suit_indexed(|suit| String::from(suit.index), " ")
    }

    /// Returns a `String` with all of the `Suit` symbols for the `Pile` separated by spaces.
    ///
    /// ```
    /// use cardpack::refactored::*;
    ///
    /// let pile: Pile<French, French> = French::deck();
    ///
    /// assert_eq!(pile.suit_symbol_index(), "♠ ♥ ♦ ♣");
    /// ```
    #[must_use]
    pub fn suit_symbol_index(&self) -> String {
        self.suit_indexed(|suit| String::from(suit.symbol()), " ")
    }

    #[must_use]
    fn suit_indexed<F>(&self, func: F, joiner: &str) -> String
    where
        F: Fn(&Suit<SuitType>) -> String,
    {
        self.suits()
            .iter()
            .map(func)
            .collect::<Vec<String>>()
            .join(joiner)
    }

    /// Returns a sorted vector of the `Suit`s within the `Pile`.
    ///
    /// ```
    /// use cardpack::refactored::*;
    ///
    /// let pile: Pile<French, French> = French::deck();
    ///
    /// assert_eq!(pile.suits(), vec![French::SPADES, French::HEARTS, French::DIAMONDS, French::CLUBS]);
    /// ```
    #[must_use]
    pub fn suits(&self) -> Vec<Suit<SuitType>> {
        let mut v: Vec<Suit<SuitType>> = self
            .iter()
            .map(|c| c.suit)
            .collect::<HashSet<Suit<SuitType>>>()
            .into_iter()
            .collect();
        v.sort();
        v.reverse();

        v
    }

    /// Returns a `String` of the cards symbol string, colored by what's defined in the
    /// `Suited` trait.
    #[must_use]
    pub fn to_color_symbol_string(&self) -> String {
        self.0
            .iter()
            .map(Card::to_color_symbol_string)
            .collect::<Vec<String>>()
            .join(" ")
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
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

    /// Returns a `index` string representation of the `Card`. Index, in terms of this library
    /// is defined as a single char for the `Rank` and a single char for the `Suit`.
    ///
    /// ```
    /// use cardpack::refactored::*;
    ///
    /// let card: Card<French, French> = "A♠".parse::<Card<French, French>>().unwrap();
    ///
    /// assert_eq!(card.index(), "AS");
    /// ```
    #[must_use]
    pub fn index(&self) -> String {
        format!("{}{}", self.rank.index, self.suit.index)
    }

    #[must_use]
    pub fn is_blank(&self) -> bool {
        self.rank.is_blank() | self.suit.is_blank()
    }

    /// Returns a colorized string representation of the `Card` based on the color mappings
    /// set in the `Suited::colors()` method.
    ///
    /// ```
    /// use colored::Colorize;
    /// use cardpack::refactored::*;
    ///
    /// let card: Card<French, French> = "A♥".parse::<Card<French, French>>().unwrap();
    ///
    /// assert_eq!("A♥".red().to_string(), card.to_color_symbol_string());
    ///
    /// ```
    #[must_use]
    pub fn to_color_symbol_string(&self) -> String {
        let suit_char = self.suit.index;
        if let Some(color) = SuitType::colors().get(&suit_char) {
            match color {
                colored::Color::Red => self.to_string().red().to_string(),
                colored::Color::Blue => self.to_string().blue().to_string(),
                colored::Color::Green => self.to_string().green().to_string(),
                colored::Color::Yellow => self.to_string().yellow().to_string(),
                colored::Color::Magenta => self.to_string().magenta().to_string(),
                colored::Color::Cyan => self.to_string().cyan().to_string(),
                colored::Color::BrightBlack => self.to_string().bright_black().to_string(),
                colored::Color::BrightRed => self.to_string().bright_red().to_string(),
                colored::Color::BrightGreen => self.to_string().bright_green().to_string(),
                colored::Color::BrightYellow => self.to_string().bright_yellow().to_string(),
                colored::Color::BrightBlue => self.to_string().bright_blue().to_string(),
                colored::Color::BrightMagenta => self.to_string().bright_magenta().to_string(),
                colored::Color::BrightCyan => self.to_string().bright_cyan().to_string(),
                _ => self.to_string(),
            }
        } else {
            self.to_string()
        }
    }

    // #[must_use]
    // pub fn suit_symbol(&self) -> String {
    //     self.suit.symbol()
    // }
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

/// Reversing the sort order of the `Cards` so that the highest is first.
///
/// Sort is prioritized by `Suit.weight` and then `Rank.weight`.
impl<RankType, SuitType> Ord for Card<RankType, SuitType>
where
    RankType: Ranked + Clone + Copy + PartialOrd + Ord + Default + Hash,
    SuitType: Suited + Clone + Copy + PartialOrd + Ord + Default + Hash,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .suit
            .cmp(&self.suit)
            .then_with(|| other.rank.cmp(&self.rank))
    }
}

impl<RankType, SuitType> PartialOrd for Card<RankType, SuitType>
where
    RankType: Ranked + Clone + Copy + PartialOrd + Ord + Default + Hash,
    SuitType: Suited + Clone + Copy + PartialOrd + Ord + Default + Hash,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod card_tests {
    use super::*;
    use crate::refact::decks::French;
    use crate::refact::decks::FrenchCard;

    #[test]
    fn card__sort() {
        let deuce_of_clubs = Card::<French, French>::new(French::DEUCE, French::CLUBS);
        let trey_of_diamonds = Card::<French, French>::new(French::TREY, French::DIAMONDS);
        let four_of_clubs = Card::<French, French>::new(French::FOUR, French::CLUBS);

        let mut cards = vec![deuce_of_clubs, trey_of_diamonds, four_of_clubs];
        cards.sort();

        let expected = vec![trey_of_diamonds, four_of_clubs, deuce_of_clubs];

        assert_eq!(cards, expected);
    }

    #[test]
    fn card__is_blank() {
        let card = FrenchCard::default();

        assert!(card.is_blank());
    }

    #[test]
    fn card__to_color_symbol_string__default() {
        let card = FrenchCard::from_str("AS").unwrap();

        assert_eq!(card.to_color_symbol_string(), "A♠");
    }

    #[test]
    fn card__display() {
        let card = FrenchCard::new(French::DEUCE, French::CLUBS);

        assert_eq!(card.to_string(), "2♣");
    }

    #[test]
    fn card__from_str() {
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
    fn card__from_str__error() {
        assert!("2S♣".parse::<Card<French, French>>().is_err());
    }
}
