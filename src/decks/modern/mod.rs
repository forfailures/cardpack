use crate::types::rank::Rank;
use crate::types::Ranked;

/// `Standard52` with Jokers.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Modern {}

impl Ranked for Modern {
    fn rank_chars() -> Vec<char> {
        vec![
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 't', '0', 'J', 'j', 'Q', 'q', 'K', 'k',
            'A', 'a', 'B', 'b', 'L', 'l',
        ]
    }

    fn rank_names() -> Vec<&'static str> {
        vec![
            Rank::<Modern>::BIG,
            Rank::<Modern>::LITTLE,
            Rank::<Modern>::ACE,
            Rank::<Modern>::KING,
            Rank::<Modern>::QUEEN,
            Rank::<Modern>::JACK,
            Rank::<Modern>::TEN,
            Rank::<Modern>::NINE,
            Rank::<Modern>::EIGHT,
            Rank::<Modern>::SEVEN,
            Rank::<Modern>::SIX,
            Rank::<Modern>::FIVE,
            Rank::<Modern>::FOUR,
            Rank::<Modern>::THREE,
            Rank::<Modern>::TWO,
        ]
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod decks__modern__tests {
    use super::*;
    use crate::localization::{FluentName, Named};
    use std::str::FromStr;

    #[test]
    fn new() {
        let rank = Rank::<Modern>::new(Rank::<Modern>::LITTLE);

        assert_eq!(rank.name, FluentName::new(Rank::<Modern>::LITTLE));
        assert_eq!(rank.weight, 13);
        assert_eq!(rank.prime, 43);
    }

    #[test]
    fn new_with_weight() {
        let rank = Rank::<Modern>::new_with_weight(Rank::<Modern>::BIG, 13);

        assert_eq!(rank.name, FluentName::new(Rank::<Modern>::BIG));
        assert_eq!(rank.weight, 13);
        assert_eq!(rank.prime, 47);
    }

    #[test]
    fn update_weight() {
        let rank = Rank::<Modern>::new(Rank::<Modern>::ACE);
        let updated_rank = rank.update_weight(14);

        assert_eq!(updated_rank.name, FluentName::new(Rank::<Modern>::ACE));
        assert_eq!(updated_rank.weight, 14);
        assert_eq!(updated_rank.prime, 41);
    }

    #[test]
    fn ranked__is_valid_char() {
        assert!(Rank::<Modern>::is_valid_rank_char(&'A'));
        assert!(!Rank::<Modern>::is_valid_rank_char(&'Z'));
    }

    #[test]
    fn from_char() {
        let rank = Rank::<Modern>::from('A');

        assert_eq!(rank.name, FluentName::new(Rank::<Modern>::ACE));
        assert_eq!(rank.weight, 12);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn from_str() {
        let rank = Rank::<Modern>::from_str("A'").unwrap();

        assert_eq!(rank.name, FluentName::new(Rank::<Modern>::ACE));
        assert_eq!(rank.weight, 12);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn named__fluent_name() {
        let rank = Rank::<Modern>::new(Rank::<Modern>::KING);

        assert_eq!(rank.fluent_name(), &FluentName::new(Rank::<Modern>::KING));
    }

    #[test]
    fn named__fluent_name_string() {
        let rank = Rank::<Modern>::new(Rank::<Modern>::QUEEN);

        assert_eq!(rank.fluent_name_string(), Rank::<Modern>::QUEEN);
    }

    #[test]
    fn named__is_blank() {
        let rank = Rank::<Modern>::new(Rank::<Modern>::ACE);

        assert!(!rank.is_blank());
    }

    #[test]
    fn ranked__names() {
        let names = Rank::<Modern>::rank_names();

        assert_eq!(names.len(), 15);
        assert_eq!(names[0], Rank::<Modern>::BIG);
        assert_eq!(names[1], Rank::<Modern>::LITTLE);
        assert_eq!(names[2], Rank::<Modern>::ACE);
        assert_eq!(names[3], Rank::<Modern>::KING);
        assert_eq!(names[4], Rank::<Modern>::QUEEN);
        assert_eq!(names[5], Rank::<Modern>::JACK);
        assert_eq!(names[6], Rank::<Modern>::TEN);
        assert_eq!(names[7], Rank::<Modern>::NINE);
        assert_eq!(names[8], Rank::<Modern>::EIGHT);
        assert_eq!(names[9], Rank::<Modern>::SEVEN);
        assert_eq!(names[10], Rank::<Modern>::SIX);
        assert_eq!(names[11], Rank::<Modern>::FIVE);
        assert_eq!(names[12], Rank::<Modern>::FOUR);
        assert_eq!(names[13], Rank::<Modern>::THREE);
        assert_eq!(names[14], Rank::<Modern>::TWO);
    }
}
