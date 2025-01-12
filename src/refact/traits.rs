use crate::decks::FluentName;
use crate::refact::{Card, Pile, Rank, Suit};
use std::hash::Hash;

pub trait Decked<
    RankType: Ranked + Clone + Copy + PartialOrd + Ord + Default + Hash,
    SuitType: Suited + Clone + Copy + PartialOrd + Ord + Default + Hash,
>
{
    fn deck() -> Pile<RankType, SuitType> {
        let mut pile = Pile::<RankType, SuitType>::default();

        for suit_char in SuitType::suit_indexes() {
            for rank_char in RankType::rank_indexes() {
                let suit = Suit::<SuitType>::from(suit_char);

                let card = Card::<RankType, SuitType> {
                    suit: Suit::<SuitType>::from(suit_char),
                    rank: Rank::<RankType>::from(rank_char),
                };

                pile.push(card);
            }
        }

        pile
    }
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
