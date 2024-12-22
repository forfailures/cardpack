use crate::types::card_error::CardError;
use crate::types::pile::Pile;
use crate::types::traits::{Decked, Ranked, Suited};
use colored::Color;
use std::collections::HashMap;
use std::str::FromStr;

#[macro_export]
#[allow(clippy::pedantic)]
macro_rules! skat_card {
    ($card_str:expr) => {
        Card::<Skat, Skat>::from_str($card_str).unwrap_or_else(|_| Card::<Skat, Skat>::default())
    };
}

#[macro_export]
#[allow(clippy::pedantic)]
macro_rules! skat {
    ($card_str:expr) => {
        Pile::<Skat, Skat>::from_str($card_str)
    };
}

/// Skat is a German, trick based card game for three players.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Skat {}

impl Skat {
    pub const DECK_NAME: &'static str = "Skat";

    // Skat Deck Ranks:
    pub const DAUS: &'static str = "daus";
    pub const ZHEN: &'static str = "zhen";
    pub const KÖNIG: &'static str = "konig";
    pub const OBER: &'static str = "ober";
    pub const UNTER: &'static str = "unter";
    pub const NEUN: &'static str = "neun";
    pub const ACHT: &'static str = "acht";
    pub const SIEBEN: &'static str = "sieben";

    // Skat Suit Fluent Identifiers
    pub const EICHEL: &'static str = "eichel"; // Acorns
    pub const LAUB: &'static str = "laub"; // Leaves
    pub const HERZ: &'static str = "herz"; // Hearts
    pub const SHELLEN: &'static str = "schellen"; // Bells

    /// # Errors
    ///
    /// Returns a `CardError` if the index is out of bounds.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(index: &str) -> Result<Pile<Skat, Skat>, CardError> {
        Pile::<Skat, Skat>::from_str(index)
    }
}

impl Decked<Skat, Skat> for Skat {
    fn pack(&self) -> Pile<Skat, Skat> {
        Skat::deck()
    }
}

impl Ranked for Skat {
    fn rank_chars() -> Vec<char> {
        vec![
            '7', '8', '9', 'T', 't', '0', 'U', 'u', 'O', 'o', 'K', 'k', 'D', 'd',
        ]
    }

    fn rank_names() -> Vec<&'static str> {
        vec![
            Skat::DAUS,
            Skat::ZHEN,
            Skat::KÖNIG,
            Skat::OBER,
            Skat::UNTER,
            Skat::NEUN,
            Skat::ACHT,
            Skat::SIEBEN,
        ]
    }

    fn type_name() -> &'static str {
        Skat::DECK_NAME
    }
}

impl Suited for Skat {
    fn colors() -> HashMap<char, Color> {
        let mut mappie = HashMap::new();

        mappie.insert('L', Color::Green);
        mappie.insert('H', Color::Red);
        mappie.insert('S', Color::BrightBlue);

        mappie
    }

    fn suit_chars() -> Vec<char> {
        vec![
            '♧', '♣', 'E', 'e', '♤', '♠', 'L', 'l', '♡', '♥', 'H', 'h', '♢', '♦', 'S', 's',
        ]
    }

    fn suit_names() -> Vec<&'static str> {
        vec![Skat::EICHEL, Skat::LAUB, Skat::HERZ, Skat::SHELLEN]
    }

    fn type_name() -> &'static str {
        Skat::DECK_NAME
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod decks__skat__tests {
    use super::*;
    use crate::types::card::Card;
    use crate::types::rank::Rank;
    use crate::types::suit::Suit;
    use rstest::rstest;

    #[test]
    fn rank__new_with_weight() {
        let rank = Rank::<Skat>::new_with_weight(Skat::DAUS, 20);

        assert_eq!(rank.weight, 20);
    }

    #[test]
    fn rank__update_weight() {
        let rank = Rank::<Skat>::new(Skat::DAUS);
        let updated_rank = rank.update_weight(21);

        assert_eq!(updated_rank.weight, 21);
    }

    #[test]
    fn pile__sort() {
        let deck = Skat::deck();
        let mut shuffled = deck.shuffle_default();

        shuffled.shuffle_in_place_default();
        shuffled.sort_in_place();

        assert_eq!(deck.to_string(), shuffled.to_string());
    }

    #[test]
    fn from_str() {
        let card = skat_card!("D♣");

        assert_eq!(card.rank.name.to_string(), Skat::DAUS);
        assert_eq!(card.suit.name.to_string(), Skat::EICHEL);
    }

    #[test]
    fn ranked__from() {
        let rank = Rank::<Skat>::from('D');

        assert_eq!(rank.name.to_string(), Skat::DAUS);
    }

    #[rstest]
    #[case('D', Skat::DAUS)]
    #[case('d', Skat::DAUS)]
    #[case('T', Skat::ZHEN)]
    #[case('t', Skat::ZHEN)]
    #[case('0', Skat::ZHEN)]
    #[case('K', Skat::KÖNIG)]
    #[case('k', Skat::KÖNIG)]
    #[case('O', Skat::OBER)]
    #[case('o', Skat::OBER)]
    #[case('U', Skat::UNTER)]
    #[case('u', Skat::UNTER)]
    #[case('9', Skat::NEUN)]
    #[case('8', Skat::ACHT)]
    #[case('7', Skat::SIEBEN)]
    fn rank__from__char(#[case] input: char, #[case] expected: &str) {
        assert_eq!(Rank::<Skat>::new(expected), Rank::<Skat>::from(input));
    }

    #[test]
    fn suited__is_valid_suit_char() {
        assert!(Suit::<Skat>::is_valid_suit_char(&'♧'));
        assert!(Suit::<Skat>::is_valid_suit_char(&'♣'));
        assert!(Suit::<Skat>::is_valid_suit_char(&'E'));
        assert!(Suit::<Skat>::is_valid_suit_char(&'e'));
        assert!(Suit::<Skat>::is_valid_suit_char(&'♤'));
        assert!(Suit::<Skat>::is_valid_suit_char(&'♠'));
        assert!(Suit::<Skat>::is_valid_suit_char(&'L'));
        assert!(Suit::<Skat>::is_valid_suit_char(&'l'));
        assert!(Suit::<Skat>::is_valid_suit_char(&'H'));
        assert!(Suit::<Skat>::is_valid_suit_char(&'h'));
        assert!(Suit::<Skat>::is_valid_suit_char(&'♥'));
        assert!(Suit::<Skat>::is_valid_suit_char(&'♡'));
        assert!(Suit::<Skat>::is_valid_suit_char(&'♦'));
        assert!(Suit::<Skat>::is_valid_suit_char(&'♢'));
        assert!(Suit::<Skat>::is_valid_suit_char(&'S'));
        assert!(Suit::<Skat>::is_valid_suit_char(&'s'));
        assert!(!Suit::<Skat>::is_valid_suit_char(&'_'));
        assert!(!Suit::<Skat>::is_valid_suit_char(&'W'));
    }

    /// In trying to get this test to pass I've realized that I need
    /// to give every Rank and Suit its own unique name, that doesn't
    /// overlap with the Standard52 names, so that I can ensure proper
    /// sorting.
    #[test]
    fn to_string__from_str() {
        let deck = Skat::deck();
        let shuffled = deck.shuffle_default().to_string();
        let parsed = Skat::from_str(&shuffled).unwrap();

        assert!(deck.same(&parsed));
    }
}
