use crate::modern_card;
use crate::old::decks::french::French;
use crate::old::decks::modern::Modern;
use crate::old::types::card::Card;
use crate::old::types::pile::Pile;
use crate::old::types::rank::Rank;
use crate::old::types::suit::Suit;
use crate::old::types::traits::Decked;
use crate::types::errors::CardError;
use std::str::FromStr;

#[macro_export]
#[allow(clippy::pedantic)]
macro_rules! spades_card {
    ($card_str:expr) => {
        Spades::card($card_str)
    };
}

#[macro_export]
macro_rules! spades {
    ($card_str:expr) => {
        Spades::from_str($card_str)
    };
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Spades {}

impl Spades {
    pub const DECK_NAME: &'static str = "Spades";

    #[allow(dead_code)]
    fn card(s: &str) -> Card<Modern, Modern> {
        let card = modern_card!(s);

        match card.index.as_str() {
            "2C" | "2D" => Card::<Modern, Modern>::default(),
            _ => card,
        }
    }

    /// # Errors
    ///
    /// Returns a `CardError` if the index is out of bounds.
    ///
    /// TODO: Add deck validation
    #[allow(dead_code, clippy::should_implement_trait)]
    fn from_str(index: &str) -> Result<Pile<Modern, Modern>, CardError> {
        let pile = Pile::<Modern, Modern>::from_str(index)?;

        if pile.contains(&modern_card!("2C")) {
            return Err(CardError::InvalidCard(
                "2♣ is not in a Spades deck".to_string(),
            ));
        }

        if pile.contains(&modern_card!("2D")) {
            return Err(CardError::InvalidCard(
                "2♦ is not in a Spades deck".to_string(),
            ));
        }

        Ok(pile)
    }
}

impl Decked<Modern, Modern> for Spades {
    fn deck() -> Pile<Modern, Modern> {
        let mut deck = Modern::deck();

        let two_clubs = Card::new(
            Rank::<Modern>::new(French::TWO),
            Suit::<Modern>::new(French::CLUBS),
        );
        let two_diamonds = Card::new(
            Rank::<Modern>::new(French::TWO),
            Suit::<Modern>::new(French::DIAMONDS),
        );

        deck.remove_card(&two_clubs).unwrap();
        deck.remove_card(&two_diamonds).unwrap();

        deck
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
mod decks__spades__tests {
    use super::*;

    #[test]
    fn deck() {
        let deck = Spades::deck();

        assert_eq!(deck.len(), 52);
    }

    #[test]
    fn spades_card() {
        assert_eq!(modern_card!("2H"), spades_card!("2H"));
        assert_eq!(Card::<Modern, Modern>::default(), spades_card!("2C"));
        assert_eq!(Card::<Modern, Modern>::default(), spades_card!("2D"));
    }

    #[test]
    fn from_str() {
        assert!(spades!("2H").is_ok());
        assert!(Spades::from_str("2C").is_err());
        assert!(Spades::from_str("2D").is_err());
        assert!(Spades::from_str("2H 2C").is_err());
    }

    #[test]
    fn pile__sort() {
        let deck = Spades::deck();
        let mut shuffled = deck.shuffle();

        shuffled.shuffle_in_place();
        shuffled.sort_in_place();

        assert_eq!(deck.to_string(), shuffled.to_string());
    }

    #[test]
    fn to_string__from_str() {
        let deck = Spades::deck();
        let shuffled = deck.shuffle().to_string();
        let parsed = Spades::from_str(&shuffled).unwrap();

        assert!(deck.same(&parsed));
    }
}
