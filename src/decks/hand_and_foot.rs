use crate::decks::modern::Modern;
use crate::types::card::Card;
use crate::types::card_error::CardError;
use crate::types::pile::Pile;
use crate::types::traits::Decked;
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

    fn pack(&self) -> Pile<Modern, Modern> {
        HandAndFoot::deck()
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
        let mut shuffled = deck.shuffle_default();

        shuffled.shuffle_in_place_default();
        shuffled.sort_in_place();

        assert_eq!(deck.to_string(), shuffled.to_string());
    }

    #[test]
    fn to_string__from_str() {
        let deck = HandAndFoot::deck();
        let shuffled = deck.shuffle_default().to_string();
        let parsed = HandAndFoot::from_str(&shuffled).unwrap();

        assert!(deck.same(&parsed));
    }
}
