use crate::types::card::Card;
use crate::types::card_error::CardError;
use crate::types::traits::Ranked;
use crate::types::traits::Suited;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Debug)]
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

    /// A mutable reference to the vector of cards so that they can be shuffled. I am
    /// torn about
    #[must_use]
    pub fn cards(&self) -> Vec<Card<RankType, SuitType>> {
        self.0.clone()
    }

    pub fn extend(&mut self, other: &Self) {
        self.0.extend(other.0.clone());
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

    pub fn pile_up(n: usize, f: fn() -> Vec<Card<RankType, SuitType>>) -> Self {
        let mut cards = Vec::new();
        for _ in 0..n {
            cards.extend(f());
        }
        Self(cards)
    }

    #[must_use]
    pub fn position(&self, card: &Card<RankType, SuitType>) -> Option<usize> {
        self.0.iter().position(|c| c.index == card.index)
    }

    // Takes a reference to the prepended entity, clones it, appends the original to the passed in
    // entity, and replaces the original with the new one.
    pub fn prepend(&mut self, other: &Pile<RankType, SuitType>) {
        let mut product = other.0.clone();
        product.append(&mut self.0);
        self.0 = product;
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

    pub fn remove_card(
        &mut self,
        card: &Card<RankType, SuitType>,
    ) -> Option<Card<RankType, SuitType>> {
        let index = self.position(card)?;
        Some(self.0.remove(index))
    }

    pub fn remove_cards(&mut self, cards: &Pile<RankType, SuitType>) {
        for card in &cards.0 {
            self.remove_card(card);
        }
    }

    #[must_use]
    pub fn shuffle_default(&self) -> Self {
        let mut pile = self.clone();
        pile.shuffle_in_place_default();
        pile
    }

    pub fn shuffle_in_place<F>(&mut self, mut rng: F)
    where
        F: FnMut(usize) -> usize,
    {
        let mut cards = self.0.clone();
        let mut shuffled = Vec::new();
        while !cards.is_empty() {
            let index = rng(cards.len());
            shuffled.push(cards.remove(index));
        }
        self.0 = shuffled;
    }

    pub fn shuffle_in_place_default(&mut self) {
        let mut rng = thread_rng();
        self.0.shuffle(&mut rng);
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

    #[must_use]
    pub fn index(&self) -> String {
        let mut s = String::new();
        for card in &self.0 {
            s.push_str(&card.index);
            s.push(' ');
        }
        s.trim().to_string()
    }
}

impl<SuitType: Suited + Ord + Clone, RankType: Ranked + Ord + Clone> Default
    for Pile<RankType, SuitType>
{
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<SuitType: Suited + Ord + Clone, RankType: Ranked + Ord + Clone> Display
    for Pile<RankType, SuitType>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for card in &self.0 {
            s.push_str(&card.to_string());
            s.push(' ');
        }
        write!(f, "{}", s.trim())
    }
}

impl<RankType: Ranked + Ord + Clone, SuitType: Suited + Ord + Clone>
    From<Vec<Card<RankType, SuitType>>> for Pile<RankType, SuitType>
{
    fn from(cards: Vec<Card<RankType, SuitType>>) -> Self {
        Pile::new(cards)
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
    use crate::types::traits::Decked;
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
    fn get() {
        let pile = test_pile();

        assert_eq!(pile.get(0).unwrap().index, "2S");
        assert_eq!(pile.get(1).unwrap().index, "TD");
        assert_eq!(pile.get(2).unwrap().index, "AH");
        assert_eq!(pile.get(3).unwrap().index, "AS");
        assert!(pile.get(4).is_none());
    }

    #[test]
    fn is_empty() {
        let mut pile = Pile::<Standard52, Standard52>::default();
        assert!(pile.is_empty());

        pile.push(Card::from_str("2S").unwrap());
        assert!(!pile.is_empty());
    }

    #[test]
    fn len() {
        assert_eq!(Pile::<Standard52, Standard52>::default().len(), 0);
        assert_eq!(test_pile().len(), 4);
    }

    #[test]
    fn position() {
        let deck = Standard52::deck();
        let pile = test_pile();
        let card = Card::from_str("AH").unwrap();

        assert_eq!(deck.position(&card), Some(13));
        assert_eq!(pile.position(&card), Some(2));
    }

    #[test]
    fn prepend() {
        let mut pile = test_pile();
        let pile2 = Pile::<Standard52, Standard52>::from_str("3S 9D").unwrap();
        pile.prepend(&pile2);

        assert_eq!(pile.to_string(), "3♠ 9♦ 2♠ T♦ A♥ A♠");
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
    fn remove_card() {
        let mut deck = Standard52::deck();
        let mut pile = test_pile();
        let card = Card::from_str("AH").unwrap();

        let removed_from_pile = pile.remove_card(&card);
        let removed_from_deck = deck.remove_card(&card);

        assert_eq!(pile.len(), 3);
        assert_eq!(removed_from_pile.unwrap().index, "AH");
        assert_eq!(deck.len(), 51);
        assert_eq!(removed_from_deck.unwrap().index, "AH");
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
}
