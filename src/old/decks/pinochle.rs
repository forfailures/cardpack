use crate::old::decks::french::French;
use crate::old::types::card::Card;
use crate::old::types::pile::Pile;
use crate::old::types::rank::Rank;
use crate::old::types::suit::Suit;
use crate::old::types::traits::{Decked, Ranked};
use crate::types::errors::CardError;
use std::str::FromStr;

#[macro_export]
#[allow(clippy::module_name_repetitions)]
macro_rules! pinochle_card {
    (AS) => {
        Card::<Pinochle, French>::new(
            Rank::<Pinochle>::new(Pinochle::ACE),
            Suit::<French>::new(French::SPADES),
        )
    };
    (TS) => {
        Card::<Pinochle, French>::new(
            Rank::<Pinochle>::new(Pinochle::TEN),
            Suit::<French>::new(French::SPADES),
        )
    };
    (KS) => {
        Card::<Pinochle, French>::new(
            Rank::<Pinochle>::new(Pinochle::KING),
            Suit::<French>::new(French::SPADES),
        )
    };
    (QS) => {
        Card::<Pinochle, French>::new(
            Rank::<Pinochle>::new(Pinochle::QUEEN),
            Suit::<French>::new(French::SPADES),
        )
    };
    (JS) => {
        Card::<Pinochle, French>::new(
            Rank::<Pinochle>::new(Pinochle::JACK),
            Suit::<French>::new(French::SPADES),
        )
    };
    (9S) => {
        Card::<Pinochle, French>::new(
            Rank::<Pinochle>::new(Pinochle::NINE),
            Suit::<French>::new(French::SPADES),
        )
    };
    (AH) => {
        Card::<Pinochle, French>::new(
            Rank::<Pinochle>::new(Pinochle::ACE),
            Suit::<French>::new(French::HEARTS),
        )
    };
    (TH) => {
        Card::<Pinochle, French>::new(
            Rank::<Pinochle>::new(Pinochle::TEN),
            Suit::<French>::new(French::HEARTS),
        )
    };
    (KH) => {
        Card::<Pinochle, French>::new(
            Rank::<Pinochle>::new(Pinochle::KING),
            Suit::<French>::new(French::HEARTS),
        )
    };
    (QH) => {
        Card::<Pinochle, French>::new(
            Rank::<Pinochle>::new(Pinochle::QUEEN),
            Suit::<French>::new(French::HEARTS),
        )
    };
    (JH) => {
        Card::<Pinochle, French>::new(
            Rank::<Pinochle>::new(Pinochle::JACK),
            Suit::<French>::new(French::HEARTS),
        )
    };
    (9H) => {
        Card::<Pinochle, French>::new(
            Rank::<Pinochle>::new(Pinochle::NINE),
            Suit::<French>::new(French::HEARTS),
        )
    };
    (AD) => {
        Card::<Pinochle, French>::new(
            Rank::<Pinochle>::new(Pinochle::ACE),
            Suit::<French>::new(French::DIAMONDS),
        )
    };
    (TD) => {
        Card::<Pinochle, French>::new(
            Rank::<Pinochle>::new(Pinochle::TEN),
            Suit::<French>::new(French::DIAMONDS),
        )
    };
    (KD) => {
        Card::<Pinochle, French>::new(
            Rank::<Pinochle>::new(Pinochle::KING),
            Suit::<French>::new(French::DIAMONDS),
        )
    };
    (QD) => {
        Card::<Pinochle, French>::new(
            Rank::<Pinochle>::new(Pinochle::QUEEN),
            Suit::<French>::new(French::DIAMONDS),
        )
    };
    (JD) => {
        Card::<Pinochle, French>::new(
            Rank::<Pinochle>::new(Pinochle::JACK),
            Suit::<French>::new(French::DIAMONDS),
        )
    };
    (9D) => {
        Card::<Pinochle, French>::new(
            Rank::<Pinochle>::new(Pinochle::NINE),
            Suit::<French>::new(French::DIAMONDS),
        )
    };
    (AC) => {
        Card::<Pinochle, French>::new(
            Rank::<Pinochle>::new(Pinochle::ACE),
            Suit::<French>::new(French::CLUBS),
        )
    };
    (TC) => {
        Card::<Pinochle, French>::new(
            Rank::<Pinochle>::new(Pinochle::TEN),
            Suit::<French>::new(French::CLUBS),
        )
    };
    (KC) => {
        Card::<Pinochle, French>::new(
            Rank::<Pinochle>::new(Pinochle::KING),
            Suit::<French>::new(French::CLUBS),
        )
    };
    (QC) => {
        Card::<Pinochle, French>::new(
            Rank::<Pinochle>::new(Pinochle::QUEEN),
            Suit::<French>::new(French::CLUBS),
        )
    };
    (JC) => {
        Card::<Pinochle, French>::new(
            Rank::<Pinochle>::new(Pinochle::JACK),
            Suit::<French>::new(French::CLUBS),
        )
    };
    (9C) => {
        Card::<Pinochle, French>::new(
            Rank::<Pinochle>::new(Pinochle::NINE),
            Suit::<French>::new(French::CLUBS),
        )
    };
    (__) => {
        Card::<Pinochle, French>::default()
    };
}

/// The Pinochle deck is made up of French deck suites, with custom ranks, based on the French,
/// that go from nine to ace, and where the ten is the second-highest card in the deck. Each card
/// is included twice, creating a deck of 48 cards.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pinochle {}

/// ```
/// use cardpack::old_prelude::*;
///
/// let card1 = PinochleCard::new(Rank::<Pinochle>::new(Pinochle::ACE), Suit::<French>::new(French::SPADES));
/// let card2 = Card::<Pinochle, French>::new(Rank::<Pinochle>::new(Pinochle::ACE), Suit::<French>::new(French::SPADES));
///
/// assert_eq!(card1, card2);
/// assert_eq!(card1, pinochle_card!(AS));
/// ```
#[allow(clippy::module_name_repetitions)]
pub type PinochleCard = Card<Pinochle, French>;
#[allow(clippy::module_name_repetitions)]
pub type PinochleDeck = Pile<Pinochle, French>;

impl Pinochle {
    pub const DECK_NAME: &'static str = "Pinochle";

    /// Constants representing the fluid template names for the [`Ranks`](Rank). `Pinochle` has separate ones for the ranks
    /// because they have a different order than the [`French`] deck.
    ///
    /// You can see this in action when we sort the same hands in both decks:
    ///
    /// ```
    /// use cardpack::old_prelude::*;
    ///
    /// let french_hand = French::from_str("AS KS QS JS TS").unwrap().sort();
    /// let pinochle_hand = Pinochle::from_str("AS KS QS JS TS").unwrap().sort();
    ///
    /// assert_eq!("A♠ K♠ Q♠ J♠ T♠", french_hand.to_string());
    /// assert_eq!("A♠ T♠ K♠ Q♠ J♠", pinochle_hand.to_string());
    /// ```
    pub const ACE: &'static str = "pinochle-ace";
    pub const TEN: &'static str = "pinochle-ten";
    pub const KING: &'static str = "pinochle-king";
    pub const QUEEN: &'static str = "pinochle-queen";
    pub const JACK: &'static str = "pinochle-jack";
    pub const NINE: &'static str = "pinochle-nine";

    /// # Errors
    ///
    /// Returns a `CardError` if the index is out of bounds.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(index: &str) -> Result<Pile<Pinochle, French>, CardError> {
        Pile::<Pinochle, French>::from_str(index)
    }
}

impl Decked<Pinochle, French> for Pinochle {
    /// This function creates a deck of cards for the game of Pinochle.
    ///
    /// ```
    /// use cardpack::old_prelude::*;
    ///
    /// let deck = Pinochle::deck();
    /// let expected = "A♠ A♠ T♠ T♠ K♠ K♠ Q♠ Q♠ J♠ J♠ 9♠ 9♠ A♥ A♥ T♥ T♥ K♥ K♥ Q♥ Q♥ J♥ J♥ 9♥ 9♥ A♦ A♦ T♦ T♦ K♦ K♦ Q♦ Q♦ J♦ J♦ 9♦ 9♦ A♣ A♣ T♣ T♣ K♣ K♣ Q♣ Q♣ J♣ J♣ 9♣ 9♣";
    ///
    /// assert_eq!(deck.len(), 48);
    /// assert_eq!(deck.to_string(), expected);
    /// assert!(deck.same(&deck.shuffle()));
    /// ```
    #[must_use]
    fn deck() -> Pile<Pinochle, French> {
        let ranks = Rank::<Pinochle>::ranks_from_array(&Pinochle::rank_names());
        let suits = Suit::<French>::suits();

        let mut pile = Pile::<Pinochle, French>::from(Vec::new());

        for suit in &suits {
            for rank in &ranks {
                let card = Card::<Pinochle, French>::new(rank.clone(), suit.clone());
                pile.push(card.clone());
                pile.push(card);
            }
        }

        pile
    }

    fn blank() -> Card<Pinochle, French> {
        Card::<Pinochle, French>::default()
    }

    fn guide() -> Option<String> {
        None
    }
}

impl Ranked for Pinochle {
    fn rank_chars() -> Vec<char> {
        vec!['9', 'T', 't', '0', 'J', 'j', 'Q', 'q', 'K', 'k', 'A', 'a']
    }

    fn rank_names() -> Vec<&'static str> {
        vec![
            Pinochle::ACE,
            Pinochle::TEN,
            Pinochle::KING,
            Pinochle::QUEEN,
            Pinochle::JACK,
            Pinochle::NINE,
        ]
    }

    fn type_name() -> &'static str {
        Pinochle::DECK_NAME
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod decks__pinochle__tests {
    use super::*;
    use crate::localization::Named;
    use crate::old::types::rank::Rank;

    #[test]
    fn rank__new_with_weight() {
        let rank = Rank::<Pinochle>::new_with_weight("ace", 20);

        assert_eq!(rank.weight, 20);
        assert_eq!(rank.prime, 41);
        assert_eq!(rank.fluent_name().prime(), 41);
        assert_eq!(rank.fluent_name_string(), "ace");
    }

    #[test]
    fn rank__ranks_from_array() {
        let ranks = Rank::<Pinochle>::ranks_from_array(&*Pinochle::rank_names());

        assert_eq!(ranks.len(), 6);
        assert_eq!(ranks[0].fluent_name_string(), "pinochle-ace");
        assert_eq!(ranks[0].weight, 5);
        assert_eq!(ranks[1].fluent_name_string(), "pinochle-ten");
        assert_eq!(ranks[1].weight, 4);
    }

    #[test]
    fn decked__sort() {
        let deck = Pinochle::deck();
        let mut shuffled = deck.shuffle();
        shuffled.sort_in_place();

        let expected = "A♠ A♠ T♠ T♠ K♠ K♠ Q♠ Q♠ J♠ J♠ 9♠ 9♠ A♥ A♥ T♥ T♥ K♥ K♥ Q♥ Q♥ J♥ J♥ 9♥ 9♥ A♦ A♦ T♦ T♦ K♦ K♦ Q♦ Q♦ J♦ J♦ 9♦ 9♦ A♣ A♣ T♣ T♣ K♣ K♣ Q♣ Q♣ J♣ J♣ 9♣ 9♣";

        assert_eq!(deck.to_string(), expected);
        assert_eq!(shuffled.to_string(), expected);
    }

    #[test]
    fn pile__sort() {
        let deck = Pinochle::deck();
        let mut shuffled = deck.shuffle();

        shuffled.shuffle_in_place();
        shuffled.sort_in_place();

        assert_eq!(deck.to_string(), shuffled.to_string());
    }

    #[test]
    fn to_string__from_str() {
        let deck = Pinochle::deck();
        let shuffled = deck.shuffle().to_string();
        let parsed = Pinochle::from_str(&shuffled).unwrap();

        assert!(deck.same(&parsed));
    }
}
