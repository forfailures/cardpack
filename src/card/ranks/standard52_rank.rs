use crate::card_error::CardError;
use crate::card::ranks::Rank;
use crate::fluent::{FluentName, Named};
use std::fmt;
use std::str::FromStr;

/// Weight is first for sorting.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Standard52Rank {
    weight: u32,
    prime: u32,
    name: FluentName,
}

impl Standard52Rank {
    pub const ACE: &str = "ace";
    pub const KING: &str = "king";
    pub const QUEEN: &str = "queen";
    pub const JACK: &str = "jack";
    pub const TEN: &str = "ten";
    pub const NINE: &str = "nine";
    pub const EIGHT: &str = "eight";
    pub const SEVEN: &str = "seven";
    pub const SIX: &str = "six";
    pub const FIVE: &str = "five";
    pub const FOUR: &str = "four";
    pub const THREE: &str = "three";
    pub const TWO: &str = "two";
}

impl fmt::Display for Standard52Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.index_default())
    }
}

impl Default for Standard52Rank {
    fn default() -> Standard52Rank {
        Standard52Rank::new(FluentName::BLANK)
    }
}

impl From<char> for Standard52Rank {
    fn from(c: char) -> Self {
        match c {
            '2' => Standard52Rank::new(Standard52Rank::TWO),
            '3' => Standard52Rank::new(Standard52Rank::THREE),
            '4' => Standard52Rank::new(Standard52Rank::FOUR),
            '5' => Standard52Rank::new(Standard52Rank::FIVE),
            '6' => Standard52Rank::new(Standard52Rank::SIX),
            '7' => Standard52Rank::new(Standard52Rank::SEVEN),
            '8' => Standard52Rank::new(Standard52Rank::EIGHT),
            '9' => Standard52Rank::new(Standard52Rank::NINE),
            'T' | 't' | '0' => Standard52Rank::new(Standard52Rank::TEN),
            'J' | 'j' => Standard52Rank::new(Standard52Rank::JACK),
            'Q' | 'q' => Standard52Rank::new(Standard52Rank::QUEEN),
            'K' | 'k' => Standard52Rank::new(Standard52Rank::KING),
            'A' | 'a' => Standard52Rank::new(Standard52Rank::ACE),
            _ => Standard52Rank::new(FluentName::BLANK),
        }
    }
}

impl FromStr for Standard52Rank {
    type Err = CardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(first_char) = s.chars().next() {
            let rank = Standard52Rank::from(first_char);
            if rank.name.is_blank() {
                Err(CardError::InvalidFluentRank(s.to_string()))
            } else {
                Ok(rank)
            }
        } else {
            Err(CardError::InvalidFluentRank(s.to_string()))
        }
    }
}

impl<'a> Named<'a> for Standard52Rank {
    fn fluent_name(&self) -> &FluentName {
        &self.name
    }

    fn fluent_name_string(&self) -> &String {
        self.name.fluent_name_string()
    }

    fn is_blank(&self) -> bool {
        self.name.is_blank()
    }
}

impl Rank for Standard52Rank {
    fn new(name_str: &str) -> Self {
        let name = FluentName::new(name_str);

        Self {
            weight: name.weight(),
            prime: name.prime(),
            name,
        }
    }

    fn new_with_weight(name_str: &str, weight: u32) -> Self {
        let name = FluentName::new(name_str);

        Self {
            weight,
            prime: name.prime(),
            name,
        }
    }

    fn names() -> Vec<&'static str> {
        vec![
            Standard52Rank::ACE,
            Standard52Rank::KING,
            Standard52Rank::QUEEN,
            Standard52Rank::JACK,
            Standard52Rank::TEN,
            Standard52Rank::NINE,
            Standard52Rank::EIGHT,
            Standard52Rank::SEVEN,
            Standard52Rank::SIX,
            Standard52Rank::FIVE,
            Standard52Rank::FOUR,
            Standard52Rank::THREE,
            Standard52Rank::TWO,
        ]
    }

    fn get_prime(&self) -> u32 {
        self.prime
    }

    fn get_weight(&self) -> u32 {
        self.weight
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod rank__standard52_tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn default() {
        assert_eq!(
            Standard52Rank::new(FluentName::BLANK),
            Standard52Rank::default()
        );
    }

    #[test]
    fn display() {
        assert_eq!("A", Standard52Rank::new(Standard52Rank::ACE).to_string());
        assert_eq!("K", Standard52Rank::new(Standard52Rank::KING).to_string());
        assert_eq!("Q", Standard52Rank::new(Standard52Rank::QUEEN).to_string());
        assert_eq!("J", Standard52Rank::new(Standard52Rank::JACK).to_string());
        assert_eq!("T", Standard52Rank::new(Standard52Rank::TEN).to_string());
        assert_eq!("9", Standard52Rank::new(Standard52Rank::NINE).to_string());
        assert_eq!("8", Standard52Rank::new(Standard52Rank::EIGHT).to_string());
        assert_eq!("7", Standard52Rank::new(Standard52Rank::SEVEN).to_string());
        assert_eq!("6", Standard52Rank::new(Standard52Rank::SIX).to_string());
        assert_eq!("5", Standard52Rank::new(Standard52Rank::FIVE).to_string());
        assert_eq!("4", Standard52Rank::new(Standard52Rank::FOUR).to_string());
        assert_eq!("3", Standard52Rank::new(Standard52Rank::THREE).to_string());
        assert_eq!("2", Standard52Rank::new(Standard52Rank::TWO).to_string());
        assert_eq!("_", Standard52Rank::new(FluentName::BLANK).to_string());
    }

    #[rstest]
    #[case('A', Standard52Rank::new(Standard52Rank::ACE))]
    #[case('a', Standard52Rank::new(Standard52Rank::ACE))]
    #[case('K', Standard52Rank::new(Standard52Rank::KING))]
    #[case('k', Standard52Rank::new(Standard52Rank::KING))]
    #[case('Q', Standard52Rank::new(Standard52Rank::QUEEN))]
    #[case('q', Standard52Rank::new(Standard52Rank::QUEEN))]
    #[case('J', Standard52Rank::new(Standard52Rank::JACK))]
    #[case('j', Standard52Rank::new(Standard52Rank::JACK))]
    #[case('T', Standard52Rank::new(Standard52Rank::TEN))]
    #[case('t', Standard52Rank::new(Standard52Rank::TEN))]
    #[case('0', Standard52Rank::new(Standard52Rank::TEN))]
    #[case('9', Standard52Rank::new(Standard52Rank::NINE))]
    #[case('8', Standard52Rank::new(Standard52Rank::EIGHT))]
    #[case('7', Standard52Rank::new(Standard52Rank::SEVEN))]
    #[case('6', Standard52Rank::new(Standard52Rank::SIX))]
    #[case('5', Standard52Rank::new(Standard52Rank::FIVE))]
    #[case('4', Standard52Rank::new(Standard52Rank::FOUR))]
    #[case('3', Standard52Rank::new(Standard52Rank::THREE))]
    #[case('2', Standard52Rank::new(Standard52Rank::TWO))]
    #[cfg_attr(miri, ignore)]
    fn from(#[case] input: char, #[case] expected: Standard52Rank) {
        assert_eq!(expected, Standard52Rank::from(input));
        // This assertion throws a miri error, but I love the simplicity.
        assert_eq!(
            expected,
            Standard52Rank::from_str(Box::leak(input.to_string().into_boxed_str())).unwrap()
        );
    }

    #[test]
    fn from_blank() {
        assert_eq!(
            Standard52Rank::new(FluentName::BLANK),
            Standard52Rank::from(' ')
        );
        assert_eq!(
            Standard52Rank::new(FluentName::BLANK),
            Standard52Rank::from('_')
        );
        assert!(Standard52Rank::from_str(" ").is_err());
        assert!(Standard52Rank::from_str("_").is_err());
    }

    #[test]
    fn rank_get_prime() {
        assert_eq!(41, Standard52Rank::new(Standard52Rank::ACE).get_prime());
    }

    #[test]
    fn rank_get_weight() {
        assert_eq!(12, Standard52Rank::new(Standard52Rank::ACE).get_weight());
    }

    #[test]
    fn rank_ranks() {
        let ranks = Standard52Rank::ranks();

        assert_eq!(13, ranks.len());
        assert_eq!(Standard52Rank::new(Standard52Rank::ACE), ranks[0]);
        assert_eq!(Standard52Rank::new(Standard52Rank::KING), ranks[1]);
        assert_eq!(Standard52Rank::new(Standard52Rank::QUEEN), ranks[2]);
        assert_eq!(Standard52Rank::new(Standard52Rank::JACK), ranks[3]);
        assert_eq!(Standard52Rank::new(Standard52Rank::TEN), ranks[4]);
        assert_eq!(Standard52Rank::new(Standard52Rank::NINE), ranks[5]);
        assert_eq!(Standard52Rank::new(Standard52Rank::EIGHT), ranks[6]);
        assert_eq!(Standard52Rank::new(Standard52Rank::SEVEN), ranks[7]);
        assert_eq!(Standard52Rank::new(Standard52Rank::SIX), ranks[8]);
        assert_eq!(Standard52Rank::new(Standard52Rank::FIVE), ranks[9]);
        assert_eq!(Standard52Rank::new(Standard52Rank::FOUR), ranks[10]);
        assert_eq!(Standard52Rank::new(Standard52Rank::THREE), ranks[11]);
        assert_eq!(Standard52Rank::new(Standard52Rank::TWO), ranks[12]);
    }

    #[test]
    fn rank_weighted_vector() {
        let ranks = vec![
            Standard52Rank::TWO,
            Standard52Rank::THREE,
            Standard52Rank::FOUR,
            Standard52Rank::FIVE,
            Standard52Rank::SIX,
            Standard52Rank::SEVEN,
            Standard52Rank::EIGHT,
            Standard52Rank::NINE,
            Standard52Rank::TEN,
            Standard52Rank::JACK,
            Standard52Rank::QUEEN,
            Standard52Rank::KING,
            Standard52Rank::ACE,
        ];

        let weighted_ranks = Standard52Rank::weighted_vector(ranks);

        assert_eq!(13, weighted_ranks.len());
        assert_eq!(
            Standard52Rank::new_with_weight(Standard52Rank::TWO, 0),
            weighted_ranks[0]
        );
        assert_eq!(
            Standard52Rank::new_with_weight(Standard52Rank::THREE, 1),
            weighted_ranks[1]
        );
        assert_eq!(
            Standard52Rank::new_with_weight(Standard52Rank::FOUR, 2),
            weighted_ranks[2]
        );
        assert_eq!(
            Standard52Rank::new_with_weight(Standard52Rank::FIVE, 3),
            weighted_ranks[3]
        );
        assert_eq!(
            Standard52Rank::new_with_weight(Standard52Rank::SIX, 4),
            weighted_ranks[4]
        );
        assert_eq!(
            Standard52Rank::new_with_weight(Standard52Rank::SEVEN, 5),
            weighted_ranks[5]
        );
        assert_eq!(
            Standard52Rank::new_with_weight(Standard52Rank::EIGHT, 6),
            weighted_ranks[6]
        );
        assert_eq!(
            Standard52Rank::new_with_weight(Standard52Rank::NINE, 7),
            weighted_ranks[7]
        );
        assert_eq!(
            Standard52Rank::new_with_weight(Standard52Rank::TEN, 8),
            weighted_ranks[8]
        );
        assert_eq!(
            Standard52Rank::new_with_weight(Standard52Rank::JACK, 9),
            weighted_ranks[9]
        );
        assert_eq!(
            Standard52Rank::new_with_weight(Standard52Rank::QUEEN, 10),
            weighted_ranks[10]
        );
        assert_eq!(
            Standard52Rank::new_with_weight(Standard52Rank::KING, 11),
            weighted_ranks[11]
        );
        assert_eq!(
            Standard52Rank::new_with_weight(Standard52Rank::ACE, 12),
            weighted_ranks[12]
        );
    }

    #[test]
    fn rank_update_weight() {
        let sut = Standard52Rank::new(Standard52Rank::ACE).update_weight(13);

        assert_eq!(13, sut.get_weight());
        assert_eq!(41, sut.get_prime());
        assert_eq!(FluentName::new(Standard52Rank::ACE), sut.name);
    }
}
