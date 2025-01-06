use crate::decks::french::French;
use crate::types::card::Card;
use crate::types::card_error::CardError;
use crate::types::pile::Pile;
use crate::types::traits::{Decked, Ranked};
use std::str::FromStr;

/// [Manila, aka Six Plus aka Short-deck](https://en.wikipedia.org/wiki/Six-plus_hold_%27em)
/// is a version of Texas Hold'em where the card Ranks of 2 through 5
/// are removed from the deck.
///
/// This means that they are made up of the [`French`]
/// implementation of the [`Suited`](crate::types::traits::Suited) trait that's declared in the
/// [`French`] deck and the `Short` implementation of the
/// [`Ranked`] trait.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Short {}

#[allow(clippy::module_name_repetitions)]
pub type ShortCard = Card<Short, French>;
#[allow(clippy::module_name_repetitions)]
pub type ShortDeck = Pile<Short, French>;

impl Short {
    pub const DECK_NAME: &'static str = "Short";

    /// # Errors
    ///
    /// Returns a `CardError` if the index is out of bounds.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(index: &str) -> Result<Pile<Short, French>, CardError> {
        Pile::<Short, French>::from_str(index)
    }
}

impl Decked<Short, French> for Short {
    fn blank() -> Card<Short, French> {
        Card::<Short, French>::default()
    }

    fn guide() -> Option<String> {
        None
    }
}

impl Ranked for Short {
    fn rank_chars() -> Vec<char> {
        vec![
            '6', '7', '8', '9', 'T', 't', 'J', 'j', 'Q', 'q', 'K', 'k', 'A', 'a',
        ]
    }

    fn rank_names() -> Vec<&'static str> {
        vec![
            French::ACE,
            French::KING,
            French::QUEEN,
            French::JACK,
            French::TEN,
            French::NINE,
            French::EIGHT,
            French::SEVEN,
            French::SIX,
        ]
    }

    fn type_name() -> &'static str {
        Short::DECK_NAME
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod decks__manila__tests {
    use super::*;
    use crate::types::rank::Rank;

    #[test]
    fn deck() {
        let deck = Short::deck();
        assert_eq!(deck.len(), 36);
        assert_eq!(deck.to_string(), "A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣");
    }

    #[test]
    fn rank_chars() {
        assert_eq!(
            Rank::<Short>::rank_chars(),
            vec!['6', '7', '8', '9', 'T', 't', 'J', 'j', 'Q', 'q', 'K', 'k', 'A', 'a']
        );
    }

    #[test]
    fn pile__sort() {
        let deck = Short::deck();
        let mut shuffled = deck.shuffle();

        shuffled.shuffle_in_place();
        shuffled.sort_in_place();

        assert_eq!(deck.to_string(), shuffled.to_string());
    }

    #[test]
    fn to_string__from_str() {
        let deck = Short::deck();
        let shuffled = deck.shuffle().to_string();
        let parsed = Short::from_str(&shuffled).unwrap();

        assert!(deck.same(&parsed));
    }
}
