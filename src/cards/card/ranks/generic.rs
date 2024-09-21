use std::convert::From;
use std::marker::PhantomData;
use std::str::FromStr;
use crate::card_error::CardError;
use crate::fluent::{FluentName, Named};

pub trait Ranked: {
    fn names(&self) -> Vec<&str>;

    // fn is_valid_char(c: char) -> bool;
}

pub struct Rank<RankType> where RankType: Ranked {
    weight: u32,
    prime: u32,
    name: FluentName,
    phantom_data: PhantomData<RankType>
}

struct Standard52Rank{}

impl Rank<Standard52Rank> {
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

impl<T: From<char> + Ranked> From<char> for GRank<T> {
    fn from(c: char) -> GRank<T> {
        match c {
            '2' => GRank::new(crate::cards::card::ranks::standard52_rank::Standard52Rank::TWO),
            '3' => GRank::new(crate::cards::card::ranks::standard52_rank::Standard52Rank::THREE),
            '4' => GRank::new(crate::cards::card::ranks::standard52_rank::Standard52Rank::FOUR),
            '5' => GRank::new(crate::cards::card::ranks::standard52_rank::Standard52Rank::FIVE),
            '6' => GRank::new(crate::cards::card::ranks::standard52_rank::Standard52Rank::SIX),
            '7' => GRank::new(crate::cards::card::ranks::standard52_rank::Standard52Rank::SEVEN),
            '8' => GRank::new(crate::cards::card::ranks::standard52_rank::Standard52Rank::EIGHT),
            '9' => GRank::new(crate::cards::card::ranks::standard52_rank::Standard52Rank::NINE),
            'T' | 't' | '0' => GRank::new(crate::cards::card::ranks::standard52_rank::Standard52Rank::TEN),
            'J' | 'j' => GRank::new(crate::cards::card::ranks::standard52_rank::Standard52Rank::JACK),
            'Q' | 'q' => GRank::new(crate::cards::card::ranks::standard52_rank::Standard52Rank::QUEEN),
            'K' | 'k' => GRank::new(crate::cards::card::ranks::standard52_rank::Standard52Rank::KING),
            'A' | 'a' => GRank::new(crate::cards::card::ranks::standard52_rank::Standard52Rank::ACE),
            _ => GRank::new(FluentName::BLANK),
        }
    }
}

// impl<T: Ranked> FromStr for GRank<T> {
//     type Err = CardError;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         if let Some(first_char) = s.chars().next() {
//             let rank = GRank::<T>::from(first_char);
//             if rank.name.is_blank() {
//                 Err(CardError::InvalidFluentRank(s.to_string()))
//             } else {
//                 Ok(rank)
//             }
//         } else {
//             Err(CardError::InvalidFluentRank(s.to_string()))
//         }
//     }
// }

impl Ranked for Standard52Rank {
    fn names() -> Vec<&'static str> {
        vec![
            crate::cards::card::ranks::standard52_rank::Standard52Rank::ACE,
            crate::cards::card::ranks::standard52_rank::Standard52Rank::KING,
            crate::cards::card::ranks::standard52_rank::Standard52Rank::QUEEN,
            crate::cards::card::ranks::standard52_rank::Standard52Rank::JACK,
            crate::cards::card::ranks::standard52_rank::Standard52Rank::TEN,
            crate::cards::card::ranks::standard52_rank::Standard52Rank::NINE,
            crate::cards::card::ranks::standard52_rank::Standard52Rank::EIGHT,
            crate::cards::card::ranks::standard52_rank::Standard52Rank::SEVEN,
            crate::cards::card::ranks::standard52_rank::Standard52Rank::SIX,
            crate::cards::card::ranks::standard52_rank::Standard52Rank::FIVE,
            crate::cards::card::ranks::standard52_rank::Standard52Rank::FOUR,
            crate::cards::card::ranks::standard52_rank::Standard52Rank::THREE,
            crate::cards::card::ranks::standard52_rank::Standard52Rank::TWO,
        ]
    }

    fn is_valid_char(c: char) -> bool {
        match c {
            '2'  |  '3'  |  '4'  |  '5'  |  '6'  |  '7'  |  '8'  |  '9'  |  'T'  |  't'  |  '0'  |  'J'  |  'j'  |  'Q'  |  'q'  |  'K'  |  'k'  |  'A'  |  'a' => true,
            _ => false,
        }
    }
}

pub struct GRank<Rank> {
    weight: u32,
    prime: u32,
    name: FluentName,
    phantom_data: PhantomData<Rank>
}


impl<T: From<char> + Ranked> FromStr for GRank<T> {
    type Err = CardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(first_char) = s.chars().next() {
            let rank = GRank::<T>::from(first_char);
            if rank.value().is_blank() {
                Err(CardError::InvalidFluentRank(s.to_string()))
            } else {
                Ok(rank)
            }
        } else {
            Err(CardError::InvalidFluentRank(s.to_string()))
        }
    }
}

impl<T: Ranked + From<char>> GRank<T> {
    fn new(name_str: &str) -> GRank<T> {
        let name = FluentName::new(name_str);

        GRank::<T> {
            weight: name.weight(),
            prime: name.prime(),
            name,
            phantom_data: PhantomData
        }
    }

    fn from_char(c: char) -> GRank<T> {
        GRank::<T>::from(T::from(c))
    }

    #[must_use]
    fn ranks() -> Vec<Self> {
        T::names().iter().map(|name| Self::new(name)).collect()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod rank__generic_tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn new() {
        let rank = GRank::<Standard52Rank>::new(Standard52Rank::ACE);
        assert_eq!(rank.name, FluentName::new(Standard52Rank::ACE));
        assert_eq!(rank.weight, 12);
        assert_eq!(rank.prime, 41);
    }
}
