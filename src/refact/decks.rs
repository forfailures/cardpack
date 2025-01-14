use crate::decks::FluentName;
use crate::refact::traits::{Decked, Ranked, Suited};
use crate::refact::{Rank, Suit, BLANK};
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct French {}

impl French {
    /// # REFACTOR WIN
    pub const ACE: Rank<French> = Rank {
        weight: 12,
        index: French::ACE_INDEX,
        phantom_data: PhantomData,
    };
    pub const KING: Rank<French> = Rank {
        weight: 11,
        index: French::KING_INDEX,
        phantom_data: PhantomData,
    };
    pub const QUEEN: Rank<French> = Rank {
        weight: 10,
        index: French::QUEEN_INDEX,
        phantom_data: PhantomData,
    };
    pub const JACK: Rank<French> = Rank {
        weight: 9,
        index: French::JACK_INDEX,
        phantom_data: PhantomData,
    };
    pub const TEN: Rank<French> = Rank {
        weight: 8,
        index: French::TEN_INDEX,
        phantom_data: PhantomData,
    };
    pub const NINE: Rank<French> = Rank {
        weight: 7,
        index: French::NINE_INDEX,
        phantom_data: PhantomData,
    };
    pub const EIGHT: Rank<French> = Rank {
        weight: 6,
        index: French::EIGHT_INDEX,
        phantom_data: PhantomData,
    };
    pub const SEVEN: Rank<French> = Rank {
        weight: 5,
        index: French::SEVEN_INDEX,
        phantom_data: PhantomData,
    };
    pub const SIX: Rank<French> = Rank {
        weight: 4,
        index: French::SIX_INDEX,
        phantom_data: PhantomData,
    };
    pub const FIVE: Rank<French> = Rank {
        weight: 3,
        index: French::FIVE_INDEX,
        phantom_data: PhantomData,
    };
    pub const FOUR: Rank<French> = Rank {
        weight: 2,
        index: French::FOUR_INDEX,
        phantom_data: PhantomData,
    };
    pub const TREY: Rank<French> = Rank {
        weight: 1,
        index: French::TREY_INDEX,
        phantom_data: PhantomData,
    };
    pub const DEUCE: Rank<French> = Rank {
        weight: 0,
        index: French::DEUCE_INDEX,
        phantom_data: PhantomData,
    };

    pub const SPADES: Suit<French> = Suit {
        weight: 4,
        index: French::SPADES_INDEX,
        phantom_data: PhantomData,
    };
    pub const HEARTS: Suit<French> = Suit {
        weight: 3,
        index: French::HEARTS_INDEX,
        phantom_data: PhantomData,
    };
    pub const DIAMONDS: Suit<French> = Suit {
        weight: 2,
        index: French::DIAMONDS_INDEX,
        phantom_data: PhantomData,
    };
    pub const CLUBS: Suit<French> = Suit {
        weight: 1,
        index: French::CLUBS_INDEX,
        phantom_data: PhantomData,
    };

    // Suites
    pub const SPADES_INDEX: char = 'S';
    pub const HEARTS_INDEX: char = 'H';
    pub const DIAMONDS_INDEX: char = 'D';
    pub const CLUBS_INDEX: char = 'C';

    pub const SPADES_SYMBOL: char = '♠';
    pub const HEARTS_SYMBOL: char = '♥';
    pub const DIAMONDS_SYMBOL: char = '♦';
    pub const CLUBS_SYMBOL: char = '♣';

    const FLUENT_KEY_SPADES: &'static str = "spades";
    const FLUENT_KEY_HEARTS: &'static str = "hearts";
    const FLUENT_KEY_DIAMONDS: &'static str = "diamonds";
    const FLUENT_KEY_CLUBS: &'static str = "clubs";

    // Ranks
    pub const ACE_INDEX: char = 'A';
    pub const KING_INDEX: char = 'K';
    pub const QUEEN_INDEX: char = 'Q';
    pub const JACK_INDEX: char = 'J';
    pub const TEN_INDEX: char = 'T';
    pub const NINE_INDEX: char = '9';
    pub const EIGHT_INDEX: char = '8';
    pub const SEVEN_INDEX: char = '7';
    pub const SIX_INDEX: char = '6';
    pub const FIVE_INDEX: char = '5';
    pub const FOUR_INDEX: char = '4';
    pub const TREY_INDEX: char = '3';
    pub const DEUCE_INDEX: char = '2';

    const FLUENT_KEY_ACE: &'static str = "ace";
    const FLUENT_KEY_KING: &'static str = "king";
    const FLUENT_KEY_QUEEN: &'static str = "queen";
    const FLUENT_KEY_JACK: &'static str = "jack";
    const FLUENT_KEY_TEN: &'static str = "ten";
    const FLUENT_KEY_NINE: &'static str = "nine";
    const FLUENT_KEY_EIGHT: &'static str = "eight";
    const FLUENT_KEY_SEVEN: &'static str = "seven";
    const FLUENT_KEY_SIX: &'static str = "six";
    const FLUENT_KEY_FIVE: &'static str = "five";
    const FLUENT_KEY_FOUR: &'static str = "four";
    const FLUENT_KEY_TREY: &'static str = "three";
    const FLUENT_KEY_DEUCE: &'static str = "two";

    #[must_use]
    pub fn get_rank_fluent_name(c: char) -> FluentName {
        match c {
            French::ACE_INDEX => FluentName::new(French::FLUENT_KEY_ACE),
            French::KING_INDEX => FluentName::new(French::FLUENT_KEY_KING),
            French::QUEEN_INDEX => FluentName::new(French::FLUENT_KEY_QUEEN),
            French::JACK_INDEX => FluentName::new(French::FLUENT_KEY_JACK),
            French::TEN_INDEX => FluentName::new(French::FLUENT_KEY_TEN),
            French::NINE_INDEX => FluentName::new(French::FLUENT_KEY_NINE),
            French::EIGHT_INDEX => FluentName::new(French::FLUENT_KEY_EIGHT),
            French::SEVEN_INDEX => FluentName::new(French::FLUENT_KEY_SEVEN),
            French::SIX_INDEX => FluentName::new(French::FLUENT_KEY_SIX),
            French::FIVE_INDEX => FluentName::new(French::FLUENT_KEY_FIVE),
            French::FOUR_INDEX => FluentName::new(French::FLUENT_KEY_FOUR),
            French::TREY_INDEX => FluentName::new(French::FLUENT_KEY_TREY),
            French::DEUCE_INDEX => FluentName::new(French::FLUENT_KEY_DEUCE),
            _ => FluentName::new(FluentName::BLANK),
        }
    }

    #[must_use]
    pub fn get_rank_index(c: char) -> char {
        match c {
            'a' | 'A' => French::ACE_INDEX,
            'k' | 'K' => French::KING_INDEX,
            'q' | 'Q' => French::QUEEN_INDEX,
            'j' | 'J' => French::JACK_INDEX,
            't' | 'T' | '0' => French::TEN_INDEX,
            '9' => French::NINE_INDEX,
            '8' => French::EIGHT_INDEX,
            '7' => French::SEVEN_INDEX,
            '6' => French::SIX_INDEX,
            '5' => French::FIVE_INDEX,
            '4' => French::FOUR_INDEX,
            '3' => French::TREY_INDEX,
            '2' => French::DEUCE_INDEX,
            _ => BLANK,
        }
    }

    #[must_use]
    pub fn get_rank_weight(c: char) -> u32 {
        match c {
            French::ACE_INDEX => French::ACE.weight,
            French::KING_INDEX => French::KING.weight,
            French::QUEEN_INDEX => French::QUEEN.weight,
            French::JACK_INDEX => French::JACK.weight,
            French::TEN_INDEX => French::TEN.weight,
            French::NINE_INDEX => French::NINE.weight,
            French::EIGHT_INDEX => French::EIGHT.weight,
            French::SEVEN_INDEX => French::SEVEN.weight,
            French::SIX_INDEX => French::SIX.weight,
            French::FIVE_INDEX => French::FIVE.weight,
            French::FOUR_INDEX => French::FOUR.weight,
            French::TREY_INDEX => French::TREY.weight,
            French::DEUCE_INDEX => French::DEUCE.weight,
            _ => 0,
        }
    }

    #[must_use]
    pub fn get_suit_fluent_name(c: char) -> FluentName {
        match c {
            French::SPADES_INDEX => FluentName::new(French::FLUENT_KEY_SPADES),
            French::HEARTS_INDEX => FluentName::new(French::FLUENT_KEY_HEARTS),
            French::DIAMONDS_INDEX => FluentName::new(French::FLUENT_KEY_DIAMONDS),
            French::CLUBS_INDEX => FluentName::new(French::FLUENT_KEY_CLUBS),
            _ => FluentName::new(FluentName::BLANK),
        }
    }

    #[must_use]
    pub fn get_suit_index(c: char) -> char {
        match c {
            's' | '♤' | French::SPADES_INDEX | French::SPADES_SYMBOL => French::SPADES_INDEX,
            'h' | '♡' | French::HEARTS_INDEX | French::HEARTS_SYMBOL => French::HEARTS_INDEX,
            'd' | '♢' | French::DIAMONDS_INDEX | French::DIAMONDS_SYMBOL => {
                French::DIAMONDS_INDEX
            }
            'c' | '♧' | French::CLUBS_INDEX | French::CLUBS_SYMBOL => French::CLUBS_INDEX,
            _ => BLANK,
        }
    }

    #[must_use]
    pub fn get_suit_symbol(c: char) -> char {
        match c {
            's' | '♤' | French::SPADES_INDEX | French::SPADES_SYMBOL => French::SPADES_SYMBOL,
            'h' | '♡' | French::HEARTS_INDEX | French::HEARTS_SYMBOL => French::HEARTS_SYMBOL,
            'd' | '♢' | French::DIAMONDS_INDEX | French::DIAMONDS_SYMBOL => {
                French::DIAMONDS_SYMBOL
            }
            'c' | '♧' | French::CLUBS_INDEX | French::CLUBS_SYMBOL => French::CLUBS_SYMBOL,
            _ => BLANK,
        }
    }

    #[must_use]
    pub fn get_suit_weight(c: char) -> u32 {
        match French::get_suit_index(c) {
            French::SPADES_INDEX => French::SPADES.weight,
            French::HEARTS_INDEX => French::HEARTS.weight,
            French::DIAMONDS_INDEX => French::DIAMONDS.weight,
            French::CLUBS_INDEX => French::CLUBS.weight,
            _ => 0,
        }
    }
}

impl Decked<French, French> for French {
    fn get_rank(c: char) -> Rank<French> {
        match c {
            French::ACE_INDEX | 'a' => French::ACE,
            French::KING_INDEX | 'k' => French::KING,
            French::QUEEN_INDEX | 'q' => French::QUEEN,
            French::JACK_INDEX | 'j' => French::JACK,
            French::TEN_INDEX | 't' | '0' => French::TEN,
            French::NINE_INDEX => French::NINE,
            French::EIGHT_INDEX => French::EIGHT,
            French::SEVEN_INDEX => French::SEVEN,
            French::SIX_INDEX => French::SIX,
            French::FIVE_INDEX => French::FIVE,
            French::FOUR_INDEX => French::FOUR,
            French::TREY_INDEX => French::TREY,
            French::DEUCE_INDEX => French::DEUCE,
            _ => Rank::<French>::default(),
        }
    }

    fn get_suit(c: char) -> Suit<French> {
        match French::get_suit_index(c) {
            French::SPADES_INDEX => French::SPADES,
            French::HEARTS_INDEX => French::HEARTS,
            French::DIAMONDS_INDEX => French::DIAMONDS,
            French::CLUBS_INDEX => French::CLUBS,
            _ => Suit::<French>::default(),
        }
    }
}

/// # REFACTOR WIN
///
/// This win endied up being a big W when I tried to wire it into the Card struct.
///
/// ```
/// use cardpack::refactored::*;
///
/// assert_eq!(Rank::<French>::from('a'), French::ACE);
/// ```
impl From<char> for Rank<French> {
    fn from(c: char) -> Self {
        match c {
            French::ACE_INDEX | 'a' => French::ACE,
            French::KING_INDEX | 'k' => French::KING,
            French::QUEEN_INDEX | 'q' => French::QUEEN,
            French::JACK_INDEX | 'j' => French::JACK,
            French::TEN_INDEX | 't' | '0' => French::TEN,
            French::NINE_INDEX => French::NINE,
            French::EIGHT_INDEX => French::EIGHT,
            French::SEVEN_INDEX => French::SEVEN,
            French::SIX_INDEX => French::SIX,
            French::FIVE_INDEX => French::FIVE,
            French::FOUR_INDEX => French::FOUR,
            French::TREY_INDEX => French::TREY,
            French::DEUCE_INDEX => French::DEUCE,
            _ => Rank::<French>::default(),
        }
    }
}

impl Suited for French {
    /// ```
    /// use cardpack::refactored::*;
    ///
    /// assert_eq!(French::get_suit_fluent_name('S'), FluentName::new("spades"));
    /// ```
    fn get_suit_fluent_name(c: char) -> FluentName {
        French::get_suit_fluent_name(c)
    }

    fn get_suit_index(c: char) -> char {
        French::get_suit_index(c)
    }

    fn get_suit_symbol(c: char) -> char {
        French::get_suit_symbol(c)
    }

    /// ```
    /// use cardpack::refactored::*;
    ///
    /// assert_eq!(French::get_suit_weight('S'), 4);
    /// ```
    fn get_suit_weight(c: char) -> u32 {
        French::get_suit_weight(c)
    }

    fn suit_indexes() -> Vec<char> {
        vec![
            Self::SPADES_INDEX,
            Self::HEARTS_INDEX,
            Self::DIAMONDS_INDEX,
            Self::CLUBS_INDEX,
        ]
    }
}

impl Ranked for French {
    /// ```
    /// use cardpack::refactored::*;
    ///
    /// assert_eq!(French::get_rank_fluent_name('A'), FluentName::new("ace"));
    /// ```
    fn get_rank_fluent_name(c: char) -> FluentName {
        French::get_rank_fluent_name(c)
    }

    fn get_rank_index(c: char) -> char {
        French::get_rank_index(c)
    }

    fn get_rank_weight(c: char) -> u32 {
        French::get_rank_weight(c)
    }

    fn rank_indexes() -> Vec<char> {
        vec![
            Self::ACE_INDEX,
            Self::KING_INDEX,
            Self::QUEEN_INDEX,
            Self::JACK_INDEX,
            Self::TEN_INDEX,
            Self::NINE_INDEX,
            Self::EIGHT_INDEX,
            Self::SEVEN_INDEX,
            Self::SIX_INDEX,
            Self::FIVE_INDEX,
            Self::FOUR_INDEX,
            Self::TREY_INDEX,
            Self::DEUCE_INDEX,
        ]
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod decks {
    use super::*;

    #[test]
    fn ranked__named() {
        assert_eq!(
            French::DEUCE.get_name(),
            FluentName::new(French::FLUENT_KEY_DEUCE)
        );
        assert_eq!(
            French::TREY.get_name(),
            FluentName::new(French::FLUENT_KEY_TREY)
        );
        assert_eq!(
            French::FOUR.get_name(),
            FluentName::new(French::FLUENT_KEY_FOUR)
        );
        assert_eq!(
            French::FIVE.get_name(),
            FluentName::new(French::FLUENT_KEY_FIVE)
        );
        assert_eq!(
            French::SIX.get_name(),
            FluentName::new(French::FLUENT_KEY_SIX)
        );
        assert_eq!(
            French::SEVEN.get_name(),
            FluentName::new(French::FLUENT_KEY_SEVEN)
        );
        assert_eq!(
            French::EIGHT.get_name(),
            FluentName::new(French::FLUENT_KEY_EIGHT)
        );
        assert_eq!(
            French::NINE.get_name(),
            FluentName::new(French::FLUENT_KEY_NINE)
        );
        assert_eq!(
            French::TEN.get_name(),
            FluentName::new(French::FLUENT_KEY_TEN)
        );
        assert_eq!(
            French::JACK.get_name(),
            FluentName::new(French::FLUENT_KEY_JACK)
        );
        assert_eq!(
            French::QUEEN.get_name(),
            FluentName::new(French::FLUENT_KEY_QUEEN)
        );
        assert_eq!(
            French::KING.get_name(),
            FluentName::new(French::FLUENT_KEY_KING)
        );
        assert_eq!(
            French::ACE.get_name(),
            FluentName::new(French::FLUENT_KEY_ACE)
        );
    }

    #[test]
    fn ranked__get_rank_index() {
        assert_eq!(French::get_rank_index('A'), French::ACE_INDEX);
        assert_eq!(French::get_rank_index('a'), French::ACE_INDEX);
        assert_eq!(French::get_rank_index('K'), French::KING_INDEX);
        assert_eq!(French::get_rank_index('k'), French::KING_INDEX);
        assert_eq!(French::get_rank_index('Q'), French::QUEEN_INDEX);
        assert_eq!(French::get_rank_index('q'), French::QUEEN_INDEX);
        assert_eq!(French::get_rank_index('J'), French::JACK_INDEX);
        assert_eq!(French::get_rank_index('j'), French::JACK_INDEX);
        assert_eq!(French::get_rank_index('T'), French::TEN_INDEX);
        assert_eq!(French::get_rank_index('t'), French::TEN_INDEX);
        assert_eq!(French::get_rank_index('0'), French::TEN_INDEX);
        assert_eq!(French::get_rank_index('9'), French::NINE_INDEX);
        assert_eq!(French::get_rank_index('8'), French::EIGHT_INDEX);
        assert_eq!(French::get_rank_index('7'), French::SEVEN_INDEX);
        assert_eq!(French::get_rank_index('6'), French::SIX_INDEX);
        assert_eq!(French::get_rank_index('5'), French::FIVE_INDEX);
        assert_eq!(French::get_rank_index('4'), French::FOUR_INDEX);
        assert_eq!(French::get_rank_index('3'), French::TREY_INDEX);
        assert_eq!(French::get_rank_index('2'), French::DEUCE_INDEX);
    }

    #[test]
    fn ranked__get_rank_weight() {
        assert_eq!(French::get_rank_weight('A'), 12);
        assert_eq!(French::get_rank_weight('K'), 11);
        assert_eq!(French::get_rank_weight('Q'), 10);
        assert_eq!(French::get_rank_weight('J'), 9);
        assert_eq!(French::get_rank_weight('T'), 8);
        assert_eq!(French::get_rank_weight('9'), 7);
        assert_eq!(French::get_rank_weight('8'), 6);
        assert_eq!(French::get_rank_weight('7'), 5);
        assert_eq!(French::get_rank_weight('6'), 4);
        assert_eq!(French::get_rank_weight('5'), 3);
        assert_eq!(French::get_rank_weight('4'), 2);
        assert_eq!(French::get_rank_weight('3'), 1);
        assert_eq!(French::get_rank_weight('2'), 0);
    }

    #[test]
    fn suited__get_weight() {
        assert_eq!(French::get_suit_weight('S'), 4);
        assert_eq!(French::get_suit_weight('s'), 4);
        assert_eq!(French::get_suit_weight('♠'), 4);
        assert_eq!(French::get_suit_weight('H'), 3);
        assert_eq!(French::get_suit_weight('h'), 3);
        assert_eq!(French::get_suit_weight('♥'), 3);
        assert_eq!(French::get_suit_weight('D'), 2);
        assert_eq!(French::get_suit_weight('d'), 2);
        assert_eq!(French::get_suit_weight('♦'), 2);
        assert_eq!(French::get_suit_weight('C'), 1);
        assert_eq!(French::get_suit_weight('c'), 1);
        assert_eq!(French::get_suit_weight('♣'), 1);
    }

    #[test]
    fn get_prime() {
        assert_eq!(French::DEUCE.get_prime(), 2);
        assert_eq!(French::TREY.get_prime(), 3);
        assert_eq!(French::FOUR.get_prime(), 5);
        assert_eq!(French::FIVE.get_prime(), 7);
        assert_eq!(French::SIX.get_prime(), 11);
        assert_eq!(French::SEVEN.get_prime(), 13);
        assert_eq!(French::EIGHT.get_prime(), 17);
        assert_eq!(French::NINE.get_prime(), 19);
        assert_eq!(French::TEN.get_prime(), 23);
        assert_eq!(French::JACK.get_prime(), 29);
        assert_eq!(French::QUEEN.get_prime(), 31);
        assert_eq!(French::KING.get_prime(), 37);
        assert_eq!(French::ACE.get_prime(), 41);
    }

    #[test]
    fn update_weight() {
        let heavy_card = French::TREY.update_weight(21);

        assert_eq!(heavy_card.get_prime(), 0);
    }

    #[test]
    fn decked__deck() {
        let _deck = French::deck();
    }
}
