use crate::old::decks::modern::Modern;
use crate::old::types::card::Card;
use crate::old::types::pile::Pile;
use crate::old::types::traits::Decked;
use crate::types::errors::CardError;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HandAndFoot {}

impl HandAndFoot {
    pub const DECK_NAME: &'static str = "Hand and Foot";

    /// # Errors
    ///
    /// Returns a `CardError` if the index is out of bounds.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(index: &str) -> Result<Pile<Modern, Modern>, CardError> {
        Pile::<Modern, Modern>::from_str(index)
    }
}

impl Decked<Modern, Modern> for HandAndFoot {
    fn deck() -> Pile<Modern, Modern> {
        Modern::decks(5).sort()
    }

    fn blank() -> Card<Modern, Modern> {
        Card::<Modern, Modern>::default()
    }

    fn guide() -> Option<String> {
        None
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod decks__hand_and_foot__tests {
    use super::*;

    #[test]
    fn deck() {
        let deck = HandAndFoot::deck();

        assert_eq!(deck.len(), 270);
    }

    #[test]
    fn pile__sort() {
        let deck = HandAndFoot::deck();
        let mut shuffled = deck.shuffle();

        shuffled.shuffle_in_place();
        shuffled.sort_in_place();

        assert_eq!(deck.to_string(), shuffled.to_string());
    }

    #[test]
    fn to_string__from_str() {
        let deck = HandAndFoot::deck();
        let shuffled = deck.shuffle().to_string();
        let parsed = HandAndFoot::from_str(&shuffled).unwrap();

        assert!(deck.same(&parsed));
    }
}