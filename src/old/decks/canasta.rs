use crate::old::decks::french::French;
use crate::old::decks::modern::Modern;
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

    #[must_use]
    pub fn two_of_spades() -> Card<Modern, Modern> {
        Card::new_weighted(
            Rank::<Modern>::new(French::TWO),
            Suit::<Modern>::new(French::SPADES),
            5000,
        )
    }

    #[must_use]
    pub fn two_of_hearts() -> Card<Modern, Modern> {
        Card::new_weighted(
            Rank::<Modern>::new(French::TWO),
            Suit::<Modern>::new(French::HEARTS),
            5000,
        )
    }

    #[must_use]
    pub fn two_of_diamonds() -> Card<Modern, Modern> {
        Card::new_weighted(
            Rank::<Modern>::new(French::TWO),
            Suit::<Modern>::new(French::DIAMONDS),
            5000,
        )
    }

    #[must_use]
    pub fn two_of_clubs() -> Card<Modern, Modern> {
        Card::new_weighted(
            Rank::<Modern>::new(French::TWO),
            Suit::<Modern>::new(French::CLUBS),
            5000,
        )
    }

    #[must_use]
    pub fn three_of_hearts() -> Card<Modern, Modern> {
        Card::new_weighted(
            Rank::<Modern>::new(French::THREE),
            Suit::<Modern>::new(French::HEARTS),
            6_000,
        )
    }

    #[must_use]
    pub fn three_of_diamonds() -> Card<Modern, Modern> {
        Card::new_weighted(
            Rank::<Modern>::new(French::THREE),
            Suit::<Modern>::new(French::DIAMONDS),
            6_000,
        )
    }

    /// # Errors
    ///
    /// Returns a `CardError` if the index is out of bounds.
    ///
    /// TODO: Add deck validation
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(index: &str) -> Result<Pile<Modern, Modern>, CardError> {
        match Pile::<Modern, Modern>::from_str(index) {
            Ok(pile) => Ok(Canasta::wash(&pile).sort()),
            Err(e) => Err(e),
        }
    }

    /// Bumps up the weight of red threes and twos so that they sort correctly.
    ///
    /// The original experiment with this code was much more complex using direct checks:
    ///
    /// ```rust
    /// use cardpack::old::decks::canasta::Canasta;
    /// use cardpack::old::decks::modern::Modern;
    /// use cardpack::old::decks::french::French;
    /// use cardpack::types::card::Card;
    /// use cardpack::types::rank::Rank;
    /// use cardpack::types::suit::Suit;
    /// use cardpack::types::traits::{Ranked, Suited};
    ///
    /// fn is_red_three<RankType: Ranked + Clone, SuitType: Suited + Clone>(
    ///         card: &Card<RankType, SuitType>,
    ///     ) -> bool
    ///     where
    ///         Rank<RankType>: PartialEq<Rank<Modern>>,
    ///         Suit<SuitType>: PartialEq<Suit<Modern>>,
    ///     {
    ///         card.rank == Rank::<Modern>::new(French::THREE)
    ///             && (card.suit == Suit::<Modern>::new(French::HEARTS)
    ///                 || card.suit == Suit::<Modern>::new(French::DIAMONDS))
    ///     }
    ///
    /// let three_of_hearts = Canasta::three_of_hearts();
    ///
    /// assert!(is_red_three(&Canasta::three_of_hearts()));
    /// assert!(is_red_three(&Canasta::three_of_diamonds()));
    /// assert!(!is_red_three(&Canasta::two_of_spades()));
    /// ```
    fn bump<RankType: Ranked + Clone, SuitType: Suited + Clone>(
        card: &Card<Modern, Modern>,
    ) -> Card<Modern, Modern>
    where
        Rank<RankType>: PartialEq<Rank<Modern>>,
        Suit<SuitType>: PartialEq<Suit<Modern>>,
    {
        match card.index.as_str() {
            "3H" => Canasta::three_of_hearts(),
            "3D" => Canasta::three_of_diamonds(),
            "2S" => Canasta::two_of_spades(),
            "2H" => Canasta::two_of_hearts(),
            "2D" => Canasta::two_of_diamonds(),
            "2C" => Canasta::two_of_clubs(),
            _ => card.clone(),
        }
    }

    fn wash(pile: &Pile<Modern, Modern>) -> Pile<Modern, Modern> {
        let washed: Vec<Card<Modern, Modern>> = pile.v().iter().map(Canasta::bump).collect();
        Pile::from(washed)
    }
}

impl Decked<Modern, Modern> for Canasta {
    fn deck() -> Pile<Modern, Modern> {
        let deck = Modern::decks(2);

        Canasta::wash(&deck).sort()
    }

    fn blank() -> Card<Modern, Modern> {
        Card::<Modern, Modern>::default()
    }

    fn guide() -> Option<String> {
        None
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
    fn pile__sort() {
        let deck = Canasta::deck();
        let mut shuffled = deck.shuffle();

        shuffled.shuffle_in_place();
        shuffled.sort_in_place();

        assert_eq!(deck.to_string(), shuffled.to_string());
    }

    /// This doesn't work because of the way the red threes are being processed from
    /// `Decked::deck()`
    ///
    /// UPDATE: wash() fixes it
    #[test]
    fn to_string__from_str() {
        let deck = Canasta::deck();
        let shuffled = deck.shuffle().to_string();
        let parsed = Canasta::from_str(&shuffled).unwrap();

        assert!(deck.same(&parsed));
    }
}
