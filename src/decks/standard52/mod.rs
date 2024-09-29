use crate::types::rank::Rank;
use crate::types::suit::Suit;
use crate::types::{Ranked, Suited};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Standard52 {}

impl Ranked for Standard52 {
    fn rank_chars() -> Vec<char> {
        vec![
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 't', '0', 'J', 'j', 'Q', 'q', 'K', 'k',
            'A', 'a',
        ]
    }

    fn rank_names() -> Vec<&'static str> {
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

impl Suited for Standard52 {
    fn suit_chars() -> Vec<char> {
        vec![
            '♤', '♠', 'S', 's', '♡', '♥', 'H', 'h', '♢', '♦', 'D', 'd', '♧', '♣', 'C', 'c',
        ]
    }

    fn suit_names() -> Vec<&'static str> {
        vec![
            Suit::<Standard52>::SPADES,
            Suit::<Standard52>::HEARTS,
            Suit::<Standard52>::DIAMONDS,
            Suit::<Standard52>::CLUBS,
        ]
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod decks__standard52__tests {
    use super::*;
    use crate::localization::{FluentName, Named};
    use crate::types::card_error::CardError;
    use rstest::rstest;
    use std::str::FromStr;

    #[test]
    fn new() {
        let rank = Rank::<Standard52>::new(Rank::<Standard52>::ACE);

        assert_eq!(rank.name, FluentName::new(Rank::<Standard52>::ACE));
        assert_eq!(rank.weight, 12);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn new_with_weight() {
        let rank = Rank::<Standard52>::new_with_weight(Rank::<Standard52>::ACE, 13);

        assert_eq!(rank.name, FluentName::new(Rank::<Standard52>::ACE));
        assert_eq!(rank.weight, 13);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn update_weight() {
        let rank = Rank::<Standard52>::new(Rank::<Standard52>::ACE);
        let updated_rank = rank.update_weight(14);

        assert_eq!(updated_rank.name, FluentName::new(Rank::<Standard52>::ACE));
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
    fn from_char() {
        let rank = Rank::<Standard52>::from('A');

        assert_eq!(rank.name, FluentName::new(Rank::<Standard52>::ACE));
        assert_eq!(rank.weight, 12);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn from_str() {
        let rank = Rank::<Standard52>::from_str("A'").unwrap();

        assert_eq!(rank.name, FluentName::new(Rank::<Standard52>::ACE));
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

    #[test]
    fn ranked__named__fluent_name() {
        let rank = Rank::<Standard52>::new(Rank::<Standard52>::KING);

        assert_eq!(
            rank.fluent_name(),
            &FluentName::new(Rank::<Standard52>::KING)
        );
    }

    #[test]
    fn ranked__named__fluent_name_string() {
        let rank = Rank::<Standard52>::new(Rank::<Standard52>::QUEEN);

        assert_eq!(rank.fluent_name_string(), Rank::<Standard52>::QUEEN);
    }

    #[test]
    fn ranked__named__is_blank() {
        let rank = Rank::<Standard52>::new(Rank::<Standard52>::ACE);

        assert!(!rank.is_blank());
    }

    #[test]
    fn ranked__ranked__names() {
        let names = Rank::<Standard52>::rank_names();

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

    #[test]
    fn ranked__is_valid_char() {
        assert!(Rank::<Standard52>::is_valid_rank_char(&'A'));
        assert!(!Rank::<Standard52>::is_valid_rank_char(&'Z'));
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
            Suit::<Standard52>::SPADES,
            Suit::<Standard52>::HEARTS,
            Suit::<Standard52>::DIAMONDS,
            Suit::<Standard52>::CLUBS,
        ];

        let names = Suit::<Standard52>::suit_names();

        assert_eq!(names, expected);
    }

    #[rstest]
    #[case('♠', Suit::<Standard52>::SPADES)]
    #[case('♤', Suit::<Standard52>::SPADES)]
    #[case('S', Suit::<Standard52>::SPADES)]
    #[case('s', Suit::<Standard52>::SPADES)]
    #[case('♥', Suit::<Standard52>::HEARTS)]
    #[case('♡', Suit::<Standard52>::HEARTS)]
    #[case('H', Suit::<Standard52>::HEARTS)]
    #[case('h', Suit::<Standard52>::HEARTS)]
    #[case('♦', Suit::<Standard52>::DIAMONDS)]
    #[case('♢', Suit::<Standard52>::DIAMONDS)]
    #[case('D', Suit::<Standard52>::DIAMONDS)]
    #[case('d', Suit::<Standard52>::DIAMONDS)]
    #[case('♣', Suit::<Standard52>::CLUBS)]
    #[case('♧', Suit::<Standard52>::CLUBS)]
    #[case('C', Suit::<Standard52>::CLUBS)]
    #[case('c', Suit::<Standard52>::CLUBS)]
    #[case('🃟', FluentName::BLANK)]
    #[case('T', FluentName::BLANK)]
    #[case('t', FluentName::BLANK)]
    #[case(' ', FluentName::BLANK)]
    #[case('F', FluentName::BLANK)]
    fn from__char(#[case] input: char, #[case] expected: &str) {
        assert_eq!(
            Suit::<Standard52>::new(expected),
            Suit::<Standard52>::from(input)
        );
    }
}
