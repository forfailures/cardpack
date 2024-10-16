use crate::decks::standard52::Standard52;
use crate::types::traits::Ranked;

/// `Standard52` with Jokers.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Modern {}

impl Modern {
    // Jokers Ranks
    pub const BIG: &'static str = "big";
    pub const LITTLE: &'static str = "little";

    // Rank
    pub const TRUMP: &'static str = "trump";
}

impl Ranked for Modern {
    fn rank_chars() -> Vec<char> {
        vec![
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 't', '0', 'J', 'j', 'Q', 'q', 'K', 'k',
            'A', 'a', 'B', 'b', 'L', 'l',
        ]
    }

    fn rank_names() -> Vec<&'static str> {
        vec![
            Modern::BIG,
            Modern::LITTLE,
            Standard52::ACE,
            Standard52::KING,
            Standard52::QUEEN,
            Standard52::JACK,
            Standard52::TEN,
            Standard52::NINE,
            Standard52::EIGHT,
            Standard52::SEVEN,
            Standard52::SIX,
            Standard52::FIVE,
            Standard52::FOUR,
            Standard52::THREE,
            Standard52::TWO,
        ]
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod decks__modern__tests {
    use super::*;
    use crate::localization::{FluentName, Named};
    use crate::types::rank::Rank;
    use std::str::FromStr;

    #[test]
    fn new() {
        let rank = Rank::<Modern>::new(Modern::LITTLE);

        assert_eq!(rank.name, FluentName::new(Modern::LITTLE));
        assert_eq!(rank.weight, 13);
        assert_eq!(rank.prime, 43);
    }

    #[test]
    fn new_with_weight() {
        let rank = Rank::<Modern>::new_with_weight(Modern::BIG, 13);

        assert_eq!(rank.name, FluentName::new(Modern::BIG));
        assert_eq!(rank.weight, 13);
        assert_eq!(rank.prime, 47);
    }

    #[test]
    fn update_weight() {
        let rank = Rank::<Modern>::new(Standard52::ACE);
        let updated_rank = rank.update_weight(14);

        assert_eq!(updated_rank.name, FluentName::new(Standard52::ACE));
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

        assert_eq!(rank.name, FluentName::new(Standard52::ACE));
        assert_eq!(rank.weight, 12);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn from_str() {
        let rank = Rank::<Modern>::from_str("A'").unwrap();

        assert_eq!(rank.name, FluentName::new(Standard52::ACE));
        assert_eq!(rank.weight, 12);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn named__fluent_name() {
        let rank = Rank::<Modern>::new(Standard52::KING);

        assert_eq!(rank.fluent_name(), &FluentName::new(Standard52::KING));
    }

    #[test]
    fn named__fluent_name_string() {
        let rank = Rank::<Modern>::new(Standard52::QUEEN);

        assert_eq!(rank.fluent_name_string(), Standard52::QUEEN);
    }

    #[test]
    fn named__is_blank() {
        let rank = Rank::<Modern>::new(Standard52::ACE);

        assert!(!rank.is_blank());
    }

    #[test]
    fn ranked__names() {
        let names = Rank::<Modern>::rank_names();

        assert_eq!(names.len(), 15);
        assert_eq!(names[0], Modern::BIG);
        assert_eq!(names[1], Modern::LITTLE);
        assert_eq!(names[2], Standard52::ACE);
        assert_eq!(names[3], Standard52::KING);
        assert_eq!(names[4], Standard52::QUEEN);
        assert_eq!(names[5], Standard52::JACK);
        assert_eq!(names[6], Standard52::TEN);
        assert_eq!(names[7], Standard52::NINE);
        assert_eq!(names[8], Standard52::EIGHT);
        assert_eq!(names[9], Standard52::SEVEN);
        assert_eq!(names[10], Standard52::SIX);
        assert_eq!(names[11], Standard52::FIVE);
        assert_eq!(names[12], Standard52::FOUR);
        assert_eq!(names[13], Standard52::THREE);
        assert_eq!(names[14], Standard52::TWO);
    }
}
