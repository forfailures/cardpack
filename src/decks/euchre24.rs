use crate::decks::standard52::Standard52;
use crate::types::traits::{Decked, Ranked};

/// This deck represents the most common 24 card form of
/// [Euchre](https://en.wikipedia.org/wiki/Euchre) with
/// `A K Q J T 9` ranks.
///
/// This means that they are made up of the [Standard52]
/// implementation of the [Suited] trait that's declared in the
/// [Standard52] deck and the [Euchre24] implementation of the
/// [Ranked] trait.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Euchre24 {}

impl Decked<Euchre24, Standard52> for Euchre24 {}

impl Ranked for Euchre24 {
    fn rank_chars() -> Vec<char> {
        vec!['9', 'T', 't', '0', 'J', 'j', 'Q', 'q', 'K', 'k', 'A', 'a']
    }

    fn rank_names() -> Vec<&'static str> {
        vec![
            Standard52::ACE,
            Standard52::KING,
            Standard52::QUEEN,
            Standard52::JACK,
            Standard52::TEN,
            Standard52::NINE,
        ]
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod decks__euchre__tests {
    use super::*;
    use crate::localization::{FluentName, Named};
    use crate::types::rank::Rank;
    use std::str::FromStr;

    #[test]
    fn new() {
        let rank = Rank::<Euchre24>::new(Standard52::ACE);

        assert_eq!(rank.name, FluentName::new(Standard52::ACE));
        assert_eq!(rank.weight, 12);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn new_with_weight() {
        let rank = Rank::<Euchre24>::new_with_weight(Standard52::ACE, 13);

        assert_eq!(rank.name, FluentName::new(Standard52::ACE));
        assert_eq!(rank.weight, 13);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn update_weight() {
        let rank = Rank::<Euchre24>::new(Standard52::ACE);
        let updated_rank = rank.update_weight(14);

        assert_eq!(updated_rank.name, FluentName::new(Standard52::ACE));
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

        assert_eq!(rank.name, FluentName::new(Standard52::ACE));
        assert_eq!(rank.weight, 12);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn from_str() {
        let rank = Rank::<Euchre24>::from_str("A'").unwrap();

        assert_eq!(rank.name, FluentName::new(Standard52::ACE));
        assert_eq!(rank.weight, 12);
        assert_eq!(rank.prime, 41);
    }

    #[test]
    fn named__fluent_name() {
        let rank = Rank::<Euchre24>::new(Standard52::KING);

        assert_eq!(rank.fluent_name(), &FluentName::new(Standard52::KING));
    }

    #[test]
    fn named__fluent_name_string() {
        let rank = Rank::<Euchre24>::new(Standard52::QUEEN);

        assert_eq!(rank.fluent_name_string(), Standard52::QUEEN);
    }

    #[test]
    fn named__is_blank() {
        let rank = Rank::<Euchre24>::new(Standard52::ACE);

        assert!(!rank.is_blank());
    }

    #[test]
    fn ranked__names() {
        let names = Rank::<Euchre24>::rank_names();

        assert_eq!(names.len(), 6);
        assert_eq!(names[0], Standard52::ACE);
        assert_eq!(names[1], Standard52::KING);
        assert_eq!(names[2], Standard52::QUEEN);
        assert_eq!(names[3], Standard52::JACK);
        assert_eq!(names[4], Standard52::TEN);
        assert_eq!(names[5], Standard52::NINE);
    }

    #[test]
    fn pile__sort() {
        let deck = Euchre24::deck();
        let mut shuffled = deck.shuffle_default();

        shuffled.shuffle_in_place_default();
        shuffled.sort_in_place();

        assert_eq!(deck.to_string(), shuffled.to_string());
    }
}
