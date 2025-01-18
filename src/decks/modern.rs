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
    pub const BIG: &'static str = "big-joker";
    pub const LITTLE: &'static str = "little-joker";
}
