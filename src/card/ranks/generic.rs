use std::convert::From;
use std::marker::PhantomData;
use std::str::FromStr;
use crate::card_error::CardError;
use crate::fluent::{FluentName, Named};
use crate::Rank;
use crate::traits::Ranked;

struct Standard52 {}

impl Ranked for Standard52 {
    fn chars() -> Vec<char> {
        vec!['2', '3', '4', '5', '6', '7', '8', '9', 'T', 't', '0', 'J', 'j', 'Q', 'q', 'K', 'k', 'A', 'a']
    }

    fn names() -> Vec<&'static str> {
        vec![
            Rank::<Standard52>::ACE,
            Rank::<Standard52>::KING,
            Rank::<Standard52>::QUEEN,
            Rank::<Standard52>::JACK,
            Rank::<Standard52>::TEN,
            Rank::<Standard52>::NINE,
            Rank::<Standard52>::EIGHT,
            Rank::<Standard52>::SEVEN,
            Rank::<Standard52>::SIX,
            Rank::<Standard52>::FIVE,
            Rank::<Standard52>::FOUR,
            Rank::<Standard52>::THREE,
            Rank::<Standard52>::TWO,
        ]
    }
}

impl<RankType: Ranked> From<char> for Rank<RankType> {
    fn from(value: char) -> Self {
        if !RankType::is_valid_char(&value) {
            return Rank::<RankType> {
                weight: 0,
                prime: 0,
                name: FluentName::default(),
                phantom_data: PhantomData
            }
        }
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
            'B' | 'b' => Rank::new(Rank::<RankType>::BIG),
            'L' | 'l' => Rank::new(Rank::<RankType>::LITTLE),
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
        let rank = Rank::<Standard52>::new(Rank::<Standard52>::ACE);

        assert_eq!(rank.name, FluentName::new(Rank::<Standard52>::ACE));
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
    fn from_char() {
        let rank = Rank::<Standard52>::from('A');

        assert_eq!(rank.name, FluentName::new(Rank::<Standard52>::ACE));
        assert_eq!(rank.weight, 12);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn standard52_names() {
        let names = Rank::<Standard52>::names();

        assert_eq!(names.len(), 13);
        assert_eq!(names[0], Rank::<Standard52>::ACE);
        assert_eq!(names[1], Rank::<Standard52>::KING);
        assert_eq!(names[2], Rank::<Standard52>::QUEEN);
        assert_eq!(names[3], Rank::<Standard52>::JACK);
        assert_eq!(names[4], Rank::<Standard52>::TEN);
        assert_eq!(names[5], Rank::<Standard52>::NINE);
        assert_eq!(names[6], Rank::<Standard52>::EIGHT);
        assert_eq!(names[7], Rank::<Standard52>::SEVEN);
        assert_eq!(names[8], Rank::<Standard52>::SIX);
        assert_eq!(names[9], Rank::<Standard52>::FIVE);
        assert_eq!(names[10], Rank::<Standard52>::FOUR);
        assert_eq!(names[11], Rank::<Standard52>::THREE);
        assert_eq!(names[12], Rank::<Standard52>::TWO);
    }
}
