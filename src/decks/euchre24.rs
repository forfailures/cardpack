use crate::types::rank::Rank;
use crate::types::suit::Suit;
use crate::types::traits::{Decked, Ranked, Suited};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Euchre24 {}

impl Decked<Euchre24, Euchre24> for Euchre24 {}

impl Ranked for Euchre24 {
    fn rank_chars() -> Vec<char> {
        vec!['9', 'T', 't', '0', 'J', 'j', 'Q', 'q', 'K', 'k', 'A', 'a']
    }

    fn rank_names() -> Vec<&'static str> {
        vec![
            Rank::<Euchre24>::ACE,
            Rank::<Euchre24>::KING,
            Rank::<Euchre24>::QUEEN,
            Rank::<Euchre24>::JACK,
            Rank::<Euchre24>::TEN,
            Rank::<Euchre24>::NINE,
        ]
    }
}

impl Suited for Euchre24 {
    fn suit_chars() -> Vec<char> {
        vec![
            '♤', '♠', 'S', 's', '♡', '♥', 'H', 'h', '♢', '♦', 'D', 'd', '♧', '♣', 'C', 'c',
        ]
    }

    fn suit_names() -> Vec<&'static str> {
        vec![
            Suit::<Euchre24>::SPADES,
            Suit::<Euchre24>::HEARTS,
            Suit::<Euchre24>::DIAMONDS,
            Suit::<Euchre24>::CLUBS,
        ]
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod decks__euchre__tests {
    use super::*;
    use crate::localization::{FluentName, Named};
    use std::str::FromStr;

    #[test]
    fn new() {
        let rank = Rank::<Euchre24>::new(Rank::<Euchre24>::ACE);

        assert_eq!(rank.name, FluentName::new(Rank::<Euchre24>::ACE));
        assert_eq!(rank.weight, 12);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn new_with_weight() {
        let rank = Rank::<Euchre24>::new_with_weight(Rank::<Euchre24>::ACE, 13);

        assert_eq!(rank.name, FluentName::new(Rank::<Euchre24>::ACE));
        assert_eq!(rank.weight, 13);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn update_weight() {
        let rank = Rank::<Euchre24>::new(Rank::<Euchre24>::ACE);
        let updated_rank = rank.update_weight(14);

        assert_eq!(updated_rank.name, FluentName::new(Rank::<Euchre24>::ACE));
        assert_eq!(updated_rank.weight, 14);
        assert_eq!(updated_rank.prime, 41);
    }

    #[test]
    fn ranked__is_valid_char() {
        assert!(Rank::<Euchre24>::is_valid_rank_char(&'A'));
        assert!(!Rank::<Euchre24>::is_valid_rank_char(&'Z'));
    }

    #[test]
    fn from_char() {
        let rank = Rank::<Euchre24>::from('A');

        assert_eq!(rank.name, FluentName::new(Rank::<Euchre24>::ACE));
        assert_eq!(rank.weight, 12);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn from_str() {
        let rank = Rank::<Euchre24>::from_str("A'").unwrap();

        assert_eq!(rank.name, FluentName::new(Rank::<Euchre24>::ACE));
        assert_eq!(rank.weight, 12);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn named__fluent_name() {
        let rank = Rank::<Euchre24>::new(Rank::<Euchre24>::KING);

        assert_eq!(rank.fluent_name(), &FluentName::new(Rank::<Euchre24>::KING));
    }

    #[test]
    fn named__fluent_name_string() {
        let rank = Rank::<Euchre24>::new(Rank::<Euchre24>::QUEEN);

        assert_eq!(rank.fluent_name_string(), Rank::<Euchre24>::QUEEN);
    }

    #[test]
    fn named__is_blank() {
        let rank = Rank::<Euchre24>::new(Rank::<Euchre24>::ACE);

        assert!(!rank.is_blank());
    }

    #[test]
    fn ranked__names() {
        let names = Rank::<Euchre24>::rank_names();

        assert_eq!(names.len(), 6);
        assert_eq!(names[0], Rank::<Euchre24>::ACE);
        assert_eq!(names[1], Rank::<Euchre24>::KING);
        assert_eq!(names[2], Rank::<Euchre24>::QUEEN);
        assert_eq!(names[3], Rank::<Euchre24>::JACK);
        assert_eq!(names[4], Rank::<Euchre24>::TEN);
        assert_eq!(names[5], Rank::<Euchre24>::NINE);
    }
}
