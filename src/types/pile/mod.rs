use crate::types::suit::Suit;
use crate::types::rank::Rank;
use crate::types::card::Card;
use crate::types::card_error::CardError;
use crate::types::Ranked;
use crate::types::Suited;
use std::str::FromStr;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Pile<
    RankType: Ranked + PartialOrd + Ord + Clone,
    SuitType: Suited + PartialOrd + Ord + Clone,
>(Vec<Card<RankType, SuitType>>)
where
    RankType: Ranked,
    SuitType: Suited;

impl<RankType: Ranked + Ord + Clone, SuitType: Suited + Ord + Clone> Pile<RankType, SuitType> {
    #[must_use]
    pub fn new(cards: Vec<Card<RankType, SuitType>>) -> Self {
        Self(cards)
    }

    pub fn extend(&mut self, other: &Self) {
        self.0.extend(other.0.clone());
    }

    fn fold_in(&mut self, suits: &[Suit<SuitType>], ranks: &[Rank<RankType>]) {
        for suit in suits {
            for rank in ranks {
                self.push(Card::<RankType, SuitType>::new(rank.clone(), suit.clone()));
            }
        }
    }

    #[must_use]
    pub fn get(&self, index: usize) -> Option<&Card<RankType, SuitType>> {
        self.0.get(index)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn pile_on(piles: Vec<Pile<RankType, SuitType>>) -> Self {
        let mut cards = Vec::new();
        for pile in piles {
            cards.extend(pile.0);
        }
        Self(cards)
    }

    pub fn pile_up(n: usize, f: fn() -> Vec<Card<RankType, SuitType>>) -> Self {
        let mut cards = Vec::new();
        for _ in 0..n {
            cards.extend(f());
        }
        Self(cards)
    }

    /// Places the Card at the bottom (end) of the Pile.
    pub fn push(&mut self, card: Card<RankType, SuitType>) -> bool {
        if card.is_blank() {
            false
        } else {
            self.0.push(card);
            true
        }
    }

    #[must_use]
    pub fn sort(&self) -> Self {
        let mut cards: Vec<Card<RankType, SuitType>> = self.0.clone();
        cards.sort();
        cards.reverse();
        Self(cards)
    }

    pub fn sort_in_place(&mut self) {
        self.0.sort();
        self.0.reverse();
    }

    #[must_use]
    pub fn contains(&self, card: &Card<RankType, SuitType>) -> bool {
        self.0.contains(card)
    }
}

impl<SuitType: Suited + Ord + Clone, RankType: Ranked + Ord + Clone> Default
    for Pile<RankType, SuitType>
{
    fn default() -> Self {
        Self(Vec::new())
    }
}

/// This is probably my biggest embarrassment when coding this library the first time. I had no
/// idea that this trait existed, and bent over backwards trying to duplicate its functionality.
impl<RankType: Ranked + Ord + Clone, SuitType: Suited + Ord + Clone> FromStr
    for Pile<RankType, SuitType>
{
    type Err = CardError;

    fn from_str(index: &str) -> Result<Self, Self::Err> {
        let mut cards = Pile::<RankType, SuitType>::default();
        for s in index.split_whitespace() {
            if !cards.push(Card::<RankType, SuitType>::from_str(s)?) {
                return Err(CardError::InvalidIndex(s.to_string()));
            }
        }

        if cards.is_empty() {
            Err(CardError::InvalidIndex(index.to_string()))
        } else {
            Ok(cards)
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod types__pile__tests {
    use super::*;
    use crate::decks::standard52::Standard52;
    use std::str::FromStr;

    fn test_pile() -> Pile<Standard52, Standard52> {
        Pile::<Standard52, Standard52>::new(vec![
            Card::from_str("2S").unwrap(),
            Card::from_str("TD").unwrap(),
            Card::from_str("AH").unwrap(),
            Card::from_str("AS").unwrap(),
        ])
    }

    #[test]
    fn clone() {
        let pile = test_pile();

        let mut pile2 = pile.clone();
        pile2.sort_in_place();

        assert_eq!(pile2.get(0).unwrap().index, "AS");
        assert_eq!(pile2.get(1).unwrap().index, "2S");
        assert_eq!(pile2.get(2).unwrap().index, "AH");
        assert_eq!(pile2.get(3).unwrap().index, "TD");
    }

    #[test]
    fn extend() {
        let mut pile = test_pile();
        let pile2 = Pile::<Standard52, Standard52>::from_str("3S 9D").unwrap();
        pile.extend(&pile2);

        assert_eq!(pile.len(), 6);
        assert_eq!(pile.get(4).unwrap().index, "3S");
        assert_eq!(pile.get(5).unwrap().index, "9D");
    }

    #[test]
    fn push() {
        let mut pile = Pile::<Standard52, Standard52>::default();
        pile.push(Card::from_str("2S").unwrap());
        pile.push(Card::from_str("TD").unwrap());
        pile.push(Card::from_str("AH").unwrap());
        pile.push(Card::from_str("AS").unwrap());

        assert_eq!(pile, test_pile());
    }

    #[test]
    fn from_str() {
        let pile = Pile::<Standard52, Standard52>::from_str("2S TD AH AS").unwrap();

        assert_eq!(pile, test_pile());
    }

    #[test]
    fn from_str_invalid() {
        assert!(Pile::<Standard52, Standard52>::from_str("2S TD AH AS 2X").is_err());
        assert!(Pile::<Standard52, Standard52>::from_str("   ").is_err());
    }

    #[test]
    fn fold_in() {
        let ranks = Rank::<Standard52>::ranks();
        let suits = Suit::<Standard52>::suits();
    }
}
