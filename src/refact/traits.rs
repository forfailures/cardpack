use crate::decks::FluentName;
use crate::refact::pips::{Rank, Suit};
use crate::refact::{Card, Pile};
use std::collections::HashMap;
use std::hash::Hash;

pub trait Decked<
    RankType: Ranked + Clone + Copy + PartialOrd + Ord + Default + Hash,
    SuitType: Suited + Clone + Copy + PartialOrd + Ord + Default + Hash,
>
{
    /// ```
    /// use cardpack::refactored::*;
    ///
    /// let deck = French::deck();
    ///
    /// assert_eq!(deck.len(), 52);
    /// assert_eq!(deck.to_string(), "A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣");
    /// ```
    #[must_use]
    fn deck() -> Pile<RankType, SuitType> {
        let mut pile = Pile::<RankType, SuitType>::default();

        for suit_char in SuitType::suit_indexes() {
            for rank_char in RankType::rank_indexes() {
                let suit = <Self as Decked<RankType, SuitType>>::get_suit(suit_char);
                let rank = <Self as Decked<RankType, SuitType>>::get_rank(rank_char);

                let card = Card::<RankType, SuitType> { suit, rank };

                pile.push(card);
            }
        }

        pile
    }

    #[must_use]
    fn decks(n: usize) -> Pile<RankType, SuitType> {
        Pile::<RankType, SuitType>::pile_up(n, <Self as Decked<RankType, SuitType>>::deck)
    }

    fn get_rank(index_char: char) -> Rank<RankType>;
    fn get_suit(index_char: char) -> Suit<SuitType>;
}

pub trait Ranked {
    fn get_rank_fluent_name(c: char) -> FluentName;

    fn get_rank_index(c: char) -> char;

    fn get_rank_weight(c: char) -> u32;

    fn rank_indexes() -> Vec<char>;
}

pub trait Suited {
    /// Color rendering of `Cards` is determined by their suit.
    fn colors() -> HashMap<char, colored::Color>;

    fn get_suit_fluent_name(c: char) -> FluentName;

    fn get_suit_index(c: char) -> char;

    fn get_suit_symbol(c: char) -> char;

    fn get_suit_weight(c: char) -> u32;

    fn suit_indexes() -> Vec<char>;
}
