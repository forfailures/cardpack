use crate::decks::french::French;
use crate::types::card::Card;
use crate::types::card_error::CardError;
use crate::types::pile::Pile;
use crate::types::rank::Rank;
use crate::types::suit::Suit;
use crate::types::traits::{Decked, Ranked, Suited};
use colored::Color;
use std::collections::HashMap;
use std::str::FromStr;

#[macro_export]
#[allow(clippy::pedantic)]
macro_rules! modern_card {
    ($card_str:expr) => {
        Card::<Modern, Modern>::from_str($card_str)
            .unwrap_or_else(|_| Card::<Modern, Modern>::default())
    };
}

#[macro_export]
macro_rules! modern {
    ($card_str:expr) => {
        Pile::<Modern, Modern>::from_str($card_str)
    };
}

/// `French` with Jokers.
///
/// <https://www.pagat.com/rummy/canasta.html#classic-threes>
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Modern {}

#[allow(clippy::module_name_repetitions)]
pub type ModernCard = Card<Modern, Modern>;
#[allow(clippy::module_name_repetitions)]
pub type ModernDeck = Pile<Modern, Modern>;

impl Modern {
    pub const DECK_NAME: &'static str = "Modern";

    // Jokers Fluent Names
    pub const BIG: &'static str = "big-joker";
    pub const LITTLE: &'static str = "little-joker";

    // Rank
    pub const JOKER: &'static str = "joker";

    #[must_use]
    pub fn big_joker() -> Card<Modern, Modern> {
        Card::new(&Rank::new(Self::BIG), &Suit::new(Self::JOKER))
    }

    #[must_use]
    pub fn little_joker() -> Card<Modern, Modern> {
        Card::new(&Rank::new(Self::LITTLE), &Suit::new(Self::JOKER))
    }

    #[must_use]
    pub fn jokers() -> Pile<Modern, Modern> {
        let mut pile = Pile::<Modern, Modern>::from(Vec::new());

        pile.push(Self::big_joker());
        pile.push(Self::little_joker());

        pile
    }

    /// # Errors
    ///
    /// Returns a `CardError` if the index is out of bounds.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(index: &str) -> Result<Pile<Modern, Modern>, CardError> {
        Pile::<Modern, Modern>::from_str(index)
    }
}

impl Decked<Modern, Modern> for Modern {
    fn deck() -> Pile<Modern, Modern> {
        let mut deck = Modern::jokers();

        // TODO: HACK
        let raw52 = French::deck().to_string();
        let base52 = Pile::<Modern, Modern>::from_str(&raw52).unwrap();

        deck.extend(&base52);

        deck
    }

    fn blank() -> Card<Modern, Modern> {
        Card::<Modern, Modern>::default()
    }

    fn guide() -> Option<String> {
        None
    }
}

impl Ranked for Modern {
    fn rank_chars() -> Vec<char> {
        vec![
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 't', '0', 'J', 'j', 'Q', 'q', 'K', 'k',
            'A', 'a', 'B', 'b', 'L', 'l',
        ]
    }

    fn rank_names() -> Vec<&'static str> {
        vec![
            Modern::BIG,
            Modern::LITTLE,
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
        Modern::DECK_NAME
    }
}

impl Suited for Modern {
    fn colors() -> HashMap<char, Color> {
        let mut mappie = HashMap::new();

        mappie.insert('T', Color::BrightBlue);
        mappie.insert('H', Color::Red);
        mappie.insert('D', Color::Red);

        mappie
    }

    fn suit_chars() -> Vec<char> {
        vec![
            '♤', '♠', 'S', 's', '♡', '♥', 'H', 'h', '♢', '♦', 'D', 'd', '♧', '♣', 'C', 'c', '🃟',
            'T', 't', 'J', 'j',
        ]
    }

    fn suit_names() -> Vec<&'static str> {
        vec![
            Modern::JOKER,
            French::SPADES,
            French::HEARTS,
            French::DIAMONDS,
            French::CLUBS,
        ]
    }

    fn type_name() -> &'static str {
        Modern::DECK_NAME
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod decks__modern__tests {
    use super::*;
    use crate::localization::{FluentName, Named};
    use crate::modern;
    use crate::modern_card;
    use crate::types::rank::Rank;
    use std::str::FromStr;

    #[test]
    fn new() {
        let rank = Rank::<Modern>::new(Modern::LITTLE);

        assert_eq!(rank.name, FluentName::new(Modern::LITTLE));
        assert_eq!(rank.weight, 13);
        assert_eq!(rank.prime, 43);
    }

    #[test]
    fn new_with_weight() {
        let rank = Rank::<Modern>::new_with_weight(Modern::BIG, 13);

        assert_eq!(rank.name, FluentName::new(Modern::BIG));
        assert_eq!(rank.weight, 13);
        assert_eq!(rank.prime, 47);
    }

    #[test]
    fn update_weight() {
        let rank = Rank::<Modern>::new(French::ACE);
        let updated_rank = rank.update_weight(14);

        assert_eq!(updated_rank.name, FluentName::new(French::ACE));
        assert_eq!(updated_rank.weight, 14);
        assert_eq!(updated_rank.prime, 41);
    }

    #[test]
    fn big_joker() {
        let card = Modern::big_joker();

        assert_eq!("BJ", card.index);
        assert_eq!(card.rank.name, FluentName::new(Modern::BIG));
        assert_eq!(card.suit.name, FluentName::new(Modern::JOKER));
    }

    #[test]
    fn little_joker() {
        let card = Modern::little_joker();

        assert_eq!("LJ", card.index);
        assert_eq!(card.rank.name, FluentName::new(Modern::LITTLE));
        assert_eq!(card.suit.name, FluentName::new(Modern::JOKER));
    }

    #[test]
    fn card__from_str() {
        let card = modern_card!("A♠");

        assert_eq!(card.index, "AS");
        assert_eq!(card.rank.name, FluentName::new(French::ACE));
        assert_eq!(card.suit.name, FluentName::new(French::SPADES));
    }

    #[test]
    fn decked__deck() {
        let deck = Modern::deck();
        let mut shuffled = deck.shuffle();
        shuffled.sort_in_place();

        assert_eq!(54, deck.len());
        assert_eq!("B🃟 L🃟 A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣", deck.to_string());
        assert_eq!(deck.to_string(), shuffled.to_string());
    }

    #[test]
    fn ranked__is_valid_char() {
        assert!(Rank::<Modern>::is_valid_rank_char(&'A'));
        assert!(!Rank::<Modern>::is_valid_rank_char(&'Z'));
    }

    #[test]
    fn rank__from_char() {
        let rank = Rank::<Modern>::from('A');

        assert_eq!(rank.name, FluentName::new(French::ACE));
        assert_eq!(rank.weight, 12);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn rank__from_str() {
        let rank = Rank::<Modern>::from_str("A'").unwrap();

        assert_eq!(rank.name, FluentName::new(French::ACE));
        assert_eq!(rank.weight, 12);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn named__fluent_name() {
        let rank = Rank::<Modern>::new(French::KING);

        assert_eq!(rank.fluent_name(), &FluentName::new(French::KING));
    }

    #[test]
    fn named__fluent_name_string() {
        let rank = Rank::<Modern>::new(French::QUEEN);

        assert_eq!(rank.fluent_name_string(), French::QUEEN);
    }

    #[test]
    fn named__is_blank() {
        let rank = Rank::<Modern>::new(French::ACE);

        assert!(!rank.is_blank());
    }

    #[test]
    fn ranked__names() {
        let names = Rank::<Modern>::rank_names();

        assert_eq!(names.len(), 15);
        assert_eq!(names[0], Modern::BIG);
        assert_eq!(names[1], Modern::LITTLE);
        assert_eq!(names[2], French::ACE);
        assert_eq!(names[3], French::KING);
        assert_eq!(names[4], French::QUEEN);
        assert_eq!(names[5], French::JACK);
        assert_eq!(names[6], French::TEN);
        assert_eq!(names[7], French::NINE);
        assert_eq!(names[8], French::EIGHT);
        assert_eq!(names[9], French::SEVEN);
        assert_eq!(names[10], French::SIX);
        assert_eq!(names[11], French::FIVE);
        assert_eq!(names[12], French::FOUR);
        assert_eq!(names[13], French::THREE);
        assert_eq!(names[14], French::TWO);
    }

    #[test]
    fn pile__sort() {
        let deck = Modern::deck();
        let mut shuffled = deck.shuffle();

        shuffled.shuffle_in_place();
        shuffled.sort_in_place();

        assert_eq!(deck.to_string(), shuffled.to_string());
    }

    #[test]
    fn to_string__from_str() {
        let deck = Modern::deck();
        let shuffled = deck.shuffle().to_string();
        let parsed = modern!(&shuffled).unwrap();

        assert!(deck.same(&parsed));
    }
}
