use crate::types::card::Card;
use crate::types::Ranked;
use crate::types::Suited;

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
    pub fn push(&mut self, card: Card<RankType, SuitType>) {
        self.0.push(card);
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

#[cfg(test)]
#[allow(non_snake_case)]
mod types__pile__tests {
    use super::*;
    use crate::decks::standard52::Standard52;
    use std::str::FromStr;

    #[test]
    fn clone() {
        let mut pile = Pile::<Standard52, Standard52>::new(vec![]);
        pile.push(Card::from_str("2S").unwrap());
        pile.push(Card::from_str("TD").unwrap());
        pile.push(Card::from_str("AH").unwrap());
        pile.push(Card::from_str("AS").unwrap());

        let mut pile2 = pile.clone();
        pile2.sort_in_place();

        assert_eq!(pile2.0[0].index, "AS");
        assert_eq!(pile2.0[1].index, "2S");
        assert_eq!(pile2.0[2].index, "AH");
        assert_eq!(pile2.0[3].index, "TD");
    }
}
