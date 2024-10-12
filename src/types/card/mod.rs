use std::str::FromStr;
use crate::types::rank::Rank;
use crate::types::suit::Suit;
use crate::types::{Ranked, Suited};
use crate::types::card_error::CardError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card<RankType, SuitType>
where
    RankType: Ranked,
    SuitType: Suited,
{
    pub rank: Rank<RankType>,
    pub suit: Suit<SuitType>,
}

impl<RankType, SuitType> Card<RankType, SuitType>
where
    RankType: Ranked,
    SuitType: Suited,
{
    #[must_use]
    pub fn new(rank: Rank<RankType>, suit: Suit<SuitType>) -> Self {
        Self { rank, suit }
    }
}

impl<RankType: Ranked, SuitType: Suited> FromStr for Card<RankType, SuitType> {
    type Err = CardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();;

        todo!()
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
    fn from_str() {
        let ace = Rank::<Standard52>::from('A');
        let spades = Suit::<Standard52>::from('S');
        let expected_card: Card<Standard52, Standard52> = Card::new(ace, spades);

        let card  = Card::<Standard52, Standard52>::from_str("AS").unwrap();
    }
}
