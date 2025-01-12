use crate::decks::FluentName;
use std::hash::Hash;
use crate::refact::{Card, Pile, Rank, Suit};

pub trait Decked<
    RankType: Ranked + Clone + Copy + PartialOrd + Ord + Default + Hash,
    SuitType: Suited + Clone + Copy + PartialOrd + Ord + Default + Hash,
>
{
    fn deck() -> Pile<RankType, SuitType> {
        let mut pile = Pile::<RankType, SuitType>::default();

        for suit_char in SuitType::suit_indexes() {
            for rank_char in RankType::rank_indexes() {

                let suit = SuitType::from(suit_char);
                let rank = RankType::from(rank_char);

                let card = Card::<RankType, SuitType> {
                    suit,
                    rank,
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
