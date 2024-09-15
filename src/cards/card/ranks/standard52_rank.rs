use crate::card_error::CardError;
use crate::cards::card::ranks::Rank;
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

    pub fn new(name_str: &str) -> Self {
        let name = FluentName::new(name_str);

        Self {
            weight: name.weight(),
            prime: name.prime(),
            name,
        }
    }
}

impl fmt::Display for Standard52Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.index_default())
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
    fn get_prime(&self) -> u32 {
        self.prime
    }

    fn get_weight(&self) -> u32 {
        self.weight
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod rank_tests {
    use super::*;
    use rstest::rstest;

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
    #[case('_', Standard52Rank::new(FluentName::BLANK))]
    #[case(' ', Standard52Rank::new(FluentName::BLANK))]
    fn from__char(#[case] input: char, #[case] expected: Standard52Rank) {
        assert_eq!(expected, Standard52Rank::from(input));
    }
}
