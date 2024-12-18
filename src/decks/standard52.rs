use crate::types::pile::Pile;
use crate::types::traits::{Decked, Ranked, Suited};
use colored::Color;
use std::collections::HashMap;
use std::str::FromStr;
use crate::types::card_error::CardError;

/// The [Standard52](https://en.wikipedia.org/wiki/Standard_52-card_deck)
/// deck with French suited playing cards is
/// the one used for Bridge, Blackjack, and most variations of
/// Poker. Many other decks will use its implementation of the
/// [Suited] trait while creating their own variation of [Ranked].
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Standard52 {}

impl Standard52 {
    pub const DECK_NAME: &'static str = "Standard52";

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
    pub fn from_str(index: &str) -> Result<Pile<Standard52, Standard52>, CardError> {
        Pile::<Standard52, Standard52>::from_str(index)
    }
}

impl Decked<Standard52, Standard52> for Standard52 {
    fn pack(&self) -> Pile<Standard52, Standard52> {
        Standard52::deck()
    }
}

impl Ranked for Standard52 {
    fn rank_chars() -> Vec<char> {
        vec![
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 't', '0', 'J', 'j', 'Q', 'q', 'K', 'k',
            'A', 'a',
        ]
    }

    fn rank_names() -> Vec<&'static str> {
        vec![
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
        Standard52::DECK_NAME
    }
}

impl Suited for Standard52 {
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
            Standard52::SPADES,
            Standard52::HEARTS,
            Standard52::DIAMONDS,
            Standard52::CLUBS,
        ]
    }

    fn type_name() -> &'static str {
        Standard52::DECK_NAME
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod decks__standard52__tests {
    use super::*;
    use crate::localization::{FluentName, Named};
    use crate::types::card_error::CardError;
    use crate::types::rank::Rank;
    use crate::types::suit::Suit;
    use rstest::rstest;
    use std::str::FromStr;

    #[test]
    fn decked__decks() {
        let pile = Standard52::decks(2);

        assert_eq!(
            pile.to_string(),
            "A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣ A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣"
        );
    }

    #[test]
    fn decked__name() {
        assert_eq!(Standard52::name(), "Standard52");
    }

    #[test]
    fn rank__new() {
        let rank = Rank::<Standard52>::new(Standard52::ACE);

        assert_eq!(rank.name, FluentName::new(Standard52::ACE));
        assert_eq!(rank.weight, 12);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn rank__new_with_weight() {
        let rank = Rank::<Standard52>::new_with_weight(Standard52::ACE, 13);

        assert_eq!(rank.name, FluentName::new(Standard52::ACE));
        assert_eq!(rank.weight, 13);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn rank__update_weight() {
        let rank = Rank::<Standard52>::new(Standard52::ACE);
        let updated_rank = rank.update_weight(14);

        assert_eq!(updated_rank.name, FluentName::new(Standard52::ACE));
        assert_eq!(updated_rank.weight, 14);
        assert_eq!(updated_rank.prime, 41);
    }

    #[test]
    fn rank__weighted_vector() {
        let mut v = Rank::<Standard52>::rank_names();
        v.reverse();

        let ranks = Rank::<Standard52>::weighted_vector(&v);

        assert_eq!(ranks.len(), 13);
        assert_eq!(ranks[0].weight, 0);
        assert_eq!(ranks[0].name.fluent_name_string(), "two");
        assert_eq!(ranks[1].weight, 1);
        assert_eq!(ranks[1].name.fluent_name_string(), "three");
    }

    #[test]
    fn rank__display() {
        let rank = Rank::<Standard52>::new(Standard52::ACE);

        assert_eq!("A", format!("{rank}"));
    }

    #[test]
    fn rank__display_blank() {
        let rank = Rank::<Standard52>::from('_');

        assert_eq!("_", format!("{rank}"));
    }

    #[test]
    fn rank__ranks() {
        assert_eq!(
            "A K Q J T 9 8 7 6 5 4 3 2",
            Rank::<Standard52>::ranks_index_all(" ")
        );
    }

    #[test]
    fn ranked__named__fluent_name() {
        let rank = Rank::<Standard52>::new(Standard52::KING);

        assert_eq!(rank.fluent_name(), &FluentName::new(Standard52::KING));
    }

    #[test]
    fn ranked__named__fluent_name_string() {
        let rank = Rank::<Standard52>::new(Standard52::QUEEN);

        assert_eq!(rank.fluent_name_string(), Standard52::QUEEN);
    }

    #[test]
    fn ranked__named__is_blank() {
        let rank = Rank::<Standard52>::new(Standard52::ACE);

        assert!(!rank.is_blank());
    }

    #[test]
    fn ranked__ranked__names() {
        let names = Rank::<Standard52>::rank_names();

        assert_eq!(names.len(), 13);
        assert_eq!(names[0], Standard52::ACE);
        assert_eq!(names[1], Standard52::KING);
        assert_eq!(names[2], Standard52::QUEEN);
        assert_eq!(names[3], Standard52::JACK);
        assert_eq!(names[4], Standard52::TEN);
        assert_eq!(names[5], Standard52::NINE);
        assert_eq!(names[6], Standard52::EIGHT);
        assert_eq!(names[7], Standard52::SEVEN);
        assert_eq!(names[8], Standard52::SIX);
        assert_eq!(names[9], Standard52::FIVE);
        assert_eq!(names[10], Standard52::FOUR);
        assert_eq!(names[11], Standard52::THREE);
        assert_eq!(names[12], Standard52::TWO);
    }

    #[test]
    fn ranked__is_valid_char() {
        assert!(Rank::<Standard52>::is_valid_rank_char(&'A'));
        assert!(!Rank::<Standard52>::is_valid_rank_char(&'Z'));
    }

    #[test]
    fn suit__binary_signature() {
        assert_eq!(4096, Suit::<Standard52>::from('S').binary_signature());
        assert_eq!(8192, Suit::<Standard52>::from('H').binary_signature());
        assert_eq!(16384, Suit::<Standard52>::from('D').binary_signature());
        assert_eq!(32768, Suit::<Standard52>::from('C').binary_signature());
        assert_eq!(61440, Suit::<Standard52>::from('_').binary_signature());
    }

    #[test]
    fn suit__binary_signature_revised() {
        assert_eq!(
            32768,
            Suit::<Standard52>::from('S').binary_signature_revised()
        );
        assert_eq!(
            16384,
            Suit::<Standard52>::from('H').binary_signature_revised()
        );
        assert_eq!(
            8192,
            Suit::<Standard52>::from('D').binary_signature_revised()
        );
        assert_eq!(
            4096,
            Suit::<Standard52>::from('C').binary_signature_revised()
        );
        assert_eq!(
            61440,
            Suit::<Standard52>::from('_').binary_signature_revised()
        );
    }

    #[test]
    fn suit__symbol() {
        let suit = Suit::<Standard52>::new(Standard52::SPADES);

        assert_eq!(suit.symbol(), "♠");
        assert_eq!(suit.to_string(), suit.symbol())
    }

    #[test]
    fn suit__symbol_blank() {
        let suit = Suit::<Standard52>::from('_');

        assert_eq!(suit.symbol(), "_");
        assert_eq!(suit.to_string(), suit.symbol())
    }

    #[test]
    fn suit__weighted_vector() {
        let mut v = Suit::<Standard52>::suit_names();
        v.reverse();

        let suits = Suit::<Standard52>::weighted_vector(&v);

        assert_eq!(suits.len(), 4);
        assert_eq!(suits[0].fluent_name_string(), "clubs");
        assert_eq!(suits[0].weight, 0);
        assert_eq!(suits[1].fluent_name_string(), "diamonds");
        assert_eq!(suits[1].weight, 1);
        assert_eq!(suits[2].fluent_name_string(), "hearts");
        assert_eq!(suits[2].weight, 2);
        assert_eq!(suits[3].fluent_name_string(), "spades");
        assert_eq!(suits[3].weight, 3);
    }

    #[test]
    fn suited__colors() {
        let mut expected = HashMap::new();
        expected.insert('H', Color::Red);
        expected.insert('D', Color::Red);

        let actual = Suit::<Standard52>::colors();

        assert_eq!(actual, expected);
    }

    #[test]
    fn suited__is_valid_suit_char() {
        assert!(Suit::<Standard52>::is_valid_suit_char(&'H'));
        assert!(Suit::<Standard52>::is_valid_suit_char(&'h'));
        assert!(Suit::<Standard52>::is_valid_suit_char(&'♥'));
        assert!(!Suit::<Standard52>::is_valid_suit_char(&'_'));
        assert!(!Suit::<Standard52>::is_valid_suit_char(&'W'));
    }

    #[test]
    fn suited__suit_chars() {
        let expected = vec![
            '♤', '♠', 'S', 's', '♡', '♥', 'H', 'h', '♢', '♦', 'D', 'd', '♧', '♣', 'C', 'c',
        ];

        let chars = Suit::<Standard52>::suit_chars();

        assert_eq!(chars, expected);
    }

    #[test]
    fn suited__suit_names() {
        let expected = vec![
            Standard52::SPADES,
            Standard52::HEARTS,
            Standard52::DIAMONDS,
            Standard52::CLUBS,
        ];

        let names = Suit::<Standard52>::suit_names();

        assert_eq!(names, expected);
    }

    #[test]
    fn from_char() {
        let rank = Rank::<Standard52>::from('A');

        assert_eq!(rank.name, FluentName::new(Standard52::ACE));
        assert_eq!(rank.weight, 12);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn from_str() {
        let rank = Rank::<Standard52>::from_str("A'").unwrap();

        assert_eq!(rank.name, FluentName::new(Standard52::ACE));
        assert_eq!(rank.weight, 12);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn from_str__invalid() {
        let rank = Rank::<Standard52>::from_str("Z'");

        assert!(rank.is_err());
        if let Err(CardError::InvalidFluentRank(_)) = rank {
            // The error is of type CardError::InvalidFluentRank
            // There has got to be a better way to test this.
        } else {
            panic!("Expected CardError::InvalidFluentRank");
        }
    }

    #[rstest]
    #[case('♠', Standard52::SPADES)]
    #[case('♤', Standard52::SPADES)]
    #[case('S', Standard52::SPADES)]
    #[case('s', Standard52::SPADES)]
    #[case('♥', Standard52::HEARTS)]
    #[case('♡', Standard52::HEARTS)]
    #[case('H', Standard52::HEARTS)]
    #[case('h', Standard52::HEARTS)]
    #[case('♦', Standard52::DIAMONDS)]
    #[case('♢', Standard52::DIAMONDS)]
    #[case('D', Standard52::DIAMONDS)]
    #[case('d', Standard52::DIAMONDS)]
    #[case('♣', Standard52::CLUBS)]
    #[case('♧', Standard52::CLUBS)]
    #[case('C', Standard52::CLUBS)]
    #[case('c', Standard52::CLUBS)]
    #[case('🃟', FluentName::BLANK)]
    #[case('T', FluentName::BLANK)]
    #[case('t', FluentName::BLANK)]
    #[case(' ', FluentName::BLANK)]
    #[case('F', FluentName::BLANK)]
    fn suit__from__char(#[case] input: char, #[case] expected: &str) {
        assert_eq!(
            Suit::<Standard52>::new(expected),
            Suit::<Standard52>::from(input)
        );
    }

    #[test]
    fn pile__sort() {
        let deck = Standard52::deck();
        let mut shuffled = deck.shuffle_default();

        shuffled.shuffle_in_place_default();
        shuffled.sort_in_place();

        assert_eq!(deck.to_string(), shuffled.to_string());
    }

    #[test]
    fn pile__ranks_by_suit() {
        let deck = Standard52::deck();

        let ranks = deck
            .ranks_by_suit(&Suit::<Standard52>::new(Standard52::SPADES))
            .unwrap();
        let index = Rank::<Standard52>::ranks_index(&ranks, " ");

        assert_eq!(13, ranks.len());
        assert_eq!("A K Q J T 9 8 7 6 5 4 3 2", index);
    }

    #[test]
    fn pile__ranks_by_suit__none() {
        let deck = Pile::<Standard52, Standard52>::default();

        let ranks = deck.ranks_by_suit(&Suit::<Standard52>::new(Standard52::CLUBS));

        assert!(ranks.is_none());
    }

    #[test]
    fn to_string__from_str() {
        let deck = Standard52::deck();
        let shuffled = deck.shuffle_default().to_string();
        let parsed = Standard52::from_str(&shuffled).unwrap();

        assert!(deck.same(&parsed));
    }
}
