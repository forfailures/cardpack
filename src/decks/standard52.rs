use crate::types::card_error::CardError;
use crate::types::pile::Pile;
use crate::types::traits::{Decked, Ranked, Suited};
use colored::Color;
use std::collections::HashMap;
use std::str::FromStr;

/// These macros make me very happy. They wallpaper over a lot of headaches from the generics.
#[macro_export]
macro_rules! s52card {
    ($card_str:expr) => {
        Card::<Standard52, Standard52>::from_str($card_str)
            .unwrap_or_else(|_| Card::<Standard52, Standard52>::default())
    };
}

#[macro_export]
macro_rules! standard52 {
    ($card_str:expr) => {
        Pile::<Standard52, Standard52>::from_str($card_str)
    };
}

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
    use crate::s52card;
    use crate::standard52;
    use crate::types::card::Card;
    use crate::types::rank::Rank;
    use crate::types::suit::Suit;
    use ckc_rs::CardNumber;
    use rstest::rstest;
    use std::str::FromStr;

    #[test]
    fn macro_standard52() {
        let deck = standard52!("A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠ 7♠ 6♠ 5♠ 4♠ 3♠ 2♠ A♥ K♥ Q♥ J♥ T♥ 9♥ 8♥ 7♥ 6♥ 5♥ 4♥ 3♥ 2♥ A♦ K♦ Q♦ J♦ T♦ 9♦ 8♦ 7♦ 6♦ 5♦ 4♦ 3♦ 2♦ A♣ K♣ Q♣ J♣ T♣ 9♣ 8♣ 7♣ 6♣ 5♣ 4♣ 3♣ 2♣").unwrap();

        assert_eq!(deck.to_string(), Standard52::deck().to_string());
        assert!(standard52!("AA xx __").is_err());
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
        assert_eq!(expected_ckc, s52card!(input).get_ckc_number());
    }

    #[test]
    fn card__get_ckc_number__blank() {
        assert_eq!(0, s52card!("__").get_ckc_number());
    }

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
        assert_eq!(ranks[0].weight, 12);
        assert_eq!(ranks[0].name.fluent_name_string(), "two");
        assert_eq!(ranks[1].weight, 11);
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
