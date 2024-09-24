use crate::card::ranks::standard52_rank::Standard52Rank;
use crate::card::ranks::Rank;
use crate::card_error::CardError;
use crate::localization::{FluentName, Named};
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EuchreRank {
    weight: u32,
    prime: u32,
    name: FluentName,
}

impl fmt::Display for EuchreRank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.index_default())
    }
}

impl Default for EuchreRank {
    fn default() -> Self {
        EuchreRank::new(FluentName::BLANK)
    }
}

impl From<char> for EuchreRank {
    fn from(c: char) -> Self {
        match c {
            '9' => EuchreRank::new(Standard52Rank::NINE),
            'T' | 't' | '0' => EuchreRank::new(Standard52Rank::TEN),
            'J' | 'j' => EuchreRank::new(Standard52Rank::JACK),
            'Q' | 'q' => EuchreRank::new(Standard52Rank::QUEEN),
            'K' | 'k' => EuchreRank::new(Standard52Rank::KING),
            'A' | 'a' => EuchreRank::new(Standard52Rank::ACE),
            _ => EuchreRank::new(FluentName::BLANK),
        }
    }
}

impl FromStr for EuchreRank {
    type Err = CardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(first_char) = s.chars().next() {
            let rank = EuchreRank::from(first_char);
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

impl<'a> Named<'a> for EuchreRank {
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

impl Rank for EuchreRank {
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
mod rank__euchre_tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case('A', EuchreRank::new(Standard52Rank::ACE))]
    #[case('a', EuchreRank::new(Standard52Rank::ACE))]
    #[case('K', EuchreRank::new(Standard52Rank::KING))]
    #[case('k', EuchreRank::new(Standard52Rank::KING))]
    #[case('Q', EuchreRank::new(Standard52Rank::QUEEN))]
    #[case('q', EuchreRank::new(Standard52Rank::QUEEN))]
    #[case('J', EuchreRank::new(Standard52Rank::JACK))]
    #[case('j', EuchreRank::new(Standard52Rank::JACK))]
    #[case('T', EuchreRank::new(Standard52Rank::TEN))]
    #[case('t', EuchreRank::new(Standard52Rank::TEN))]
    #[case('0', EuchreRank::new(Standard52Rank::TEN))]
    #[case('9', EuchreRank::new(Standard52Rank::NINE))]
    #[cfg_attr(miri, ignore)]
    fn from(#[case] input: char, #[case] expected: EuchreRank) {
        assert_eq!(expected, EuchreRank::from(input));
        // This assertion throws a miri error, but I love the simplicity.
        assert_eq!(
            expected,
            EuchreRank::from_str(Box::leak(input.to_string().into_boxed_str())).unwrap()
        );
    }
}
