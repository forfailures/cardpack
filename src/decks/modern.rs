use crate::decks::standard52::Standard52;
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

/// `Standard52` with Jokers.
///
/// <https://www.pagat.com/rummy/canasta.html#classic-threes>
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Modern {}

impl Modern {
    pub const DECK_NAME: &'static str = "Modern";

    // Jokers Fluent Names
    pub const BIG: &'static str = "big-joker";
    pub const LITTLE: &'static str = "little-joker";

    // Rank
    pub const TRUMP: &'static str = "trump";

    #[must_use]
    pub fn big_joker() -> Card<Modern, Modern> {
        Card::new(Rank::new(Self::BIG), Suit::new(Self::TRUMP))
    }

    #[must_use]
    pub fn little_joker() -> Card<Modern, Modern> {
        Card::new(Rank::new(Self::LITTLE), Suit::new(Self::TRUMP))
    }

    #[must_use]
    pub fn jokers() -> Pile<Modern, Modern> {
        let mut pile = Pile::<Modern, Modern>::new(Vec::new());

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
        let raw52 = Standard52::deck().to_string();
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

    fn pack(&self) -> Pile<Modern, Modern> {
        Modern::deck()
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
            Standard52::ACE,
            Standard52::KING,
            Standard52::QUEEN,
            Standard52::JACK,
            Standard52::TEN,
            Standard52::NINE,
            Standard52::EIGHT,
            Standard52::SEVEN,
            Standard52::SIX,
            Standard52::FIVE,
            Standard52::FOUR,
            Standard52::THREE,
            Standard52::TWO,
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
            'T', 't',
        ]
    }

    fn suit_names() -> Vec<&'static str> {
        vec![
            Modern::TRUMP,
            Standard52::SPADES,
            Standard52::HEARTS,
            Standard52::DIAMONDS,
            Standard52::CLUBS,
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
        let rank = Rank::<Modern>::new(Standard52::ACE);
        let updated_rank = rank.update_weight(14);

        assert_eq!(updated_rank.name, FluentName::new(Standard52::ACE));
        assert_eq!(updated_rank.weight, 14);
        assert_eq!(updated_rank.prime, 41);
    }

    #[test]
    fn big_joker() {
        let card = Modern::big_joker();

        assert_eq!("BT", card.index);
        assert_eq!(card.rank.name, FluentName::new(Modern::BIG));
        assert_eq!(card.suit.name, FluentName::new(Modern::TRUMP));
    }

    #[test]
    fn little_joker() {
        let card = Modern::little_joker();

        assert_eq!("LT", card.index);
        assert_eq!(card.rank.name, FluentName::new(Modern::LITTLE));
        assert_eq!(card.suit.name, FluentName::new(Modern::TRUMP));
    }

    #[test]
    fn card__from_str() {
        let card = modern_card!("A♠");

        assert_eq!(card.index, "AS");
        assert_eq!(card.rank.name, FluentName::new(Standard52::ACE));
        assert_eq!(card.suit.name, FluentName::new(Standard52::SPADES));
    }

    #[test]
    fn decked__deck() {
        let deck = Modern::deck();
        let mut shuffled = deck.shuffle_default();
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

        assert_eq!(rank.name, FluentName::new(Standard52::ACE));
        assert_eq!(rank.weight, 12);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn rank__from_str() {
        let rank = Rank::<Modern>::from_str("A'").unwrap();

        assert_eq!(rank.name, FluentName::new(Standard52::ACE));
        assert_eq!(rank.weight, 12);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn named__fluent_name() {
        let rank = Rank::<Modern>::new(Standard52::KING);

        assert_eq!(rank.fluent_name(), &FluentName::new(Standard52::KING));
    }

    #[test]
    fn named__fluent_name_string() {
        let rank = Rank::<Modern>::new(Standard52::QUEEN);

        assert_eq!(rank.fluent_name_string(), Standard52::QUEEN);
    }

    #[test]
    fn named__is_blank() {
        let rank = Rank::<Modern>::new(Standard52::ACE);

        assert!(!rank.is_blank());
    }

    #[test]
    fn ranked__names() {
        let names = Rank::<Modern>::rank_names();

        assert_eq!(names.len(), 15);
        assert_eq!(names[0], Modern::BIG);
        assert_eq!(names[1], Modern::LITTLE);
        assert_eq!(names[2], Standard52::ACE);
        assert_eq!(names[3], Standard52::KING);
        assert_eq!(names[4], Standard52::QUEEN);
        assert_eq!(names[5], Standard52::JACK);
        assert_eq!(names[6], Standard52::TEN);
        assert_eq!(names[7], Standard52::NINE);
        assert_eq!(names[8], Standard52::EIGHT);
        assert_eq!(names[9], Standard52::SEVEN);
        assert_eq!(names[10], Standard52::SIX);
        assert_eq!(names[11], Standard52::FIVE);
        assert_eq!(names[12], Standard52::FOUR);
        assert_eq!(names[13], Standard52::THREE);
        assert_eq!(names[14], Standard52::TWO);
    }

    #[test]
    fn pile__sort() {
        let deck = Modern::deck();
        let mut shuffled = deck.shuffle_default();

        shuffled.shuffle_in_place_default();
        shuffled.sort_in_place();

        assert_eq!(deck.to_string(), shuffled.to_string());
    }

    #[test]
    fn to_string__from_str() {
        let deck = Modern::deck();
        let shuffled = deck.shuffle_default().to_string();
        let parsed = modern!(&shuffled).unwrap();

        assert!(deck.same(&parsed));
    }
}
