use crate::old::types::card::Card;
use crate::old::types::pile::Pile;
use crate::old::types::traits::{Decked, Ranked, Suited};
use crate::types::errors::CardError;
use crate::types::utils::Bit;
use colored::Color;
use std::collections::HashMap;
use std::str::FromStr;

/// These macros make me very happy. They plaster over a lot of headaches
/// from the generics.
///
/// ```
/// use cardpack::old_prelude::*;
///
/// assert_eq!(old_card!(AS), Card::<French, French>::from_str("A♠").unwrap());
/// assert_eq!(old_card!(KS), Card::<French, French>::from_str("K♠").unwrap());
/// assert_eq!(old_card!(QS), Card::<French, French>::from_str("Q♠").unwrap());
/// assert_eq!(old_card!(JS), Card::<French, French>::from_str("J♠").unwrap());
/// assert_eq!(old_card!(TS), Card::<French, French>::from_str("T♠").unwrap());
/// assert_eq!(old_card!(9S), Card::<French, French>::from_str("9♠").unwrap());
/// assert_eq!(old_card!(8S), Card::<French, French>::from_str("8♠").unwrap());
/// assert_eq!(old_card!(7S), Card::<French, French>::from_str("7♠").unwrap());
/// assert_eq!(old_card!(6S), Card::<French, French>::from_str("6♠").unwrap());
/// assert_eq!(old_card!(5S), Card::<French, French>::from_str("5♠").unwrap());
/// assert_eq!(old_card!(4S), Card::<French, French>::from_str("4♠").unwrap());
/// assert_eq!(old_card!(3S), Card::<French, French>::from_str("3♠").unwrap());
/// assert_eq!(old_card!(2S), Card::<French, French>::from_str("2♠").unwrap());
/// assert_eq!(old_card!(AH), Card::<French, French>::from_str("A♥").unwrap());
/// assert_eq!(old_card!(KH), Card::<French, French>::from_str("K♥").unwrap());
/// assert_eq!(old_card!(QH), Card::<French, French>::from_str("Q♥").unwrap());
/// assert_eq!(old_card!(JH), Card::<French, French>::from_str("J♥").unwrap());
/// assert_eq!(old_card!(TH), Card::<French, French>::from_str("T♥").unwrap());
/// assert_eq!(old_card!(9H), Card::<French, French>::from_str("9♥").unwrap());
/// assert_eq!(old_card!(8H), Card::<French, French>::from_str("8♥").unwrap());
/// assert_eq!(old_card!(7H), Card::<French, French>::from_str("7♥").unwrap());
/// assert_eq!(old_card!(6H), Card::<French, French>::from_str("6♥").unwrap());
/// assert_eq!(old_card!(5H), Card::<French, French>::from_str("5♥").unwrap());
/// assert_eq!(old_card!(4H), Card::<French, French>::from_str("4♥").unwrap());
/// assert_eq!(old_card!(3H), Card::<French, French>::from_str("3♥").unwrap());
/// assert_eq!(old_card!(2H), Card::<French, French>::from_str("2♥").unwrap());
/// assert_eq!(old_card!(AD), Card::<French, French>::from_str("A♦").unwrap());
/// assert_eq!(old_card!(KD), Card::<French, French>::from_str("K♦").unwrap());
/// assert_eq!(old_card!(QD), Card::<French, French>::from_str("Q♦").unwrap());
/// assert_eq!(old_card!(JD), Card::<French, French>::from_str("J♦").unwrap());
/// assert_eq!(old_card!(JD), Card::<French, French>::from_str("J♦").unwrap());
/// assert_eq!(old_card!(TD), Card::<French, French>::from_str("T♦").unwrap());
/// assert_eq!(old_card!(9D), Card::<French, French>::from_str("9♦").unwrap());
/// assert_eq!(old_card!(8D), Card::<French, French>::from_str("8♦").unwrap());
/// assert_eq!(old_card!(7D), Card::<French, French>::from_str("7♦").unwrap());
/// assert_eq!(old_card!(6D), Card::<French, French>::from_str("6♦").unwrap());
/// assert_eq!(old_card!(5D), Card::<French, French>::from_str("5♦").unwrap());
/// assert_eq!(old_card!(4D), Card::<French, French>::from_str("4♦").unwrap());
/// assert_eq!(old_card!(3D), Card::<French, French>::from_str("3♦").unwrap());
/// assert_eq!(old_card!(2D), Card::<French, French>::from_str("2♦").unwrap());
/// ```
#[macro_export]
macro_rules! old_card {
    (AS) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::ACE),
            Suit::<French>::new(French::SPADES),
        )
    };
    (KS) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::KING),
            Suit::<French>::new(French::SPADES),
        )
    };
    (QS) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::QUEEN),
            Suit::<French>::new(French::SPADES),
        )
    };
    (JS) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::JACK),
            Suit::<French>::new(French::SPADES),
        )
    };
    (TS) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::TEN),
            Suit::<French>::new(French::SPADES),
        )
    };
    (9S) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::NINE),
            Suit::<French>::new(French::SPADES),
        )
    };
    (8S) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::EIGHT),
            Suit::<French>::new(French::SPADES),
        )
    };
    (7S) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::SEVEN),
            Suit::<French>::new(French::SPADES),
        )
    };
    (6S) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::SIX),
            Suit::<French>::new(French::SPADES),
        )
    };
    (5S) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::FIVE),
            Suit::<French>::new(French::SPADES),
        )
    };
    (4S) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::FOUR),
            Suit::<French>::new(French::SPADES),
        )
    };
    (3S) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::THREE),
            Suit::<French>::new(French::SPADES),
        )
    };
    (2S) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::TWO),
            Suit::<French>::new(French::SPADES),
        )
    };
    (AH) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::ACE),
            Suit::<French>::new(French::HEARTS),
        )
    };
    (KH) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::KING),
            Suit::<French>::new(French::HEARTS),
        )
    };
    (QH) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::QUEEN),
            Suit::<French>::new(French::HEARTS),
        )
    };
    (JH) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::JACK),
            Suit::<French>::new(French::HEARTS),
        )
    };
    (TH) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::TEN),
            Suit::<French>::new(French::HEARTS),
        )
    };
    (9H) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::NINE),
            Suit::<French>::new(French::HEARTS),
        )
    };
    (8H) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::EIGHT),
            Suit::<French>::new(French::HEARTS),
        )
    };
    (7H) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::SEVEN),
            Suit::<French>::new(French::HEARTS),
        )
    };
    (6H) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::SIX),
            Suit::<French>::new(French::HEARTS),
        )
    };
    (5H) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::FIVE),
            Suit::<French>::new(French::HEARTS),
        )
    };
    (4H) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::FOUR),
            Suit::<French>::new(French::HEARTS),
        )
    };
    (3H) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::THREE),
            Suit::<French>::new(French::HEARTS),
        )
    };
    (2H) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::TWO),
            Suit::<French>::new(French::HEARTS),
        )
    };
    (AD) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::ACE),
            Suit::<French>::new(French::DIAMONDS),
        )
    };
    (KD) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::KING),
            Suit::<French>::new(French::DIAMONDS),
        )
    };
    (QD) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::QUEEN),
            Suit::<French>::new(French::DIAMONDS),
        )
    };
    (JD) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::JACK),
            Suit::<French>::new(French::DIAMONDS),
        )
    };
    (TD) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::TEN),
            Suit::<French>::new(French::DIAMONDS),
        )
    };
    (9D) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::NINE),
            Suit::<French>::new(French::DIAMONDS),
        )
    };
    (8D) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::EIGHT),
            Suit::<French>::new(French::DIAMONDS),
        )
    };
    (7D) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::SEVEN),
            Suit::<French>::new(French::DIAMONDS),
        )
    };
    (6D) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::SIX),
            Suit::<French>::new(French::DIAMONDS),
        )
    };
    (5D) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::FIVE),
            Suit::<French>::new(French::DIAMONDS),
        )
    };
    (4D) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::FOUR),
            Suit::<French>::new(French::DIAMONDS),
        )
    };
    (3D) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::THREE),
            Suit::<French>::new(French::DIAMONDS),
        )
    };
    (2D) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::TWO),
            Suit::<French>::new(French::DIAMONDS),
        )
    };
    (AC) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::ACE),
            Suit::<French>::new(French::CLUBS),
        )
    };
    (KC) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::KING),
            Suit::<French>::new(French::CLUBS),
        )
    };
    (QC) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::QUEEN),
            Suit::<French>::new(French::CLUBS),
        )
    };
    (JC) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::JACK),
            Suit::<French>::new(French::CLUBS),
        )
    };
    (TC) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::TEN),
            Suit::<French>::new(French::CLUBS),
        )
    };
    (9C) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::NINE),
            Suit::<French>::new(French::CLUBS),
        )
    };
    (8C) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::EIGHT),
            Suit::<French>::new(French::CLUBS),
        )
    };
    (7C) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::SEVEN),
            Suit::<French>::new(French::CLUBS),
        )
    };
    (6C) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::SIX),
            Suit::<French>::new(French::CLUBS),
        )
    };
    (5C) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::FIVE),
            Suit::<French>::new(French::CLUBS),
        )
    };
    (4C) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::FOUR),
            Suit::<French>::new(French::CLUBS),
        )
    };
    (3C) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::THREE),
            Suit::<French>::new(French::CLUBS),
        )
    };
    (2C) => {
        Card::<French, French>::new(
            Rank::<French>::new(French::TWO),
            Suit::<French>::new(French::CLUBS),
        )
    };
    (__) => {
        Card::<French, French>::default()
    };
    ($rank:expr, $suit:expr) => {
        Card::<French, French>::new(Rank::<French>::from($rank), Suit::<French>::from($suit))
    };
    ($card_str:expr) => {
        Card::<French, French>::from_str($card_str)
            .unwrap_or_else(|_| Card::<French, French>::default())
    };
}

#[macro_export]
macro_rules! old_cards {
    ($card_str:expr) => {
        Pile::<French, French>::from_str($card_str)
    };
}

/// `French` is a
/// [unit-like struct](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#unit-like-structs-without-any-fields)
/// that represents a deck made up of with [French suited playing cards](https://en.wikipedia.org/wiki/Standard_52-card_deck)
/// used for Bridge, Blackjack, and most variations of Poker. Many other decks will use its implementation of the
/// [`Suited`] trait while creating their own variation of [`Ranked`].
///
/// Here's how we instantiate a `French` deck from its base structs and traits. These are
/// used to create a `Pile` of `Cards`. It is functionally same as [Decked's](Decked) `deck()`
/// trait method.
///
/// ```
/// use cardpack::old::decks::french::French;
/// use cardpack::old::types::card::Card;
/// use cardpack::old::types::pile::Pile;
/// use cardpack::old::types::rank::Rank;
/// use cardpack::old::types::suit::Suit;
/// use cardpack::old::types::traits::Decked;
///
/// // use cardpack::prelude::*; also works
///
/// let ranks = Rank::<French>::ranks();
/// let suits = Suit::<French>::suits();
///
/// let mut pile = Pile::<French, French>::from(Vec::new());
///
/// for suit in &suits {
///     for rank in &ranks {
///         let card = Card::<French, French>::new(rank.clone(), suit.clone());
///         assert!(!card.is_blank());
///         pile.push(card);
///     }
/// }
///
/// let deck: Pile<French, French> = French::deck();
///
/// assert_eq!(deck, pile);
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct French {}

/// `Standard52` type alias for `French` to allow for people to use the old name for the `Deck`.
pub type Standard52 = French;

/// Type aliass that make it easier to work with the generic types.
#[allow(clippy::module_name_repetitions)]
pub type FrenchCard = Card<French, French>;
#[allow(clippy::module_name_repetitions)]
pub type FrenchDeck = Pile<French, French>;

impl French {
    pub const DECK_NAME: &'static str = "French";
    const GUIDE: &'static str = "xxxAKQJT 98765432 ♠♥♦♣rrrr xxpppppp";

    // https://github.com/forfailures/cardpack/actions/runs/11375156606/job/31645291021
    // I can't believe that running the tests through GitHub Actions against
    // Rust version 1.74 finally showed why the IDE was complaining about
    // `pub const ACE: & str = "ace";`. It needed the `'static` lifeline, which
    // for some reason still worked for the earlier and later versions of Rust.
    //
    // Here's the error from the logs:
    // error: `&` without an explicit lifetime name cannot be used here
    //   --> src/types/rank.rs:39:23
    //    |
    // 39 |     pub const LITTLE: &str = "little";
    //    |                       ^
    //    |
    //    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    //    = note: for more information, see issue #115010 <https://github.com/rust-lang/rust/issues/115010>
    // help: use the `'static` lifetime
    //    |
    // 39 |     pub const LITTLE: &'static str = "little";
    //    |                        +++++++
    pub const ACE: &'static str = "ace";
    pub const KING: &'static str = "king";
    pub const QUEEN: &'static str = "queen";
    pub const JACK: &'static str = "jack";
    pub const TEN: &'static str = "ten";
    pub const NINE: &'static str = "nine";
    pub const EIGHT: &'static str = "eight";
    pub const SEVEN: &'static str = "seven";
    pub const SIX: &'static str = "six";
    pub const FIVE: &'static str = "five";
    pub const FOUR: &'static str = "four";
    pub const THREE: &'static str = "three";
    pub const TWO: &'static str = "two";

    // Standard Suits
    pub const SPADES: &'static str = "spades";
    pub const HEARTS: &'static str = "hearts";
    pub const DIAMONDS: &'static str = "diamonds";
    pub const CLUBS: &'static str = "clubs";

    /// # Errors
    ///
    /// Returns a `CardError` if the index is out of bounds.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(index: &str) -> Result<Pile<French, French>, CardError> {
        Pile::<French, French>::from_str(index)
    }

    #[must_use]
    pub fn string_guided(ckc: u32) -> String {
        format!("{}\n{}", French::GUIDE, Bit::string(ckc))
    }
}

impl Decked<French, French> for French {
    fn blank() -> Card<French, French> {
        Card::<French, French>::default()
    }

    fn guide() -> Option<String> {
        Some(French::GUIDE.to_string())
    }
}

impl Ranked for French {
    fn rank_chars() -> Vec<char> {
        vec![
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 't', '0', 'J', 'j', 'Q', 'q', 'K', 'k',
            'A', 'a',
        ]
    }

    fn rank_names() -> Vec<&'static str> {
        vec![
            French::ACE,
            French::KING,
            French::QUEEN,
            French::JACK,
            French::TEN,
            French::NINE,
            French::EIGHT,
            French::SEVEN,
            French::SIX,
            French::FIVE,
            French::FOUR,
            French::THREE,
            French::TWO,
        ]
    }

    fn type_name() -> &'static str {
        French::DECK_NAME
    }
}

impl Suited for French {
    fn colors() -> HashMap<char, Color> {
        let mut mappie = HashMap::new();

        mappie.insert('H', Color::Red);
        mappie.insert('D', Color::Red);

        mappie
    }

    fn suit_chars() -> Vec<char> {
        vec![
            '♤', '♠', 'S', 's', '♡', '♥', 'H', 'h', '♢', '♦', 'D', 'd', '♧', '♣', 'C', 'c',
        ]
    }

    fn suit_names() -> Vec<&'static str> {
        vec![
            French::SPADES,
            French::HEARTS,
            French::DIAMONDS,
            French::CLUBS,
        ]
    }

    fn type_name() -> &'static str {
        French::DECK_NAME
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod decks__standard52__tests {
    use super::*;
    use crate::localization::{FluentName, Named};
    use crate::old::types::card::Card;
    use crate::old::types::rank::Rank;
    use crate::old::types::suit::Suit;
    use crate::old_card;
    use crate::old_cards;
    use ckc_rs::CardNumber;
    use rstest::rstest;
    use std::str::FromStr;

    #[test]
    fn card__from_str() {
        assert_eq!(old_card!("A♠"), old_card!(AS));
        assert_eq!(old_card!("K♠"), old_card!(KS));
        assert_eq!(old_card!("Q♠"), old_card!(QS));
        assert_eq!(old_card!("J♠"), old_card!(JS));
        assert_eq!(old_card!("T♠"), old_card!(TS));
        assert_eq!(old_card!("9♠"), old_card!(9S));
        assert_eq!(old_card!("8♠"), old_card!(8S));
        assert_eq!(old_card!("7♠"), old_card!(7S));
        assert_eq!(old_card!("6♠"), old_card!(6S));
        assert_eq!(old_card!("5♠"), old_card!(5S));
        assert_eq!(old_card!("4♠"), old_card!(4S));
        assert_eq!(old_card!("3♠"), old_card!(3S));
        assert_eq!(old_card!("2♠"), old_card!(2S));
        assert_eq!(old_card!("A♥"), old_card!(AH));
        assert_eq!(old_card!("K♥"), old_card!(KH));
        assert_eq!(old_card!("Q♥"), old_card!(QH));
        assert_eq!(old_card!("J♥"), old_card!(JH));
        assert_eq!(old_card!("T♥"), old_card!(TH));
        assert_eq!(old_card!("9♥"), old_card!(9H));
        assert_eq!(old_card!("8♥"), old_card!(8H));
        assert_eq!(old_card!("7♥"), old_card!(7H));
        assert_eq!(old_card!("6♥"), old_card!(6H));
        assert_eq!(old_card!("5♥"), old_card!(5H));
        assert_eq!(old_card!("4♥"), old_card!(4H));
        assert_eq!(old_card!("3♥"), old_card!(3H));
        assert_eq!(old_card!("2♥"), old_card!(2H));
        assert_eq!(old_card!("A♦"), old_card!(AD));
        assert_eq!(old_card!("K♦"), old_card!(KD));
        assert_eq!(old_card!("Q♦"), old_card!(QD));
        assert_eq!(old_card!("J♦"), old_card!(JD));
        assert_eq!(old_card!("T♦"), old_card!(TD));
        assert_eq!(old_card!("9♦"), old_card!(9D));
        assert_eq!(old_card!("8♦"), old_card!(8D));
        assert_eq!(old_card!("7♦"), old_card!(7D));
        assert_eq!(old_card!("6♦"), old_card!(6D));
        assert_eq!(old_card!("5♦"), old_card!(5D));
        assert_eq!(old_card!("4♦"), old_card!(4D));
        assert_eq!(old_card!("3♦"), old_card!(3D));
        assert_eq!(old_card!("2♦"), old_card!(2D));
        assert_eq!(old_card!("A♣"), old_card!(AC));
        assert_eq!(old_card!("K♣"), old_card!(KC));
        assert_eq!(old_card!("Q♣"), old_card!(QC));
        assert_eq!(old_card!("J♣"), old_card!(JC));
        assert_eq!(old_card!("T♣"), old_card!(TC));
        assert_eq!(old_card!("9♣"), old_card!(9C));
        assert_eq!(old_card!("8♣"), old_card!(8C));
        assert_eq!(old_card!("7♣"), old_card!(7C));
        assert_eq!(old_card!("6♣"), old_card!(6C));
        assert_eq!(old_card!("5♣"), old_card!(5C));
        assert_eq!(old_card!("4♣"), old_card!(4C));
        assert_eq!(old_card!("3♣"), old_card!(3C));
        assert_eq!(old_card!("2♣"), old_card!(2C));
        assert_eq!(old_card!("__"), old_card!(__));
    }

    #[test]
    fn macro_cards() {
        let deck = old_cards!("A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣").unwrap();

        assert_eq!(deck.to_string(), French::deck().to_string());
        assert!(old_cards!("AA xx __").is_err());
    }

    #[rstest]
    #[case("A♠", CardNumber::ACE_SPADES)]
    #[case("ks", CardNumber::KING_SPADES)]
    #[case("QS", CardNumber::QUEEN_SPADES)]
    #[case("J♠", CardNumber::JACK_SPADES)]
    #[case("TS", CardNumber::TEN_SPADES)]
    #[case("9s", CardNumber::NINE_SPADES)]
    #[case("8♠", CardNumber::EIGHT_SPADES)]
    #[case("7S", CardNumber::SEVEN_SPADES)]
    #[case("6♠", CardNumber::SIX_SPADES)]
    #[case("5S", CardNumber::FIVE_SPADES)]
    #[case("4♠", CardNumber::FOUR_SPADES)]
    #[case("3s", CardNumber::TREY_SPADES)]
    #[case("2S", CardNumber::DEUCE_SPADES)]
    #[case("A♥", CardNumber::ACE_HEARTS)]
    #[case("k♥", CardNumber::KING_HEARTS)]
    #[case("QH", CardNumber::QUEEN_HEARTS)]
    #[case("jh", CardNumber::JACK_HEARTS)]
    #[case("T♥", CardNumber::TEN_HEARTS)]
    #[case("9♥", CardNumber::NINE_HEARTS)]
    #[case("8h", CardNumber::EIGHT_HEARTS)]
    #[case("7H", CardNumber::SEVEN_HEARTS)]
    #[case("6h", CardNumber::SIX_HEARTS)]
    #[case("5H", CardNumber::FIVE_HEARTS)]
    #[case("4♥", CardNumber::FOUR_HEARTS)]
    #[case("3♥", CardNumber::TREY_HEARTS)]
    #[case("2h", CardNumber::DEUCE_HEARTS)]
    #[case("A♦", CardNumber::ACE_DIAMONDS)]
    #[case("k♦", CardNumber::KING_DIAMONDS)]
    #[case("Q♦", CardNumber::QUEEN_DIAMONDS)]
    #[case("Jd", CardNumber::JACK_DIAMONDS)]
    #[case("tD", CardNumber::TEN_DIAMONDS)]
    #[case("9♦", CardNumber::NINE_DIAMONDS)]
    #[case("8D", CardNumber::EIGHT_DIAMONDS)]
    #[case("7♦", CardNumber::SEVEN_DIAMONDS)]
    #[case("6D", CardNumber::SIX_DIAMONDS)]
    #[case("5D", CardNumber::FIVE_DIAMONDS)]
    #[case("4♦", CardNumber::FOUR_DIAMONDS)]
    #[case("3♦", CardNumber::TREY_DIAMONDS)]
    #[case("2d", CardNumber::DEUCE_DIAMONDS)]
    #[case("a♣", CardNumber::ACE_CLUBS)]
    #[case("k♣", CardNumber::KING_CLUBS)]
    #[case("QC", CardNumber::QUEEN_CLUBS)]
    #[case("jc", CardNumber::JACK_CLUBS)]
    #[case("tC", CardNumber::TEN_CLUBS)]
    #[case("9♣", CardNumber::NINE_CLUBS)]
    #[case("8♣", CardNumber::EIGHT_CLUBS)]
    #[case("7c", CardNumber::SEVEN_CLUBS)]
    #[case("6♣", CardNumber::SIX_CLUBS)]
    #[case("5C", CardNumber::FIVE_CLUBS)]
    #[case("4c", CardNumber::FOUR_CLUBS)]
    #[case("3C", CardNumber::TREY_CLUBS)]
    #[case("2C", CardNumber::DEUCE_CLUBS)]
    #[case("__", 0u32)]
    fn card__get_ckc_number(#[case] input: &str, #[case] expected_ckc: u32) {
        assert_eq!(expected_ckc, old_card!(input).get_ckc_number());
    }

    #[test]
    fn card__get_ckc_number__blank() {
        let blank = old_card!("__");
        assert!(blank.is_blank());
        assert_eq!(0, blank.get_ckc_number());
    }

    #[test]
    fn decked__decks() {
        let pile = French::decks(2);

        assert_eq!(
            pile.to_string(),
            "A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣ A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣"
        );
    }

    #[test]
    fn decked__name() {
        assert_eq!(French::name(), "French");
    }

    #[test]
    fn rank__new() {
        let rank = Rank::<French>::new(French::ACE);

        assert_eq!(rank.name, FluentName::new(French::ACE));
        assert_eq!(rank.weight, 12);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn rank__new_with_weight() {
        let rank = Rank::<French>::new_with_weight(French::ACE, 13);

        assert_eq!(rank.name, FluentName::new(French::ACE));
        assert_eq!(rank.weight, 13);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn rank__update_weight() {
        let rank = Rank::<French>::new(French::ACE);
        let updated_rank = rank.update_weight(14);

        assert_eq!(updated_rank.name, FluentName::new(French::ACE));
        assert_eq!(updated_rank.weight, 14);
        assert_eq!(updated_rank.prime, 41);
    }

    #[test]
    fn rank__weighted_vector() {
        let mut v = Rank::<French>::rank_names();
        v.reverse();

        let ranks = Rank::<French>::weighted_vector(&v);

        assert_eq!(ranks.len(), 13);
        assert_eq!(ranks[0].weight, 12);
        assert_eq!(ranks[0].name.fluent_name_string(), "two");
        assert_eq!(ranks[1].weight, 11);
        assert_eq!(ranks[1].name.fluent_name_string(), "three");
    }

    #[test]
    fn rank__display() {
        let rank = Rank::<French>::new(French::ACE);

        assert_eq!("A", format!("{rank}"));
    }

    #[test]
    fn rank__display_blank() {
        let rank = Rank::<French>::from('_');

        assert_eq!("_", format!("{rank}"));
    }

    #[test]
    fn rank__ranks() {
        assert_eq!(
            "A K Q J T 9 8 7 6 5 4 3 2",
            &Rank::<French>::ranks_index_all(" ")
        );
    }

    #[test]
    fn ranked__named__fluent_name() {
        let rank = Rank::<French>::new(French::KING);

        assert_eq!(rank.fluent_name(), &FluentName::new(French::KING));
    }

    #[test]
    fn ranked__named__fluent_name_string() {
        let rank = Rank::<French>::new(French::QUEEN);

        assert_eq!(rank.fluent_name_string(), French::QUEEN);
    }

    #[test]
    fn ranked__named__is_blank() {
        let rank = Rank::<French>::new(French::ACE);

        assert!(!rank.is_blank());
    }

    #[test]
    fn ranked__ranked__names() {
        let names = Rank::<French>::rank_names();

        assert_eq!(names.len(), 13);
        assert_eq!(names[0], French::ACE);
        assert_eq!(names[1], French::KING);
        assert_eq!(names[2], French::QUEEN);
        assert_eq!(names[3], French::JACK);
        assert_eq!(names[4], French::TEN);
        assert_eq!(names[5], French::NINE);
        assert_eq!(names[6], French::EIGHT);
        assert_eq!(names[7], French::SEVEN);
        assert_eq!(names[8], French::SIX);
        assert_eq!(names[9], French::FIVE);
        assert_eq!(names[10], French::FOUR);
        assert_eq!(names[11], French::THREE);
        assert_eq!(names[12], French::TWO);
    }

    #[test]
    fn ranked__is_valid_char() {
        assert!(Rank::<French>::is_valid_rank_char(&'A'));
        assert!(!Rank::<French>::is_valid_rank_char(&'Z'));
    }

    #[test]
    fn suited__colors() {
        let mut expected = HashMap::new();
        expected.insert('H', Color::Red);
        expected.insert('D', Color::Red);

        let actual = Suit::<French>::colors();

        assert_eq!(actual, expected);
    }

    #[test]
    fn suited__is_valid_suit_char() {
        assert!(Suit::<French>::is_valid_suit_char(&'H'));
        assert!(Suit::<French>::is_valid_suit_char(&'h'));
        assert!(Suit::<French>::is_valid_suit_char(&'♥'));
        assert!(!Suit::<French>::is_valid_suit_char(&'_'));
        assert!(!Suit::<French>::is_valid_suit_char(&'W'));
    }

    #[test]
    fn suited__suit_chars() {
        let expected = vec![
            '♤', '♠', 'S', 's', '♡', '♥', 'H', 'h', '♢', '♦', 'D', 'd', '♧', '♣', 'C', 'c',
        ];

        let chars = Suit::<French>::suit_chars();

        assert_eq!(chars, expected);
    }

    #[test]
    fn suited__suit_names() {
        let expected = vec![
            French::SPADES,
            French::HEARTS,
            French::DIAMONDS,
            French::CLUBS,
        ];

        let names = Suit::<French>::suit_names();

        assert_eq!(names, expected);
    }

    #[test]
    fn pile__sort() {
        let deck = French::deck();
        let mut shuffled = deck.shuffle();

        shuffled.shuffle_in_place();
        shuffled.sort_in_place();

        assert_eq!(deck.to_string(), shuffled.to_string());
    }

    #[test]
    fn pile__ranks_by_suit() {
        let deck = French::deck();

        let ranks = deck
            .ranks_by_suit(&Suit::<French>::new(French::SPADES))
            .unwrap();
        let index = Rank::<French>::ranks_index(&ranks, " ");

        assert_eq!(13, ranks.len());
        assert_eq!("A K Q J T 9 8 7 6 5 4 3 2", index);
    }

    #[test]
    fn pile__ranks_by_suit__none() {
        let deck = Pile::<French, French>::default();

        let ranks = deck.ranks_by_suit(&Suit::<French>::new(French::CLUBS));

        assert!(ranks.is_none());
    }

    #[test]
    fn to_string__from_str() {
        let deck = French::deck();
        let shuffled = deck.shuffle().to_string();
        let parsed = French::from_str(&shuffled).unwrap();

        assert!(deck.same(&parsed));
    }

    #[test]
    fn string_guided() {
        let ckc = 0b0000_0000_0000_0000_0000_0000_0000_0000;
        let expected = "xxxAKQJT 98765432 ♠♥♦♣rrrr xxpppppp\n00000000 00000000 00000000 00000000";
        assert_eq!(French::string_guided(ckc), expected);

        let ckc = 0b1111_1111_1111_1111_1111_1111_1111_1111;
        let expected = "xxxAKQJT 98765432 ♠♥♦♣rrrr xxpppppp\n11111111 11111111 11111111 11111111";
        assert_eq!(French::string_guided(ckc), expected);

        let ckc = 0b1010_1010_1010_1010_1010_1010_1010_1010;
        let expected = "xxxAKQJT 98765432 ♠♥♦♣rrrr xxpppppp\n10101010 10101010 10101010 10101010";
        assert_eq!(French::string_guided(ckc), expected);
    }
}
