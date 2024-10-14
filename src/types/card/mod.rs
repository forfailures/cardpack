use crate::localization::Named;
use crate::types::card_error::CardError;
use crate::types::rank::Rank;
use crate::types::suit::Suit;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Card<RankType, SuitType>
where
    RankType: Ranked + Clone,
    SuitType: Suited + Clone,
{
    /// Used to sort Cards.
    pub weight: u32,
    /// The identity indicator in the corner of a playing card, such as `AS` for ace of spades.
    pub index: String,
    pub suit: Suit<SuitType>,
    pub rank: Rank<RankType>,
}
use crate::types::{Ranked, Suited};

use std::str::FromStr;

impl<RankType, SuitType> Card<RankType, SuitType>
where
    RankType: Ranked + Clone,
    SuitType: Suited + Clone,
{
    #[must_use]
    pub fn new(rank: Rank<RankType>, suit: Suit<SuitType>) -> Self {
        Self {
            weight: Self::determine_weight(&suit, &rank),
            index: Card::determine_default_index(&suit, &rank),
            suit,
            rank,
        }
    }

    // Private methods

    /// The index is the most boring way to represent a `Card` as a `String` using
    /// only basic characters. Used to set `Card.index` when `Card::new()` is called.
    /// For example, the Jack of Diamonds index value is `JD`, while it's display
    /// value is `J♦`:
    ///
    /// ```rust
    /// use std::str::FromStr;
    /// use cardpack::decks::standard52::Standard52;
    /// use cardpack::types::card::Card;
    ///
    /// let jack_of_diamonds = Card::<Standard52, Standard52>::from_str("jd").unwrap();
    ///
    /// assert_eq!(jack_of_diamonds.index, "JD");
    /// assert_eq!(jack_of_diamonds.to_string(), "J♦");
    /// ```
    fn determine_default_index(suit: &Suit<SuitType>, rank: &Rank<RankType>) -> String {
        let rank = rank.index_default();
        let suit = suit.index_default();
        format!("{rank}{suit}")
    }

    /// Prioritizes sorting by Suit and then by Rank.
    ///
    /// I'm going to be lazy and trust the new test to test this function as part
    /// of its testing tested test.
    fn determine_weight(suit: &Suit<SuitType>, rank: &Rank<RankType>) -> u32 {
        (suit.weight * 1000) + rank.weight
    }

    #[must_use]
    pub fn is_blank(&self) -> bool {
        self.rank.name.is_blank() || self.suit.name.is_blank()
    }
}

impl<RankType: Ranked + Clone, SuitType: Suited + Clone> Display
    for Card<RankType, SuitType>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_blank() {
            write!(f, "__")
        } else {
            write!(f, "{}{}", self.rank, self.suit)
        }
    }
}

impl<RankType: Ranked + Clone, SuitType: Suited + Clone> FromStr for Card<RankType, SuitType> {
    type Err = CardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.len() != 2 {
            return Err(CardError::InvalidIndex(s.to_string()));
        }
        if let Some(c) = s.chars().next() {
            let rank = Rank::<RankType>::from(c);
            if let Some(c) = s.chars().last() {
                let suit = Suit::<SuitType>::from(c);
                return Ok(Card::new(rank, suit));
            }
        }

        Err(CardError::Fubar)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod types__card__tests {
    use super::*;
    use crate::decks::standard52::Standard52;

    #[test]
    fn new() {
        let expected: Card<Standard52, Standard52> = Card {
            weight: 4012,
            index: "AS".to_string(),
            rank: Rank::<Standard52>::from('A'),
            suit: Suit::<Standard52>::from('S'),
        };

        let ace = Rank::<Standard52>::from('A');
        let spades = Suit::<Standard52>::from('S');
        let card: Card<Standard52, Standard52> = Card::new(ace, spades);

        assert_eq!(card, expected);
    }

    /// I want to make sure that the weight field in the `Card` struct correctly affects the sorting
    /// of cards. For Decks of cards, the sort would be in reverse, with the higher weighted `Card`
    /// coming first.
    #[test]
    fn test_sort_from_weight() {
        let ace_of_spades = Card::<Standard52, Standard52>::from_str("AS").unwrap();
        let ace_of_hearts = Card::<Standard52, Standard52>::from_str("AH").unwrap();
        let ace_of_diamonds = Card::<Standard52, Standard52>::from_str("AD").unwrap();
        let ace_of_clubs = Card::<Standard52, Standard52>::from_str("AC").unwrap();

        let mut cards = vec![
            ace_of_clubs.clone(),
            ace_of_hearts.clone(),
            ace_of_spades.clone(),
            ace_of_diamonds.clone(),
        ];

        cards.sort();

        assert_eq!(cards[0], ace_of_clubs);
        assert_eq!(cards[1], ace_of_diamonds);
        assert_eq!(cards[2], ace_of_hearts);
        assert_eq!(cards[3], ace_of_spades);
    }

    #[test]
    fn display() {
        let card = Card::<Standard52, Standard52>::from_str("KD").unwrap();

        assert_eq!("K♦", format!("{card}"));
    }

    #[test]
    fn from_str() {
        let ace = Rank::<Standard52>::from('A');
        let spades = Suit::<Standard52>::from('S');
        let expected_card: Card<Standard52, Standard52> = Card::new(ace, spades);

        let card = Card::<Standard52, Standard52>::from_str("  AS   ").unwrap();

        assert_eq!(card, expected_card);
        assert!(!card.is_blank());
    }

    #[test]
    fn from_str_blank() {
        let card = Card::<Standard52, Standard52>::from_str(" BW ").unwrap();

        assert!(card.is_blank());
    }
}
