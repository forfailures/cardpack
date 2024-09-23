use crate::traits::Ranked;
use crate::Rank;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Standard52 {}

impl Ranked for Standard52 {
    fn chars() -> Vec<char> {
        vec![
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 't', '0', 'J', 'j', 'Q', 'q', 'K', 'k',
            'A', 'a',
        ]
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

#[cfg(test)]
#[allow(non_snake_case)]
mod decks_standard52_tests {
    use super::*;
    use crate::fluent::{FluentName, Named};
    use rstest::rstest;

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
    fn is_valid_char() {
        assert!(Rank::<Standard52>::is_valid_char(&'A'));
        assert!(!Rank::<Standard52>::is_valid_char(&'Z'));
    }

    #[test]
    fn from_char() {
        let rank = Rank::<Standard52>::from('A');

        assert_eq!(rank.name, FluentName::new(Rank::<Standard52>::ACE));
        assert_eq!(rank.weight, 12);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn named__fluent_name() {
        let rank = Rank::<Standard52>::new(Rank::<Standard52>::KING);

        assert_eq!(rank.fluent_name(), &FluentName::new(Rank::<Standard52>::KING));
    }

    #[test]
    fn named__fluent_name_string() {
        let rank = Rank::<Standard52>::new(Rank::<Standard52>::QUEEN);

        assert_eq!(rank.fluent_name_string(), Rank::<Standard52>::QUEEN);
    }

    #[test]
    fn named__is_blank() {
        let rank = Rank::<Standard52>::new(Rank::<Standard52>::ACE);

        assert!(!rank.is_blank());
    }

    #[test]
    fn ranked__names() {
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
