use crate::decks::modern::Modern;
use crate::decks::standard52::Standard52;
use crate::types::card::Card;
use crate::types::card_error::CardError;
use crate::types::pile::Pile;
use crate::types::rank::Rank;
use crate::types::suit::Suit;
use crate::types::traits::{Decked, Ranked, Suited};
use std::str::FromStr;

/// [Canasta](https://en.wikipedia.org/wiki/Canasta)deck
///
/// TODO: WIP
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Canasta {}

impl Canasta {
    pub const DECK_NAME: &'static str = "Canasta";

    /// This is probably the clearest way to show just how insane this codebase is with generics.
    /// I'm wondering if macros will help with this...
    fn is_red_three<RankType: Ranked + Clone, SuitType: Suited + Clone>(
        card: &Card<RankType, SuitType>,
    ) -> bool
    where
        Rank<RankType>: PartialEq<Rank<Modern>>,
        Suit<SuitType>: PartialEq<Suit<Modern>>,
    {
        card.rank == Rank::<Modern>::new(Standard52::THREE)
            && (card.suit == Suit::<Modern>::new(Standard52::HEARTS)
                || card.suit == Suit::<Modern>::new(Standard52::DIAMONDS))
    }

    fn is_two<RankType: Ranked + Clone, SuitType: Suited + Clone>(
        card: &Card<RankType, SuitType>,
    ) -> bool
    where
        Rank<RankType>: PartialEq<Rank<Modern>>,
        Suit<SuitType>: PartialEq<Suit<Modern>>,
    {
        card.rank == Rank::<Modern>::new(Standard52::TWO)
            && (card.suit == Suit::<Modern>::new(Standard52::SPADES)
                || card.suit == Suit::<Modern>::new(Standard52::HEARTS)
                || card.suit == Suit::<Modern>::new(Standard52::DIAMONDS)
                || card.suit == Suit::<Modern>::new(Standard52::CLUBS))
    }

    fn two_of_spades() -> Card<Modern, Modern> {
        Card::new_weighted(
            Rank::<Modern>::new(Standard52::TWO),
            Suit::<Modern>::new(Standard52::SPADES),
            5003,
        )
    }

    fn two_of_hearts() -> Card<Modern, Modern> {
        Card::new_weighted(
            Rank::<Modern>::new(Standard52::TWO),
            Suit::<Modern>::new(Standard52::HEARTS),
            5002,
        )
    }

    fn two_of_diamonds() -> Card<Modern, Modern> {
        Card::new_weighted(
            Rank::<Modern>::new(Standard52::TWO),
            Suit::<Modern>::new(Standard52::DIAMONDS),
            5001,
        )
    }

    fn two_of_clubs() -> Card<Modern, Modern> {
        Card::new_weighted(
            Rank::<Modern>::new(Standard52::TWO),
            Suit::<Modern>::new(Standard52::CLUBS),
            5000,
        )
    }

    fn three_of_hearts() -> Card<Modern, Modern> {
        Card::new_weighted(
            Rank::<Modern>::new(Standard52::THREE),
            Suit::<Modern>::new(Standard52::HEARTS),
            6_000,
        )
    }

    fn three_of_diamonds() -> Card<Modern, Modern> {
        Card::new_weighted(
            Rank::<Modern>::new(Standard52::THREE),
            Suit::<Modern>::new(Standard52::DIAMONDS),
            6_000,
        )
    }

    fn red_threes() -> Pile<Modern, Modern> {
        let mut pile = Pile::<Modern, Modern>::new(Vec::new());

        pile.push(Canasta::three_of_hearts());
        pile.push(Canasta::three_of_hearts());
        pile.push(Canasta::three_of_diamonds());
        pile.push(Canasta::three_of_diamonds());

        pile
    }

    fn twos() -> Pile<Modern, Modern> {
        let mut pile = Pile::<Modern, Modern>::new(Vec::new());

        pile.push(Canasta::two_of_spades());
        pile.push(Canasta::two_of_spades());
        pile.push(Canasta::two_of_hearts());
        pile.push(Canasta::two_of_hearts());
        pile.push(Canasta::two_of_diamonds());
        pile.push(Canasta::two_of_diamonds());
        pile.push(Canasta::two_of_clubs());
        pile.push(Canasta::two_of_clubs());

        pile
    }

    /// # Errors
    ///
    /// Returns a `CardError` if the index is out of bounds.
    ///
    /// TODO: Add deck validation
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(index: &str) -> Result<Pile<Modern, Modern>, CardError> {
        Pile::<Modern, Modern>::from_str(index)
    }
}

impl Decked<Modern, Modern> for Canasta {
    fn deck() -> Pile<Modern, Modern> {
        let mut deck = Modern::decks(2);

        let twos = Canasta::twos();
        deck.remove_cards(&twos);
        deck.prepend(&twos);

        let red_threes = Canasta::red_threes();
        deck.remove_cards(&red_threes);
        deck.prepend(&red_threes);

        deck.sort_in_place();
        deck
    }

    fn pack(&self) -> Pile<Modern, Modern> {
        Canasta::deck()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod decks__canasta__tests {
    use super::*;

    #[test]
    fn deck() {
        let deck = Canasta::deck();

        let expected = "3♥ 3♥ 3♦ 3♦ B🃟 B🃟 L🃟 L🃟 2♠ 2♠ 2♥ 2♥ 2♦ 2♦ 2♣ 2♣ A♠ A♠ K♠ K♠ Q♠ Q♠ J♠ J♠ T♠ T♠ 9♠ 9♠ 8♠ 8♠ 7♠ 7♠ 6♠ 6♠ 5♠ 5♠ 4♠ 4♠ 3♠ 3♠ A♥ A♥ K♥ K♥ Q♥ Q♥ J♥ J♥ T♥ T♥ 9♥ 9♥ 8♥ 8♥ 7♥ 7♥ 6♥ 6♥ 5♥ 5♥ 4♥ 4♥ A♦ A♦ K♦ K♦ Q♦ Q♦ J♦ J♦ T♦ T♦ 9♦ 9♦ 8♦ 8♦ 7♦ 7♦ 6♦ 6♦ 5♦ 5♦ 4♦ 4♦ A♣ A♣ K♣ K♣ Q♣ Q♣ J♣ J♣ T♣ T♣ 9♣ 9♣ 8♣ 8♣ 7♣ 7♣ 6♣ 6♣ 5♣ 5♣ 4♣ 4♣ 3♣ 3♣";

        assert_eq!(deck.len(), 108);
        assert_eq!(deck.to_string(), expected);
    }

    #[test]
    fn is_red_three() {
        let three_of_hearts = Canasta::three_of_hearts();
        let three_of_diamonds = Canasta::three_of_diamonds();
        let three_of_spades = Card::new_weighted(
            Rank::<Modern>::new(Standard52::THREE),
            Suit::<Modern>::new(Standard52::SPADES),
            6_000,
        );

        assert!(Canasta::is_red_three(&three_of_hearts));
        assert!(Canasta::is_red_three(&three_of_diamonds));
        assert!(!Canasta::is_red_three(&three_of_spades));
    }

    #[test]
    fn pile__sort() {
        let deck = Canasta::deck();
        let mut shuffled = deck.shuffle_default();

        shuffled.shuffle_in_place_default();
        shuffled.sort_in_place();

        assert_eq!(deck.to_string(), shuffled.to_string());
    }

    /// This doesn't work because of the way the red threes are being processed from
    /// `Decked::deck()`
    #[test]
    #[ignore]
    fn to_string__from_str() {
        let deck = Canasta::deck();
        let deck_sorted = deck.shuffle_default().sort();
        let shuffled = deck.shuffle_default().to_string();
        let parsed = Canasta::from_str(&shuffled).unwrap();
        let sorted = parsed.sort();

        println!("{deck}");
        println!("{deck_sorted}");
        println!("{sorted}");

        println!("{:?}", deck);

        assert!(deck.same(&parsed));
    }
}
