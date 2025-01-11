use crate::decks::FluentName;
use std::hash::Hash;

pub trait Decked<
    RankType: Ranked + Clone + Copy + PartialOrd + Ord + Default + Hash,
    SuitType: Suited + Clone + Copy + PartialOrd + Ord + Default + Hash,
>
{
}

pub trait Ranked {
    fn get_rank_fluent_name(index: char) -> FluentName;

    fn rank_indexes() -> Vec<char>;
}

pub trait Suited {
    fn get_suit_fluent_name(index: char) -> FluentName;

    fn get_suit_symbol(index: char) -> char;

    fn suit_indexes() -> Vec<char>;

    // fn suits() -> Vec<Self> where Self: std::marker::Sized {
    //     Self::suit_indexes()
    //         .iter()
    //         .map(|index| Self::from(*index))
    //         .collect()
    // }
}
