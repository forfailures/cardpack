use std::marker::PhantomData;
use std::str::FromStr;
use crate::card_error::CardError;
use crate::fluent::{FluentName, Named};

pub trait Ranked: {
    fn names() -> Vec<&'static str>;
}

struct Standard52Rank;

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

impl From<char> for GRank<Standard52Rank> {
    fn from(c: char) -> Self {
        match c {
            '2' => GRank::new(crate::cards::card::ranks::standard52_rank::Standard52Rank::TWO),
            '3' => GRank::new(crate::cards::card::ranks::standard52_rank::Standard52Rank::THREE),
            '4' => GRank::new(crate::cards::card::ranks::standard52_rank::Standard52Rank::FOUR),
            '5' => GRank::new(crate::cards::card::ranks::standard52_rank::Standard52Rank::FIVE),
            '6' => GRank::new(crate::cards::card::ranks::standard52_rank::Standard52Rank::SIX),
            '7' => GRank::new(crate::cards::card::ranks::standard52_rank::Standard52Rank::SEVEN),
            '8' => GRank::new(crate::cards::card::ranks::standard52_rank::Standard52Rank::EIGHT),
            '9' => GRank::new(crate::cards::card::ranks::standard52_rank::Standard52Rank::NINE),
            'T' | 't' | '0' => crate::cards::card::ranks::generic::GRank::new(crate::cards::card::ranks::standard52_rank::Standard52Rank::TEN),
            'J' | 'j' => GRank::new(crate::cards::card::ranks::standard52_rank::Standard52Rank::JACK),
            'Q' | 'q' => GRank::new(crate::cards::card::ranks::standard52_rank::Standard52Rank::QUEEN),
            'K' | 'k' => GRank::new(crate::cards::card::ranks::standard52_rank::Standard52Rank::KING),
            'A' | 'a' => GRank::new(crate::cards::card::ranks::standard52_rank::Standard52Rank::ACE),
            _ => GRank::new(FluentName::BLANK),
        }
    }
}

impl<T: Ranked> FromStr for GRank<T> {
    type Err = CardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(first_char) = s.chars().next() {
            let rank = GRank::<T>::from(first_char);
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
}

pub struct GRank<T : Ranked> {
    weight: u32,
    prime: u32,
    name: FluentName,
    phantom_data: PhantomData<T>
}

impl<T: Ranked> GRank<T>  {
    fn new(name_str: &str) -> GRank<T> {
        let name = FluentName::new(name_str);

        GRank::<T> {
            weight: name.weight(),
            prime: name.prime(),
            name,
            phantom_data: PhantomData
        }
    }

    #[must_use]
    fn ranks() -> Vec<Self> {
        T::names().iter().map(|name| Self::new(name)).collect()
    }
}