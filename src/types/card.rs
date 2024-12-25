use crate::localization::Named;
use crate::types::card_error::CardError;
use crate::types::rank::Rank;
use crate::types::suit::Suit;
use crate::types::traits::{Ranked, Suited};
use std::fmt::Display;

use colored::Colorize;
use std::str::FromStr;

/// `Card` is the fundamental struct of this library, being an abstract representation of a playing
/// card.
///
/// Each Card is made up of [unit-like struct](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#unit-like-structs-without-any-fields)
/// that is tied to a [`Rank`] that implements the [`Ranked`] trait, and a [`Suit`]
/// that implements the [`Suited`] trait.
///
/// The goal of this structure is to allow for the flexibility in how decks of cards are represented.
/// For example, a standard 52 card deck would use the [`Standard52`](crate::decks::standard52::Standard52)
/// _unit-like struct_.
///
/// ```
/// use cardpack::prelude::*;
///
/// let rank = Rank::<Standard52>::new(Standard52::JACK);
/// let suit = Suit::<Standard52>::new(Standard52::CLUBS);
/// let card = Card::<Standard52, Standard52>::new(rank, suit);
///
/// assert_eq!(card.to_string(), "J♣");
/// ```
///
/// Many of the decks have macros to make instantiating a single `Card` or a
/// [`Pile`](crate::types::pile::Pile) of `Cards`:
///
/// ```
/// use cardpack::prelude::*;
///
/// let card = s52card!("JC");
/// assert_eq!(card.to_string(), "J♣");
///
/// let pile = standard52!("KS QC");
/// assert_eq!(pile.unwrap().to_string(), "K♠ Q♣");
/// ```
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

impl<RankType, SuitType> Card<RankType, SuitType>
where
    RankType: Ranked + Clone,
    SuitType: Suited + Clone,
{
    /// Creates a new `Card` with a [`Rank`] and [`Suit`]. The `weight` field is determined by
    /// `(suit.weight * 1000) + rank.weight`.
    ///
    /// ```
    /// use cardpack::prelude::*;
    ///
    /// let expected: Card<Standard52, Standard52> = Card {
    ///     weight: 4012,
    ///     index: "AS".to_string(),
    ///     rank: Rank::<Standard52>::from('A'),
    ///     suit: Suit::<Standard52>::from('S'),
    /// };
    ///
    /// let ace = Rank::<Standard52>::from('A');
    /// let spades = Suit::<Standard52>::from('S');
    /// let card: Card<Standard52, Standard52> = Card::new(ace, spades);
    ///
    /// assert_eq!(card, expected);
    /// ```
    #[must_use]
    pub fn new(rank: Rank<RankType>, suit: Suit<SuitType>) -> Self {
        Self {
            weight: Self::determine_weight(&suit, &rank),
            index: Card::determine_default_index(&suit, &rank),
            suit,
            rank,
        }
    }

    #[must_use]
    pub fn new_weighted(rank: Rank<RankType>, suit: Suit<SuitType>, weight: u32) -> Self {
        Self {
            weight,
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

    /// Used to generate the `Card`'s binary signature, aka [Cactus Kev](https://suffe.cool/poker/evaluator.html)
    /// numbers.
    #[must_use]
    pub fn get_ckc_number(&self) -> u32 {
        if self.is_blank() {
            return 0;
        }
        self.rank.ckc_number() + self.suit.ckc_number()
    }

    #[must_use]
    pub fn get_index_suit_char(&self) -> char {
        self.index.chars().last().unwrap_or('_')
    }

    #[must_use]
    pub fn is_blank(&self) -> bool {
        self.rank.name.is_blank() || self.suit.name.is_blank()
    }

    #[must_use]
    pub fn to_color_symbol_string(&self) -> String {
        let suit_char = self.get_index_suit_char();
        if let Some(color) = Suit::<SuitType>::colors().get(&suit_char) {
            match color {
                colored::Color::Red => self.to_string().red().to_string(),
                colored::Color::Blue => self.to_string().blue().to_string(),
                colored::Color::Green => self.to_string().green().to_string(),
                colored::Color::Yellow => self.to_string().yellow().to_string(),
                colored::Color::Magenta => self.to_string().magenta().to_string(),
                colored::Color::Cyan => self.to_string().cyan().to_string(),
                colored::Color::BrightBlack => self.to_string().bright_black().to_string(),
                colored::Color::BrightRed => self.to_string().bright_red().to_string(),
                colored::Color::BrightGreen => self.to_string().bright_green().to_string(),
                colored::Color::BrightYellow => self.to_string().bright_yellow().to_string(),
                colored::Color::BrightBlue => self.to_string().bright_blue().to_string(),
                colored::Color::BrightMagenta => self.to_string().bright_magenta().to_string(),
                colored::Color::BrightCyan => self.to_string().bright_cyan().to_string(),
                _ => self.to_string(),
            }
        } else {
            self.to_string()
        }
    }
}

impl<RankType: Ranked + Clone, SuitType: Suited + Clone> Default for Card<RankType, SuitType> {
    fn default() -> Self {
        Card {
            weight: 0,
            index: "__".to_string(),
            rank: Rank::<RankType>::default(),
            suit: Suit::<SuitType>::default(),
        }
    }
}

impl<RankType: Ranked + Clone, SuitType: Suited + Clone> Display for Card<RankType, SuitType> {
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

    /// Just got hit with this nasty bug:
    /// [Calculating String length and width – Fun with Unicode](https://tomdebruijn.com/posts/rust-string-length-width-calculations/)
    ///
    /// Unicode string length is bite sized not char count
    ///
    /// Man, I look at how I handled this with cardpacjk.rs and Fudd, and it is the hackiest hacky hack I have
    /// ever seen. It's an abomination of pain.
    ///
    /// TODO: One thing I would highly recommend is that you draw out a sequence diagram
    /// of your code flows to see just how a bill becomes a low. (I am a bill, and I am only a bill...)
    /// See just how convoluted your code is.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        // original was `if s.len() != 2 {`
        if s.chars().count() != 2 {
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
    use crate::s52card;

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

    #[test]
    fn get_index_suit_char() {
        assert_eq!(
            Card::<Standard52, Standard52>::from_str("AS")
                .unwrap()
                .get_index_suit_char(),
            'S'
        );
        assert_eq!(
            Card::<Standard52, Standard52>::from_str("__")
                .unwrap()
                .get_index_suit_char(),
            '_'
        );
    }

    /// I want to make sure that the weight field in the `Card` struct correctly affects the sorting
    /// of cards. For Decks of cards, the sort would be in reverse, with the higher weighted `Card`
    /// coming first.
    #[test]
    fn test_sort_from_weight() {
        let ace_of_spades = s52card!("AS");
        let ace_of_hearts = s52card!("AH");
        let ace_of_diamonds = s52card!("AD");
        let ace_of_clubs = s52card!("AC");

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
    fn to_color_symbol_string__default() {
        let card = s52card!("AS");

        assert_eq!("A♠".to_string(), card.to_color_symbol_string());
    }

    #[test]
    fn to_color_symbol_string() {
        let card = s52card!("AH");

        assert_eq!("A♥".red().to_string(), card.to_color_symbol_string());
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

        let card = s52card!("AS");

        assert_eq!(card, expected_card);
        assert!(!card.is_blank());
    }

    #[test]
    fn from_str_blank() {
        let card = s52card!("BW");

        assert!(card.is_blank());
    }

    #[test]
    fn from_str__symbol() {
        let card = s52card!("AS");

        assert_eq!(card.index, "AS");
        assert_eq!(card.rank.name, FluentName::new(Standard52::ACE));
        assert_eq!(card.suit.name, FluentName::new(Standard52::SPADES));
    }
}
