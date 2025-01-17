use crate::types::card::Card;
use crate::types::card_error::CardError;
use crate::types::rank::Rank;
use crate::types::suit::Suit;
use crate::types::traits::Ranked;
use crate::types::traits::Suited;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;
use std::str::FromStr;

/// `Pile` is a [generic data type](https://doc.rust-lang.org/book/ch10-01-syntax.html)
/// whose specific implementations implement the [`Decked`](crate::types::traits::Decked) trait,
/// which is made of a [`Vec`] of [`Cards`](Card) that implement the [`Ranked`], and [`Suited`] traits.
/// Implementations of a specific type of `Pile` are stored in the [`decks`](crate::decks) module.
///
/// The most common deck is the [`French`](crate::decks::french::French) deck:
///
/// ```rust
/// use cardpack::prelude::{Decked, French, Pile};
/// let mut french_deck: Pile<French, French> = French::deck();
///
/// assert_eq!(french_deck.rank_index_joined(" "), "A K Q J T 9 8 7 6 5 4 3 2");
/// assert_eq!(french_deck.suit_symbol_index(), "♠ ♥ ♦ ♣");
/// assert_eq!(french_deck.suit_index(), "S H D C");
/// assert_eq!(french_deck.draw(5).to_string(), "A♠ K♠ Q♠ J♠ T♠");
/// assert_eq!(french_deck.len(), 47);
/// ```
///
/// The [`Modern`](crate::decks::modern::Modern) deck is simply the
/// [`French`](crate::decks::french::French) deck with the addition of the big and little jokers,
/// which belong to the joker suit.
///
/// ```rust
/// use cardpack::prelude::{Decked, Modern, Pile};
/// let modern_deck: Pile<Modern, Modern> = Modern::deck();
///
/// assert_eq!(modern_deck.rank_index(), "BLAKQJT98765432");
/// assert_eq!(modern_deck.suit_symbol_index(), "🃟 ♠ ♥ ♦ ♣");
/// assert_eq!(modern_deck.suit_index(), "J S H D C");
/// ```
#[derive(Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Pile<
    RankType: Ranked + PartialOrd + Ord + Clone + Default + Hash,
    SuitType: Suited + PartialOrd + Ord + Clone + Default + Hash,
>(Vec<Card<RankType, SuitType>>)
where
    RankType: Ranked,
    SuitType: Suited;

impl<
        RankType: Ranked + Ord + Clone + Default + Hash,
        SuitType: Suited + Ord + Clone + Default + Hash,
    > Pile<RankType, SuitType>
{
    /// ```
    /// use std::collections::HashSet;
    /// use cardpack::prelude::*;
    /// let five_deck = French::decks(5);
    ///
    /// let hashset: HashSet<Card<French, French>> = five_deck.as_hashset();
    ///
    /// assert_eq!(five_deck.len(), 260);
    /// assert_eq!(hashset.len(), 52);
    /// assert_eq!(FrenchDeck::from(hashset), French::deck());
    /// ```
    #[must_use]
    pub fn as_hashset(&self) -> HashSet<Card<RankType, SuitType>> {
        self.clone().0.into_iter().collect()
    }

    ///
    ///
    /// Here's the original code:
    ///
    /// ```txt
    /// #[must_use]
    /// pub fn card_by_index(&self, index: &str) -> Option<&Card> {
    ///   self.0.iter().find(|c| c.index_default() == index)
    /// }
    /// ```
    ///
    /// Why TF not just use `Card::from_str()?` I guess the big difference is that
    /// the card is actually in the Pile in question. Do I need this?
    #[must_use]
    pub fn card_by_index<S: Into<String>>(&self, index: S) -> Option<Card<RankType, SuitType>> {
        match Card::<RankType, SuitType>::from_str(index.into().as_str()) {
            Ok(c) => {
                if self.contains(&c) {
                    Some(c)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Returns true if the card is in the `Pile`.
    ///
    /// ```
    /// use cardpack::prelude::*;
    /// let pile = FrenchDeck::from_str("K♦ K♣ K♠").unwrap();
    /// let king_of_diamonds = FrenchCard::from_str("K♦").unwrap();
    /// let king_of_hearts = FrenchCard::from_str("K♥").unwrap();
    ///
    /// assert!(pile.contains(&king_of_diamonds));
    /// assert!(!pile.contains(&king_of_hearts));
    /// ```
    #[must_use]
    pub fn contains(&self, card: &Card<RankType, SuitType>) -> bool {
        self.0.contains(card)
    }

    /// ```
    /// use cardpack::prelude::*;
    /// let mut pile = French::deck();
    ///
    /// let ak = pile.draw(2);
    ///
    /// assert_eq!(ak, FrenchDeck::from_str("A♠ K♠").unwrap());
    /// ```
    #[must_use]
    pub fn draw(&mut self, n: usize) -> Self {
        let mut pile = Pile::<RankType, SuitType>::default();
        for _ in 0..n {
            if let Some(card) = self.draw_first() {
                pile.push(card);
            }
        }
        pile
    }

    /// ```
    /// use cardpack::prelude::*;
    /// let mut pile = French::deck().reverse();
    /// let card = pile.draw_first().unwrap();
    ///
    /// assert_eq!(card.to_string(), "2♣");
    /// ```
    pub fn draw_first(&mut self) -> Option<Card<RankType, SuitType>> {
        match self.len() {
            0 => None,
            _ => Some(self.remove(0)),
        }
    }

    /// ```
    /// use cardpack::prelude::*;
    /// let mut pile = French::deck();
    /// let card = pile.draw_last().unwrap();
    ///
    /// assert_eq!(card.to_string(), "2♣");
    /// ```
    pub fn draw_last(&mut self) -> Option<Card<RankType, SuitType>> {
        self.0.pop()
    }

    /// ```
    /// use cardpack::prelude::*;
    /// let mut pile = French::deck();
    /// let card = pile.remove(1);
    ///
    /// assert_eq!(card.to_string(), "K♠");
    /// assert_eq!(pile.draw(2).to_string(), "A♠ Q♠");
    /// ```
    pub fn remove(&mut self, index: usize) -> Card<RankType, SuitType> {
        self.0.remove(index)
    }

    /// Extends the `Pile` with the contents of the passed in `Pile`.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let mut pile = FrenchDeck::from_str("K♦ K♣ K♠").unwrap();
    /// let other_pile = FrenchDeck::from_str("A♦ A♣ A♠").unwrap();
    /// pile.extend(&other_pile);
    ///
    /// assert_eq!(pile.to_string(), "K♦ K♣ K♠ A♦ A♣ A♠");
    /// ```
    pub fn extend(&mut self, other: &Self) {
        self.0.extend(other.0.clone());
    }

    /// Returns a Card at the specific passed in position.
    ///
    /// ```
    /// use cardpack::prelude::*;
    /// let deck = French::deck();
    ///
    /// assert_eq!(deck.get(0).unwrap().to_string(), "A♠");
    /// assert_eq!(deck.get(51).unwrap().to_string(), "2♣");
    /// assert!(deck.get(52).is_none());
    /// ```
    #[must_use]
    pub fn get(&self, position: usize) -> Option<&Card<RankType, SuitType>> {
        self.0.get(position)
    }

    /// Returns a string of the index of the cards in the `Pile`.
    ///
    /// ```
    /// use cardpack::prelude::*;
    /// let pile = FrenchDeck::from_str("K♦ K♣ K♠").unwrap();
    ///
    /// assert_eq!(pile.index(), "KD KC KS");
    /// ```
    #[must_use]
    pub fn index(&self) -> String {
        self.iter()
            .map(|c| c.index)
            .collect::<Vec<String>>()
            .join(" ")
    }

    /// ```
    /// use cardpack::prelude::*;
    /// assert!(FrenchDeck::default().is_empty());
    /// assert!(!French::deck().is_empty());
    /// assert!(ModernDeck::default().is_empty());
    /// assert!(!Modern::deck().is_empty());
    /// assert!(ShortDeck::default().is_empty());
    /// assert!(!Short::deck().is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the length of the `Pile`.
    ///
    /// ```
    /// use cardpack::prelude::*;
    /// assert_eq!(French::deck().len(), 52);
    /// assert_eq!(Modern::deck().len(), 54);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// ```
    /// use cardpack::prelude::*;
    /// let pile = French::deck();
    ///
    /// for card in pile.iter() {
    ///   assert!(pile.contains(&card));
    /// }
    /// ```
    #[must_use]
    pub fn iter(&self) -> std::vec::IntoIter<<Pile<RankType, SuitType> as IntoIterator>::Item> {
        <&Self as IntoIterator>::into_iter(self)
    }

    /// Takes the `Pile` and returns a `HashMap` of the cards mapped by their [`Suit`].
    ///
    /// ```
    /// use cardpack::prelude::*;
    /// let pile = Short::deck();
    ///
    /// let map = pile.map_by_suit();
    ///
    /// assert_eq!(map.len(), 4);
    /// assert_eq!(map.get(&suit!(S)).unwrap().to_string(), "A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠");
    /// assert_eq!(map.get(&suit!(H)).unwrap().to_string(), "A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥");
    /// assert_eq!(map.get(&suit!(D)).unwrap().to_string(), "A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦");
    /// assert_eq!(map.get(&suit!(C)).unwrap().to_string(), "A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣");
    /// ```
    ///
    /// A more advanced example of this can be found in the Bridge example in the `examples` directory.
    ///
    /// # Panics
    ///
    /// No idea how it could. Too lazy to find a cleaner way.
    #[must_use]
    pub fn map_by_suit(&self) -> HashMap<Suit<SuitType>, Pile<RankType, SuitType>> {
        let mut map = HashMap::new();
        for card in &self.0 {
            let suit = card.suit.clone();
            if !map.contains_key(&suit) {
                map.insert(suit.clone(), Pile::default());
            }
            map.get_mut(&suit).unwrap().push(card.clone());
        }
        map
    }

    /// Returns a simple new `Pile` from the consolidated passed in vector of `Piles`.
    ///
    /// ```
    /// use cardpack::prelude::*;
    /// // `FrenchDeck` is the same as `Pile<French, French>`.
    /// let pile1 = FrenchDeck::from_str("2♠ 8♠ 4♠").unwrap();
    /// let pile2 = FrenchDeck::from_str("5♠ 6♠ 7♠").unwrap();
    /// let piles = vec![pile1, pile2];
    ///
    /// let pile = FrenchDeck::pile_on(&piles);
    ///
    /// assert_eq!(pile.to_string(), "2♠ 8♠ 4♠ 5♠ 6♠ 7♠");
    /// ```
    ///
    /// ## ASIDE
    ///
    /// How cool is it that you can create a type alias that prepopulates the generic types?
    #[must_use]
    pub fn pile_on(piles: &Vec<Pile<RankType, SuitType>>) -> Self {
        let mut pile = Pile::default();
        for p in piles {
            pile.extend(p);
        }
        pile
    }

    /// Returns a `Pile` by calling the passed in function `n` times and consolidating the results
    /// into a single `Pile`.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// fn ak() -> Pile<French, French> {
    ///     Pile::<French, French>::from_str("A♠ K♠").unwrap()
    /// }
    ///
    /// let pile = Pile::<French, French>::pile_up(3, ak);
    ///
    /// assert_eq!(pile.to_string(), "A♠ K♠ A♠ K♠ A♠ K♠");
    /// ```
    pub fn pile_up(n: usize, f: fn() -> Pile<RankType, SuitType>) -> Self {
        let mut cards = Pile::<RankType, SuitType>::default();
        for _ in 0..n {
            cards.extend(&f());
        }
        cards
    }

    /// Returns the zero indexed position of a [`Card`] in the `Pile`.
    ///
    /// ```rust
    /// use cardpack::prelude::*;
    /// let pile = Modern::deck();
    ///
    /// let card = ModernCard::from_str("2♣").unwrap();
    ///
    /// assert_eq!(pile.position(&card).unwrap(), 53);
    /// ```
    #[must_use]
    pub fn position(&self, card: &Card<RankType, SuitType>) -> Option<usize> {
        self.0.iter().position(|c| c.index == card.index)
    }

    /// Adds a Pile of [`Cards`](Card) to the end of another pile.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let mut hand = Pile::<French, French>::from_str("K♦ J♦").unwrap();
    /// let flop = Pile::<French, French>::from_str("A♦ T♦ Q♦").unwrap();
    /// hand.prepend(&flop);
    /// hand.sort_in_place();
    ///
    /// assert_eq!(hand.to_string(), "A♦ K♦ Q♦ J♦ T♦");
    /// ```
    pub fn prepend(&mut self, other: &Pile<RankType, SuitType>) {
        let mut product = other.0.clone();
        product.append(&mut self.0);
        self.0 = product;
    }

    /// Places the Card at the bottom (end) of the Pile.
    ///
    /// ```rust
    /// use cardpack::prelude::*;
    ///
    /// let mut hand = Pile::<French, French>::from_str("K♠ A♠").unwrap();
    /// hand.push(Card::<French, French>::from_str("Q♠").unwrap());
    ///
    /// assert_eq!(hand.to_string(), "K♠ A♠ Q♠");
    /// ```
    pub fn push(&mut self, card: Card<RankType, SuitType>) -> bool {
        if card.is_blank() {
            false
        } else {
            self.0.push(card);
            true
        }
    }

    /// String of all the [`Ranks`](Rank) in the `Pile`.
    ///
    /// ```rust
    /// use cardpack::prelude::*;
    /// let pile = Short::deck();
    /// assert_eq!(pile.rank_index(), "AKQJT9876");
    /// ```
    #[must_use]
    pub fn rank_index(&self) -> String {
        self.rank_index_joined("")
    }

    /// String of all the [`Ranks`](Rank) in the `Pile`, joined by the passed in separator.
    ///
    /// ```rust
    /// use cardpack::prelude::*;
    /// let pile = Short::deck();
    /// assert_eq!(pile.rank_index_joined(" "), "A K Q J T 9 8 7 6");
    /// ```
    pub fn rank_index_joined(&self, sep: &str) -> String {
        self.ranks()
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(sep)
    }

    /// ```
    /// use cardpack::prelude::*;
    /// let pile = cards!("A♠ K♠ A♣ Q♣ K♥").unwrap();
    ///
    /// assert_eq!(pile.rank_index_by_suit(&suit!(S), "-"), Some("A-K".to_string()));
    /// assert_eq!(pile.rank_index_by_suit(&suit!(H), "-"), Some("K".to_string()));
    /// assert_eq!(pile.rank_index_by_suit(&suit!(C), "-"), Some("A-Q".to_string()));
    /// assert_eq!(pile.rank_index_by_suit(&suit!(D), "-"), None);
    /// ```
    #[must_use]
    pub fn rank_index_by_suit(&self, suit: &Suit<SuitType>, joiner: &str) -> Option<String> {
        self.ranks_by_suit(suit)
            .map(|ranks| Rank::<RankType>::ranks_index(&ranks, joiner))
    }

    /// ```
    /// use cardpack::prelude::*;
    /// let pile = cards!("A♠ K♠ A♣ Q♣ K♥").unwrap();
    ///
    /// let expected = vec![
    ///     Rank::<French>::new(French::ACE),
    ///     Rank::<French>::new(French::KING),
    ///     Rank::<French>::new(French::QUEEN),
    /// ];
    ///
    /// assert_eq!(pile.ranks(), expected);
    /// ```
    #[must_use]
    pub fn ranks(&self) -> Vec<Rank<RankType>> {
        let hashset: HashSet<Rank<RankType>> = self.iter().map(|c| c.rank.clone()).collect();
        let mut ranks: Vec<Rank<RankType>> = Vec::from_iter(hashset);
        ranks.sort();
        ranks.reverse();
        ranks
    }

    /// ```
    /// use cardpack::prelude::*;
    /// let pile = cards!("A♠ K♠").unwrap();
    ///
    /// let expected = vec![
    ///     Rank::<French>::new(French::ACE),
    ///     Rank::<French>::new(French::KING),
    /// ];
    ///
    /// // ASIDE: this is so clunky.
    /// assert_eq!(pile.ranks_by_suit(&Suit::<French>::new(French::SPADES)).unwrap(), expected);
    /// assert!(pile.ranks_by_suit(&Suit::<French>::new(French::HEARTS)).is_none());
    /// ```
    #[must_use]
    pub fn ranks_by_suit(&self, suit: &Suit<SuitType>) -> Option<Vec<Rank<RankType>>> {
        let ranks: Vec<Rank<RankType>> = self
            .v()
            .iter()
            .filter(|c| c.suit == *suit)
            .map(|c| c.rank.clone())
            .collect();

        match ranks.len() {
            0 => None,
            _ => Some(ranks),
        }
    }

    /// ```
    /// use cardpack::prelude::*;
    /// let mut pile = French::deck();
    /// pile.remove_card(&card!(KS));
    ///
    /// assert_eq!(pile.draw(2), cards!("AS QS").unwrap());
    /// ```
    pub fn remove_card(
        &mut self,
        card: &Card<RankType, SuitType>,
    ) -> Option<Card<RankType, SuitType>> {
        let index = self.position(card)?;
        Some(self.0.remove(index))
    }

    /// ```
    /// use cardpack::prelude::*;
    /// let mut pile = French::deck();
    /// pile.remove_cards(&cards!("K♠ A♠").unwrap());
    ///
    /// assert_eq!(pile.len(), 50);
    /// assert_eq!(pile.draw_first().unwrap_or(French::blank()), card!(QS));
    /// ```
    pub fn remove_cards(&mut self, cards: &Pile<RankType, SuitType>) {
        for card in &cards.0 {
            self.remove_card(card);
        }
    }

    /// ```
    /// use cardpack::prelude::*;
    /// let ak = cards!("A♠ K♠").unwrap();
    /// let ka = cards!("K♠ A♠").unwrap();
    ///
    /// assert_eq!(ak.reverse(), ka);
    /// assert_eq!(ka.reverse(), ak);
    /// ```
    #[must_use]
    pub fn reverse(&self) -> Self {
        let mut pile = self.clone();
        pile.reverse_in_place();
        pile
    }

    /// ```
    /// use cardpack::prelude::*;
    /// let mut ak = cards!("A♠ K♠").unwrap();
    /// let ka = cards!("K♠ A♠").unwrap();
    ///
    /// ak.reverse_in_place();
    ///
    /// assert_eq!(ak, ka);
    /// ```
    pub fn reverse_in_place(&mut self) {
        self.0.reverse();
    }

    /// Returns true if the Cards of the passed in `Pile` are identical to the `Pile`, regqrdless
    /// of order.
    ///
    /// ```
    /// use cardpack::prelude::*;
    /// let pile = Canasta::deck();
    /// let other_pile = pile.shuffle();
    ///
    /// assert!(pile.same(&pile));
    /// assert!(pile.same(&other_pile));
    /// ```
    #[must_use]
    pub fn same(&self, cards: &Pile<RankType, SuitType>) -> bool {
        let left = self.sort();
        let right = cards.sort();

        left == right
    }

    /// ```
    /// use cardpack::prelude::*;
    /// let pile = French::deck();
    /// let shuffled = pile.shuffle();
    ///
    /// assert!(pile.same(&shuffled));
    /// ```
    #[must_use]
    pub fn shuffle(&self) -> Self {
        let mut pile = self.clone();
        pile.shuffle_in_place();
        pile
    }

    /// TODO WIP
    #[allow(dead_code)]
    fn shuffle_in_place_custom<F>(&mut self, mut rng: F)
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

    /// ```
    /// use cardpack::prelude::*;
    /// let mut pile = French::deck();
    /// pile.shuffle_in_place();
    ///
    /// assert!(pile.same(&French::deck()));
    /// ```
    pub fn shuffle_in_place(&mut self) {
        let mut rng = thread_rng();
        self.0.shuffle(&mut rng);
    }

    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let pile = FrenchDeck::from_str("K♠ A♠").unwrap();
    /// let sorted = pile.sort();
    ///
    /// assert_eq!(sorted.to_string(), "A♠ K♠");
    /// ```
    #[must_use]
    pub fn sort(&self) -> Self {
        let mut cards: Vec<Card<RankType, SuitType>> = self.0.clone();
        cards.sort();
        cards.reverse();
        Self(cards)
    }

    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let mut pile = FrenchDeck::from_str("K♠ A♠").unwrap();
    /// pile.sort_in_place();
    ///
    /// assert_eq!(pile.to_string(), "A♠ K♠");
    /// ```
    pub fn sort_in_place(&mut self) {
        self.0.sort();
        self.0.reverse();
    }

    /// Returns a vector of all the [`Suits`](Suit) in the `Pile`.
    ///
    /// ```
    /// use cardpack::prelude::*;
    /// let pile = Short::deck();
    /// assert_eq!(
    ///     pile.suits(),
    ///     vec![
    ///         Suit::<French>::new(French::SPADES),
    ///         Suit::<French>::new(French::HEARTS),
    ///         Suit::<French>::new(French::DIAMONDS),
    ///         Suit::<French>::new(French::CLUBS)
    ///    ]
    /// );
    /// ```
    #[must_use]
    pub fn suits(&self) -> Vec<Suit<SuitType>> {
        let hashset: HashSet<Suit<SuitType>> = self.0.iter().map(|c| c.suit.clone()).collect();
        let mut suits: Vec<Suit<SuitType>> = Vec::from_iter(hashset);
        suits.sort();
        suits.reverse();
        suits
    }

    /// Returns a `String` with all of the [`Suit`] index letters for the `Pile` separated by spaces.
    ///
    /// ```rust
    /// use cardpack::prelude::{Decked, Pile, Skat};
    /// let skat_deck: Pile<Skat, Skat> = Skat::deck();
    /// assert_eq!(skat_deck.suit_index(), "E L H S");
    /// ```
    pub fn suit_index(&self) -> String {
        self.suit_indexed(Suit::index, " ")
    }

    /// Returns a `String` with all of the [`Suit`] symbols for the `Pile` separated by spaces.
    ///
    /// ```rust
    /// use cardpack::prelude::{Decked, Pile, Tarot};
    /// let tarot_deck: Pile<Tarot, Tarot> = Tarot::deck();
    /// assert_eq!(tarot_deck.suit_symbol_index(), "M 🪄 🏆 ⚔ ☆");
    pub fn suit_symbol_index(&self) -> String {
        self.suit_indexed(Suit::symbol, " ")
    }

    fn suit_indexed<F>(&self, map_fn: F, joiner: &str) -> String
    where
        F: Fn(&Suit<SuitType>) -> String,
    {
        self.suits()
            .iter()
            .map(map_fn)
            .collect::<Vec<String>>()
            .join(joiner)
    }

    /// Returns a `String` of the cards symbol string, colored by what's defined in the
    /// [`Suited`] trait.
    ///
    /// ```rust
    /// use cardpack::prelude::*;
    /// let mut pile = FrenchDeck::from_str("2♠ A♥").unwrap();
    /// assert_eq!(pile.to_color_symbol_string(), "2♠ A♥");
    /// ```
    #[must_use]
    pub fn to_color_symbol_string(&self) -> String {
        self.0
            .iter()
            .map(Card::to_color_symbol_string)
            .collect::<Vec<String>>()
            .join(" ")
    }

    /// Returns the Pile's internal vector of [`Cards`](Card).
    ///
    /// ```rust
    /// use cardpack::prelude::*;
    /// let mut pile = French::deck();
    /// let v: Vec<FrenchCard> = vec![
    ///     FrenchCard::from_str("A♠").unwrap(),
    ///     FrenchCard::from_str("K♠").unwrap()
    /// ];
    /// assert_eq!(pile.draw(2).v(), &v);
    /// ```
    #[must_use]
    pub fn v(&self) -> &Vec<Card<RankType, SuitType>> {
        &self.0
    }
}

impl<
        SuitType: Suited + Ord + Clone + Default + Hash,
        RankType: Ranked + Ord + Clone + Default + Hash,
    > Display for Pile<RankType, SuitType>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .0
            .iter()
            .map(Card::to_string)
            .collect::<Vec<String>>()
            .join(" ");
        write!(f, "{s}")
    }
}

impl<
        RankType: Ranked + Ord + Clone + Default + Hash,
        SuitType: Suited + Ord + Clone + Default + Hash,
    > From<HashSet<Card<RankType, SuitType>>> for Pile<RankType, SuitType>
{
    fn from(cards: HashSet<Card<RankType, SuitType>>) -> Self {
        Pile(cards.into_iter().collect()).sort()
    }
}

impl<
        RankType: Ranked + Ord + Clone + Default + Hash,
        SuitType: Suited + Ord + Clone + Default + Hash,
    > From<Vec<Card<RankType, SuitType>>> for Pile<RankType, SuitType>
{
    fn from(cards: Vec<Card<RankType, SuitType>>) -> Self {
        Pile(cards)
    }
}

/// Takes an index string or a symbol string and converts it into a `Pile`.
///
/// An index string is defined, in terms of this library as letters representing a [`Rank`] and a
/// [`Suit`] for a specific deck. For example:
///
/// ```rust
/// use cardpack::prelude::*;
/// let wheel = Pile::<French, French>::from_str("5S 4S 3s 2S AS").unwrap();
///
/// assert_eq!(wheel.to_string(), "5♠ 4♠ 3♠ 2♠ A♠");
/// ```
///
/// A symbol string is defined as a string that represents a card using the Unicode symbols for the
/// [`Suit`] and letters or Unicode symbols for the [`Rank`]. For example:
///
/// ```rust
/// use cardpack::prelude::*;
/// let wheel = Pile::<French, French>::from_str("5♠ 4♠ 3♠ 2♠ a♠").unwrap();
///
/// assert_eq!(wheel.index(), "5S 4S 3S 2S AS");
/// ```
///
/// It is also possible to mix and match:
///
/// ```rust
/// use cardpack::prelude::*;
///
/// let wheel = Pile::<French, French>::from_str("5S 4♠ 3♠ 2s A♠").unwrap();
///
/// assert_eq!(wheel.to_string(), "5♠ 4♠ 3♠ 2♠ A♠");
/// ```
///
/// **ASIDE:** This is probably my biggest embarrassment when coding this library the first time. I had no
/// idea that this trait existed, and bent over backwards trying to duplicate its functionality.
///
/// TODO: Add a step that validates that the cards are of the correct number for the type of deck.
/// Perhaps using the `Decked::deck()` method.
impl<
        RankType: Ranked + Ord + Clone + Default + Hash,
        SuitType: Suited + Ord + Clone + Default + Hash,
    > FromStr for Pile<RankType, SuitType>
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

/// ```rust
/// use cardpack::prelude::*;
/// let pile = French::deck();
///
/// // Need to clone since `Pile` doesn't implement `Copy`.
/// for card in pile.clone() {
///    assert!(pile.contains(&card));
/// }
/// ```
///
/// ## ASIDE
///
/// Implementing this eliminates the need for this old function in `Pile`:
///
/// ```txt
/// pub fn cards(&self) -> Vec<Card<RankType, SuitType>> {
///     self.0.clone()
/// }
/// ```
impl<
        RankType: Ranked + Ord + Clone + Default + Hash,
        SuitType: Suited + Ord + Clone + Default + Hash,
    > IntoIterator for Pile<RankType, SuitType>
{
    type Item = Card<RankType, SuitType>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// For a `Pile` reference, the implementation of the trait will internally `clone()` the `Cards`.
///
/// ```rust
/// use cardpack::prelude::*;
/// let pile = French::deck();
///
/// for card in &pile {
///    assert!(pile.contains(&card));
/// }
/// ```
impl<
        RankType: Ranked + Ord + Clone + Default + Hash,
        SuitType: Suited + Ord + Clone + Default + Hash,
    > IntoIterator for &Pile<RankType, SuitType>
{
    type Item = Card<RankType, SuitType>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        <Vec<Card<RankType, SuitType>> as Clone>::clone(&self.0).into_iter()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod types__pile__tests {
    use super::*;
    use crate::decks::french::French;
    use crate::types::traits::Decked;
    use std::str::FromStr;

    fn test_pile() -> Pile<French, French> {
        Pile::<French, French>::from(vec![
            Card::from_str("2S").unwrap(),
            Card::from_str("TD").unwrap(),
            Card::from_str("AH").unwrap(),
            Card::from_str("AS").unwrap(),
        ])
    }

    #[test]
    fn as_hashset() {
        assert_eq!(4, test_pile().as_hashset().len());
        assert_eq!(52, French::deck().as_hashset().len());
    }

    #[test]
    fn card_by_index() {
        let pile = test_pile();

        assert_eq!(pile.card_by_index("2S").unwrap().index, "2S");
        assert_eq!(pile.card_by_index("TD").unwrap().index, "TD");
        assert_eq!(pile.card_by_index("AH").unwrap().index, "AH");
        assert_eq!(pile.card_by_index("AS").unwrap().index, "AS");
        assert!(pile.card_by_index("AD").is_none());
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
        let pile2 = Pile::<French, French>::from_str("3S 9D").unwrap();
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
        let mut pile = Pile::<French, French>::default();
        assert!(pile.is_empty());

        pile.push(Card::from_str("2S").unwrap());
        assert!(!pile.is_empty());
    }

    #[test]
    fn map_by_suit() {
        let pile = Pile::<French, French>::from_str("QS 9S QC QH QD").unwrap();

        let qs = pile.get(0).unwrap();
        let qc = pile.get(2).unwrap();
        let spades = Suit::new(French::SPADES);
        let clubs = Suit::new(French::CLUBS);

        let mappie = pile.map_by_suit();

        assert_eq!(
            qs.index,
            mappie.get(&spades).unwrap().0.first().unwrap().index
        );
        assert_eq!(
            qc.index,
            mappie.get(&clubs).unwrap().0.first().unwrap().index
        );
    }

    #[test]
    fn len() {
        assert_eq!(Pile::<French, French>::default().len(), 0);
        assert_eq!(test_pile().len(), 4);
    }

    #[test]
    fn position() {
        let deck = French::deck();
        let pile = test_pile();
        let card = Card::from_str("AH").unwrap();

        assert_eq!(deck.position(&card), Some(13));
        assert_eq!(pile.position(&card), Some(2));
    }

    #[test]
    fn prepend() {
        let mut pile = test_pile();
        let pile2 = Pile::<French, French>::from_str("3S 9D").unwrap();
        pile.prepend(&pile2);

        assert_eq!(pile.to_string(), "3♠ 9♦ 2♠ T♦ A♥ A♠");
    }

    #[test]
    fn push() {
        let mut pile = Pile::<French, French>::default();
        pile.push(Card::from_str("2S").unwrap());
        pile.push(Card::from_str("TD").unwrap());
        pile.push(Card::from_str("AH").unwrap());
        pile.push(Card::from_str("AS").unwrap());

        assert_eq!(pile, test_pile());
    }

    #[test]
    fn remove_card() {
        let mut deck = French::deck();
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
    fn same() {
        let deck = French::deck();
        let alt = deck.shuffle();

        assert!(deck.same(&alt));
        assert!(alt.same(&deck));
        assert!(alt.same(&alt));
        assert!(deck.same(&deck));
    }

    #[test]
    fn same__false() {
        let deck = French::deck();
        let mut alt = deck.shuffle();
        alt.draw_last();

        assert!(!deck.same(&alt));
        assert!(!alt.same(&deck));
    }

    #[test]
    fn to_color_symbol_string() {
        let expected = vec![
            Card::<French, French>::from_str("2S").unwrap().to_string(),
            Card::<French, French>::from_str("TD")
                .unwrap()
                .to_color_symbol_string(),
            Card::<French, French>::from_str("AH")
                .unwrap()
                .to_color_symbol_string(),
            Card::<French, French>::from_str("AS").unwrap().to_string(),
        ]
        .join(" ");

        let actual = test_pile().to_color_symbol_string();

        assert_eq!(expected, actual);
    }

    #[test]
    fn from_str() {
        let pile = Pile::<French, French>::from_str("2S TD AH AS").unwrap();

        assert_eq!(pile, test_pile());
    }

    #[test]
    fn from_str_invalid() {
        assert!(Pile::<French, French>::from_str("2S TD AH AS 2X").is_err());
        assert!(Pile::<French, French>::from_str("   ").is_err());
    }
}
