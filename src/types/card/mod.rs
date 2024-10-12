use crate::localization::Named;
use crate::types::card_error::CardError;
use crate::types::rank::Rank;
use crate::types::suit::Suit;
use std::fmt::Display;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card<RankType, SuitType>
where
    RankType: Ranked,
    SuitType: Suited,
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
    RankType: Ranked,
    SuitType: Suited,
{
    #[must_use]
    pub fn new(rank: Rank<RankType>, suit: Suit<SuitType>) -> Self {
        Self {
            weight: 0,
            index: String::new(),
            suit,
            rank,
        }
    }

    #[must_use]
    pub fn is_blank(&self) -> bool {
        self.rank.name.is_blank() || self.suit.name.is_blank()
    }
}

impl<RankType: Ranked, SuitType: Suited> Display for Card<RankType, SuitType> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_blank() {
            write!(f, "__")
        } else {
            write!(f, "{}{}", self.rank, self.suit)
        }
    }
}

impl<RankType: Ranked, SuitType: Suited> FromStr for Card<RankType, SuitType> {
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
    use crate::localization::FluentName;

    #[test]
    fn new() {
        let ace = Rank::<Standard52>::from('A');
        let spades = Suit::<Standard52>::from('S');
        let card: Card<Standard52, Standard52> = Card::new(ace, spades);

        assert_eq!(card.rank.name, FluentName::new(Rank::<Standard52>::ACE));
        assert_eq!(card.suit.name, FluentName::new(Suit::<Standard52>::SPADES));
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