use std::convert::From;
use std::marker::PhantomData;
use std::str::FromStr;
use crate::card_error::CardError;
use crate::fluent::{FluentName, Named};

pub trait Ranked: {
    fn chars() -> Vec<char>;
    fn names() -> Vec<&'static str>;

    fn is_valid_char(c: &char) -> bool;
}

pub struct Rank<RankType> where RankType: Ranked {
    weight: u32,
    prime: u32,
    name: FluentName,
    phantom_data: PhantomData<RankType>
}

impl<RankType: Ranked> Ranked for Rank<RankType> {
    fn chars() -> Vec<char> {
        RankType::chars()
    }

    fn names() -> Vec<&'static str> {
        RankType::names()
    }

    fn is_valid_char(c: &char) -> bool {
        RankType::chars().contains(c)
    }
}

impl<RankType> Rank<RankType> where RankType: Ranked {
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

    fn new(name_str: &str) -> Rank<RankType> {
        let name = FluentName::new(name_str);

        Rank::<RankType> {
            weight: name.weight(),
            prime: name.prime(),
            name,
            phantom_data: PhantomData
        }
    }

    // fn from_char(c: char) -> Rank<RankType> {
    //     Rank::<RankType>::from(RankType::from(c))
    // }

    #[must_use]
    fn ranks(&self) -> Vec<Self> {
        RankType::names().iter().map(|name| Self::new(name)).collect()
    }
}

struct Standard52Rank{}

impl Ranked for Standard52Rank {
    fn chars() -> Vec<char> {
        vec!['2', '3', '4', '5', '6', '7', '8', '9', 'T', 't', '0', 'J', 'j', 'Q', 'q', 'K', 'k', 'A', 'a']
    }

    fn names() -> Vec<&'static str> {
        vec![
            Rank::<Standard52Rank>::ACE,
            Rank::<Standard52Rank>::KING,
            Rank::<Standard52Rank>::QUEEN,
            Rank::<Standard52Rank>::JACK,
            Rank::<Standard52Rank>::TEN,
            Rank::<Standard52Rank>::NINE,
            Rank::<Standard52Rank>::EIGHT,
            Rank::<Standard52Rank>::SEVEN,
            Rank::<Standard52Rank>::SIX,
            Rank::<Standard52Rank>::FIVE,
            Rank::<Standard52Rank>::FOUR,
            Rank::<Standard52Rank>::THREE,
            Rank::<Standard52Rank>::TWO,
        ]
    }

    fn is_valid_char(c: &char) -> bool {
        match c {
            '2'  |  '3'  |  '4'  |  '5'  |  '6'  |  '7'  |  '8'  |  '9'  |  'T'  |  't'  |  '0'  |  'J'  |  'j'  |  'Q'  |  'q'  |  'K'  |  'k'  |  'A'  |  'a' => true,
            _ => false,
        }
    }
}

impl<RankType: Ranked> From<char> for Rank<RankType> {
    fn from(value: char) -> Self {
        match value {
            '2' => Rank::new(Rank::<RankType>::TWO),
            '3' => Rank::new(Rank::<RankType>::THREE),
            '4' => Rank::new(Rank::<RankType>::FOUR),
            '5' => Rank::new(Rank::<RankType>::FIVE),
            '6' => Rank::new(Rank::<RankType>::SIX),
            '7' => Rank::new(Rank::<RankType>::SEVEN),
            '8' => Rank::new(Rank::<RankType>::EIGHT),
            '9' => Rank::new(Rank::<RankType>::NINE),
            'T' | 't' | '0' => Rank::new(Rank::<RankType>::TEN),
            'J' | 'j' => Rank::new(Rank::<RankType>::JACK),
            'Q' | 'q' => Rank::new(Rank::<RankType>::QUEEN),
            'K' | 'k' => Rank::new(Rank::<RankType>::KING),
            'A' | 'a' => Rank::new(Rank::<RankType>::ACE),
            _ => Rank::new(FluentName::BLANK),
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod rank__generic_tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn new() {
        let rank = Rank::<Standard52Rank>::new(Rank::<Standard52Rank>::ACE);

        assert_eq!(rank.name, FluentName::new(Rank::<Standard52Rank>::ACE));
        assert_eq!(rank.weight, 12);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn is_valid_char() {
        // let chars = Rank::chars();
    }

    #[test]
    fn names() {
        // let names = Rank::names();
    }

    #[test]
    fn standard52_names() {
        let names = Rank::<Standard52Rank>::names();

        assert_eq!(names.len(), 13);
        assert_eq!(names[0], Rank::<Standard52Rank>::ACE);
        assert_eq!(names[1], Rank::<Standard52Rank>::KING);
        assert_eq!(names[2], Rank::<Standard52Rank>::QUEEN);
        assert_eq!(names[3], Rank::<Standard52Rank>::JACK);
        assert_eq!(names[4], Rank::<Standard52Rank>::TEN);
        assert_eq!(names[5], Rank::<Standard52Rank>::NINE);
        assert_eq!(names[6], Rank::<Standard52Rank>::EIGHT);
        assert_eq!(names[7], Rank::<Standard52Rank>::SEVEN);
        assert_eq!(names[8], Rank::<Standard52Rank>::SIX);
        assert_eq!(names[9], Rank::<Standard52Rank>::FIVE);
        assert_eq!(names[10], Rank::<Standard52Rank>::FOUR);
        assert_eq!(names[11], Rank::<Standard52Rank>::THREE);
        assert_eq!(names[12], Rank::<Standard52Rank>::TWO);
    }
}
