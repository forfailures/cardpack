use crate::decks::FluentName;
use crate::refact::{Card, Pile, Rank, Suit};
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

    fn get_rank(index_char: char) -> Rank<RankType>;
    fn get_suit(index_char: char) -> Suit<SuitType>;
}

pub trait Ranked {
    fn get_rank_fluent_name(index: char) -> FluentName;

    fn rank_indexes() -> Vec<char>;
}

pub trait Suited {
    fn get_suit_fluent_name(index: char) -> FluentName;

    fn get_suit_symbol(index: char) -> char;

    fn suit_indexes() -> Vec<char>;
}
