use std::marker::PhantomData;
use crate::prelude::{French, Suit};
// use crate::types::pips::{Rank, Suit, BLANK};
// use crate::types::traits::{Decked, Ranked, Suited};
use crate::types::{Card, Pile};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Modern {}

#[allow(clippy::module_name_repetitions)]
pub type ModernCard = Card<Modern, Modern>;
#[allow(clippy::module_name_repetitions)]
pub type ModernDeck = Pile<Modern, Modern>;

impl Modern {
    // Jokers Fluent Names
    pub const FLUENT_KEY_BIG: &'static str = "big-joker";
    pub const FLUENT_KEY_LITTLE: &'static str = "little-joker";

    // Rank
    pub const FLUENT_KEY_JOKER: &'static str = "joker";

    // Suites
    pub const JOKERS_INDEX: char = 'S';
    pub const JOKERS: Suit<French> = Suit {
        weight: 5,
        index: Modern::JOKERS_INDEX,
        phantom_data: PhantomData,
    };
}
